use actix_web::{App, HttpResponse, HttpServer, Result as ActixResult, web};
use serde_json::json;
use std::sync::{Arc, Mutex};
use tracing::{error, info, level_filters::LevelFilter};
use tracing_subscriber::EnvFilter;

mod config;
mod processor;
mod solana_monitor;

use config::Config;
use processor::LiteProcessor;
use solana_monitor::SolanaMonitor;

type ProcessorState = Arc<Mutex<Option<LiteProcessor>>>;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_target(false)
        .with_thread_ids(true)
        .init();

    info!("Starting hackr-ixproc lite instruction processor");

    // Load configuration
    let config = Config::from_env()?;
    let program_ids = config.program_ids()?;

    info!("Configuration loaded:");
    info!("  RPC URL: {}", config.rpc_url);
    info!("  WebSocket URL: {}", config.rpc_ws_url);
    info!("  Programs to monitor: {}", program_ids.len());
    for program_id in &program_ids {
        info!("    - {}", program_id);
    }

    // Create shared processor state
    let processor_state: ProcessorState = Arc::new(Mutex::new(None));
    let processor_state_clone = processor_state.clone();

    // Start RPC monitoring in background task
    let rpc_url = config.rpc_url.clone();
    let ws_url = config.rpc_ws_url.clone();
    let programs = program_ids.clone();

    tokio::spawn(async move {
        let processor = LiteProcessor::new(programs.clone());

        // Store processor in shared state before starting monitor
        {
            let mut state = processor_state_clone.lock().unwrap();
            *state = Some(processor);
        }

        let monitor = SolanaMonitor::new(rpc_url, ws_url, programs);

        // Get processor from shared state
        let processor = {
            let mut state = processor_state_clone.lock().unwrap();
            state.take().unwrap()
        };

        if let Err(e) = monitor.start_monitoring(processor).await {
            error!("RPC monitoring failed: {}", e);
        }
    });

    // Start HTTP server
    let server_processor_state = processor_state.clone();

    info!("Starting HTTP server on 127.0.0.1:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server_processor_state.clone()))
            .route("/health", web::get().to(health_check))
            .route("/stats", web::get().to(get_stats))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}

async fn health_check() -> ActixResult<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "status": "healthy",
        "service": "hackr-ixproc",
        "version": "0.0.1"
    })))
}

async fn get_stats(processor_state: web::Data<ProcessorState>) -> ActixResult<HttpResponse> {
    let stats = {
        let state = processor_state.lock().unwrap();
        if let Some(ref processor) = *state {
            let stats_map = processor.get_stats();
            let mut stats = Vec::new();

            for (program_id, count) in stats_map {
                stats.push(json!({
                    "program_id": program_id.to_string(),
                    "instruction_count": count
                }));
            }

            stats
        } else {
            Vec::new()
        }
    };

    Ok(HttpResponse::Ok().json(json!({
        "stats": stats,
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}
