use clap::Parser;
use serde::Deserialize;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub struct EnvConfig {
    pub rpc_url: String,
    pub rpc_ws_url: String,
    pub programs: String,
    pub database_url: Option<String>,
}

#[derive(Parser, Debug)]
#[command(name = "hackr-ixproc")]
#[command(about = "A lightweight Solana program activity monitor")]
pub struct Args {
    /// Enable database persistence (requires --features database)
    #[arg(long)]
    pub database: bool,

    /// Database URL (SQLite file path)
    #[arg(long, default_value = "hackr_ixproc.db")]
    pub database_url: String,

    /// Dump all existing accounts for monitored programs on startup
    #[arg(long)]
    pub dump_accounts: bool,

    /// Server port
    #[arg(long, default_value = "8080")]
    pub port: u16,
}

#[derive(Debug)]
pub struct Config {
    pub rpc_url: String,
    pub rpc_ws_url: String,
    pub programs: String,
    pub database_url: Option<String>,
    pub args: Args,
}

impl Config {
    pub fn from_env_and_args() -> anyhow::Result<Self> {
        dotenv::dotenv().ok();
        let env_config = envy::from_env::<EnvConfig>()?;
        let args = Args::parse();

        let database_url = if args.database {
            Some(args.database_url.clone())
        } else {
            env_config.database_url
        };

        Ok(Config {
            rpc_url: env_config.rpc_url,
            rpc_ws_url: env_config.rpc_ws_url,
            programs: env_config.programs,
            database_url,
            args,
        })
    }

    pub fn program_ids(&self) -> anyhow::Result<Vec<Pubkey>> {
        self.programs
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| Pubkey::from_str(s).map_err(anyhow::Error::from))
            .collect()
    }
}
