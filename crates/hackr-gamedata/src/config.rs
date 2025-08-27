use anyhow::Result;
use clap::Parser;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct Config {
    pub env: EnvConfig,
    pub args: Args,
    pub database_url: String,
    pub output_format: OutputFormat,
    pub output_dir: String,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct EnvConfig {
    #[serde(default)]
    pub database_url: Option<String>,
}

#[derive(Parser, Debug, Clone)]
#[command(
    author,
    version,
    about = "Extract Star Atlas game balance data for game engine initialization"
)]
pub struct Args {
    /// Database URL (SQLite file path)
    #[arg(long)]
    pub database_url: Option<String>,

    /// Game account public key (optional - will fetch all Game accounts if not provided)
    #[arg(long)]
    pub game_pubkey: Option<String>,

    /// Also fetch associated GameState accounts
    #[arg(long, default_value = "true")]
    pub include_game_state: bool,

    /// Output format (ron or json)
    #[arg(long, default_value = "ron")]
    pub format: OutputFormat,

    /// Output directory for exported files
    #[arg(long, default_value = "./gamedata")]
    pub output_dir: String,

    /// Output filename (without extension)
    #[arg(long)]
    pub output_name: Option<String>,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum OutputFormat {
    Ron,
    Json,
}

impl Config {
    pub fn from_env_and_args() -> Result<Self> {
        dotenv::dotenv().ok();

        let env_config = envy::from_env::<EnvConfig>()?;
        let args = Args::parse();

        // Database URL priority: CLI arg > env var > default
        let database_url = args
            .database_url
            .clone()
            .or(env_config.database_url.clone())
            .unwrap_or_else(|| "hackr.db".to_string());

        Ok(Config {
            env: env_config,
            args: args.clone(),
            database_url,
            output_format: args.format,
            output_dir: args.output_dir,
        })
    }
}
