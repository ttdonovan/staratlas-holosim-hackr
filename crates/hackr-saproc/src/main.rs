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
    if config.args.write {
        info!("  Write mode: ENABLED (will save parsed accounts)");
        info!("  Batch size: {} accounts", config.args.batch_size);
    }

    // Connect to database
    info!("🗄️ Connecting to database...");
    let db = Database::new(&config.database_url, config.args.write).await?;
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

    // Collections for batching
    let mut holosim_batch = Vec::new();
    let mut player_profile_batch = Vec::new();
    let mut profile_faction_batch = Vec::new();

    for account in accounts_to_process {
        match parser.parse_account(&account) {
            Ok((parsed_account, db_account)) => {
                parsed_count += 1;
                *type_counts
                    .entry(parsed_account.account_type.clone())
                    .or_insert(0) += 1;

                // Collect accounts for batch processing
                if config.args.write {
                    if let Some(db_account) = db_account {
                        match account.program_id.as_str() {
                            "SAgeTraQfBMdvGVDJYoEvjnbq5szW7RJPi6obDTDQUF" => {
                                holosim_batch.push(db_account);
                            }
                            "PprofUW1pURCnMW2si88GWPXEEK3Bvh9Tksy8WtnoYJ" => {
                                player_profile_batch.push(db_account);
                            }
                            "pFACzkX2eSpAjDyEohD6i3VRJvREtH9ynbtM1DwVFsj" => {
                                profile_faction_batch.push(db_account);
                            }
                            _ => {}
                        }

                        // Flush batches when they reach the configured size
                        if holosim_batch.len() >= config.args.batch_size {
                            info!(
                                "💾 Writing batch of {} Holosim accounts...",
                                holosim_batch.len()
                            );
                            db.batch_upsert_holosim_accounts(&holosim_batch).await?;
                            holosim_batch.clear();
                        }
                        if player_profile_batch.len() >= config.args.batch_size {
                            info!(
                                "💾 Writing batch of {} Player Profile accounts...",
                                player_profile_batch.len()
                            );
                            db.batch_upsert_player_profile_accounts(&player_profile_batch)
                                .await?;
                            player_profile_batch.clear();
                        }
                        if profile_faction_batch.len() >= config.args.batch_size {
                            info!(
                                "💾 Writing batch of {} Profile Faction accounts...",
                                profile_faction_batch.len()
                            );
                            db.batch_upsert_profile_faction_accounts(&profile_faction_batch)
                                .await?;
                            profile_faction_batch.clear();
                        }
                    }
                }

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

    // Write any remaining batches
    if config.args.write {
        if !holosim_batch.is_empty() {
            info!(
                "💾 Writing final batch of {} Holosim accounts...",
                holosim_batch.len()
            );
            db.batch_upsert_holosim_accounts(&holosim_batch).await?;
        }
        if !player_profile_batch.is_empty() {
            info!(
                "💾 Writing final batch of {} Player Profile accounts...",
                player_profile_batch.len()
            );
            db.batch_upsert_player_profile_accounts(&player_profile_batch)
                .await?;
        }
        if !profile_faction_batch.is_empty() {
            info!(
                "💾 Writing final batch of {} Profile Faction accounts...",
                profile_faction_batch.len()
            );
            db.batch_upsert_profile_faction_accounts(&profile_faction_batch)
                .await?;
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

    info!("Raw accounts by program:");
    let stats = db.get_account_stats().await?;
    let mut total_accounts = 0;

    for (program_id, count) in stats {
        info!("  {}: {} accounts", program_id, count);
        total_accounts += count;
    }

    info!("  Total: {} accounts", total_accounts);

    // Show parsed account stats
    info!("\nParsed accounts:");

    let holosim_stats = db.get_holosim_account_stats().await?;
    if !holosim_stats.is_empty() {
        info!("  Holosim accounts:");
        for (account_type, count) in holosim_stats {
            info!("    {}: {}", account_type, count);
        }
    }

    let player_profile_stats = db.get_player_profile_account_stats().await?;
    if !player_profile_stats.is_empty() {
        info!("  Player Profile accounts:");
        for (account_type, count) in player_profile_stats {
            info!("    {}: {}", account_type, count);
        }
    }

    let profile_faction_stats = db.get_profile_faction_account_stats().await?;
    if !profile_faction_stats.is_empty() {
        info!("  Profile Faction accounts:");
        for (account_type, count) in profile_faction_stats {
            info!("    {}: {}", account_type, count);
        }
    }

    Ok(())
}
