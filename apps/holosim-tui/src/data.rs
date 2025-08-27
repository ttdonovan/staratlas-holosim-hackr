use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use ui_holosim::{
    MineItemUI, PlanetUI, ResourceUI, SectorUI, ShipUI, StarUI, StarbaseUI,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameData {
    pub game_id: String,
    pub version: u8,
    pub update_id: u64,
    pub planets: Vec<PlanetUI>,
    pub mine_items: Vec<MineItemUI>,
    pub starbases: Vec<StarbaseUI>,
    pub resources: Vec<ResourceUI>,
    pub sectors: Vec<SectorUI>,
    pub ships: Vec<ShipUI>,
    pub stars: Vec<StarUI>,
}

impl GameData {
    pub fn load_from_file(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let balance: GameBalance = ron::from_str(&content)?;
        
        Ok(Self {
            game_id: balance.game_id,
            version: balance.version,
            update_id: balance.update_id,
            planets: balance.planets,
            mine_items: balance.mine_items,
            starbases: balance.starbases,
            resources: balance.resources,
            sectors: balance.sectors,
            ships: balance.ships,
            stars: balance.stars,
        })
    }
}

// Simplified GameBalance structure for deserialization
#[derive(Debug, Clone, Serialize, Deserialize)]
struct GameBalance {
    game_id: String,
    version: u8,
    update_id: u64,
    config: GameConfig,
    game_state: Option<GameStateData>,
    planets: Vec<PlanetUI>,
    mine_items: Vec<MineItemUI>,
    starbases: Vec<StarbaseUI>,
    resources: Vec<ResourceUI>,
    sectors: Vec<SectorUI>,
    ships: Vec<ShipUI>,
    stars: Vec<StarUI>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GameConfig {
    profile: String,
    points_config: ConfigPlaceholder,
    cargo_config: ConfigPlaceholder,
    crafting_config: ConfigPlaceholder,
    mints_config: ConfigPlaceholder,
    vaults_config: ConfigPlaceholder,
    risk_zones: ConfigPlaceholder,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GameStateData {
    state_id: String,
    version: u8,
    update_id: u64,
    fleet_info: ConfigPlaceholder,
    misc_variables: ConfigPlaceholder,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ConfigPlaceholder {
    placeholder: String,
}