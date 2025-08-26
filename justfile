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
hackr-run:
    cargo run -p hackr-ixproc

# Run hackr-ixproc with database support
hackr-run-db:
    cargo run -p hackr-ixproc --features database -- --database

# Run hackr-ixproc with database and dump existing accounts on startup
hackr-run-db-dump:
    cargo run -p hackr-ixproc --features database -- --database --dump-accounts

# Run hackr-ixproc with custom database file and account dumping
hackr-run-db-custom DB_FILE:
    cargo run -p hackr-ixproc --features database -- --database --database-url {{DB_FILE}} --dump-accounts

# Check hackr-ixproc compilation (lite mode)
hackr-check:
    cargo check -p hackr-ixproc

# Check hackr-ixproc compilation with database features
hackr-check-db:
    cargo check -p hackr-ixproc --features database

# Build hackr-ixproc release binary with database support
hackr-build-release:
    cargo build -p hackr-ixproc --features database --release

# Show hackr-ixproc help
hackr-help:
    cargo run -p hackr-ixproc --features database -- --help
