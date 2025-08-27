use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
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

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self> {
        let connection_string = if database_url.starts_with("sqlite:") {
            database_url.to_string()
        } else {
            format!("sqlite:{}?mode=ro", database_url)
        };

        let pool = SqlitePool::connect(&connection_string).await?;

        Ok(Self { pool })
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
}
