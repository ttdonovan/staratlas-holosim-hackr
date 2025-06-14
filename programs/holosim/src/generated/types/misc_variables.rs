//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Miscellaneous game state variables
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MiscVariables {
    /// Percentage by which the "warp lane" movement type reduces warp fuel cost
    pub warp_lane_fuel_cost_reduction: i16,
    /// Respawn fee; You cannot enter into the respawning state without paying this fee
    /// Since ATLAS has 8 decimal places, units are in the smallest value of ATLAS possible.
    pub respawn_fee: u64,
    /// Percentage by which to reduce the asteroid mining rate if a starbase ammo upkeep coffer is empty
    pub upkeep_mining_emissions_penalty: i16,
}
