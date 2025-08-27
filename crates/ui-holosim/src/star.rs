use crate::UIConversionError;
use serde::{Deserialize, Serialize};
use staratlas_holosim::generated::accounts::Star;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarUI {
    pub account_type: String,
    pub discriminator: String,
    pub version: u8,
    pub name: String,
    pub game_id: String,
    pub sector: (i64, i64),
    pub size: u64,
    pub sub_coordinates: (i64, i64),
    pub star_type: u8,
}

impl TryFrom<&Star> for StarUI {
    type Error = UIConversionError;

    fn try_from(star: &Star) -> Result<Self, Self::Error> {
        // Convert name from bytes to string
        let name_bytes: Vec<u8> = star.name.iter().copied().take_while(|&b| b != 0).collect();
        let name =
            String::from_utf8(name_bytes).unwrap_or_else(|_| format!("Star_{}", star.star_type));

        Ok(StarUI {
            account_type: "Star".to_string(),
            discriminator: hex::encode(&star.discriminator),
            version: star.version,
            name,
            game_id: star.game_id.to_string(),
            sector: (star.sector[0], star.sector[1]),
            size: star.size,
            sub_coordinates: (star.sub_coordinates[0], star.sub_coordinates[1]),
            star_type: star.star_type,
        })
    }
}

impl StarUI {
    /// Parse from borsh serialized data
    pub fn from_bytes(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let star = Star::from_bytes(data)?;
        Ok((&star).try_into()?)
    }
}
