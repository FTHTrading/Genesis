// settlement.rs — World settlement events + background loop
//
// The settlement loop consumes SettlementEvent structs from a channel and
// writes a LineageRecord for each one, optionally posting an epoch anchor
// to the lineage ledger.
//
// In the future, EpochAnchor events can be pinned to a cheap L2 call or
// Polygon checkpoint for tamper-evidence without per-event gas.

use std::sync::Arc;
use std::time::Duration;

use tokio::sync::mpsc;
use tracing::{error, info, warn};

use crate::{
    config::X402Config,
    lineage::{LineageLedger, LineageRecord},
};

// ── Event types ───────────────────────────────────────────────────────────

/// World-level events that produce lineage records.
#[derive(Debug, Clone)]
pub enum SettlementEvent {
    /// An AI agent was born.
    AgentBorn {
        /// First 8 hex chars of the genome hash.
        genome_hex_prefix: String,
        epoch:      u64,
        generation: u32,
        wallet:     Option<String>,   // patron who funded spawn, if any
    },
    /// An AI agent died (energy depleted or pruned).
    AgentDied {
        genome_hex_prefix: String,
        epoch:      u64,
    },
    /// End-of-epoch world anchor: a snapshot of world state.
    EpochAnchor {
        epoch:        u64,
        world_root:   String,   // hex hash of the canonical world snapshot
        population:   u64,
        total_supply: u64,      // in atomic WorldToken units
    },
    /// A microcredit batch was closed and settled on-chain.
    BatchClosed {
        batch_id:    String,
        tx_hash:     String,
        total_usdc:  u64,
        wallet:      String,
    },
    /// Any custom domain event.
    Custom {
        action_type: String,
        resource_id: String,
        amount_usdc: u64,
        wallet:      String,
        metadata:    Option<serde_json::Value>,
    },
}

// ── Channel types ─────────────────────────────────────────────────────────

pub type SettlementTx = mpsc::Sender<SettlementEvent>;
pub type SettlementRx = mpsc::Receiver<SettlementEvent>;

pub fn settlement_channel(capacity: usize) -> (SettlementTx, SettlementRx) {
    mpsc::channel(capacity)
}

// ── Background loop ───────────────────────────────────────────────────────

/// Start the settlement consumer loop in a spawned tokio task.
///
/// * `rx`      — Receives SettlementEvent values from the world runtime.
/// * `config`  — x402 config (for network/world_id labels).
/// * `lineage` — The lineage ledger to append records to.
pub fn start_settlement_loop(
    mut rx:     SettlementRx,
    config:     Arc<X402Config>,
    lineage:    Arc<LineageLedger>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        info!("x402 settlement loop started (network={})", config.network);
        loop {
            match rx.recv().await {
                None => {
                    warn!("x402 settlement channel closed — loop exiting");
                    break;
                }
                Some(event) => {
                    if let Err(e) = handle_event(&event, &config, &lineage) {
                        error!("settlement lineage write failed: {e}");
                    }
                }
            }
        }
    })
}

