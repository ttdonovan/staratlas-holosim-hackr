# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Star Atlas Holosim hackathon project focused on blockchain interactions with Solana programs. The project uses a mixed Rust/TypeScript architecture with Codama for client code generation from Solana program IDLs.

## Development Commands

### Code Generation
- `just codama-create-idl` - Generate Codama IDL from Anchor IDL
- `just codama-generate-rs` - Generate Rust client code from Codama IDL

### Documentation
- `just doc-holosim` - Build and open Holosim documentation

### Build and Test
- `cargo build` - Build Rust workspace
- `cargo test` - Run tests
- `cargo clippy` - Run linter
- `cargo fmt` - Format code
- `bun install` - Install TypeScript dependencies

### hackr-ixproc (Solana Program Monitor)
```bash
# Run without database (lite mode)
just hackr-ix-run

# Run with database support
just hackr-ix-run-db

# Run with database and dump existing accounts on startup
just hackr-ix-run-db-dump

# Build release binary with database support
just hackr-ix-build-release

# Clean database files
just hackr-ix-clean-db
```

hackr-ixproc features:
- Real-time monitoring of Solana program activity via WebSocket
- Optional SQLite database persistence (feature-gated)
- Dual monitoring: Transaction logs + account changes
- Account dumping on startup (fetches all program accounts)
- HTTP API endpoints: `/health` and `/stats`
- Batch database operations for performance

### hackr-saproc (Star Atlas Account Processor)
```bash
# Show database statistics
just hackr-sa-stats

# Process all accounts
just hackr-sa-run

# Process with different output formats
just hackr-sa-run-json
just hackr-sa-run-detailed

# Process specific program accounts
just hackr-sa-holosim
just hackr-sa-player-profile
just hackr-sa-profile-faction

# Test with limited accounts
just hackr-sa-run-limit 10
```

hackr-saproc features:
- Reads account data from hackr-ixproc SQLite database
- Identifies account types using discriminators
- Multiple output formats (summary, detailed, JSON)
- Program ID filtering
- Currently identifies account types but full deserialization is TODO

### Examples
- `cargo run -p holosim-examples --example ch2_get_accounts` - Get program accounts
- `cargo run -p holosim-examples --example ch2_pubsub_accounts` - Monitor program activity

## Architecture

### Workspace Structure
- **programs/**: Solana program bindings and generated client code
  - `holosim/` - Main Star Atlas Holosim program bindings
  - `player_profile/` - Player profile program bindings
  - `profile_faction/` - Profile faction program bindings
  - `c4_sage/` - C4 Sage program bindings
- **crates/**: Additional Rust crates and tools
  - `hackr-ixproc/` - Real-time Solana program monitor with optional database persistence
  - `hackr-saproc/` - Star Atlas account data processor and parser
- **holosim-examples/**: Example usage and integration code
- **scripts/**: TypeScript scripts for code generation

### Key Program IDs
- Holosim: `SAgeTraQfBMdvGVDJYoEvjnbq5szW7RJPi6obDTDQUF`
- Player Profile: `PprofUW1pURCnMW2si88GWPXEEK3Bvh9Tksy8WtnoYJ`
- Profile Faction: `pFACzkX2eSpAjDyEohD6i3VRJvREtH9ynbtM1DwVFsj`
- C4 Sage: `C4SAgeKLgb3pTLWhVr6NRwWyYFuTR7ZeSXFrzoLwfMzF`

### Data Flow Architecture

1. **hackr-ixproc** monitors Solana programs in real-time:
   - Subscribes to program logs and account changes via WebSocket
   - Optionally persists data to SQLite database
   - Can dump all program accounts on startup
   - Stores complete account data including discriminators

2. **hackr-saproc** processes stored account data:
   - Reads from hackr-ixproc's SQLite database
   - Uses discriminators to identify account types
   - Parses account data using workspace crates (TODO: full implementation)
   - Outputs in various formats for analysis

### Database Schema
The SQLite database (when enabled) contains:
- **accounts** table: Stores Solana account data with program_id, pubkey, data blob, etc.
- **transaction_logs** table: Stores program transaction logs
- Indexes on program_id and account_pubkey for performance

### Discriminator Handling
- Account types are identified by 8-byte discriminators at the start of account data
- Discriminator values are extracted from codamaIDL.json files
- Note: Actual on-chain discriminators may differ from IDL values (related to anchor-idl-build feature)

### Code Generation Flow
1. Anchor IDLs stored alongside programs (e.g., `*-idl.json`)
2. `createCodamaIDL.ts` converts Anchor IDLs to Codama format
3. `generateCode.ts` renders Rust client code from Codama IDLs
4. Generated code placed in `programs/*/src/generated/`

### Environment Configuration
- `.env` files for configuration (RPC_URL, KEYPAIR_PATH, DATABASE_URL)
- Default RPC: `https://rpc.ironforge.network/devnet?apiKey=...`
- Development keypairs in `vault/` directory
- Database URL defaults to `hackr.db`

## Development Workflow

1. Set up environment and dependencies
2. Run hackr-ixproc with database to collect account data
3. Use hackr-saproc to analyze collected accounts
4. Regenerate client code when IDLs change: `just codama-create-idl && just codama-generate-rs`

## Common Tasks

### Monitoring Star Atlas Programs
```bash
# Start monitoring with database persistence
just hackr-ix-run-db-dump

# In another terminal, view collected accounts
just hackr-sa-stats
just hackr-sa-run
```

### Adding New Account Type Parsing
1. Update discriminator mappings in `crates/hackr-saproc/src/discriminator.rs`
2. Implement parsing logic in `crates/hackr-saproc/src/parser.rs`
3. Use generated types from workspace program crates

## Dependencies

- **Rust**: Latest stable (1.81.0+)
- **Solana CLI**: v2.0.3+ (Agave)
- **Bun**: v1.0.26+ for TypeScript
- **Just**: Command runner
- **SQLite**: For optional database features

## Important Notes

- Generated code should not be manually edited
- Database feature in hackr-ixproc is optional (use `--features database`)
- Account discriminators may not match IDL values due to anchor-idl-build
- Batch operations used for performance with large account sets (55,000+)
- Program binaries (.so files) stored for reference
