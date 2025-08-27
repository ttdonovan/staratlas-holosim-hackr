use crate::UIConversionError;
use serde::{Deserialize, Serialize};
use staratlas_holosim::generated::accounts::Loot;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootUI {
    pub account_type: String,
    pub discriminator: String,
    pub version: u8,
    pub sector: [i64; 2],
    pub game_id: String,
    pub creator: String,
    pub items: Vec<LootItemUI>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootItemUI {
    // TODO: Add actual LootInfo fields once we examine the structure
    pub placeholder: String,
}

impl TryFrom<&Loot> for LootUI {
    type Error = UIConversionError;

    fn try_from(loot: &Loot) -> Result<Self, Self::Error> {
        Ok(LootUI {
            account_type: "Loot".to_string(),
            discriminator: hex::encode(&loot.discriminator),
            version: loot.version,
            sector: loot.sector,
            game_id: loot.game_id.to_string(),
            creator: loot.creator.to_string(),
            items: loot
                .items
                .iter()
                .map(|_item| {
                    LootItemUI {
                        // TODO: Map actual LootInfo fields
                        placeholder: "placeholder".to_string(),
                    }
                })
                .collect(),
        })
    }
}

impl LootUI {
    pub fn from_bytes(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let loot = Loot::from_bytes(data)?;
        Ok(Self::try_from(&loot)?)
    }
}
