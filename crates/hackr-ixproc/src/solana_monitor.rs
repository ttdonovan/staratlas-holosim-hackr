use anyhow::Result;
use solana_client::{
    rpc_client::RpcClient, 
    rpc_config::{RpcTransactionLogsConfig, RpcTransactionLogsFilter, RpcProgramAccountsConfig},
};
use solana_pubsub_client::nonblocking::pubsub_client::PubsubClient;
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};
use solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta;
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

use crate::processor::LiteProcessor;

#[derive(Debug)]
pub enum SubscriptionEvent {
    LogsUpdate(Pubkey, solana_client::rpc_response::Response<solana_client::rpc_response::RpcLogsResponse>),
    AccountUpdate(Pubkey, solana_client::rpc_response::Response<solana_client::rpc_response::RpcKeyedAccount>),
}

pub struct SolanaMonitor {
    rpc_client: RpcClient,
    ws_url: String,
    programs: Vec<Pubkey>,
}

impl SolanaMonitor {
    pub fn new(rpc_url: String, ws_url: String, programs: Vec<Pubkey>) -> Self {
        let rpc_client = RpcClient::new(rpc_url);
        
        Self {
            rpc_client,
            ws_url,
            programs,
        }
    }

    pub async fn start_monitoring(&self, mut processor: LiteProcessor) -> Result<()> {
        info!("🚀 Starting Solana monitor for {} programs", self.programs.len());
        for program in &self.programs {
            info!("📡 Monitoring program: {}", program);
        }

        // Create a channel for receiving subscription events
        let (tx, mut rx) = mpsc::unbounded_channel();
        
        // Spawn subscription tasks for each program (both logs and accounts)
        for program_id in &self.programs {
            let ws_url_logs = self.ws_url.clone();
            let ws_url_accounts = self.ws_url.clone();
            let program_id_clone = *program_id;
            let tx_logs = tx.clone();
            let tx_accounts = tx.clone();
            
            info!("📋 Starting subscription tasks for program: {}", program_id);
            
            // Spawn task for log subscriptions
            tokio::spawn(async move {
                use futures_util::StreamExt;
                
                info!("🔌 Connecting to Solana pubsub for LOGS: {}", program_id_clone);
                let pubsub_client = match PubsubClient::new(&ws_url_logs).await {
                    Ok(client) => {
                        info!("✅ Solana pubsub connected for logs: {}", program_id_clone);
                        client
                    }
                    Err(e) => {
                        error!("❌ Failed to connect pubsub for logs {}: {}", program_id_clone, e);
                        return;
                    }
                };
                
                let (mut logs_stream, logs_unsubscribe) = match pubsub_client
                    .logs_subscribe(
                        RpcTransactionLogsFilter::Mentions(vec![program_id_clone.to_string()]),
                        RpcTransactionLogsConfig {
                            commitment: Some(CommitmentConfig::confirmed()),
                        },
                    )
                    .await 
                {
                    Ok(subscription) => {
                        info!("✅ Subscribed to LOGS for program: {}", program_id_clone);
                        subscription
                    }
                    Err(e) => {
                        error!("❌ Failed to subscribe to logs for program {}: {}", program_id_clone, e);
                        return;
                    }
                };
                
                while let Some(logs_response) = logs_stream.next().await {
                    debug!(
                        program_id = %program_id_clone,
                        signature = %logs_response.value.signature,
                        "📨 Received log notification"
                    );
                    
                    if let Err(e) = tx_logs.send(SubscriptionEvent::LogsUpdate(program_id_clone, logs_response)) {
                        error!("Failed to send log notification: {}", e);
                        break;
                    }
                }
                
                // Clean up subscription when done
                let _ = logs_unsubscribe().await;
                info!("🛑 Log subscription ended for program: {}", program_id_clone);
            });
            
            // Spawn task for account subscriptions
            let program_id_accounts = program_id_clone;
            tokio::spawn(async move {
                use futures_util::StreamExt;
                
                info!("🔌 Connecting to Solana pubsub for ACCOUNTS: {}", program_id_accounts);
                let pubsub_client = match PubsubClient::new(&ws_url_accounts).await {
                    Ok(client) => {
                        info!("✅ Solana pubsub connected for accounts: {}", program_id_accounts);
                        client
                    }
                    Err(e) => {
                        error!("❌ Failed to connect pubsub for accounts {}: {}", program_id_accounts, e);
                        return;
                    }
                };
                
                let (mut accounts_stream, accounts_unsubscribe) = match pubsub_client
                    .program_subscribe(
                        &program_id_accounts,
                        Some(RpcProgramAccountsConfig {
                            filters: None, // Monitor all accounts owned by this program
                            account_config: solana_client::rpc_config::RpcAccountInfoConfig {
                                encoding: Some(solana_account_decoder::UiAccountEncoding::Base64),
                                commitment: Some(CommitmentConfig::confirmed()),
                                data_slice: None,
                                min_context_slot: None,
                            },
                            with_context: Some(true),
                            sort_results: None,
                        }),
                    )
                    .await 
                {
                    Ok(subscription) => {
                        info!("✅ Subscribed to ACCOUNTS for program: {}", program_id_accounts);
                        subscription
                    }
                    Err(e) => {
                        error!("❌ Failed to subscribe to accounts for program {}: {}", program_id_accounts, e);
                        return;
                    }
                };
                
                while let Some(account_response) = accounts_stream.next().await {
                    debug!(
                        program_id = %program_id_accounts,
                        account_pubkey = %account_response.value.pubkey,
                        "💰 Received account update notification"
                    );
                    
                    if let Err(e) = tx_accounts.send(SubscriptionEvent::AccountUpdate(program_id_accounts, account_response)) {
                        error!("Failed to send account notification: {}", e);
                        break;
                    }
                }
                
                // Clean up subscription when done
                let _ = accounts_unsubscribe().await;
                info!("🛑 Account subscription ended for program: {}", program_id_accounts);
            });
        }

        info!("🎯 All subscriptions active, monitoring for transactions...");

        // Process notifications
        while let Some(event) = rx.recv().await {
            match event {
                SubscriptionEvent::LogsUpdate(program_id, logs_notification) => {
                    if let Err(e) = self.handle_logs_notification(program_id, logs_notification, &mut processor).await {
                        error!("Error handling logs notification: {}", e);
                    }
                }
                SubscriptionEvent::AccountUpdate(program_id, account_notification) => {
                    if let Err(e) = self.handle_account_notification(program_id, account_notification, &mut processor).await {
                        error!("Error handling account notification: {}", e);
                    }
                }
            }
        }

        info!("🛑 Monitoring stopped");

        Ok(())
    }

