use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{Row, SqlitePool};

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
pub struct HolosimAccount {
    pub id: String,
    pub account_pubkey: String,
    pub account_type: String,
    pub parsed_data: Value,
    pub raw_data_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct Database {
    pool: SqlitePool,
    write_enabled: bool,
}

impl Database {
    pub async fn new(database_url: &str, write_enabled: bool) -> Result<Self> {
        let connection_string = if database_url.starts_with("sqlite:") {
            database_url.to_string()
        } else {
            if write_enabled {
                format!("sqlite:{}?mode=rwc", database_url)
            } else {
                format!("sqlite:{}?mode=ro", database_url)
            }
        };

        let pool = SqlitePool::connect(&connection_string).await?;

        // Run migrations if write enabled
        if write_enabled {
            Self::run_migrations(&pool).await?;
        }

        Ok(Self {
            pool,
            write_enabled,
        })
    }

    async fn run_migrations(pool: &SqlitePool) -> Result<()> {
        // Create holosim_accounts table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS holosim_accounts (
                id TEXT PRIMARY KEY,
                account_pubkey TEXT NOT NULL UNIQUE,
                account_type TEXT NOT NULL,
                parsed_data TEXT NOT NULL,
                raw_data_hash TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            "#,
        )
        .execute(pool)
        .await?;

