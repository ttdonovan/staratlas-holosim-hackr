use solana_program::{pubkey, pubkey::Pubkey};

pub mod generated;

/// Re-export `generated` as `player_profile`;
pub use generated as player_profile;

/// `player_profile` program ID.
pub const PLAYER_PROFILE_ID: Pubkey = pubkey!("PprofUW1pURCnMW2si88GWPXEEK3Bvh9Tksy8WtnoYJ");