fn handle_event(
    event:   &SettlementEvent,
    config:  &X402Config,
    lineage: &LineageLedger,
) -> Result<(), crate::lineage::LineageError> {
    let network = config.network.clone();
    match event {
        SettlementEvent::AgentBorn { genome_hex_prefix, epoch, generation, wallet } => {
            let mut record = LineageRecord::new(
                wallet.as_deref().unwrap_or("0x0000000000000000000000000000000000000000"),
                "AGENT_SPAWN",
                format!("/world/agent/{genome_hex_prefix}"),
                0,
                &network,
            )
            .with_agent(genome_hex_prefix.clone());
            record.entitlement_id = Some(format!(
                "agent:{genome_hex_prefix}:gen{generation}:epoch{epoch}"
            ));
            lineage.append(&record)?;
            info!(
                agent = %genome_hex_prefix,
                epoch,
                generation,
                "SettlementEvent: AgentBorn → lineage"
            );
        }

        SettlementEvent::AgentDied { genome_hex_prefix, epoch } => {
            let record = LineageRecord::new(
                "0x0000000000000000000000000000000000000000",
                "AGENT_DIED",
                format!("/world/agent/{genome_hex_prefix}"),
                0,
                &network,
            )
            .with_agent(genome_hex_prefix.clone());
            lineage.append(&record)?;
            info!(
                agent = %genome_hex_prefix,
                epoch,
                "SettlementEvent: AgentDied → lineage"
            );
        }

        SettlementEvent::EpochAnchor { epoch, world_root, population, total_supply } => {
            let mut record = LineageRecord::new(
                "0x0000000000000000000000000000000000000000",
                "EPOCH_ANCHOR",
                format!("/world/epoch/{epoch}"),
                0,
                &network,
            );
            record.authorization_hash = world_root.clone();
            record.entitlement_id = Some(format!(
                "epoch:{epoch}:pop{population}:supply{total_supply}"
            ));
            lineage.append(&record)?;
            info!(
                epoch,
                world_root = %world_root,
                population,
                total_supply,
                "SettlementEvent: EpochAnchor → lineage"
            );
        }

        SettlementEvent::BatchClosed { batch_id, tx_hash, total_usdc, wallet } => {
            let record = LineageRecord::new(
                wallet.clone(),
                "BATCH_CLOSE",
                format!("/settlements/{batch_id}"),
                *total_usdc,
                &network,
            )
            .with_settlement_tx(tx_hash.clone());
            lineage.append(&record)?;
            info!(
                batch_id = %batch_id,
                tx_hash = %tx_hash,
                total_usdc,
                "SettlementEvent: BatchClosed → lineage"
            );
        }

        SettlementEvent::Custom { action_type, resource_id, amount_usdc, wallet, metadata } => {
            let mut record = LineageRecord::new(
                wallet.clone(),
                action_type.clone(),
                resource_id.clone(),
                *amount_usdc,
                &network,
            );
            if let Some(meta) = metadata {
                record.entitlement_id = meta.get("entitlement_id")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
            }
            lineage.append(&record)?;
            info!(
                action_type = %action_type,
                resource_id = %resource_id,
                amount_usdc,
                wallet = %wallet,
                "SettlementEvent: Custom → lineage"
            );
        }
    }
    Ok(())
}

// ── Retry wrapper ─────────────────────────────────────────────────────────

/// Retry a settlement loop if it exits unexpectedly (rare: only on channel close).
pub fn start_settlement_loop_with_retry(
    config:  Arc<X402Config>,
    lineage: Arc<LineageLedger>,
    capacity: usize,
) -> (SettlementTx, tokio::task::JoinHandle<()>) {
    let (tx, rx) = settlement_channel(capacity);
    let handle = {
        let config  = config.clone();
        let lineage = lineage.clone();
        tokio::spawn(async move {
            start_settlement_loop(rx, config, lineage).await.ok();
            // Re-launching not needed; channel is dead if sender dropped.
            warn!("x402 settlement loop permanently exited");
        })
    };
    (tx, handle)
}

// ── Periodic epoch anchor ─────────────────────────────────────────────────

/// Spawn a task that emits an EpochAnchor event every `interval` epochs.
/// The caller must supply a function that reads current world state.
pub fn start_epoch_anchor_task<F>(
    tx:          SettlementTx,
    interval:    Duration,
    get_epoch:   F,                // async fn() -> (epoch, world_root, population, total_supply)
) -> tokio::task::JoinHandle<()>
where
    F: Fn() -> (u64, String, u64, u64) + Send + Sync + 'static,
{
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(interval).await;
            let (epoch, world_root, population, total_supply) = get_epoch();
            if tx.send(SettlementEvent::EpochAnchor {
                epoch,
                world_root,
                population,
                total_supply,
            }).await.is_err() {
                warn!("epoch anchor: settlement channel closed");
                break;
            }
        }
    })
}
