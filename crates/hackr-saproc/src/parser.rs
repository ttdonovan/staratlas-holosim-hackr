use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use uuid::Uuid;

use crate::database::{AccountData, HolosimAccount};
use crate::discriminator::Discriminators;

// Import UI types
use ui_holosim::{
    FleetShipsUI, FleetUI, GameStateUI, GameUI, LootUI, MineItemUI, PlanetUI,
    SagePlayerProfileUI, ShipUI, StarbaseUI,
};

pub struct AccountParser {
    discriminators: Discriminators,
}

impl AccountParser {
    pub fn new() -> Self {
        Self {
            discriminators: Discriminators::new(),
        }
    }

    pub fn parse_account(
        &self,
        account: &AccountData,
    ) -> Result<(ParsedAccount, Option<HolosimAccount>)> {
        let program_id = Pubkey::from_str(&account.program_id)?;
        let account_pubkey = Pubkey::from_str(&account.account_pubkey)?;

        // Identify account type by discriminator
        let account_type = self
            .discriminators
            .identify_account_type(&account.data)
            .unwrap_or("Unknown");

        // Parse based on program ID and discriminator
        let parsed_data = match account.program_id.as_str() {
            // Holosim program
            "SAgeTraQfBMdvGVDJYoEvjnbq5szW7RJPi6obDTDQUF" => {
                self.parse_holosim_account(&account.data, account_type)?
            }
            // Player Profile program
            "PprofUW1pURCnMW2si88GWPXEEK3Bvh9Tksy8WtnoYJ" => {
                self.parse_player_profile_account(&account.data, account_type)?
            }
            // Profile Faction program
            "pFACzkX2eSpAjDyEohD6i3VRJPi6obDTDQUF" => {
                self.parse_profile_faction_account(&account.data, account_type)?
            }
            _ => {
                json!({
                    "error": "Unknown program ID",
                    "raw_data_length": account.data.len(),
                    "discriminator_hex": hex::encode(&account.data.get(0..8).unwrap_or(&[]))
                })
            }
        };

        let parsed_account = ParsedAccount {
            program_id,
            account_pubkey,
            account_type: account_type.to_string(),
            lamports: account.lamports,
            parsed_data,
            raw_data_length: account.data.len(),
            created_at: account.created_at,
            updated_at: account.updated_at,
        };

        // Create HolosimAccount for database storage
        let mut hasher = Sha256::new();
        hasher.update(&account.data);
        let hash_result = hasher.finalize();
        let raw_data_hash = hex::encode(hash_result);

        let holosim_account = HolosimAccount {
            id: Uuid::new_v4().to_string(),
            account_pubkey: account_pubkey.to_string(),
            account_type: account_type.to_string(),
            parsed_data: parsed_account.parsed_data.clone(),
            raw_data_hash,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        Ok((parsed_account, Some(holosim_account)))
    }

    fn parse_holosim_account(&self, data: &[u8], account_type: &str) -> Result<Value> {
        // Helper to create error response
        let error_response = |e: &dyn std::error::Error| -> Value {
            json!({
                "account_type": account_type,
                "error": format!("Failed to deserialize {}: {}", account_type, e),
                "discriminator": hex::encode(&data.get(0..8).unwrap_or(&[]))
            })
        };

        match account_type {
            "Fleet" => match FleetUI::from_bytes(data) {
                Ok(fleet_ui) => serde_json::to_value(fleet_ui).map_err(Into::into),
                Err(e) => Ok(error_response(e.as_ref())),
            },
            "FleetShips" => match FleetShipsUI::from_bytes(data) {
                Ok(fleet_ships_ui) => serde_json::to_value(fleet_ships_ui).map_err(Into::into),
                Err(e) => Ok(error_response(e.as_ref())),
            },
            "Loot" => match LootUI::from_bytes(data) {
                Ok(loot_ui) => serde_json::to_value(loot_ui).map_err(Into::into),
                Err(e) => Ok(error_response(e.as_ref())),
            },
            "SagePlayerProfile" => match SagePlayerProfileUI::from_bytes(data) {
                Ok(profile_ui) => serde_json::to_value(profile_ui).map_err(Into::into),
                Err(e) => Ok(error_response(e.as_ref())),
            },
            "Ship" => match ShipUI::from_bytes(data) {
                Ok(ship_ui) => serde_json::to_value(ship_ui).map_err(Into::into),
                Err(e) => Ok(error_response(e.as_ref())),
            },
            "Starbase" => match StarbaseUI::from_bytes(data) {
                Ok(starbase_ui) => serde_json::to_value(starbase_ui).map_err(Into::into),
                Err(e) => Ok(error_response(e.as_ref())),
            },
            "Game" => match GameUI::from_bytes(data) {
                Ok(game_ui) => serde_json::to_value(game_ui).map_err(Into::into),
                Err(e) => Ok(error_response(e.as_ref())),
            },
            "GameState" => match GameStateUI::from_bytes(data) {
                Ok(game_state_ui) => serde_json::to_value(game_state_ui).map_err(Into::into),
                Err(e) => Ok(error_response(e.as_ref())),
            },
            "MineItem" => match MineItemUI::from_bytes(data) {
                Ok(mine_item_ui) => serde_json::to_value(mine_item_ui).map_err(Into::into),
                Err(e) => Ok(error_response(e.as_ref())),
            },
            "Planet" => match PlanetUI::from_bytes(data) {
                Ok(planet_ui) => serde_json::to_value(planet_ui).map_err(Into::into),
                Err(e) => Ok(error_response(e.as_ref())),
            },
            _ => {
                // For now, return basic parsing for other account types
                // TODO: Implement remaining account types as needed
                Ok(json!({
                    "account_type": account_type,
                    "program": "Holosim",
                    "data_length": data.len(),
                    "discriminator": hex::encode(&data.get(0..8).unwrap_or(&[])),
                    "parsing_status": "not_implemented"
                }))
            }
        }
    }

    fn parse_player_profile_account(&self, data: &[u8], account_type: &str) -> Result<Value> {
        // TODO: Use the actual Player Profile account types from the workspace crate
        Ok(json!({
            "account_type": account_type,
            "program": "PlayerProfile",
            "data_length": data.len(),
            "discriminator": hex::encode(&data.get(0..8).unwrap_or(&[])),
            "parsing_status": "not_implemented_yet"
        }))
    }

    fn parse_profile_faction_account(&self, data: &[u8], account_type: &str) -> Result<Value> {
        // TODO: Use the actual Profile Faction account types from the workspace crate
        Ok(json!({
            "account_type": account_type,
            "program": "ProfileFaction",
            "data_length": data.len(),
            "discriminator": hex::encode(&data.get(0..8).unwrap_or(&[])),
            "parsing_status": "not_implemented_yet"
        }))
    }
}

#[derive(Debug, Serialize)]
pub struct ParsedAccount {
    pub program_id: Pubkey,
    pub account_pubkey: Pubkey,
    pub account_type: String,
    pub lamports: u64,
    pub parsed_data: Value,
    pub raw_data_length: usize,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for AccountParser {
    fn default() -> Self {
        Self::new()
    }
}
