use crate::UIConversionError;
use serde::{Deserialize, Serialize};
use staratlas_holosim::generated::accounts::Planet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetUI {
    pub account_type: String,
    pub discriminator: String,
    pub version: u8,
    pub name: String,
    pub game_id: String,
    pub sector: [i64; 2],
    pub sub_coordinates: [i64; 2],
    pub planet_type: u8,
    pub position: u8,
    pub size: u64,
    pub max_hp: u64,
    pub current_health: u64,
    pub amount_mined: u64,
    pub num_resources: u8,
    pub num_miners: u64,
}

impl TryFrom<&Planet> for PlanetUI {
    type Error = UIConversionError;

    fn try_from(planet: &Planet) -> Result<Self, Self::Error> {
        Ok(PlanetUI {
            account_type: "Planet".to_string(),
            discriminator: hex::encode(&planet.discriminator),
            version: planet.version,
            name: std::str::from_utf8(&planet.name)?
                .trim_matches('\0')
                .to_string(),
            game_id: planet.game_id.to_string(),
            sector: planet.sector,
            sub_coordinates: planet.sub_coordinates,
            planet_type: planet.planet_type,
            position: planet.position,
            size: planet.size,
            max_hp: planet.max_hp,
            current_health: planet.current_health,
            amount_mined: planet.amount_mined,
            num_resources: planet.num_resources,
            num_miners: planet.num_miners,
        })
    }
}

impl PlanetUI {
    pub fn from_bytes(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let planet = Planet::from_bytes(data)?;
        Ok(Self::try_from(&planet)?)
    }
}