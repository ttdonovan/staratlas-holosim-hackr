#[cfg(feature = "database")]
pub mod db {
    use anyhow::Result;
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};
    use solana_sdk::pubkey::Pubkey;
    use sqlx::{Row, SqlitePool};
    use uuid::Uuid;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AccountData {
        pub id: String,
        pub program_id: String,
        pub account_pubkey: String,
        pub lamports: u64,
        pub data: Vec<u8>,
        pub owner: String,
        pub executable: bool,
        pub rent_epoch: u64,
        pub created_at: DateTime<Utc>,
        pub updated_at: DateTime<Utc>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct TransactionLog {
        pub id: String,
        pub program_id: String,
        pub signature: String,
        pub slot: u64,
        pub logs: String, // JSON array of log messages
        pub created_at: DateTime<Utc>,
    }

    pub struct Database {
        pool: SqlitePool,
    }

    impl Database {
        pub async fn new(database_url: &str) -> Result<Self> {
            // Create database file if it doesn't exist by using sqlite: prefix
            let connection_string = if database_url.starts_with("sqlite:") {
                database_url.to_string()
            } else {
                format!("sqlite:{}?mode=rwc", database_url)
            };

            let pool = SqlitePool::connect(&connection_string).await?;

            // Run migrations
            Self::run_migrations(&pool).await?;

            Ok(Self { pool })
        }

        async fn run_migrations(pool: &SqlitePool) -> Result<()> {
            // Create accounts table
            sqlx::query(
                r#"
                CREATE TABLE IF NOT EXISTS accounts (
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
                "#,
            )
            .execute(pool)
            .await?;

            // Create index on account_pubkey for fast lookups
            sqlx::query(
                "CREATE INDEX IF NOT EXISTS idx_accounts_pubkey ON accounts(account_pubkey);",
            )
            .execute(pool)
            .await?;

            // Create index on program_id for filtering
            sqlx::query(
                "CREATE INDEX IF NOT EXISTS idx_accounts_program_id ON accounts(program_id);",
            )
            .execute(pool)
            .await?;

            // Create transaction_logs table
            sqlx::query(
                r#"
                CREATE TABLE IF NOT EXISTS transaction_logs (
                    id TEXT PRIMARY KEY,
                    program_id TEXT NOT NULL,
                    signature TEXT NOT NULL,
                    slot INTEGER NOT NULL,
                    logs TEXT NOT NULL,
                    created_at TEXT NOT NULL
                );
                "#,
            )
            .execute(pool)
            .await?;

            // Create index on signature
            sqlx::query("CREATE INDEX IF NOT EXISTS idx_transaction_logs_signature ON transaction_logs(signature);")
                .execute(pool)
                .await?;

            // Create index on program_id
            sqlx::query("CREATE INDEX IF NOT EXISTS idx_transaction_logs_program_id ON transaction_logs(program_id);")
                .execute(pool)
                .await?;

            Ok(())
        }

        pub async fn upsert_account(&self, account_data: &AccountData) -> Result<()> {
            sqlx::query(
                r#"
                INSERT INTO accounts (
                    id, program_id, account_pubkey, lamports, data, owner,
                    executable, rent_epoch, created_at, updated_at
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                ON CONFLICT(account_pubkey) DO UPDATE SET
                    lamports = excluded.lamports,
                    data = excluded.data,
                    owner = excluded.owner,
                    executable = excluded.executable,
                    rent_epoch = excluded.rent_epoch,
                    updated_at = excluded.updated_at
                "#,
            )
            .bind(&account_data.id)
            .bind(&account_data.program_id)
            .bind(&account_data.account_pubkey)
            .bind(account_data.lamports as i64)
            .bind(&account_data.data)
            .bind(&account_data.owner)
            .bind(account_data.executable)
            .bind(account_data.rent_epoch as i64)
            .bind(account_data.created_at.to_rfc3339())
            .bind(account_data.updated_at.to_rfc3339())
            .execute(&self.pool)
            .await?;

            Ok(())
        }

        pub async fn insert_transaction_log(&self, log: &TransactionLog) -> Result<()> {
            sqlx::query(
                r#"
                INSERT INTO transaction_logs (id, program_id, signature, slot, logs, created_at)
                VALUES (?, ?, ?, ?, ?, ?)
                "#,
            )
            .bind(&log.id)
            .bind(&log.program_id)
            .bind(&log.signature)
            .bind(log.slot as i64)
            .bind(&log.logs)
            .bind(log.created_at.to_rfc3339())
            .execute(&self.pool)
            .await?;

            Ok(())
        }

        pub async fn batch_upsert_accounts(&self, accounts: &[AccountData]) -> Result<()> {
            let mut tx = self.pool.begin().await?;

            for account_data in accounts {
                sqlx::query(
                    r#"
                    INSERT INTO accounts (
                        id, program_id, account_pubkey, lamports, data, owner, 
                        executable, rent_epoch, created_at, updated_at
                    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                    ON CONFLICT(account_pubkey) DO UPDATE SET
                        lamports = excluded.lamports,
                        data = excluded.data,
                        owner = excluded.owner,
                        executable = excluded.executable,
                        rent_epoch = excluded.rent_epoch,
                        updated_at = excluded.updated_at
                    "#,
                )
                .bind(&account_data.id)
                .bind(&account_data.program_id)
                .bind(&account_data.account_pubkey)
                .bind(account_data.lamports as i64)
                .bind(&account_data.data)
                .bind(&account_data.owner)
                .bind(account_data.executable)
                .bind(account_data.rent_epoch as i64)
                .bind(account_data.created_at.to_rfc3339())
                .bind(account_data.updated_at.to_rfc3339())
                .execute(&mut *tx)
                .await?;
            }

            tx.commit().await?;
            Ok(())
        }

        pub async fn get_accounts_by_program(
            &self,
            program_id: &Pubkey,
        ) -> Result<Vec<AccountData>> {
            let program_id_str = program_id.to_string();

            let rows = sqlx::query("SELECT * FROM accounts WHERE program_id = ?")
                .bind(program_id_str)
                .fetch_all(&self.pool)
                .await?;

            let mut accounts = Vec::new();
            for row in rows {
                let created_at_str: String = row.get("created_at");
                let updated_at_str: String = row.get("updated_at");

                accounts.push(AccountData {
                    id: row.get("id"),
                    program_id: row.get("program_id"),
                    account_pubkey: row.get("account_pubkey"),
                    lamports: row.get::<i64, _>("lamports") as u64,
                    data: row.get("data"),
                    owner: row.get("owner"),
                    executable: row.get("executable"),
                    rent_epoch: row.get::<i64, _>("rent_epoch") as u64,
                    created_at: DateTime::parse_from_rfc3339(&created_at_str)?.with_timezone(&Utc),
                    updated_at: DateTime::parse_from_rfc3339(&updated_at_str)?.with_timezone(&Utc),
                });
            }

            Ok(accounts)
        }

        pub async fn get_recent_transaction_logs(
            &self,
            program_id: &Pubkey,
            limit: i32,
        ) -> Result<Vec<TransactionLog>> {
            let program_id_str = program_id.to_string();

            let rows = sqlx::query(
                "SELECT * FROM transaction_logs WHERE program_id = ? ORDER BY created_at DESC LIMIT ?"
            )
            .bind(program_id_str)
            .bind(limit)
            .fetch_all(&self.pool)
            .await?;

            let mut logs = Vec::new();
            for row in rows {
                let created_at_str: String = row.get("created_at");

                logs.push(TransactionLog {
                    id: row.get("id"),
                    program_id: row.get("program_id"),
                    signature: row.get("signature"),
                    slot: row.get::<i64, _>("slot") as u64,
                    logs: row.get("logs"),
                    created_at: DateTime::parse_from_rfc3339(&created_at_str)?.with_timezone(&Utc),
                });
            }

            Ok(logs)
        }

        pub async fn get_account_count_by_program(&self, program_id: &Pubkey) -> Result<i64> {
            let program_id_str = program_id.to_string();

            let row = sqlx::query("SELECT COUNT(*) as count FROM accounts WHERE program_id = ?")
                .bind(program_id_str)
                .fetch_one(&self.pool)
                .await?;

            Ok(row.get("count"))
        }

        pub async fn get_total_stats(&self) -> Result<(i64, i64)> {
            let accounts_row = sqlx::query("SELECT COUNT(*) as count FROM accounts")
                .fetch_one(&self.pool)
                .await?;

            let logs_row = sqlx::query("SELECT COUNT(*) as count FROM transaction_logs")
                .fetch_one(&self.pool)
                .await?;

            Ok((accounts_row.get("count"), logs_row.get("count")))
        }
    }
}

#[cfg(not(feature = "database"))]
pub mod db {
    // Stub implementation when database feature is not enabled
    use anyhow::Result;
    use solana_sdk::pubkey::Pubkey;

    pub struct Database;

    impl Database {
        pub async fn new(_database_url: &str) -> Result<Self> {
            Err(anyhow::anyhow!(
                "Database support not compiled. Enable with --features database"
            ))
        }
    }
}
