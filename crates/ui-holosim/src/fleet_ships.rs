use crate::UIConversionError;
use serde::{Deserialize, Serialize};
use staratlas_holosim::generated::accounts::FleetShips;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FleetShipsUI {
    pub account_type: String,
    pub discriminator: String,
    pub version: u8,
    pub fleet: String,
    pub fleet_ships_info_count: u32,
    pub bump: u8,
}

impl TryFrom<&FleetShips> for FleetShipsUI {
    type Error = UIConversionError;

    fn try_from(fleet_ships: &FleetShips) -> Result<Self, Self::Error> {
        Ok(FleetShipsUI {
            account_type: "FleetShips".to_string(),
            discriminator: hex::encode(&fleet_ships.discriminator),
            version: fleet_ships.version,
            fleet: fleet_ships.fleet.to_string(),
            fleet_ships_info_count: fleet_ships.fleet_ships_info_count,
            bump: fleet_ships.bump,
        })
    }
}

impl FleetShipsUI {
    pub fn from_bytes(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let fleet_ships = FleetShips::from_bytes(data)?;
        Ok(Self::try_from(&fleet_ships)?)
    }
}
