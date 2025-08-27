CREATE TABLE accounts (
                    id TEXT PRIMARY KEY,
                    program_id TEXT NOT NULL,
                    account_pubkey TEXT NOT NULL UNIQUE,
                    lamports INTEGER NOT NULL,
                    data BLOB NOT NULL,
                    owner TEXT NOT NULL,
                    executable BOOLEAN NOT NULL,
                    rent_epoch INTEGER NOT NULL,
                    created_at TEXT NOT NULL,
                    updated_at TEXT NOT NULL
                );
CREATE INDEX idx_accounts_pubkey ON accounts(account_pubkey);
CREATE INDEX idx_accounts_program_id ON accounts(program_id);
CREATE TABLE transaction_logs (
                    id TEXT PRIMARY KEY,
                    program_id TEXT NOT NULL,
                    signature TEXT NOT NULL,
                    slot INTEGER NOT NULL,
                    logs TEXT NOT NULL,
                    created_at TEXT NOT NULL
                );
CREATE INDEX idx_transaction_logs_signature ON transaction_logs(signature);
CREATE INDEX idx_transaction_logs_program_id ON transaction_logs(program_id);
CREATE TABLE holosim_accounts (
                id TEXT PRIMARY KEY,
                account_pubkey TEXT NOT NULL UNIQUE,
                account_type TEXT NOT NULL,
                parsed_data TEXT NOT NULL,
                raw_data_hash TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
CREATE INDEX idx_holosim_accounts_pubkey ON holosim_accounts(account_pubkey);
CREATE INDEX idx_holosim_accounts_type ON holosim_accounts(account_type);
CREATE TABLE player_profile_accounts (
                id TEXT PRIMARY KEY,
                account_pubkey TEXT NOT NULL UNIQUE,
                account_type TEXT NOT NULL,
                parsed_data TEXT NOT NULL,
                raw_data_hash TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
CREATE INDEX idx_player_profile_accounts_pubkey ON player_profile_accounts(account_pubkey);
CREATE TABLE profile_faction_accounts (
                id TEXT PRIMARY KEY,
                account_pubkey TEXT NOT NULL UNIQUE,
                account_type TEXT NOT NULL,
                parsed_data TEXT NOT NULL,
                raw_data_hash TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
CREATE INDEX idx_profile_faction_accounts_pubkey ON profile_faction_accounts(account_pubkey);
