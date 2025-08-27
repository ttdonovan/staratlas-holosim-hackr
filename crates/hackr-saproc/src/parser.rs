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

// Import Holosim account types
use staratlas_holosim::generated::accounts::{
    Fleet, FleetShips, Loot, SagePlayerProfile, Ship, Starbase,
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
            "Fleet" => {
                match Fleet::from_bytes(data) {
                    Ok(fleet) => {
                        // Manually construct JSON for Fleet
                        Ok(json!({
                            "account_type": account_type,
                            "discriminator": hex::encode(&fleet.discriminator),
                            "version": fleet.version,
                            "game_id": fleet.game_id.to_string(),
                            "owner_profile": fleet.owner_profile.to_string(),
                            "fleet_ships": fleet.fleet_ships.to_string(),
                            "sub_profile": fleet.sub_profile.key.to_string(),
                            "sub_profile_invalidator": fleet.sub_profile_invalidator.to_string(),
                            "fleet_label": std::str::from_utf8(&fleet.fleet_label).unwrap_or("").trim_matches('\0'),
                            "cargo_hold": fleet.cargo_hold.to_string(),
                            "fuel_tank": fleet.fuel_tank.to_string(),
                            "ammo_bank": fleet.ammo_bank.to_string(),
                            "update_id": fleet.update_id,
                            "bump": fleet.bump,
                        }))
                    }
                    Err(e) => Ok(error_response(&e)),
                }
            }
            "FleetShips" => {
                match FleetShips::from_bytes(data) {
                    Ok(fleet_ships) => {
                        // Manually construct JSON for FleetShips
                        Ok(json!({
                            "account_type": account_type,
                            "discriminator": hex::encode(&fleet_ships.discriminator),
                            "version": fleet_ships.version,
                            "fleet": fleet_ships.fleet.to_string(),
                            "fleet_ships_info_count": fleet_ships.fleet_ships_info_count,
                            "bump": fleet_ships.bump,
                        }))
                    }
                    Err(e) => Ok(error_response(&e)),
                }
            }
            "Loot" => {
                match Loot::from_bytes(data) {
                    Ok(loot) => {
                        // Manually construct JSON for Loot
                        Ok(json!({
                            "account_type": account_type,
                            "discriminator": hex::encode(&loot.discriminator),
                            "version": loot.version,
                            "sector": loot.sector,
                            "game_id": loot.game_id.to_string(),
                            "creator": loot.creator.to_string(),
                            "items": loot.items.iter().map(|_item| {
                                json!({
                                    // Add fields based on LootInfo structure
                                    "item": "placeholder" // TODO: Add actual LootInfo fields
                                })
                            }).collect::<Vec<_>>(),
                        }))
                    }
                    Err(e) => Ok(error_response(&e)),
                }
            }
            "SagePlayerProfile" => {
                match SagePlayerProfile::from_bytes(data) {
                    Ok(profile) => {
                        // Manually construct JSON for SagePlayerProfile
                        Ok(json!({
                            "account_type": account_type,
                            "discriminator": hex::encode(&profile.discriminator),
                            "version": profile.version,
                            "player_profile": profile.player_profile.to_string(),
                            "game_id": profile.game_id.to_string(),
                            "bump": profile.bump,
                            "bump": profile.bump,
                        }))
                    }
                    Err(e) => Ok(error_response(&e)),
                }
            }
            "Ship" => {
                match Ship::from_bytes(data) {
                    Ok(ship) => {
                        // Manually construct JSON for Ship
                        Ok(json!({
                            "account_type": account_type,
                            "discriminator": hex::encode(&ship.discriminator),
                            "version": ship.version,
                            "name": std::str::from_utf8(&ship.name).unwrap_or("").trim_matches('\0'),
                            "size_class": ship.size_class,
                            "stats": {
                                "movement_stats": {
                                    "max_warp_distance": ship.stats.movement_stats.max_warp_distance,
                                    "warp_cool_down": ship.stats.movement_stats.warp_cool_down,
                                    "subwarp_speed": ship.stats.movement_stats.subwarp_speed,
                                    "warp_speed": ship.stats.movement_stats.warp_speed,
                                    "subwarp_fuel_consumption_rate": ship.stats.movement_stats.subwarp_fuel_consumption_rate,
                                    "warp_fuel_consumption_rate": ship.stats.movement_stats.warp_fuel_consumption_rate,
                                    "planet_exit_fuel_amount": ship.stats.movement_stats.planet_exit_fuel_amount,
                                },
                                "cargo_stats": {
                                    "cargo_capacity": ship.stats.cargo_stats.cargo_capacity,
                                    "fuel_capacity": ship.stats.cargo_stats.fuel_capacity,
                                    "ammo_capacity": ship.stats.cargo_stats.ammo_capacity,
                                    "mining_rate": ship.stats.cargo_stats.mining_rate,
                                    "upgrade_rate": ship.stats.cargo_stats.upgrade_rate,
                                    "cargo_transfer_rate": ship.stats.cargo_stats.cargo_transfer_rate,
                                    "tractor_beam_gather_rate": ship.stats.cargo_stats.tractor_beam_gather_rate,
                                    "ammo_consumption_rate": ship.stats.cargo_stats.ammo_consumption_rate,
                                    "food_consumption_rate": ship.stats.cargo_stats.food_consumption_rate,
                                },
                                "misc_stats": {
                                    "required_crew": ship.stats.misc_stats.required_crew,
                                    "passenger_capacity": ship.stats.misc_stats.passenger_capacity,
                                    "crew_count": ship.stats.misc_stats.crew_count,
                                    "rented_crew": ship.stats.misc_stats.rented_crew,
                                    "respawn_time": ship.stats.misc_stats.respawn_time,
                                    "scan_cool_down": ship.stats.misc_stats.scan_cool_down,
                                    "sdu_per_scan": ship.stats.misc_stats.sdu_per_scan,
                                    "scan_cost": ship.stats.misc_stats.scan_cost,
                                }
                            },
                            "mint": ship.mint.to_string(),
                            "update_id": ship.update_id,
                            "max_update_id": ship.max_update_id,
                            "next": ship.next.key.to_string(),
                        }))
                    }
                    Err(e) => Ok(error_response(&e)),
                }
            }
            "Starbase" => {
                match Starbase::from_bytes(data) {
                    Ok(starbase) => {
                        // Manually construct JSON for Starbase
                        Ok(json!({
                            "account_type": account_type,
                            "discriminator": hex::encode(&starbase.discriminator),
                            "version": starbase.version,
                            "game_id": starbase.game_id.to_string(),
                            "sector": [starbase.sector[0], starbase.sector[1]],
                            "name": std::str::from_utf8(&starbase.name).unwrap_or("").trim_matches('\0'),
                            "sub_coordinates": [starbase.sub_coordinates[0], starbase.sub_coordinates[1]],
                            "faction": starbase.faction,
                            "bump": starbase.bump,
                            "seq_id": starbase.seq_id,
                        }))
                    }
                    Err(e) => Ok(error_response(&e)),
                }
            }
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
