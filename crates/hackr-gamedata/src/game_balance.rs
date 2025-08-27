use serde::{Deserialize, Serialize};
use ui_holosim::{GameUI, GameStateUI, PlanetUI, MineItemUI, StarbaseUI};

/// Game balance data structure for export
/// This represents the core configuration data needed to initialize a game world
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameBalance {
    /// Game ID (pubkey)
    pub game_id: String,
    
    /// Version information
    pub version: u8,
    pub update_id: u64,
    
    /// Core game configuration
    pub config: GameConfig,
    
    /// Associated game state (if available)
    pub game_state: Option<GameStateData>,
    
    /// Planets associated with this game
    pub planets: Vec<PlanetUI>,
    
    /// Mine items associated with this game  
    pub mine_items: Vec<MineItemUI>,
    
    /// Starbases associated with this game
    pub starbases: Vec<StarbaseUI>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    /// Profile that manages sector permissions
    pub profile: String,
    
    /// Points/scoring configuration
    pub points_config: PointsConfig,
    
    /// Cargo/inventory settings
    pub cargo_config: CargoConfig,
    
    /// Crafting system settings
    pub crafting_config: CraftingConfig,
    
    /// Token mint settings
    pub mints_config: MintsConfig,
    
    /// Vault/treasury settings  
    pub vaults_config: VaultsConfig,
    
    /// Risk zones configuration
    pub risk_zones: RiskZonesConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameStateData {
    pub state_id: String,
    pub version: u8,
    pub update_id: u64,
    pub fleet_info: FleetConfig,
    pub misc_variables: MiscConfig,
}

// Placeholder configurations - these would be expanded based on actual Game account data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointsConfig {
    // TODO: Extract from actual Points struct
    pub placeholder: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CargoConfig {
    // TODO: Extract from actual Cargo struct
    pub placeholder: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingConfig {
    // TODO: Extract from actual Crafting struct  
    pub placeholder: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MintsConfig {
    // TODO: Extract from actual Mints struct
    pub placeholder: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultsConfig {
    // TODO: Extract from actual Vaults struct
    pub placeholder: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskZonesConfig {
    // TODO: Extract from actual RiskZonesData struct
    pub placeholder: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FleetConfig {
    // TODO: Extract from actual FleetInfo struct
    pub placeholder: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiscConfig {
    // TODO: Extract from actual MiscVariables struct
    pub placeholder: String,
}

impl GameBalance {
    /// Create from UI types with additional game data
    pub fn from_ui(
        game_pubkey: &str, 
        game: &GameUI, 
        game_state: Option<&GameStateUI>,
        planets: Vec<PlanetUI>,
        mine_items: Vec<MineItemUI>,
        starbases: Vec<StarbaseUI>,
    ) -> Self {
        GameBalance {
            game_id: game_pubkey.to_string(),
            version: game.version,
            update_id: game.update_id,
            config: GameConfig {
                profile: game.profile.clone(),
                points_config: PointsConfig {
                    placeholder: game.points.placeholder.clone(),
                },
                cargo_config: CargoConfig {
                    placeholder: game.cargo.placeholder.clone(),
                },
                crafting_config: CraftingConfig {
                    placeholder: game.crafting.placeholder.clone(),
                },
                mints_config: MintsConfig {
                    placeholder: game.mints.placeholder.clone(),
                },
                vaults_config: VaultsConfig {
                    placeholder: game.vaults.placeholder.clone(),
                },
                risk_zones: RiskZonesConfig {
                    placeholder: game.risk_zones.placeholder.clone(),
                },
            },
            game_state: game_state.map(|gs| GameStateData {
                state_id: gs.game_id.clone(),
                version: gs.version,
                update_id: gs.update_id,
                fleet_info: FleetConfig {
                    placeholder: gs.fleet.placeholder.clone(),
                },
                misc_variables: MiscConfig {
                    placeholder: gs.misc.placeholder.clone(),
                },
            }),
            planets,
            mine_items,
            starbases,
        }
    }
    
    /// Export to RON format
    pub fn to_ron(&self) -> Result<String, ron::Error> {
        ron::ser::to_string_pretty(self, Default::default())
    }
    
    /// Export to JSON format  
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}