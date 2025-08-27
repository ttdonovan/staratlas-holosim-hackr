use actix_web::{App, HttpResponse, HttpServer, Result as ActixResult, web};
use serde_json::json;
use std::sync::{Arc, Mutex};
use tracing::{error, info, level_filters::LevelFilter};
use tracing_subscriber::EnvFilter;

mod config;
mod database;
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
    let config = Config::from_env_and_args()?;
    let program_ids = config.program_ids()?;

    info!("Configuration loaded:");
    info!("  RPC URL: {}", config.rpc_url);
    info!("  WebSocket URL: {}", config.rpc_ws_url);
    info!("  Database enabled: {}", config.database_url.is_some());
    if let Some(ref db_url) = config.database_url {
        info!("  Database URL: {}", db_url);
    }
    info!("  Programs to monitor: {}", program_ids.len());
    for program_id in &program_ids {
        info!("    - {}", program_id);
    }

    // Initialize database if enabled
    #[cfg(feature = "database")]
    let _database = if let Some(ref database_url) = config.database_url {
        info!("🗄️ Initializing database...");
        let db = database::db::Database::new(database_url).await?;
        info!("✅ Database initialized successfully");

        // Dump accounts if requested
        if config.args.dump_accounts {
            info!("📥 Dumping existing accounts for all monitored programs...");
            if let Err(e) = dump_program_accounts(&config.rpc_url, &program_ids, &db).await {
                error!("Failed to dump accounts: {}", e);
            } else {
                info!("✅ Account dump completed");
            }
        }

        Some(db)
    } else {
        None
    };

    #[cfg(not(feature = "database"))]
    let database: Option<()> = if config.database_url.is_some() {
        error!("❌ Database support not compiled. Rebuild with --features database");
        return Err(anyhow::anyhow!("Database support not available"));
    } else {
        None
    };

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

    let bind_address = format!("127.0.0.1:{}", config.args.port);
    info!("Starting HTTP server on {}", bind_address);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server_processor_state.clone()))
            .route("/health", web::get().to(health_check))
            .route("/stats", web::get().to(get_stats))
    })
    .bind(&bind_address)?
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

#[cfg(feature = "database")]
async fn dump_program_accounts(
    rpc_url: &str,
    program_ids: &[solana_sdk::pubkey::Pubkey],
    database: &database::db::Database,
) -> anyhow::Result<()> {
    use chrono::Utc;
    use database::db::AccountData;
    use solana_client::nonblocking::rpc_client::RpcClient;
    use uuid::Uuid;

    let client = RpcClient::new(rpc_url.to_string());

    for program_id in program_ids {
        info!("📥 Fetching accounts for program: {}", program_id);

        let accounts = client.get_program_accounts(program_id).await?;
        info!(
            "Found {} accounts for program {}",
            accounts.len(),
            program_id
        );

        let account_count = accounts.len();

        // Convert accounts to AccountData and batch insert
        let account_data_vec: Vec<AccountData> = accounts
            .into_iter()
            .map(|(account_pubkey, account)| AccountData {
                id: Uuid::new_v4().to_string(),
                program_id: program_id.to_string(),
                account_pubkey: account_pubkey.to_string(),
                lamports: account.lamports,
                data: account.data,
                owner: account.owner.to_string(),
                executable: account.executable,
                rent_epoch: account.rent_epoch,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            })
            .collect();

        if let Err(e) = database.batch_upsert_accounts(&account_data_vec).await {
            error!(
                "Failed to batch insert accounts for program {}: {}",
                program_id, e
            );
        }

        info!(
            "✅ Completed dumping {} accounts for program {}",
            account_count, program_id
        );
    }

    Ok(())
}
