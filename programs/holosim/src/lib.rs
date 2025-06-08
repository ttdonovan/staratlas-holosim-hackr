use solana_program::{pubkey, pubkey::Pubkey};

pub mod generated;

/// Re-export `generated` as `holosim`;
pub use generated as holosim;

/// `sage` program ID.
pub const SAGE_ID: Pubkey = pubkey!("SAgeTraQfBMdvGVDJYoEvjnbq5szW7RJPi6obDTDQUF");
