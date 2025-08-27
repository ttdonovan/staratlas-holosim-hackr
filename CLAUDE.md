# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository Overview

This is the **Star Atlas: Holosim Hackr** project - a Solana blockchain analysis toolkit focused on Star Atlas programs. The codebase consists of client libraries generated from IDL files, data processing tools, and examples for interacting with Star Atlas on-chain programs.

## Architecture

### Key Programs Monitored
- **Holosim (SAGE)**: `SAgeTraQfBMdvGVDJYoEvjnbq5szW7RJPi6obDTDQUF`
- **Player Profile**: `PprofUW1pURCnMW2si88GWPXEEK3Bvh9Tksy8WtnoYJ`
- **Profile Faction**: `pFACzkX2eSpAjDyEohD6i3VRJvREtH9ynbtM1DwVFsj`

### Crate Structure

1. **Generated Program Libraries** (`programs/`)
   - `staratlas-holosim`: Auto-generated Rust client from Holosim IDL
   - `staratlas-player-profile`: Auto-generated client for Player Profile
   - `staratlas-profile-faction`: Auto-generated client for Profile Faction
   - Generated using Codama from IDL files

2. **Processing Tools** (`crates/`)
   - `hackr-ixproc`: Real-time instruction processor monitoring blockchain activity
     - Optional SQLite database feature for storing account data
     - Dual pubsub streams (logs + account changes)
   - `hackr-saproc`: Star Atlas account parser reading from hackr-ixproc database
     - Batch processing for performance
     - Discriminator-based account type identification
   - `ui-holosim`: UI-friendly representations of Holosim account types
     - Strongly-typed structs with `TryFrom` implementations
     - Clean JSON serialization for frontend consumption

3. **Examples** (`holosim-examples/`)
   - Example code for interacting with Star Atlas programs

### Data Flow Architecture

```
Solana Blockchain
    ↓
hackr-ixproc (real-time monitoring)
    ├─ Transaction logs
    └─ Account changes
    ↓
SQLite Database (hackr.db)
    ↓
hackr-saproc (batch processing)
    ├─ Discriminator identification
    └─ Account deserialization (via ui-holosim)
    ↓
Parsed JSON data
```

## Development Commands

### Environment Setup
```bash
# Install dependencies
rustup update
bun install
cargo install just

# Generate client code from IDL
just codama-create-idl
just codama-generate-rs
```

### Common Development Tasks
```bash
# Check workspace compilation
just check

# Run formatting
just fmt

# Run clippy linter
just clippy

# Build entire workspace
just build

# Run tests
just test

# Generate documentation
just doc
```

### Running the Instruction Processor
```bash
# Basic mode (no persistence)
just ix-run

# With database storage and account dumping
just ix-run-db

# Build release version
just ix-build
```

### Running the Account Parser
```bash
# Show database statistics
just sa-stats

# Process accounts (read-only)
just sa-run

# Process and save parsed data
just sa-run-write

# Build release version
just sa-build
```

### Database Management
```bash
# View database schema
just db-schema

# Export schema documentation
just db-schema-export

# Clean database files
just clean-db
```

## Working with Account Types

When adding new account types to parse:

1. **Create UI type** in `crates/ui-holosim/src/`
   ```rust
   pub struct NewAccountUI {
       pub account_type: String,
       pub discriminator: String,
       // ... fields
   }

   impl TryFrom<&NewAccount> for NewAccountUI { ... }
   ```

2. **Export from lib.rs**
   ```rust
   pub mod new_account;
   pub use new_account::NewAccountUI;
   ```

3. **Update parser** in `crates/hackr-saproc/src/parser.rs`
   ```rust
   "NewAccount" => match NewAccountUI::from_bytes(data) {
       Ok(ui) => serde_json::to_value(ui).map_err(Into::into),
       Err(e) => Ok(error_response(e.as_ref())),
   },
   ```

## Key Implementation Details

### Discriminators
Account types are identified by 8-byte discriminators at the start of account data. These are mapped in `crates/hackr-saproc/src/discriminator.rs`.

### Borsh Versioning
Note that the workspace uses borsh 1.5 while hackr-saproc uses borsh 0.10. This affects deserialization method names:
- Use `from_bytes()` instead of `try_from_slice()` for generated types

### Database Performance
hackr-saproc uses batch transactions for database writes. Default batch size is 1000 records, configurable via `--batch-size`.

### Environment Variables
Create `.env` files for configuration:
- `RPC_URL`: Solana HTTP endpoint
- `RPC_WS_URL`: Solana WebSocket endpoint
- `DATABASE_URL`: SQLite database path
- `PROGRAMS`: Comma-separated program IDs to monitor

## Testing

### Single Test
```bash
cargo test -p <crate_name> <test_name>
```

### Integration Testing
```bash
# Run hackr-ixproc to collect data
just ix-run-db

# Process collected accounts
just sa-run --limit 100 --output detailed
```

## Examples

```bash
# Get program accounts example
cargo run -p holosim-examples --example ch2_get_accounts

# Monitor program activity example
cargo run -p holosim-examples --example ch2_pubsub_accounts
```

## Claude Memories

- Observed memory: `to meorize` (seems like a misspelled fragment)