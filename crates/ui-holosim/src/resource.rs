use crate::UIConversionError;
use serde::{Deserialize, Serialize};
use staratlas_holosim::generated::accounts::Resource;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUI {
    pub account_type: String,
    pub discriminator: String,
    pub version: u8,
    pub game_id: String,
    pub location: String,
    pub mine_item: String,
    pub location_type: u8,
    pub system_richness: u16,
    pub amount_mined: u64,
    pub num_miners: u64,
    pub bump: u8,
}

impl TryFrom<&Resource> for ResourceUI {
    type Error = UIConversionError;

    fn try_from(resource: &Resource) -> Result<Self, Self::Error> {
        Ok(ResourceUI {
            account_type: "Resource".to_string(),
            discriminator: hex::encode(&resource.discriminator),
            version: resource.version,
            game_id: resource.game_id.to_string(),
            location: resource.location.to_string(),
            mine_item: resource.mine_item.to_string(),
            location_type: resource.location_type,
            system_richness: resource.system_richness,
            amount_mined: resource.amount_mined,
            num_miners: resource.num_miners,
            bump: resource.bump,
        })
    }
}

impl ResourceUI {
    /// Parse from borsh serialized data
    pub fn from_bytes(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let resource = Resource::from_bytes(data)?;
        Ok((&resource).try_into()?)
    }
}
