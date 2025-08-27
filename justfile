# Create a Codama IDL
codama-create-idl:
    bun run scripts/createCodamaIDL.ts

# Generate (render) Rust client code
codama-generate-rs:
    bun run scripts/generateCode.ts

# Build Holosim's documentation
doc-holosim:
    cargo doc -p staratlas-holosim --no-deps --open

# hackr-ixproc commands
# Run hackr-ixproc in lite mode (no database)
hackr-ix-run:
    cargo run -p hackr-ixproc

# Run hackr-ixproc with database support
hackr-ix-run-db:
    cargo run -p hackr-ixproc --features database -- --database

# Run hackr-ixproc with database and dump existing accounts on startup
hackr-ix-run-db-dump:
    cargo run -p hackr-ixproc --features database -- --database --dump-accounts

# Run hackr-ixproc with custom database file and account dumping
hackr-ix-run-db-custom DB_FILE:
    cargo run -p hackr-ixproc --features database -- --database --database-url {{DB_FILE}} --dump-accounts

# Check hackr-ixproc compilation (lite mode)
hackr-ix-check:
    cargo check -p hackr-ixproc

# Check hackr-ixproc compilation with database features
hackr-ix-check-db:
    cargo check -p hackr-ixproc --features database

# Build hackr-ixproc release binary with database support
hackr-ix-build-release:
    cargo build -p hackr-ixproc --features database --release

# Show hackr-ixproc help
hackr-ix-help:
    cargo run -p hackr-ixproc --features database -- --help

# Clean up generated database files
hackr-ix-clean-db:
    rm -f hackr_ixproc.db hackr_ixproc.db-shm hackr_ixproc.db-wal

# hackr-saproc commands
# Show database statistics for stored accounts
hackr-sa-stats:
    cargo run -p hackr-saproc -- --stats-only

# Process all accounts in the database
hackr-sa-run:
    cargo run -p hackr-saproc

# Process accounts for a specific program
hackr-sa-run-program PROGRAM_ID:
    cargo run -p hackr-saproc -- --program-id {{PROGRAM_ID}}

# Process accounts with JSON output
hackr-sa-run-json:
    cargo run -p hackr-saproc -- --output json

# Process accounts with detailed output
hackr-sa-run-detailed:
    cargo run -p hackr-saproc -- --output detailed

# Process limited number of accounts (for testing)
hackr-sa-run-limit COUNT:
    cargo run -p hackr-saproc -- --limit {{COUNT}} --output detailed

# Process Holosim accounts only
hackr-sa-holosim:
    cargo run -p hackr-saproc -- --program-id SAgeTraQfBMdvGVDJYoEvjnbq5szW7RJPi6obDTDQUF

# Process Player Profile accounts only
hackr-sa-player-profile:
    cargo run -p hackr-saproc -- --program-id PprofUW1pURCnMW2si88GWPXEEK3Bvh9Tksy8WtnoYJ

# Process Profile Faction accounts only
hackr-sa-profile-faction:
    cargo run -p hackr-saproc -- --program-id pFACzkX2eSpAjDyEohD6i3VRJvREtH9ynbtM1DwVFsj

# Check hackr-saproc compilation
hackr-sa-check:
    cargo check -p hackr-saproc

# Build hackr-saproc release binary
hackr-sa-build-release:
    cargo build -p hackr-saproc --release

# Show hackr-saproc help
hackr-sa-help:
    cargo run -p hackr-saproc -- --help
