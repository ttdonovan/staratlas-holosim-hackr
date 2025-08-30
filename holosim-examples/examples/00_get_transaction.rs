use borsh::BorshDeserialize;
use dotenv::dotenv;
use serde::Deserialize;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::RpcTransactionConfig;
use solana_sdk::{commitment_config::CommitmentConfig, signature::Signature};
use solana_transaction_status::{
    EncodedTransaction, UiMessage, UiTransactionEncoding, option_serializer::OptionSerializer,
};
use staratlas_holosim::{
    SAGE_ID,
    generated::instructions::{ATTACK_FLEET_DISCRIMINATOR, AttackFleetInstructionArgs},
};
use std::str::FromStr;

#[derive(Deserialize)]
struct Env {
    rpc_url: String,
}

fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let env = envy::from_env::<Env>()?;

    let client = RpcClient::new_with_commitment(env.rpc_url, CommitmentConfig::confirmed());

    let tx_sig =
        "55gCBnzWtEqRLTjXRSLunsi2VPhxtj6pZWsWEfkx3yNSmgbLiHiWQ9bW5S6sZo9aquxsA7fS1tkYEEFxi9Mhfwv1";
    let signature = Signature::from_str(tx_sig)?;

    println!("Fetching transaction: {}", tx_sig);
    println!("----------------------------------------");

    let config = RpcTransactionConfig {
        encoding: Some(UiTransactionEncoding::Json),
        commitment: Some(CommitmentConfig::confirmed()),
        max_supported_transaction_version: Some(0),
    };

    let transaction = client.get_transaction_with_config(&signature, config)?;

    println!("Slot: {}", transaction.slot);

    if let Some(block_time) = transaction.block_time {
        let datetime = chrono::DateTime::from_timestamp(block_time, 0)
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string())
            .unwrap_or_else(|| "Unknown".to_string());
        println!("Block Time: {}", datetime);
    }

    if let Some(meta) = &transaction.transaction.meta {
        println!("Fee: {} lamports", meta.fee);

        if let Some(err) = &meta.err {
            println!("Error: {:?}", err);
        } else {
            println!("Status: Success");
        }

        if let OptionSerializer::Some(log_messages) = &meta.log_messages {
            println!("\nLog Messages:");
            for log in log_messages {
                println!("  {}", log);
            }

            // Parse combat-specific logs
            println!("\nCombat Analysis:");
            for log in log_messages {
                if log.contains("SAGE_COMBAT_LOG::") {
                    // Format is "Program log: SAGE_COMBAT_LOG::key: value"
                    if let Some(combat_log) = log.strip_prefix("Program log: SAGE_COMBAT_LOG::") {
                        if let Some((key, value)) = combat_log.split_once(": ") {
                            match key {
                                "combat_id" => println!("  Combat ID: {}", value),
                                "attacker_fleet" => println!("  Attacker Fleet: {}", value),
                                "defender_fleet" => println!("  Defender Fleet: {}", value),
                                "damage_to_attacker" => println!("  Damage to Attacker: {}", value),
                                "damage_to_defender" => println!("  Damage to Defender: {}", value),
                                _ => {}
                            }
                        }
                    }
                }

                // Parse base64 Program data logs
                if log.starts_with("Program data: ") {
                    let data_str = log.strip_prefix("Program data: ").unwrap_or("");
                    if let Ok(data) = bs58::decode(data_str).into_vec() {
                        if data.len() >= 8 {
                            // Check for known discriminators or patterns
                            println!("  Program data decoded: {} bytes", data.len());
                            // You could add more specific parsing here based on the data structure
                        }
                    }
                }
            }
        }
    }

    match &transaction.transaction.transaction {
        EncodedTransaction::Json(ui_transaction) => {
            println!("\nInvolved Programs:");
            let account_keys = match &ui_transaction.message {
                UiMessage::Parsed(parsed) => {
                    let mut keys = Vec::new();
                    for (i, account) in parsed.account_keys.iter().enumerate() {
                        if account.pubkey == SAGE_ID.to_string() {
                            println!("  [{}] {} (SAGE/Holosim)", i, account.pubkey);
                        } else {
                            println!("  [{}] {}", i, account.pubkey);
                        }
                        keys.push(account.pubkey.clone());
                    }
                    keys
                }
                UiMessage::Raw(raw) => {
                    let mut keys = Vec::new();
                    for (i, key) in raw.account_keys.iter().enumerate() {
                        if key == &SAGE_ID.to_string() {
                            println!("  [{}] {} (SAGE/Holosim)", i, key);
                        } else {
                            println!("  [{}] {}", i, key);
                        }
                        keys.push(key.clone());
                    }
                    keys
                }
            };

            // Parse instructions
            println!("\nInstruction Details:");
            match &ui_transaction.message {
                UiMessage::Raw(raw) => {
                    for (idx, instruction) in raw.instructions.iter().enumerate() {
                        let program_id_index = instruction.program_id_index as usize;
                        if program_id_index < account_keys.len() {
                            let program_id = &account_keys[program_id_index];
                            if program_id == &SAGE_ID.to_string() {
                                println!("\n  Instruction #{}: SAGE/Holosim", idx);

                                // Decode the instruction data
                                if let Ok(data) = bs58::decode(&instruction.data).into_vec() {
                                    if data.len() >= 8 {
                                        let discriminator = &data[0..8];
                                        if discriminator == ATTACK_FLEET_DISCRIMINATOR {
                                            println!("    Type: AttackFleet");

                                            // Decode the instruction args
                                            if let Ok(args) =
                                                AttackFleetInstructionArgs::try_from_slice(
                                                    &data[8..],
                                                )
                                            {
                                                println!("    Key Index: {}", args.key_index);
                                            }

                                            // List the accounts involved
                                            println!("    Accounts:");
                                            let account_names = vec![
                                                "key (signer)",
                                                "owning_profile",
                                                "owning_profile_faction",
                                                "fleet",
                                                "game_id",
                                                "defending_fleet",
                                                "attacking_cargo_pod",
                                                "defending_cargo_pod",
                                                "cargo_type",
                                                "cargo_stats_definition",
                                                "attacker_combat_xp",
                                                "attacker_council_rank_xp",
                                                "defender_combat_xp",
                                                "defender_council_rank_xp",
                                                "combat_xp_category",
                                                "council_rank_xp_category",
                                                "combat_xp_modifier",
                                                "council_rank_xp_modifier",
                                                "progression_config",
                                                "combat_config",
                                                "attacking_fleet_ammo_token",
                                                "defending_fleet_ammo_token",
                                                "token_mint",
                                            ];

                                            for (i, &account_index) in
                                                instruction.accounts.iter().enumerate()
                                            {
                                                if i < account_names.len()
                                                    && (account_index as usize) < account_keys.len()
                                                {
                                                    println!(
                                                        "      {}: {} -> {}",
                                                        i,
                                                        account_names[i],
                                                        account_keys[account_index as usize]
                                                    );
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {
                    println!("  Parsed message format - unable to decode raw instruction data");
                }
            }
        }
        _ => {
            println!("\nTransaction format not supported for display");
        }
    }

    Ok(())
}
