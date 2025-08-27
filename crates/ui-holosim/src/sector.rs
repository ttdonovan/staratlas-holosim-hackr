use crate::UIConversionError;
use serde::{Deserialize, Serialize};
use staratlas_holosim::generated::accounts::Sector;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectorUI {
    pub account_type: String,
    pub discriminator: String,
    pub version: u8,
    pub game_id: String,
    pub coordinates: (i64, i64),
    pub discoverer: String,
    pub name: String,
    pub num_stars: u16,
    pub num_planets: u16,
    pub num_moons: u16,
    pub num_asteroid_belts: u16,
    pub last_scan_time: i64,
    pub last_scan_chance: u32,
    pub bump: u8,
    pub num_connections: u16,
}

impl TryFrom<&Sector> for SectorUI {
    type Error = UIConversionError;

    fn try_from(sector: &Sector) -> Result<Self, Self::Error> {
        // Convert name from bytes to string
        let name_bytes: Vec<u8> = sector
            .name
            .iter()
            .copied()
            .take_while(|&b| b != 0)
            .collect();
        let name = String::from_utf8(name_bytes).unwrap_or_else(|_| {
            format!("Sector_{}_{}", sector.coordinates[0], sector.coordinates[1])
        });

        Ok(SectorUI {
            account_type: "Sector".to_string(),
            discriminator: hex::encode(&sector.discriminator),
            version: sector.version,
            game_id: sector.game_id.to_string(),
            coordinates: (sector.coordinates[0], sector.coordinates[1]),
            discoverer: sector.discoverer.to_string(),
            name,
            num_stars: sector.num_stars,
            num_planets: sector.num_planets,
            num_moons: sector.num_moons,
            num_asteroid_belts: sector.num_asteroid_belts,
            last_scan_time: sector.last_scan_time,
            last_scan_chance: sector.last_scan_chance,
            bump: sector.bump,
            num_connections: sector.num_connections,
        })
    }
}

impl SectorUI {
    /// Parse from borsh serialized data
    pub fn from_bytes(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let sector = Sector::from_bytes(data)?;
        Ok((&sector).try_into()?)
    }
}
