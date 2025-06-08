use dotenv::dotenv;
use serde::Deserialize;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig, instruction::AccountMeta, message::Message,
    signature::read_keypair_file, signer::Signer, transaction::Transaction,
};
use staratlas_player_profile::{
    PLAYER_PROFILE_ID,
    player_profile::{instructions::CreateProfileBuilder, types::AddKeyInput},
};

#[derive(Deserialize)]
struct Env {
    rpc_url: String,
    keypair_path: String,
}

fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let env = envy::from_env::<Env>()?;

    let client = RpcClient::new_with_commitment(env.rpc_url, CommitmentConfig::confirmed());
    let keypair = read_keypair_file(&env.keypair_path).expect("Failed to read keypair file");
    dbg!(keypair.pubkey());

    let auth: u64 = 1 << 0;
    let auth_bytes = auth.to_le_bytes();

    let key_permissions = vec![AddKeyInput {
        scope: PLAYER_PROFILE_ID,
        expire_time: -1,
        permissions: auth_bytes,
    }];

    let account_meta = AccountMeta::new(keypair.pubkey(), true);
    let ix = CreateProfileBuilder::default()
        .funder(keypair.pubkey())
        .profile(keypair.pubkey())
        .key_permissions(key_permissions)
        .key_threshold(1)
        .add_remaining_account(account_meta)
        .instruction();

    let latest = client.get_latest_blockhash()?;
    let message = Message::new(&[ix], Some(&keypair.pubkey()));
    let mut tx = Transaction::new_unsigned(message);
    tx.try_sign(&[&keypair], latest)?;

    let simulate = client.simulate_transaction(&tx)?;
    dbg!(&simulate);

    if simulate.value.err.is_none() {
        let result = client.send_and_confirm_transaction(&tx)?;
        dbg!(result);
    }

    Ok(())
}
