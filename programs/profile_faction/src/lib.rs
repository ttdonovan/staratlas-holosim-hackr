use solana_program::{pubkey, pubkey::Pubkey};

pub mod generated;

/// Re-export `generated` as `profile_faction`;
pub use generated as profile_faction;

/// `profile_faction` program ID.
pub const PROFILE_FACTION_ID: Pubkey = pubkey!("pFACzkX2eSpAjDyEohD6i3VRJvREtH9ynbtM1DwVFsj");