        // Create indexes
        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_holosim_accounts_pubkey ON holosim_accounts(account_pubkey);",
        )
        .execute(pool)
        .await?;

        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_holosim_accounts_type ON holosim_accounts(account_type);",
        )
        .execute(pool)
        .await?;

        // Create player_profile_accounts table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS player_profile_accounts (
                id TEXT PRIMARY KEY,
                account_pubkey TEXT NOT NULL UNIQUE,
                account_type TEXT NOT NULL,
                parsed_data TEXT NOT NULL,
                raw_data_hash TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            "#,
        )
        .execute(pool)
        .await?;

        // Create indexes for player_profile_accounts
        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_player_profile_accounts_pubkey ON player_profile_accounts(account_pubkey);",
        )
        .execute(pool)
        .await?;

        // Create profile_faction_accounts table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS profile_faction_accounts (
                id TEXT PRIMARY KEY,
                account_pubkey TEXT NOT NULL UNIQUE,
                account_type TEXT NOT NULL,
                parsed_data TEXT NOT NULL,
                raw_data_hash TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            "#,
        )
        .execute(pool)
        .await?;

        // Create indexes for profile_faction_accounts
        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_profile_faction_accounts_pubkey ON profile_faction_accounts(account_pubkey);",
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn get_accounts_by_program(&self, program_id: &str) -> Result<Vec<AccountData>> {
        let rows = sqlx::query("SELECT * FROM accounts WHERE program_id = ?")
            .bind(program_id)
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

    pub async fn get_all_accounts(&self) -> Result<Vec<AccountData>> {
        let rows = sqlx::query("SELECT * FROM accounts ORDER BY program_id, account_pubkey")
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

    pub async fn get_account_stats(&self) -> Result<Vec<(String, i64)>> {
        let rows =
            sqlx::query("SELECT program_id, COUNT(*) as count FROM accounts GROUP BY program_id")
                .fetch_all(&self.pool)
                .await?;

        let mut stats = Vec::new();
        for row in rows {
            stats.push((row.get("program_id"), row.get("count")));
        }

        Ok(stats)
    }

    pub async fn upsert_holosim_account(&self, account: &HolosimAccount) -> Result<()> {
        if !self.write_enabled {
            return Err(anyhow::anyhow!("Database is in read-only mode"));
        }

        sqlx::query(
            r#"
            INSERT INTO holosim_accounts (
                id, account_pubkey, account_type, parsed_data, raw_data_hash,
                created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(account_pubkey) DO UPDATE SET
                account_type = excluded.account_type,
                parsed_data = excluded.parsed_data,
                raw_data_hash = excluded.raw_data_hash,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(&account.id)
        .bind(&account.account_pubkey)
        .bind(&account.account_type)
        .bind(serde_json::to_string(&account.parsed_data)?)
        .bind(&account.raw_data_hash)
        .bind(account.created_at.to_rfc3339())
        .bind(account.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn upsert_player_profile_account(&self, account: &HolosimAccount) -> Result<()> {
        if !self.write_enabled {
            return Err(anyhow::anyhow!("Database is in read-only mode"));
        }

        sqlx::query(
            r#"
            INSERT INTO player_profile_accounts (
                id, account_pubkey, account_type, parsed_data, raw_data_hash,
                created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(account_pubkey) DO UPDATE SET
                account_type = excluded.account_type,
                parsed_data = excluded.parsed_data,
                raw_data_hash = excluded.raw_data_hash,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(&account.id)
        .bind(&account.account_pubkey)
        .bind(&account.account_type)
        .bind(serde_json::to_string(&account.parsed_data)?)
        .bind(&account.raw_data_hash)
        .bind(account.created_at.to_rfc3339())
        .bind(account.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn upsert_profile_faction_account(&self, account: &HolosimAccount) -> Result<()> {
        if !self.write_enabled {
            return Err(anyhow::anyhow!("Database is in read-only mode"));
        }

        sqlx::query(
            r#"
            INSERT INTO profile_faction_accounts (
                id, account_pubkey, account_type, parsed_data, raw_data_hash,
                created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(account_pubkey) DO UPDATE SET
                account_type = excluded.account_type,
                parsed_data = excluded.parsed_data,
                raw_data_hash = excluded.raw_data_hash,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(&account.id)
        .bind(&account.account_pubkey)
        .bind(&account.account_type)
        .bind(serde_json::to_string(&account.parsed_data)?)
        .bind(&account.raw_data_hash)
        .bind(account.created_at.to_rfc3339())
        .bind(account.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_holosim_account_stats(&self) -> Result<Vec<(String, i64)>> {
        let rows = sqlx::query(
            "SELECT account_type, COUNT(*) as count FROM holosim_accounts GROUP BY account_type",
        )
        .fetch_all(&self.pool)
        .await?;

        let mut stats = Vec::new();
        for row in rows {
            stats.push((row.get("account_type"), row.get("count")));
        }

        Ok(stats)
    }

    pub async fn get_player_profile_account_stats(&self) -> Result<Vec<(String, i64)>> {
        let rows = sqlx::query(
            "SELECT account_type, COUNT(*) as count FROM player_profile_accounts GROUP BY account_type"
        )
        .fetch_all(&self.pool)
        .await?;

        let mut stats = Vec::new();
        for row in rows {
            stats.push((row.get("account_type"), row.get("count")));
        }

        Ok(stats)
    }

    pub async fn get_profile_faction_account_stats(&self) -> Result<Vec<(String, i64)>> {
        let rows = sqlx::query(
            "SELECT account_type, COUNT(*) as count FROM profile_faction_accounts GROUP BY account_type"
        )
        .fetch_all(&self.pool)
        .await?;

        let mut stats = Vec::new();
        for row in rows {
            stats.push((row.get("account_type"), row.get("count")));
        }

        Ok(stats)
    }

    pub async fn batch_upsert_holosim_accounts(&self, accounts: &[HolosimAccount]) -> Result<()> {
        if !self.write_enabled {
            return Err(anyhow::anyhow!("Database is in read-only mode"));
        }

        let mut tx = self.pool.begin().await?;

        for account in accounts {
            sqlx::query(
                r#"
                INSERT INTO holosim_accounts (
                    id, account_pubkey, account_type, parsed_data, raw_data_hash,
                    created_at, updated_at
                ) VALUES (?, ?, ?, ?, ?, ?, ?)
                ON CONFLICT(account_pubkey) DO UPDATE SET
                    account_type = excluded.account_type,
                    parsed_data = excluded.parsed_data,
                    raw_data_hash = excluded.raw_data_hash,
                    updated_at = excluded.updated_at
                "#,
            )
            .bind(&account.id)
            .bind(&account.account_pubkey)
            .bind(&account.account_type)
            .bind(serde_json::to_string(&account.parsed_data)?)
            .bind(&account.raw_data_hash)
            .bind(account.created_at.to_rfc3339())
            .bind(account.updated_at.to_rfc3339())
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    pub async fn batch_upsert_player_profile_accounts(
        &self,
        accounts: &[HolosimAccount],
    ) -> Result<()> {
        if !self.write_enabled {
            return Err(anyhow::anyhow!("Database is in read-only mode"));
        }

        let mut tx = self.pool.begin().await?;

        for account in accounts {
            sqlx::query(
                r#"
                INSERT INTO player_profile_accounts (
                    id, account_pubkey, account_type, parsed_data, raw_data_hash,
                    created_at, updated_at
                ) VALUES (?, ?, ?, ?, ?, ?, ?)
                ON CONFLICT(account_pubkey) DO UPDATE SET
                    account_type = excluded.account_type,
                    parsed_data = excluded.parsed_data,
                    raw_data_hash = excluded.raw_data_hash,
                    updated_at = excluded.updated_at
                "#,
            )
            .bind(&account.id)
            .bind(&account.account_pubkey)
            .bind(&account.account_type)
            .bind(serde_json::to_string(&account.parsed_data)?)
            .bind(&account.raw_data_hash)
            .bind(account.created_at.to_rfc3339())
            .bind(account.updated_at.to_rfc3339())
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    pub async fn batch_upsert_profile_faction_accounts(
        &self,
        accounts: &[HolosimAccount],
    ) -> Result<()> {
        if !self.write_enabled {
            return Err(anyhow::anyhow!("Database is in read-only mode"));
        }

        let mut tx = self.pool.begin().await?;

        for account in accounts {
            sqlx::query(
                r#"
                INSERT INTO profile_faction_accounts (
                    id, account_pubkey, account_type, parsed_data, raw_data_hash,
                    created_at, updated_at
                ) VALUES (?, ?, ?, ?, ?, ?, ?)
                ON CONFLICT(account_pubkey) DO UPDATE SET
                    account_type = excluded.account_type,
                    parsed_data = excluded.parsed_data,
                    raw_data_hash = excluded.raw_data_hash,
                    updated_at = excluded.updated_at
                "#,
            )
            .bind(&account.id)
            .bind(&account.account_pubkey)
            .bind(&account.account_type)
            .bind(serde_json::to_string(&account.parsed_data)?)
            .bind(&account.raw_data_hash)
            .bind(account.created_at.to_rfc3339())
            .bind(account.updated_at.to_rfc3339())
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }
}
