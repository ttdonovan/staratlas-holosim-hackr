use anyhow::Result;
use serde_json::Value;
use sqlx::sqlite::SqlitePool;
use std::fs;
use std::path::Path;
use tracing::{info, warn, error};
use ui_holosim::{GameUI, GameStateUI};

mod config;
mod game_balance;

use config::{Config, OutputFormat};
use game_balance::GameBalance;

#[derive(sqlx::FromRow)]
struct HolosimAccount {
    pub account_pubkey: String,
    pub account_type: String,
    pub parsed_data: Value,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "hackr_gamedata=info".into()),
        )
        .init();
    
    info!("🎮 Star Atlas Game Data Extractor starting...");
    
    // Load configuration
    let config = Config::from_env_and_args()?;
    
    // Create output directory
    fs::create_dir_all(&config.output_dir)?;
    
    // Connect to database
    info!("🗄️  Connecting to database: {}", config.database_url);
    let database_url = format!("sqlite:{}", config.database_url);
    let pool = SqlitePool::connect(&database_url).await?;
    
    // Query for Game accounts
    let game_query = if let Some(ref pubkey) = config.args.game_pubkey {
        info!("🎯 Fetching specific Game account: {}", pubkey);
        sqlx::query_as::<_, HolosimAccount>(
            "SELECT account_pubkey, account_type, parsed_data 
             FROM holosim_accounts 
             WHERE account_type = 'Game' AND account_pubkey = ?"
        )
        .bind(pubkey)
    } else {
        info!("🎯 Fetching all Game accounts");
        sqlx::query_as::<_, HolosimAccount>(
            "SELECT account_pubkey, account_type, parsed_data 
             FROM holosim_accounts 
             WHERE account_type = 'Game'"
        )
    };
    
    let game_accounts = game_query.fetch_all(&pool).await?;
    
    if game_accounts.is_empty() {
        error!("❌ No Game accounts found in database");
        return Err(anyhow::anyhow!("No Game accounts found"));
    }
    
    info!("📊 Found {} Game account(s)", game_accounts.len());
    
    // Process each Game account
    for game_account in game_accounts {
        info!("🎮 Processing Game account: {}", game_account.account_pubkey);
        
        // Convert JSON Value to GameUI
        let game_ui: GameUI = serde_json::from_value(game_account.parsed_data)?;
        
        info!("  Version: {}", game_ui.version);
        info!("  Update ID: {}", game_ui.update_id);
        info!("  Profile: {}", game_ui.profile);
        info!("  GameState: {}", game_ui.game_state);
        
        // Optionally fetch GameState account
        let game_state_ui = if config.args.include_game_state && !game_ui.game_state.is_empty() {
            info!("🎯 Fetching GameState account: {}", game_ui.game_state);
            
            let game_state_result = sqlx::query_as::<_, HolosimAccount>(
                "SELECT account_pubkey, account_type, parsed_data 
                 FROM holosim_accounts 
                 WHERE account_type = 'GameState' AND account_pubkey = ?"
            )
            .bind(&game_ui.game_state)
            .fetch_optional(&pool)
            .await?;
            
            match game_state_result {
                Some(game_state_account) => {
                    match serde_json::from_value::<GameStateUI>(game_state_account.parsed_data) {
                        Ok(game_state) => {
                            info!("✅ Successfully loaded GameState account");
                            info!("  Version: {}", game_state.version);
                            info!("  Update ID: {}", game_state.update_id);
                            Some(game_state)
                        }
                        Err(e) => {
                            warn!("⚠️  Failed to parse GameState data: {}", e);
                            None
                        }
                    }
                }
                None => {
                    warn!("⚠️  GameState account not found in database");
                    None
                }
            }
        } else {
            info!("⏭️  Skipping GameState fetch");
            None
        };
        
        // Create game balance structure
        let game_balance = GameBalance::from_ui(&game_account.account_pubkey, &game_ui, game_state_ui.as_ref());
        
        // Determine output filename
        let base_name = config.args.output_name
            .clone()
            .unwrap_or_else(|| format!("game_balance_{}", game_account.account_pubkey));
        
        // Export based on format
        let (content, extension) = match config.output_format {
            OutputFormat::Ron => {
                info!("📝 Exporting to RON format...");
                (game_balance.to_ron()?, "ron")
            }
            OutputFormat::Json => {
                info!("📝 Exporting to JSON format...");
                (game_balance.to_json()?, "json")
            }
        };
        
        // Write to file
        let output_path = Path::new(&config.output_dir)
            .join(format!("{}.{}", base_name, extension));
        
        fs::write(&output_path, content)?;
        info!("✅ Exported game balance data to: {}", output_path.display());
        
        // Also create a metadata file with extraction info
        let metadata = serde_json::json!({
            "extracted_at": chrono::Utc::now().to_rfc3339(),
            "game_pubkey": game_account.account_pubkey,
            "game_version": game_ui.version,
            "game_update_id": game_ui.update_id,
            "game_state_included": game_state_ui.is_some(),
            "database_source": config.database_url,
        });
        
        let metadata_path = Path::new(&config.output_dir)
            .join(format!("{}_metadata.json", base_name));
        
        fs::write(&metadata_path, serde_json::to_string_pretty(&metadata)?)?;
        info!("📋 Metadata written to: {}", metadata_path.display());
    }
    
    info!("🎉 Game data extraction complete!");
    
    Ok(())
}