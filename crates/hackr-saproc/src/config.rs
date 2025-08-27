use clap::Parser;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EnvConfig {
    pub database_url: Option<String>,
}

#[derive(Parser, Debug)]
#[command(name = "hackr-saproc")]
#[command(about = "Star Atlas Solana account processor - parses stored account data")]
pub struct Args {
    /// Database URL (SQLite file path from hackr-ixproc)
    #[arg(long, default_value = "hackr.db")]
    pub database_url: String,

    /// Program ID to filter accounts (optional, processes all if not specified)
    #[arg(long)]
    pub program_id: Option<String>,

    /// Only show account statistics without processing
    #[arg(long)]
    pub stats_only: bool,

    /// Output format (json, summary, detailed)
    #[arg(long, default_value = "summary")]
    pub output: String,

    /// Limit number of accounts to process (for testing)
    #[arg(long)]
    pub limit: Option<usize>,

    /// Enable writing parsed accounts to database tables
    #[arg(long)]
    pub write: bool,
}

#[derive(Debug)]
pub struct Config {
    pub database_url: String,
    pub args: Args,
}

impl Config {
    pub fn from_env_and_args() -> anyhow::Result<Self> {
        dotenv::dotenv().ok();
        let _env_config = envy::from_env::<EnvConfig>().unwrap_or(EnvConfig { database_url: None });
        let args = Args::parse();

        let database_url = args.database_url.clone();

        Ok(Config { database_url, args })
    }
}
