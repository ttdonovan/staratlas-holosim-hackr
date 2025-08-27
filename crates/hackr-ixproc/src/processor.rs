use solana_sdk::pubkey::Pubkey;
use solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta;
use std::collections::HashMap;
use tracing::{debug, info};

pub struct LiteProcessor {
    tracked_programs: Vec<Pubkey>,
    stats: HashMap<Pubkey, u64>,
}

impl LiteProcessor {
    pub fn new(programs: Vec<Pubkey>) -> Self {
        let mut stats = HashMap::new();
        for program in &programs {
            stats.insert(*program, 0);
        }

        Self {
            tracked_programs: programs,
            stats,
        }
    }

    pub fn process_transaction(
        &mut self,
        signature: &str,
        slot: u64,
        transaction: &EncodedConfirmedTransactionWithStatusMeta,
    ) -> anyhow::Result<()> {
        // Check if transaction failed
        if let Some(ref meta) = transaction.transaction.meta {
            if meta.err.is_some() {
                debug!("Skipping failed transaction: {}", signature);
                return Ok(());
            }
        }

        // Always log that we received a transaction (this should show up in logs)
        info!(
            signature = %signature,
            slot = slot,
            "🎉 Processing transaction for monitored programs"
        );

        // Extract program IDs from the transaction
        let mut found_programs = Vec::new();

        // Try to extract account keys and find program interactions
        if let Some(ref meta) = transaction.transaction.meta {
            // Log the fact that we have meta information
            debug!("Transaction has meta information");

            // Look through log messages for our program IDs
            // Note: log_messages is OptionSerializer, need to check if it contains data
            match &meta.log_messages {
                solana_transaction_status::option_serializer::OptionSerializer::Some(
                    log_messages,
                ) => {
                    debug!("Found {} log messages", log_messages.len());

                    for (i, log_msg) in log_messages.iter().enumerate() {
                        debug!("Log message {}: {}", i, log_msg);

                        // Check if any of our tracked programs appear in the logs
                        for program_id in &self.tracked_programs {
                            let program_str = program_id.to_string();
                            if log_msg.contains(&program_str) {
                                found_programs.push(*program_id);

                                info!(
                                    signature = %signature,
                                    slot = slot,
                                    program_id = %program_id,
                                    log_message = %log_msg,
                                    "🚀 Found interaction with tracked program!"
                                );
                            }
                        }
                    }
                }
                solana_transaction_status::option_serializer::OptionSerializer::None => {
                    debug!("No log messages in transaction meta");
                }
                solana_transaction_status::option_serializer::OptionSerializer::Skip => {
                    debug!("Log messages skipped in transaction meta");
                }
            }
        }

        // If we found program interactions, update stats
        if !found_programs.is_empty() {
            for program_id in found_programs {
                if let Some(count) = self.stats.get_mut(&program_id) {
                    *count += 1;
                    info!(
                        program_id = %program_id,
                        new_count = count,
                        "📊 Updated stats for program"
                    );
                }
            }
        } else {
            // Even if we didn't find specific program interactions, still log activity
            info!(
                signature = %signature,
                slot = slot,
                tracked_programs = self.tracked_programs.len(),
                "🔍 Transaction processed but no tracked program interactions found"
            );

            // Update stats for all tracked programs (simplified approach for lite version)
            for program_id in &self.tracked_programs {
                if let Some(count) = self.stats.get_mut(program_id) {
                    *count += 1;
                }
            }
        }

        Ok(())
    }

    pub fn get_stats(&self) -> &HashMap<Pubkey, u64> {
        &self.stats
    }
}
