# hackr-saproc

Star Atlas Solana account processor - parses stored account data from the hackr-ixproc database.

## Overview

`hackr-saproc` reads Solana account data stored by `hackr-ixproc` and attempts to parse it into structured Star Atlas account types. It uses discriminators (the first 8 bytes of account data) to identify account types and can process accounts from the following Star Atlas programs:

- **Holosim** (`SAgeTraQfBMdvGVDJYoEvjnbq5szW7RJPi6obDTDQUF`)
- **Player Profile** (`PprofUW1pURCnMW2si88GWPXEEK3Bvh9Tksy8WtnoYJ`)
- **Profile Faction** (`pFACzkX2eSpAjDyEohD6i3VRJvREtH9ynbtM1DwVFsj`)

## Prerequisites

- A SQLite database created by `hackr-ixproc` containing account data
- Rust toolchain installed

## Installation

Build the project:
```bash
cargo build -p hackr-saproc
```

Or build a release binary:
```bash
cargo build -p hackr-saproc --release
```

## Usage

### Basic Commands

Show database statistics:
```bash
hackr-saproc --stats-only
```

Process all accounts (summary mode):
```bash
hackr-saproc
```

Process accounts with different output formats:
```bash
# JSON output for each account
hackr-saproc --output json

# Detailed human-readable output
hackr-saproc --output detailed

# Summary mode (default)
hackr-saproc --output summary
```

### Filter by Program

Process only accounts from a specific program:
```bash
# Process only Holosim accounts
hackr-saproc --program-id SAgeTraQfBMdvGVDJYoEvjnbq5szW7RJPi6obDTDQUF

# Process only Player Profile accounts
hackr-saproc --program-id PprofUW1pURCnMW2si88GWPXEEK3Bvh9Tksy8WtnoYJ

# Process only Profile Faction accounts
hackr-saproc --program-id pFACzkX2eSpAjDyEohD6i3VRJvREtH9ynbtM1DwVFsj
```

### Testing with Limited Data

Process only a limited number of accounts (useful for testing):
```bash
hackr-saproc --limit 10 --output detailed
```

### Custom Database Path

Use a different database file:
```bash
hackr-saproc --database-url /path/to/custom.db
```

## Command Line Options

```
hackr-saproc [OPTIONS]

Options:
      --database-url <DATABASE_URL>
          Database URL (SQLite file path from hackr-ixproc) [default: hackr.db]

      --program-id <PROGRAM_ID>
          Program ID to filter accounts (optional, processes all if not specified)

      --stats-only
          Only show account statistics without processing

      --output <OUTPUT>
          Output format (json, summary, detailed) [default: summary]

      --limit <LIMIT>
          Limit number of accounts to process (for testing)

      --write
          Enable writing parsed accounts to database tables

      --batch-size <BATCH_SIZE>
          Batch size for database writes (default: 1000)

  -h, --help
          Print help
```

## Justfile Commands

If using `just`, the following commands are available:

```bash
# Show database statistics
just hackr-sa-stats

# Process all accounts
just hackr-sa-run

# Process accounts with specific output format
just hackr-sa-run-json
just hackr-sa-run-detailed

# Process limited accounts for testing
just hackr-sa-run-limit 10

# Process specific program accounts
just hackr-sa-holosim
just hackr-sa-player-profile
just hackr-sa-profile-faction

# Development commands
just hackr-sa-check         # Check compilation
just hackr-sa-build-release # Build release binary
just hackr-sa-help          # Show help
```

## Account Type Identification

The processor identifies account types using discriminators (8-byte identifiers at the beginning of account data). Currently configured discriminators are extracted from the Codama IDL files for each program.

### Known Account Types

**Holosim Accounts:**
- CombatConfig, CraftingInstance, DisbandedFleet, Fleet, FleetShips, Game, GameState, Loot, MineItem, Planet, PlayerCrewRecord, ProgressionConfig, Resource, SageCrewConfig, SagePlayerProfile, Sector, Ship, Star, Starbase, StarbasePlayer, SurveyDataUnitTracker

**Player Profile Accounts:**
- PlayerName, Profile, ProfileRoleMembership, Role

**Profile Faction Accounts:**
- ProfileFactionAccount

## Current Status

The parser currently:
1. Reads account data from the SQLite database
2. Identifies the program each account belongs to
3. Attempts to match discriminators to known account types
4. Returns basic information about each account
5. Can save parsed data to program-specific tables with `--write` flag
6. Uses batch processing for efficient database writes

### Not Yet Implemented

- Full deserialization of account data into strongly-typed structures
- Custom account type parsing using the generated Rust types from the workspace

## Development

To extend the parser with actual account deserialization:

1. Update `discriminator.rs` with correct discriminator values if needed
2. Implement the parsing logic in `parser.rs` using the generated types from the workspace crates
3. Add proper error handling for malformed account data

## Troubleshooting

**"Unknown" Account Types**: If many accounts show as "Unknown", the discriminators may not match. This can happen if:
- The discriminators in the code don't match the actual on-chain discriminators
- The account data format has changed
- The accounts belong to different programs than expected

**Database Connection Errors**: Ensure:
- The database file exists and is readable
- The path is correct (relative to where you run the command)
- The database was created by hackr-ixproc

**Performance**: Processing large numbers of accounts:
- Use `--limit` to process fewer accounts during testing
- Use `--program-id` to focus on specific programs
- Use `--output summary` to reduce output overhead
- Adjust `--batch-size` for write performance (default: 1000)
- With batch processing, writing 5000+ accounts takes seconds instead of minutes
