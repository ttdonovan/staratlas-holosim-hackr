# hackr-gamedata

Extract Star Atlas game balance/configuration data from the local database for game engine initialization.

## Overview

`hackr-gamedata` reads Game accounts and all associated game data from the SQLite database populated by `hackr-ixproc` and `hackr-saproc`, then exports them in formats suitable for loading into game engines like Macroquad or Godot. This allows you to initialize your game world with the same configuration parameters and world data used by the on-chain game.

## Prerequisites

1. Run `hackr-ixproc` with database enabled to collect account data
2. Run `hackr-saproc` with write mode to parse accounts into the `holosim_accounts` table
3. Ensure you have Game accounts in the database

## Purpose

When building an off-chain game client or simulation that mirrors Star Atlas gameplay, you need access to the core game configuration data and world state. This tool:

1. Reads Game accounts from the `holosim_accounts` table
2. Fetches associated GameState accounts
3. Extracts all related game world data:
   - Planet accounts (all planets in the game)
   - MineItem accounts (minable resources)
   - Starbase accounts (when available)
4. Exports the data in RON or JSON format
5. Organizes output in game-specific subdirectories
6. Creates metadata about when/how the data was extracted

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

The tool creates a subdirectory for each game (using the game pubkey) containing:

1. **Game balance data** (`game_balance.ron` or `.json`)
   - Game configuration data
   - GameState information
   - All Planets in the game world
   - All MineItems (resources)
   - All Starbases (when available)
   - Ready to load into your game engine

2. **Metadata** (`game_balance_metadata.json`)
   - Extraction timestamp
   - Version information
   - Count of each entity type
   - Database source

Example output structure:
```
gamedata/
└── GAMEC7U7cqmFFaRow33j1LwuV8u4YhAS1mJ5Dqjnar2k/
    ├── game_balance.ron
    └── game_balance_metadata.json
```

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

- **World Data**
  - **Planets**: All planets with their properties
    - Name, position (sector & sub-coordinates)
    - Planet type, size, health
    - Mining stats (amount mined, resources, miners)
  - **Mine Items**: All minable resources
    - Resource name, mint address
    - Hardness, number of resource accounts
  - **Starbases**: Space station data (when available)

## Future Enhancements

Currently, the nested configuration structures (Points, Cargo, Crafting, etc.) are exported as placeholders. To fully extract these:

1. Examine the actual Holosim type definitions
2. Update the UI types in `ui-holosim` crate
3. Map the nested structures in `GameBalance`

This would provide complete game balance data for accurate off-chain simulation.