use dotenv::dotenv;
use serde::Deserialize;
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use staratlas_holosim::SAGE_ID;

#[derive(Deserialize)]
struct Env {
    rpc_url: String,
}

fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let env = envy::from_env::<Env>()?;

    let client = RpcClient::new_with_commitment(env.rpc_url, CommitmentConfig::confirmed());

    let accounts = client.get_program_accounts(&SAGE_ID)?;
    dbg!(&accounts.len());

    Ok(())
}
