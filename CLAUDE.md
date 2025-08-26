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
- `cargo doc -p staratlas-holosim --no-deps --open` - Alternative doc command

### Build and Test
- `cargo build` - Build Rust workspace
- `cargo test` - Run tests
- `bun install` - Install TypeScript dependencies
- `bun run scripts/createCodamaIDL.ts` - Run IDL creation script
- `bun run scripts/generateCode.ts` - Run code generation script

### hackr-ixproc (Solana Program Monitor)
- `cargo run -p hackr-ixproc` - Run real-time Solana program activity monitor
- `cargo build -p hackr-ixproc` - Build instruction processor binary
- Requires `.env` file in `crates/hackr-ixproc/` (copy from `.env.example`)
- **Dual monitoring**: Transaction logs + account changes for comprehensive program tracking
- HTTP API: `http://localhost:8080/health` and `http://localhost:8080/stats`
- Uses Solana's native pubsub client with automatic reconnection

### Examples
- `cargo run -p holosim-examples --example ch2_get_accounts` - Get program accounts
- `cargo run -p holosim-examples --example ch2_pubsub_accounts` - Monitor program activity
- `cargo run -p holosim-examples --example ch3_create_profile` - Create player profile

## Architecture

### Workspace Structure
- **programs/**: Solana program bindings and generated client code
  - `holosim/` - Main Star Atlas Holosim program bindings
  - `player_profile/` - Player profile program bindings  
  - `profile_faction/` - Profile faction program bindings
  - `c4_sage/` - C4 Sage program bindings
- **crates/**: Additional Rust crates and tools
  - `hackr-ixproc/` - Lite instruction processor for real-time Solana program monitoring
- **holosim-examples/**: Example usage and integration code
- **scripts/**: TypeScript scripts for code generation
- **vault/**: Keypair storage (development only)

### Key Program IDs
- Holosim: `SAgeTraQfBMdvGVDJYoEvjnbq5szW7RJPi6obDTDQUF`
- Player Profile: `PprofUW1pURCnMW2si88GWPXEEK3Bvh9Tksy8WtnoYJ`
- Profile Faction: `pFACzkX2eSpAjDyEohD6i3VRJvREtH9ynbtM1DQUF`
- C4 Sage: `C4SAgeKLgb3pTLWhVr6NRwWyYFuTR7ZeSXFrzoLwfMzF`

### Code Generation Flow
1. Anchor IDLs are stored alongside programs (e.g., `SAgeTraQfBMdvGVDJYoEvjnbq5szW7RJPi6obDTDQUF-idl.json`)
2. `createCodamaIDL.ts` converts Anchor IDLs to Codama format
3. `generateCode.ts` renders Rust client code from Codama IDLs
4. Generated code is placed in `programs/*/src/generated/`

### Environment Configuration
- Uses `.env` files for configuration (RPC_URL, KEYPAIR_PATH)
- Default RPC: `https://rpc.ironforge.network/devnet?apiKey=...`
- Development keypairs stored in `vault/` directory
- Solana devnet for development, mainnet references for program dumps

## Development Workflow

1. Set up Solana CLI and configure for devnet
2. Generate development keypair: `solana-keygen new -o ./vault/holosim_id.json`
3. Airdrop devnet SOL: `solana airdrop 2`
4. Regenerate client code when IDLs change: `just codama-create-idl && just codama-generate-rs`
5. Run examples to test functionality
6. Use `cargo doc` to browse generated documentation

## Dependencies

- **Rust**: Latest stable (1.89.0+)
- **Solana CLI**: v2.3.8+ (Agave)
- **Bun**: v1.2.15+ for TypeScript execution
- **Just**: Command runner for development tasks

## Notes

- Generated client code should not be manually edited - regenerate instead
- Some Codama instructions are disabled in `createCodamaIDL.ts` due to rendering issues
- Program binaries (.so files) are stored alongside source for reference
- Uses Solana 2.2 dependencies across the workspace