use dotenv::dotenv;
use futures_util::StreamExt;
use serde::Deserialize;
use solana_account_decoder::UiAccountEncoding;
use solana_client::{
    nonblocking::pubsub_client::PubsubClient,
    rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
};
use solana_sdk::commitment_config::CommitmentConfig;
use staratlas_holosim::SAGE_ID;

#[derive(Deserialize)]
struct Env {
    wss_url: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let env = envy::from_env::<Env>()?;

    let client = PubsubClient::new(&env.wss_url).await?;

    let program_accounts_config = RpcProgramAccountsConfig {
        account_config: RpcAccountInfoConfig {
            encoding: Some(UiAccountEncoding::Base64),
            commitment: Some(CommitmentConfig::confirmed()),
            ..Default::default()
        },
        ..Default::default()
    };

    let (mut accounts, accounts_unsubscriber) = client
        .program_subscribe(&SAGE_ID, Some(program_accounts_config))
        .await?;

    let (mut slots, slots_unsubscriber) = client.slot_subscribe().await?;

    let mut count = 0;
    loop {
        tokio::select! {
            Some(account) = accounts.next() =>  {
                dbg!(account);
            },
            Some(slot) = slots.next() => {
                dbg!(slot);
                count += 1;
                if count >= 5 {
                    break;
                }
            }
        }
    }

    accounts_unsubscriber().await;
    slots_unsubscriber().await;

    Ok(())
}
