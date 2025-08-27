# List all available commands
default:
    @just --list

# Codama Generation
codama-create-idl:
    bun run scripts/createCodamaIDL.ts

codama-generate-rs:
    bun run scripts/generateCode.ts

# Documentation
doc:
    cargo doc --no-deps --open

# hackr-ixproc (Instruction Processor)
ix-run:
    cargo run -p hackr-ixproc

ix-run-db:
    cargo run -p hackr-ixproc --features database -- --database --dump-accounts

ix-build:
    cargo build -p hackr-ixproc --features database --release

# hackr-saproc (Star Atlas Processor)  
sa-run:
    cargo run -p hackr-saproc

sa-run-write:
    cargo run -p hackr-saproc -- --write

sa-stats:
    cargo run -p hackr-saproc -- --stats-only

sa-build:
    cargo build -p hackr-saproc --release

# hackr-gamedata (Game Balance Extractor)
gd-run:
    cargo run -p hackr-gamedata

gd-run-game GAME_PUBKEY:
    cargo run -p hackr-gamedata -- --game-pubkey {{GAME_PUBKEY}}

gd-run-json:
    cargo run -p hackr-gamedata -- --format json

gd-build:
    cargo build -p hackr-gamedata --release

# Database Management
db-schema:
    sqlite3 hackr.db ".schema"

db-schema-export:
    @echo "Exporting database schema..."
    @sqlite3 hackr.db ".schema" > docs/database-schema.sql
    @echo "Schema exported to docs/database-schema.sql"

clean-db:
    rm -f hackr.db hackr.db-shm hackr.db-wal

# General Commands
check:
    cargo check --workspace

build:
    cargo build --workspace --release

test:
    cargo test --workspace

fmt:
    cargo fmt --all

clippy:
    cargo clippy --all-targets --all-features -- -D warnings