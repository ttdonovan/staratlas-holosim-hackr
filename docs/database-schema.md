# Database Schema Documentation

This document describes the database schema used by hackr-ixproc and hackr-saproc for storing Solana account data and parsed Star Atlas account information.

## Overview

The database consists of two main categories of tables:
1. **Raw account storage** - stores complete Solana account data fetched from the blockchain
2. **Parsed account storage** - stores parsed and structured Star Atlas account data

## Tables

### accounts
Stores raw Solana account data for all monitored programs.

| Column | Type | Description |
|--------|------|-------------|
| id | TEXT PRIMARY KEY | Unique identifier (UUID) |
| program_id | TEXT NOT NULL | Solana program ID that owns the account |
| account_pubkey | TEXT NOT NULL UNIQUE | Solana public key of the account |
| lamports | INTEGER NOT NULL | Account balance in lamports |
| data | BLOB NOT NULL | Raw account data bytes |
| owner | TEXT NOT NULL | Account owner program ID |
| executable | BOOLEAN NOT NULL | Whether the account is executable |
| rent_epoch | INTEGER NOT NULL | Rent epoch for the account |
| created_at | TEXT NOT NULL | ISO 8601 timestamp when first seen |
| updated_at | TEXT NOT NULL | ISO 8601 timestamp when last updated |

**Indexes:**
- `idx_accounts_pubkey` on `account_pubkey`
- `idx_accounts_program_id` on `program_id`

### transaction_logs
Stores transaction logs for monitored programs.

| Column | Type | Description |
|--------|------|-------------|
| id | TEXT PRIMARY KEY | Unique identifier (UUID) |
| program_id | TEXT NOT NULL | Solana program ID |
| signature | TEXT NOT NULL | Transaction signature |
| slot | INTEGER NOT NULL | Slot number when transaction occurred |
| logs | TEXT NOT NULL | JSON array of log messages |
| created_at | TEXT NOT NULL | ISO 8601 timestamp |

**Indexes:**
- `idx_transaction_logs_signature` on `signature`
- `idx_transaction_logs_program_id` on `program_id`

### holosim_accounts
Stores parsed Star Atlas Holosim account data.

| Column | Type | Description |
|--------|------|-------------|
| id | TEXT PRIMARY KEY | Unique identifier (UUID) |
| account_pubkey | TEXT NOT NULL UNIQUE | Solana public key of the account |
| account_type | TEXT NOT NULL | Account type (e.g., Fleet, FleetShips, Loot) |
| parsed_data | TEXT NOT NULL | JSON representation of parsed account data |
| raw_data_hash | TEXT NOT NULL | SHA256 hash of raw account data |
| created_at | TEXT NOT NULL | ISO 8601 timestamp when first parsed |
| updated_at | TEXT NOT NULL | ISO 8601 timestamp when last updated |

**Indexes:**
- `idx_holosim_accounts_pubkey` on `account_pubkey`
- `idx_holosim_accounts_type` on `account_type`

### player_profile_accounts
Stores parsed Star Atlas Player Profile account data.

| Column | Type | Description |
|--------|------|-------------|
| id | TEXT PRIMARY KEY | Unique identifier (UUID) |
| account_pubkey | TEXT NOT NULL UNIQUE | Solana public key of the account |
| account_type | TEXT NOT NULL | Account type (e.g., Profile, PlayerName) |
| parsed_data | TEXT NOT NULL | JSON representation of parsed account data |
| raw_data_hash | TEXT NOT NULL | SHA256 hash of raw account data |
| created_at | TEXT NOT NULL | ISO 8601 timestamp when first parsed |
| updated_at | TEXT NOT NULL | ISO 8601 timestamp when last updated |

**Indexes:**
- `idx_player_profile_accounts_pubkey` on `account_pubkey`

### profile_faction_accounts
Stores parsed Star Atlas Profile Faction account data.

| Column | Type | Description |
|--------|------|-------------|
| id | TEXT PRIMARY KEY | Unique identifier (UUID) |
| account_pubkey | TEXT NOT NULL UNIQUE | Solana public key of the account |
| account_type | TEXT NOT NULL | Account type (e.g., ProfileFactionAccount) |
| parsed_data | TEXT NOT NULL | JSON representation of parsed account data |
| raw_data_hash | TEXT NOT NULL | SHA256 hash of raw account data |
| created_at | TEXT NOT NULL | ISO 8601 timestamp when first parsed |
| updated_at | TEXT NOT NULL | ISO 8601 timestamp when last updated |

**Indexes:**
- `idx_profile_faction_accounts_pubkey` on `account_pubkey`

## Program IDs

- **Holosim**: `SAgEeT8u14TE69JXtanGSgNkEdoPUcLabeyZD2uw8x9`
- **Player Profile**: `PprofUW1pURCnMW2si88GWPXEEK3Bvh9Tksy8WtnoYJ`
- **Profile Faction**: `pFACzkX2eSpAjDyEohD6i3VRJvREtH9ynbtM1DwVFsj`

## Usage

### Viewing Schema
```bash
# Show schema in terminal
just db-schema-show

# Export schema to SQL file
just db-schema-export
```

### Data Flow
1. **hackr-ixproc** monitors Solana programs and stores raw account data in the `accounts` table
2. **hackr-saproc** reads from the `accounts` table and:
   - Identifies account types using discriminators
   - Parses account data into structured JSON
   - Stores parsed data in program-specific tables when run with `--write` flag

### Notes
- All timestamps are stored as ISO 8601 strings (RFC 3339 format)
- Account data is stored as raw bytes in the `accounts` table
- Parsed data is stored as JSON text in the program-specific tables
- The `raw_data_hash` allows tracking when account data changes
- Account pubkeys are unique within each table to support upsert operations
