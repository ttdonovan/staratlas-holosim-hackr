use anyhow::Result;
use serde_json::Value;
use sqlx::sqlite::SqlitePool;
use std::fs;
use std::path::Path;
use tracing::{error, info, warn};
use ui_holosim::{
    GameStateUI, GameUI, MineItemUI, PlanetUI, ResourceUI, SectorUI, ShipUI, StarUI, StarbaseUI,
};

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
             WHERE account_type = 'Game' AND account_pubkey = ?",
        )
        .bind(pubkey)
    } else {
        info!("🎯 Fetching all Game accounts");
        sqlx::query_as::<_, HolosimAccount>(
            "SELECT account_pubkey, account_type, parsed_data 
             FROM holosim_accounts 
             WHERE account_type = 'Game'",
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
        info!(
            "🎮 Processing Game account: {}",
            game_account.account_pubkey
        );

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
                 WHERE account_type = 'GameState' AND account_pubkey = ?",
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

        // Fetch associated Planets
        info!("🌍 Fetching associated Planet accounts...");
        let planet_accounts = sqlx::query_as::<_, HolosimAccount>(
            "SELECT account_pubkey, account_type, parsed_data 
             FROM holosim_accounts 
             WHERE account_type = 'Planet' AND json_extract(parsed_data, '$.game_id') = ?",
        )
        .bind(&game_account.account_pubkey)
        .fetch_all(&pool)
        .await?;

        let planets: Vec<PlanetUI> = planet_accounts
            .into_iter()
            .filter_map(|account| {
                match serde_json::from_value::<PlanetUI>(account.parsed_data) {
                    Ok(mut planet) => {
                        planet.pubkey = Some(account.account_pubkey);
                        Some(planet)
                    }
                    Err(e) => {
                        warn!("Failed to parse Planet account: {}", e);
                        None
                    }
                }
            })
            .collect();

        info!("  Found {} Planet(s)", planets.len());

        // Fetch associated MineItems
        info!("⛏️  Fetching associated MineItem accounts...");
        let mine_item_accounts = sqlx::query_as::<_, HolosimAccount>(
            "SELECT account_pubkey, account_type, parsed_data 
             FROM holosim_accounts 
             WHERE account_type = 'MineItem' AND json_extract(parsed_data, '$.game_id') = ?",
        )
        .bind(&game_account.account_pubkey)
        .fetch_all(&pool)
        .await?;

        let mine_items: Vec<MineItemUI> = mine_item_accounts
            .into_iter()
            .filter_map(|account| {
                match serde_json::from_value::<MineItemUI>(account.parsed_data) {
                    Ok(mut mine_item) => {
                        mine_item.pubkey = Some(account.account_pubkey);
                        Some(mine_item)
                    }
                    Err(e) => {
                        warn!("Failed to parse MineItem account: {}", e);
                        None
                    }
                }
            })
            .collect();

        info!("  Found {} MineItem(s)", mine_items.len());

        // Fetch associated Starbases (if they exist)
        info!("🚀 Fetching associated Starbase accounts...");
        let starbase_accounts = sqlx::query_as::<_, HolosimAccount>(
            "SELECT account_pubkey, account_type, parsed_data 
             FROM holosim_accounts 
             WHERE account_type = 'Starbase' AND json_extract(parsed_data, '$.game_id') = ?",
        )
        .bind(&game_account.account_pubkey)
        .fetch_all(&pool)
        .await?;

        let starbases: Vec<StarbaseUI> = starbase_accounts
            .into_iter()
            .filter_map(
                |account| match serde_json::from_value::<StarbaseUI>(account.parsed_data) {
                    Ok(starbase) => Some(starbase),
                    Err(e) => {
                        warn!("Failed to parse Starbase account: {}", e);
                        None
                    }
                },
            )
            .collect();

        info!("  Found {} Starbase(s)", starbases.len());

        // Fetch associated Resources
        info!("💎 Fetching associated Resource accounts...");
        let resource_accounts = sqlx::query_as::<_, HolosimAccount>(
            "SELECT account_pubkey, account_type, parsed_data 
             FROM holosim_accounts 
             WHERE account_type = 'Resource' AND json_extract(parsed_data, '$.game_id') = ?",
        )
        .bind(&game_account.account_pubkey)
        .fetch_all(&pool)
        .await?;

        let resources: Vec<ResourceUI> = resource_accounts
            .into_iter()
            .filter_map(
                |account| match serde_json::from_value::<ResourceUI>(account.parsed_data) {
                    Ok(resource) => Some(resource),
                    Err(e) => {
                        warn!("Failed to parse Resource account: {}", e);
                        None
                    }
                },
            )
            .collect();

        info!("  Found {} Resource(s)", resources.len());

        // Fetch associated Sectors
        info!("🗺️  Fetching associated Sector accounts...");
        let sector_accounts = sqlx::query_as::<_, HolosimAccount>(
            "SELECT account_pubkey, account_type, parsed_data 
             FROM holosim_accounts 
             WHERE account_type = 'Sector' AND json_extract(parsed_data, '$.game_id') = ?",
        )
        .bind(&game_account.account_pubkey)
        .fetch_all(&pool)
        .await?;

        let sectors: Vec<SectorUI> = sector_accounts
            .into_iter()
            .filter_map(
                |account| match serde_json::from_value::<SectorUI>(account.parsed_data) {
                    Ok(sector) => Some(sector),
                    Err(e) => {
                        warn!("Failed to parse Sector account: {}", e);
                        None
                    }
                },
            )
            .collect();

        info!("  Found {} Sector(s)", sectors.len());

        // Fetch associated Ships
        info!("🚢 Fetching associated Ship accounts...");
        let ship_accounts = sqlx::query_as::<_, HolosimAccount>(
            "SELECT account_pubkey, account_type, parsed_data 
             FROM holosim_accounts 
             WHERE account_type = 'Ship' AND json_extract(parsed_data, '$.game_id') = ?",
        )
        .bind(&game_account.account_pubkey)
        .fetch_all(&pool)
        .await?;

        let ships: Vec<ShipUI> = ship_accounts
            .into_iter()
            .filter_map(
                |account| match serde_json::from_value::<ShipUI>(account.parsed_data) {
                    Ok(ship) => Some(ship),
                    Err(e) => {
                        warn!("Failed to parse Ship account: {}", e);
                        None
                    }
                },
            )
            .collect();

        info!("  Found {} Ship(s)", ships.len());

        // Fetch associated Stars
        info!("⭐ Fetching associated Star accounts...");
        let star_accounts = sqlx::query_as::<_, HolosimAccount>(
            "SELECT account_pubkey, account_type, parsed_data 
             FROM holosim_accounts 
             WHERE account_type = 'Star' AND json_extract(parsed_data, '$.game_id') = ?",
        )
        .bind(&game_account.account_pubkey)
        .fetch_all(&pool)
        .await?;

        let stars: Vec<StarUI> = star_accounts
            .into_iter()
            .filter_map(
                |account| match serde_json::from_value::<StarUI>(account.parsed_data) {
                    Ok(star) => Some(star),
                    Err(e) => {
                        warn!("Failed to parse Star account: {}", e);
                        None
                    }
                },
            )
            .collect();

        info!("  Found {} Star(s)", stars.len());

        // Create game balance structure
        let game_balance = GameBalance::from_ui(
            &game_account.account_pubkey,
            &game_ui,
            game_state_ui.as_ref(),
            planets,
            mine_items,
            starbases,
            resources,
            sectors,
            ships,
            stars,
        );

        // Create subdirectory for this game
        let game_output_dir = Path::new(&config.output_dir).join(&game_account.account_pubkey);
        fs::create_dir_all(&game_output_dir)?;

        // Determine output filename
        let base_name = config
            .args
            .output_name
            .clone()
            .unwrap_or_else(|| "game_balance".to_string());

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

        // Write to file in game-specific directory
        let output_path = game_output_dir.join(format!("{}.{}", base_name, extension));

        fs::write(&output_path, content)?;
        info!(
            "✅ Exported game balance data to: {}",
            output_path.display()
        );

        // Also create a metadata file with extraction info
        let metadata = serde_json::json!({
            "extracted_at": chrono::Utc::now().to_rfc3339(),
            "game_pubkey": game_account.account_pubkey,
            "game_version": game_ui.version,
            "game_update_id": game_ui.update_id,
            "game_state_included": game_state_ui.is_some(),
            "planet_count": game_balance.planets.len(),
            "mine_item_count": game_balance.mine_items.len(),
            "starbase_count": game_balance.starbases.len(),
            "resource_count": game_balance.resources.len(),
            "sector_count": game_balance.sectors.len(),
            "ship_count": game_balance.ships.len(),
            "star_count": game_balance.stars.len(),
            "database_source": config.database_url,
        });

        let metadata_path = game_output_dir.join(format!("{}_metadata.json", base_name));

        fs::write(&metadata_path, serde_json::to_string_pretty(&metadata)?)?;
        info!("📋 Metadata written to: {}", metadata_path.display());
    }

    info!("🎉 Game data extraction complete!");

    Ok(())
}
