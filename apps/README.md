# Star Atlas Holosim Visualization Apps

This directory contains visualization and interaction tools for the extracted Star Atlas game data.

## Apps

### holosim-tui
A Terminal User Interface (TUI) application for browsing and analyzing game data.

**Features:**
- Browse Ships, Planets, Sectors, and Starbases
- Paginated table views with detailed information
- Keyboard navigation
- Fast data exploration

**Run:**
```bash
cargo run -p holosim-tui
```

**Controls:**
- Main Menu: Press 1-4 to select data type
- Data Views: ↑/↓ to navigate, PgUp/PgDn for pages, ESC/q to go back

### holosim-viz
A graphical visualization using Macroquad for exploring the galaxy map.

**Features:**
- Interactive 2D galaxy map
- Visual representation of sectors, starbases
- Pan and zoom controls
- Minimap for navigation
- Hover information

**Run:**
```bash
cargo run -p holosim-viz
```

**Controls:**
- WASD/Arrow Keys: Pan camera
- Mouse Scroll: Zoom in/out
- F1: Toggle info panel
- F2: Toggle search (future feature)

## Data Format

Both applications read the extracted game data from:
```
../gamedata/GAMEC7U7cqmFFaRow33j1LwuV8u4YhAS1mJ5Dqjnar2k/game_balance.ron
```

The data includes:
- 6,244 Sectors (map regions)
- 949 Ships (ship types/configurations)
- 264 Planets
- 97 Resources (mineable locations)
- 51 Starbases
- 51 Stars
- 13 Mine Items

## Development

To add new features:
1. TUI app: Modify `src/app.rs` for new screens and `src/main.rs` for UI
2. Viz app: Add new rendering in `src/main.rs` and interaction in `handle_input`

Both apps use the shared `ui-holosim` crate for data types.