use crate::UIConversionError;
use serde::{Deserialize, Serialize};
use staratlas_holosim::generated::accounts::GameState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameStateUI {
    pub account_type: String,
    pub discriminator: String,
    pub version: u8,
    pub update_id: u64,
    pub game_id: String,
    pub fleet: FleetInfoUI,
    pub misc: MiscVariablesUI,
    pub bump: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FleetInfoUI {
    // TODO: Add actual FleetInfo fields
    pub placeholder: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiscVariablesUI {
    // TODO: Add actual MiscVariables fields
    pub placeholder: String,
}

impl TryFrom<&GameState> for GameStateUI {
    type Error = UIConversionError;

    fn try_from(game_state: &GameState) -> Result<Self, Self::Error> {
        Ok(GameStateUI {
            account_type: "GameState".to_string(),
            discriminator: hex::encode(&game_state.discriminator),
            version: game_state.version,
            update_id: game_state.update_id,
            game_id: game_state.game_id.to_string(),
            fleet: FleetInfoUI {
                // TODO: Map actual FleetInfo fields
                placeholder: "fleet_info_placeholder".to_string(),
            },
            misc: MiscVariablesUI {
                // TODO: Map actual MiscVariables fields
                placeholder: "misc_variables_placeholder".to_string(),
            },
            bump: game_state.bump,
        })
    }
}

impl GameStateUI {
    pub fn from_bytes(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let game_state = GameState::from_bytes(data)?;
        Ok(Self::try_from(&game_state)?)
    }
}