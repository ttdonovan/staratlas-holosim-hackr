# Holosim Viz

A visual galaxy map viewer for Star Atlas Holosim data using Macroquad.

## Features

- **Interactive Galaxy Map**: Pan and zoom through the entire Star Atlas galaxy
- **Sector Visualization**: Color-coded sectors based on content (stars, planets, empty)
- **Starbase Information**: Hover over starbases to see detailed information including:
  - Faction affiliation (color-coded: MUD-Red, ONI-Blue, Ustur-Yellow)
  - Level, state, HP/SP
  - Upkeep resources (Ammo, Food, Toolkit)
  - Mineable resources in the sector
- **Click-to-Pin**: Click on sectors or starbases to pin their information windows at the clicked location
- **Dynamic UI Scaling**: Modal windows scale with zoom level for better readability
- **Grid System**: Toggle grid overlay with sector boundaries and origin marker
- **Minimap**: Overview of the entire galaxy with current camera position
- **Info Panel**: Toggle with F1 to see galaxy statistics

## Controls

- **Movement**: Arrow keys or WASD
- **Zoom**: Mouse wheel
- **Click**: Pin sector or starbase information
- **Right Click / ESC**: Clear pinned information
- **Toggle Info Panel**: F1
- **Toggle Grid**: G (shows sector grid lines)

## Project Structure

```
src/
├── main.rs          # Application entry point and game loop
├── camera.rs        # Camera system for pan/zoom
├── data.rs          # Game data structures and loading
├── ui.rs            # UI state management
├── input.rs         # Input handling
└── render/          # Rendering modules
    ├── mod.rs       # Module exports
    ├── sectors.rs   # Sector rendering
    ├── starbases.rs # Starbase rendering and modals
    ├── ui.rs        # UI overlay rendering
    ├── minimap.rs   # Minimap rendering
    └── utils.rs     # Rendering utilities
```

## Building and Running

```bash
cargo run -p holosim-viz
```

Or use the provided script:

```bash
./run-holosim-viz.sh
```

## Data Requirements

The application expects game data in RON format at:
- `gamedata/GAMEC7U7cqmFFaRow33j1LwuV8u4YhAS1mJ5Dqjnar2k/game_balance.ron`

This data can be generated using the `hackr-gamedata` tool.