    async fn handle_logs_notification(
        &self,
        program_id: Pubkey,
        logs_notification: solana_client::rpc_response::Response<solana_client::rpc_response::RpcLogsResponse>,
        processor: &mut LiteProcessor,
    ) -> Result<()> {
        let signature = &logs_notification.value.signature;
        let logs = &logs_notification.value.logs;

        // Skip failed transactions
        if logs_notification.value.err.is_some() {
            debug!("❌ Skipping failed transaction: {}", signature);
            return Ok(());
        }

        info!(
            signature = %signature,
            program_id = %program_id,
            logs_count = logs.len(),
            "📥 Processing transaction logs"
        );

        // Log the actual log messages at debug level
        for (i, log_msg) in logs.iter().enumerate() {
            debug!("Log {}: {}", i, log_msg);
        }

        // Fetch the full transaction
        match self.fetch_transaction(signature).await {
            Ok(Some(transaction)) => {
                let slot = transaction.slot;
                
                info!(
                    signature = %signature,
                    slot = slot,
                    program_id = %program_id,
                    "✅ Successfully fetched transaction - processing..."
                );
                
                if let Err(e) = processor.process_transaction(signature, slot, &transaction) {
                    error!("❌ Error processing transaction: {}", e);
                } else {
                    debug!("✅ Transaction processing completed successfully");
                }
            }
            Ok(None) => {
                warn!("⚠️ Transaction not found: {}", signature);
            }
            Err(e) => {
                error!("❌ Error fetching transaction {}: {}", signature, e);
            }
        }

        Ok(())
    }

    async fn handle_account_notification(
        &self,
        program_id: Pubkey,
        account_notification: solana_client::rpc_response::Response<solana_client::rpc_response::RpcKeyedAccount>,
        processor: &mut LiteProcessor,
    ) -> Result<()> {
        let account_pubkey = &account_notification.value.pubkey;
        let account_info = &account_notification.value.account;

        info!(
            account_pubkey = %account_pubkey,
            program_id = %program_id,
            lamports = account_info.lamports,
            data_type = ?account_info.data,
            "💰 Account updated for monitored program"
        );

        // For the lite processor, we'll just count this as activity
        // Update stats for the program
        // Note: We don't have direct access to processor here, but we could extend it
        // to handle account updates separately from transaction logs

        debug!(
            account_pubkey = %account_pubkey,
            program_id = %program_id,
            owner = %account_info.owner,
            executable = account_info.executable,
            rent_epoch = account_info.rent_epoch,
            "💰 Account details"
        );

        Ok(())
    }

    async fn fetch_transaction(
        &self,
        signature: &str,
    ) -> Result<Option<EncodedConfirmedTransactionWithStatusMeta>> {
        let signature = signature.parse()?;
        
        match self.rpc_client.get_transaction_with_config(
            &signature,
            solana_client::rpc_config::RpcTransactionConfig {
                encoding: Some(solana_transaction_status::UiTransactionEncoding::JsonParsed),
                commitment: Some(CommitmentConfig::confirmed()),
                max_supported_transaction_version: Some(0),
            },
        ) {
            Ok(transaction) => Ok(Some(transaction)),
            Err(e) => {
                if e.to_string().contains("not found") {
                    Ok(None)
                } else {
                    Err(e.into())
                }
            }
        }
    }
}