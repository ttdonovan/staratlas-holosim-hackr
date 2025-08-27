use crate::UIConversionError;
use serde::{Deserialize, Serialize};
use staratlas_holosim::generated::accounts::MineItem;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MineItemUI {
    pub account_type: String,
    pub discriminator: String,
    pub version: u8,
    pub game_id: String,
    pub name: String,
    pub mint: String,
    pub resource_hardness: u16,
    pub num_resource_accounts: u64,
    pub bump: u8,
}

impl TryFrom<&MineItem> for MineItemUI {
    type Error = UIConversionError;

    fn try_from(mine_item: &MineItem) -> Result<Self, Self::Error> {
        Ok(MineItemUI {
            account_type: "MineItem".to_string(),
            discriminator: hex::encode(&mine_item.discriminator),
            version: mine_item.version,
            game_id: mine_item.game_id.to_string(),
            name: std::str::from_utf8(&mine_item.name)?
                .trim_matches('\0')
                .to_string(),
            mint: mine_item.mint.to_string(),
            resource_hardness: mine_item.resource_hardness,
            num_resource_accounts: mine_item.num_resource_accounts,
            bump: mine_item.bump,
        })
    }
}

impl MineItemUI {
    pub fn from_bytes(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let mine_item = MineItem::from_bytes(data)?;
        Ok(Self::try_from(&mine_item)?)
    }
}
