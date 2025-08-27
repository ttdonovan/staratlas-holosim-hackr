use crate::UIConversionError;
use serde::{Deserialize, Serialize};
use staratlas_holosim::generated::accounts::Starbase;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarbaseUI {
    pub account_type: String,
    pub discriminator: String,
    pub version: u8,
    pub game_id: String,
    pub sector: [i64; 2],
    pub name: String,
    pub sub_coordinates: [i64; 2],
    pub faction: u8,
    pub bump: u8,
    pub seq_id: u16,
}

impl TryFrom<&Starbase> for StarbaseUI {
    type Error = UIConversionError;

    fn try_from(starbase: &Starbase) -> Result<Self, Self::Error> {
        Ok(StarbaseUI {
            account_type: "Starbase".to_string(),
            discriminator: hex::encode(&starbase.discriminator),
            version: starbase.version,
            game_id: starbase.game_id.to_string(),
            sector: starbase.sector,
            name: std::str::from_utf8(&starbase.name)?
                .trim_matches('\0')
                .to_string(),
            sub_coordinates: starbase.sub_coordinates,
            faction: starbase.faction,
            bump: starbase.bump,
            seq_id: starbase.seq_id,
        })
    }
}

impl StarbaseUI {
    pub fn from_bytes(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let starbase = Starbase::from_bytes(data)?;
        Ok(Self::try_from(&starbase)?)
    }
}
