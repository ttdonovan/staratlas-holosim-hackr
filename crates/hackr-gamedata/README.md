# hackr-gamedata

Extract Star Atlas game balance/configuration data from the local database for game engine initialization.

## Overview

`hackr-gamedata` reads Game and GameState accounts from the SQLite database populated by `hackr-ixproc` and `hackr-saproc`, then exports them in formats suitable for loading into game engines like Macroquad or Godot. This allows you to initialize your game world with the same configuration parameters used by the on-chain game.

## Prerequisites

1. Run `hackr-ixproc` with database enabled to collect account data
2. Run `hackr-saproc` with write mode to parse accounts into the `holosim_accounts` table
3. Ensure you have Game accounts in the database

## Purpose

When building an off-chain game client or simulation that mirrors Star Atlas gameplay, you need access to the core game configuration data. This tool:

1. Reads Game accounts from the `holosim_accounts` table
2. Optionally fetches associated GameState accounts
3. Exports the data in RON or JSON format
4. Creates metadata about when/how the data was extracted

## Usage

### Basic Command

```bash
# Export all Game accounts from database
hackr-gamedata

# Export a specific game
hackr-gamedata --game-pubkey <GAME_PUBKEY>
```

### Examples

```bash
# Export all Game accounts to RON format (default)
hackr-gamedata

# Export specific game to JSON format
hackr-gamedata --game-pubkey ABC123...XYZ --format json

# Skip GameState fetch
hackr-gamedata --include-game-state false

# Custom database and output directory
hackr-gamedata --database-url ./mydata.db --output-dir ./exports

# Custom output filename
hackr-gamedata --game-pubkey ABC123...XYZ --output-name my_game_config
```

### Output Files

The tool creates two files:

1. **Game balance data** (`game_balance_<pubkey>.ron` or `.json`)
   - Contains all configuration data
   - Ready to load into your game engine

2. **Metadata** (`game_balance_<pubkey>_metadata.json`)
   - Extraction timestamp
   - Version information
   - RPC endpoint used

## Integration with Game Engines

### Macroquad Example

```rust
use serde::Deserialize;

#[derive(Deserialize)]
struct GameBalance {
    // Your game balance structure
}

async fn load_game_config() -> GameBalance {
    let config_str = std::fs::read_to_string("gamedata/game_balance.ron").unwrap();
    ron::from_str(&config_str).unwrap()
}
```

### Godot Integration

Export as JSON and load in GDScript:

```gdscript
func load_game_config():
    var file = File.new()
    file.open("res://gamedata/game_balance.json", File.READ)
    var json_text = file.get_as_text()
    file.close()
    
    var json = JSON.new()
    var parse_result = json.parse(json_text)
    return json.data
```

## Command Line Options

```
Options:
      --database-url <DATABASE_URL>
          Database URL (SQLite file path) [env: DATABASE_URL] [default: hackr.db]

      --game-pubkey <GAME_PUBKEY>
          Game account public key (optional - exports all if not specified)

      --include-game-state <INCLUDE_GAME_STATE>
          Also fetch associated GameState accounts [default: true]

      --format <FORMAT>
          Output format (ron or json) [default: ron]

      --output-dir <OUTPUT_DIR>
          Output directory for exported files [default: ./gamedata]

      --output-name <OUTPUT_NAME>
          Output filename (without extension)

  -h, --help
          Print help
```

## Data Structure

The exported data includes:

- **Game Configuration**
  - Points/scoring settings
  - Cargo/inventory configuration
  - Crafting system parameters
  - Token mint information
  - Vault/treasury settings
  - Risk zone definitions

- **Game State** (if included)
  - Fleet information
  - Miscellaneous variables
  - Dynamic game state

## Future Enhancements

Currently, the nested configuration structures (Points, Cargo, Crafting, etc.) are exported as placeholders. To fully extract these:

1. Examine the actual Holosim type definitions
2. Update the UI types in `ui-holosim` crate
3. Map the nested structures in `GameBalance`

This would provide complete game balance data for accurate off-chain simulation.