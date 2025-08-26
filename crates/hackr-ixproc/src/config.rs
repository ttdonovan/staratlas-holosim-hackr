use serde::Deserialize;
use std::str::FromStr;
use solana_sdk::pubkey::Pubkey;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub rpc_url: String,
    pub rpc_ws_url: String,
    pub programs: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenv::dotenv().ok();
        let config = envy::from_env::<Config>()?;
        Ok(config)
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