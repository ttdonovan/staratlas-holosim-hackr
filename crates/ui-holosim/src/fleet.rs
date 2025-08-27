use crate::UIConversionError;
use serde::{Deserialize, Serialize};
use staratlas_holosim::generated::accounts::Fleet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FleetUI {
    pub account_type: String,
    pub discriminator: String,
    pub version: u8,
    pub game_id: String,
    pub owner_profile: String,
    pub fleet_ships: String,
    pub sub_profile: Option<String>,
    pub sub_profile_invalidator: String,
    pub fleet_label: String,
    pub cargo_hold: String,
    pub fuel_tank: String,
    pub ammo_bank: String,
    pub update_id: u64,
    pub bump: u8,
}

impl TryFrom<&Fleet> for FleetUI {
    type Error = UIConversionError;

    fn try_from(fleet: &Fleet) -> Result<Self, Self::Error> {
        Ok(FleetUI {
            account_type: "Fleet".to_string(),
            discriminator: hex::encode(&fleet.discriminator),
            version: fleet.version,
            game_id: fleet.game_id.to_string(),
            owner_profile: fleet.owner_profile.to_string(),
            fleet_ships: fleet.fleet_ships.to_string(),
            sub_profile: if fleet.sub_profile.key == solana_sdk::system_program::ID {
                None
            } else {
                Some(fleet.sub_profile.key.to_string())
            },
            sub_profile_invalidator: fleet.sub_profile_invalidator.to_string(),
            fleet_label: std::str::from_utf8(&fleet.fleet_label)?
                .trim_matches('\0')
                .to_string(),
            cargo_hold: fleet.cargo_hold.to_string(),
            fuel_tank: fleet.fuel_tank.to_string(),
            ammo_bank: fleet.ammo_bank.to_string(),
            update_id: fleet.update_id,
            bump: fleet.bump,
        })
    }
}

impl FleetUI {
    pub fn from_bytes(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let fleet = Fleet::from_bytes(data)?;
        Ok(Self::try_from(&fleet)?)
    }
}
