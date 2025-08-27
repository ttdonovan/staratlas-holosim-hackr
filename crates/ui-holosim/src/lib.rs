pub mod fleet;
pub mod fleet_ships;
pub mod game;
pub mod game_state;
pub mod loot;
pub mod mine_item;
pub mod planet;
pub mod resource;
pub mod sage_player_profile;
pub mod sector;
pub mod ship;
pub mod star;
pub mod starbase;

pub use fleet::FleetUI;
pub use fleet_ships::FleetShipsUI;
pub use game::GameUI;
pub use game_state::GameStateUI;
pub use loot::LootUI;
pub use mine_item::MineItemUI;
pub use planet::PlanetUI;
pub use resource::ResourceUI;
pub use sage_player_profile::SagePlayerProfileUI;
pub use sector::SectorUI;
pub use ship::ShipUI;
pub use star::StarUI;
pub use starbase::StarbaseUI;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum UIConversionError {
    #[error("Failed to convert string to UTF-8: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),

    #[error("Failed to serialize to JSON: {0}")]
    SerializationError(#[from] serde_json::Error),
}
