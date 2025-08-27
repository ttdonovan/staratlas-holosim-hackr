use anyhow::Result;
use tracing::{error, info, level_filters::LevelFilter};
use tracing_subscriber::EnvFilter;

mod config;
mod database;
mod discriminator;
mod parser;

use config::Config;
use database::Database;
use parser::AccountParser;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_target(false)
        .with_thread_ids(true)
        .init();

    info!("🌟 Starting hackr-saproc Star Atlas account processor");

    // Load configuration
    let config = Config::from_env_and_args()?;

    info!("Configuration loaded:");
    info!("  Database: {}", config.database_url);
    if let Some(ref program_id) = config.args.program_id {
        info!("  Filter program: {}", program_id);
    }
    info!("  Output format: {}", config.args.output);
    if let Some(limit) = config.args.limit {
        info!("  Limit: {} accounts", limit);
    }

    // Connect to database
    info!("🗄️ Connecting to database...");
    let db = Database::new(&config.database_url).await?;
    info!("✅ Database connected");

    if config.args.stats_only {
        show_stats(&db).await?;
        return Ok(());
    }

    // Load accounts
    let accounts = if let Some(ref program_id_str) = config.args.program_id {
        info!("📥 Loading accounts for program: {}", program_id_str);
        db.get_accounts_by_program(program_id_str).await?
    } else {
        info!("📥 Loading all accounts...");
        db.get_all_accounts().await?
    };

    let total_accounts = accounts.len();
    let accounts_to_process = if let Some(limit) = config.args.limit {
        accounts.into_iter().take(limit).collect()
    } else {
        accounts
    };

    info!(
        "Found {} accounts (processing {})",
        total_accounts,
        accounts_to_process.len()
    );

    // Parse accounts
    info!("🔍 Parsing account data...");
    let parser = AccountParser::new();

    let mut parsed_count = 0;
    let mut error_count = 0;
    let mut type_counts = std::collections::HashMap::new();

    for account in accounts_to_process {
        match parser.parse_account(&account) {
            Ok(parsed_account) => {
                parsed_count += 1;
                *type_counts
                    .entry(parsed_account.account_type.clone())
                    .or_insert(0) += 1;

                match config.args.output.as_str() {
                    "json" => {
                        println!("{}", serde_json::to_string_pretty(&parsed_account)?);
                    }
                    "detailed" => {
                        println!(
                            "Account: {} ({})",
                            parsed_account.account_pubkey, parsed_account.account_type
                        );
                        println!("  Program: {}", parsed_account.program_id);
                        println!("  Lamports: {}", parsed_account.lamports);
                        println!("  Data length: {} bytes", parsed_account.raw_data_length);
                        println!(
                            "  Parsed: {}",
                            serde_json::to_string_pretty(&parsed_account.parsed_data)?
                        );
                        println!();
                    }
                    "summary" | _ => {
                        if parsed_count % 1000 == 0 {
                            info!("Processed {} accounts...", parsed_count);
                        }
                    }
                }
            }
            Err(e) => {
                error_count += 1;
                error!("Failed to parse account {}: {}", account.account_pubkey, e);
            }
        }
    }

    // Summary
    info!("✅ Processing complete:");
    info!("  Total processed: {}", parsed_count);
    info!("  Errors: {}", error_count);
    info!("  Account types found:");

    for (account_type, count) in type_counts {
        info!("    {}: {} accounts", account_type, count);
    }

    Ok(())
}

async fn show_stats(db: &Database) -> Result<()> {
    info!("📊 Database statistics:");

    let stats = db.get_account_stats().await?;
    let mut total_accounts = 0;

    for (program_id, count) in stats {
        info!("  {}: {} accounts", program_id, count);
        total_accounts += count;
    }

    info!("  Total: {} accounts", total_accounts);

    Ok(())
}
