use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use uuid::Uuid;

use crate::database::{AccountData, Database, HolosimAccount};
use crate::discriminator::Discriminators;

pub struct AccountParser<'a> {
    discriminators: Discriminators,
    database: Option<&'a Database>,
}

impl<'a> AccountParser<'a> {
    pub fn new() -> Self {
        Self {
            discriminators: Discriminators::new(),
            database: None,
        }
    }

    pub fn with_database(database: &'a Database) -> Self {
        Self {
            discriminators: Discriminators::new(),
            database: Some(database),
        }
    }

    pub async fn parse_account(&self, account: &AccountData) -> Result<ParsedAccount> {
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

        // Save to database if enabled
        if let Some(db) = self.database {
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

            match account.program_id.as_str() {
                "SAgeTraQfBMdvGVDJYoEvjnbq5szW7RJPi6obDTDQUF" => {
                    db.upsert_holosim_account(&holosim_account).await?;
                }
                "PprofUW1pURCnMW2si88GWPXEEK3Bvh9Tksy8WtnoYJ" => {
                    db.upsert_player_profile_account(&holosim_account).await?;
                }
                "pFACzkX2eSpAjDyEohD6i3VRJvREtH9ynbtM1DwVFsj" => {
                    db.upsert_profile_faction_account(&holosim_account).await?;
                }
                _ => {}
            }
        }

        Ok(parsed_account)
    }

    fn parse_holosim_account(&self, data: &[u8], account_type: &str) -> Result<Value> {
        // TODO: Use the actual Holosim account types from the workspace crate
        // For now, return basic info
        Ok(json!({
            "account_type": account_type,
            "program": "Holosim",
            "data_length": data.len(),
            "discriminator": hex::encode(&data.get(0..8).unwrap_or(&[])),
            "parsing_status": "not_implemented_yet"
        }))
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

impl<'a> Default for AccountParser<'a> {
    fn default() -> Self {
        Self::new()
    }
}
