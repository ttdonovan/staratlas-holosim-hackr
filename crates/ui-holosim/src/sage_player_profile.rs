use crate::UIConversionError;
use serde::{Deserialize, Serialize};
use staratlas_holosim::generated::accounts::SagePlayerProfile;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SagePlayerProfileUI {
    pub account_type: String,
    pub discriminator: String,
    pub version: u8,
    pub player_profile: String,
    pub game_id: String,
    pub bump: u8,
}

impl TryFrom<&SagePlayerProfile> for SagePlayerProfileUI {
    type Error = UIConversionError;

    fn try_from(profile: &SagePlayerProfile) -> Result<Self, Self::Error> {
        Ok(SagePlayerProfileUI {
            account_type: "SagePlayerProfile".to_string(),
            discriminator: hex::encode(&profile.discriminator),
            version: profile.version,
            player_profile: profile.player_profile.to_string(),
            game_id: profile.game_id.to_string(),
            bump: profile.bump,
        })
    }
}

impl SagePlayerProfileUI {
    pub fn from_bytes(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let profile = SagePlayerProfile::from_bytes(data)?;
        Ok(Self::try_from(&profile)?)
    }
}
