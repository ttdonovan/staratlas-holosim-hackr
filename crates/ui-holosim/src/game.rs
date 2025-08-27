use crate::UIConversionError;
use serde::{Deserialize, Serialize};
use staratlas_holosim::generated::accounts::Game;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameUI {
    pub account_type: String,
    pub discriminator: String,
    pub version: u8,
    pub update_id: u64,
    pub profile: String,
    pub game_state: String,
    pub points: PointsUI,
    pub cargo: CargoUI,
    pub crafting: CraftingUI,
    pub mints: MintsUI,
    pub vaults: VaultsUI,
    pub risk_zones: RiskZonesDataUI,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointsUI {
    // TODO: Add actual Points fields
    pub placeholder: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CargoUI {
    // TODO: Add actual Cargo fields
    pub placeholder: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingUI {
    // TODO: Add actual Crafting fields
    pub placeholder: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MintsUI {
    // TODO: Add actual Mints fields
    pub placeholder: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultsUI {
    // TODO: Add actual Vaults fields
    pub placeholder: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskZonesDataUI {
    // TODO: Add actual RiskZonesData fields
    pub placeholder: String,
}

impl TryFrom<&Game> for GameUI {
    type Error = UIConversionError;

    fn try_from(game: &Game) -> Result<Self, Self::Error> {
        Ok(GameUI {
            account_type: "Game".to_string(),
            discriminator: hex::encode(&game.discriminator),
            version: game.version,
            update_id: game.update_id,
            profile: game.profile.to_string(),
            game_state: game.game_state.to_string(),
            points: PointsUI {
                // TODO: Map actual Points fields
                placeholder: "points_placeholder".to_string(),
            },
            cargo: CargoUI {
                // TODO: Map actual Cargo fields
                placeholder: "cargo_placeholder".to_string(),
            },
            crafting: CraftingUI {
                // TODO: Map actual Crafting fields
                placeholder: "crafting_placeholder".to_string(),
            },
            mints: MintsUI {
                // TODO: Map actual Mints fields
                placeholder: "mints_placeholder".to_string(),
            },
            vaults: VaultsUI {
                // TODO: Map actual Vaults fields
                placeholder: "vaults_placeholder".to_string(),
            },
            risk_zones: RiskZonesDataUI {
                // TODO: Map actual RiskZonesData fields
                placeholder: "risk_zones_placeholder".to_string(),
            },
        })
    }
}

impl GameUI {
    pub fn from_bytes(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let game = Game::from_bytes(data)?;
        Ok(Self::try_from(&game)?)
    }
}