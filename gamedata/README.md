# Game Data Exports

This directory contains pre-extracted Star Atlas game balance and world data for use in offline game development.

## Purpose

These data exports allow game developers to:
- Build and test game clients without needing Solana RPC access
- Work with consistent game state snapshots
- Develop game mechanics that mirror the on-chain game

## Structure

Each subdirectory is named after a Game account pubkey and contains:
- `game_balance.ron` or `game_balance.json` - Complete game data including:
  - Game configuration (points, cargo, crafting, mints, vaults, risk zones)
  - Game state information
  - All planets with their properties (name, position, type, size, mining data)
  - All mine items (resources) with their properties
  - All starbases (when available)
- `game_balance_metadata.json` - Metadata about when and how the data was extracted

## Usage

### Rust (with RON format)
```rust
use serde::Deserialize;

#[derive(Deserialize)]
struct GameBalance {
    // Your structure matching the exported data
}

fn load_game_data() -> Result<GameBalance, Box<dyn std::error::Error>> {
    let data = std::fs::read_to_string("gamedata/GAME.../game_balance.ron")?;
    Ok(ron::from_str(&data)?)
}
```

### Godot (with JSON format)
```gdscript
func load_game_data():
    var file = File.new()
    file.open("res://gamedata/GAME.../game_balance.json", File.READ)
    var json_text = file.get_as_text()
    file.close()
    
    var json = JSON.new()
    var result = json.parse(json_text)
    if result == OK:
        return json.data
    else:
        push_error("Failed to parse game data")
        return null
```

## Updating Data

To update this data with the latest from the blockchain:

1. Run `hackr-ixproc` to collect account data
2. Run `hackr-saproc` to parse the accounts
3. Run `hackr-gamedata` to export the data

See the main project README for detailed instructions.