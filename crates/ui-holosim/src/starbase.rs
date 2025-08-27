use serde::{Deserialize, Serialize};
use staratlas_holosim::generated::accounts::Starbase;
use crate::UIConversionError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarbaseUI {
    pub account_type: String,
    pub discriminator: String,
    pub version: u8,
    pub game_id: String,
    pub sector: (i64, i64),
    pub crafting_facility: String,
    pub upgrade_facility: String,
    pub name: String,
    pub sub_coordinates: (i64, i64),
    pub faction: u8,
    pub bump: u8,
    pub seq_id: u16,
    pub state: u8,
    pub level: u8,
    pub hp: u64,
    pub sp: u64,
    pub sector_ring_available: u8,
    pub upgrade_state: u8,
    pub upgrade_ingredients_checksum: String,
    pub num_upgrade_ingredients: u8,
    pub upkeep_ammo_balance: u64,
    pub upkeep_ammo_last_update: i64,
    pub upkeep_ammo_global_last_update: i64,
    pub upkeep_food_balance: u64,
    pub upkeep_food_last_update: i64,
    pub upkeep_food_global_last_update: i64,
    pub upkeep_toolkit_balance: u64,
    pub upkeep_toolkit_last_update: i64,
    pub upkeep_toolkit_global_last_update: i64,
    pub built_destroyed_timestamp: i64,
    pub shield_break_delay_expires_at: i64,
}

impl TryFrom<&Starbase> for StarbaseUI {
    type Error = UIConversionError;

    fn try_from(starbase: &Starbase) -> Result<Self, Self::Error> {
        // Convert name from bytes to string
        let name_bytes: Vec<u8> = starbase.name.iter()
            .copied()
            .take_while(|&b| b != 0)
            .collect();
        let name = String::from_utf8(name_bytes)
            .unwrap_or_else(|_| format!("Starbase_{}", starbase.seq_id));

        Ok(StarbaseUI {
            account_type: "Starbase".to_string(),
            discriminator: hex::encode(&starbase.discriminator),
            version: starbase.version,
            game_id: starbase.game_id.to_string(),
            sector: (starbase.sector[0], starbase.sector[1]),
            crafting_facility: starbase.crafting_facility.to_string(),
            upgrade_facility: starbase.upgrade_facility.to_string(),
            name,
            sub_coordinates: (starbase.sub_coordinates[0], starbase.sub_coordinates[1]),
            faction: starbase.faction,
            bump: starbase.bump,
            seq_id: starbase.seq_id,
            state: starbase.state,
            level: starbase.level,
            hp: starbase.hp,
            sp: starbase.sp,
            sector_ring_available: starbase.sector_ring_available,
            upgrade_state: starbase.upgrade_state,
            upgrade_ingredients_checksum: hex::encode(&starbase.upgrade_ingredients_checksum),
            num_upgrade_ingredients: starbase.num_upgrade_ingredients,
            upkeep_ammo_balance: starbase.upkeep_ammo_balance,
            upkeep_ammo_last_update: starbase.upkeep_ammo_last_update,
            upkeep_ammo_global_last_update: starbase.upkeep_ammo_global_last_update,
            upkeep_food_balance: starbase.upkeep_food_balance,
            upkeep_food_last_update: starbase.upkeep_food_last_update,
            upkeep_food_global_last_update: starbase.upkeep_food_global_last_update,
            upkeep_toolkit_balance: starbase.upkeep_toolkit_balance,
            upkeep_toolkit_last_update: starbase.upkeep_toolkit_last_update,
            upkeep_toolkit_global_last_update: starbase.upkeep_toolkit_global_last_update,
            built_destroyed_timestamp: starbase.built_destroyed_timestamp,
            shield_break_delay_expires_at: starbase.shield_break_delay_expires_at,
        })
    }
}

impl StarbaseUI {
    /// Parse from borsh serialized data
    pub fn from_bytes(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let starbase = Starbase::from_bytes(data)?;
        Ok((&starbase).try_into()?)
    }
}