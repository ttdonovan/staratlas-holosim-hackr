# ui-holosim

A UI-friendly representation layer for Star Atlas Holosim account types.

## Overview

This crate provides strongly-typed Rust structures for parsed Holosim account data, designed specifically for UI consumption. It converts raw on-chain account data into clean, serializable structures.

## Features

- Type-safe UI structures for all major Holosim account types
- Automatic conversion from raw account bytes using `TryFrom` trait
- Clean JSON serialization with proper field names
- Error handling for deserialization failures
- Handles special Solana types (like `OptionalNonSystemPubkey`)

## Supported Account Types

- `FleetUI` - Fleet accounts with owner, ships, and resources
- `FleetShipsUI` - Fleet ship composition data
- `LootUI` - Loot available at locations
- `SagePlayerProfileUI` - Player profile data
- `ShipUI` - Ship specifications and stats
- `StarbaseUI` - Starbase location and faction data

## Usage

```rust
use ui_holosim::FleetUI;

// From raw bytes
let fleet_ui = FleetUI::from_bytes(&account_data)?;

// From Fleet struct
let fleet = Fleet::from_bytes(&account_data)?;
let fleet_ui = FleetUI::try_from(&fleet)?;

// Serialize to JSON
let json = serde_json::to_value(fleet_ui)?;
```

## Example

```rust
use ui_holosim::ShipUI;

// Parse ship account data
let ship_ui = ShipUI::from_bytes(&ship_account_data)?;

// Access nested stats
println!("Ship name: {}", ship_ui.name);
println!("Cargo capacity: {}", ship_ui.stats.cargo_stats.cargo_capacity);
println!("Max warp distance: {}", ship_ui.stats.movement_stats.max_warp_distance);
```

## Integration with hackr-saproc

This crate is designed to work seamlessly with `hackr-saproc`:

```rust
match account_type {
    "Fleet" => {
        match FleetUI::from_bytes(data) {
            Ok(fleet_ui) => serde_json::to_value(fleet_ui)?,
            Err(e) => // handle error
        }
    }
    // ... other account types
}
```