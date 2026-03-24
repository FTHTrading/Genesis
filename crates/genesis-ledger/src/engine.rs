// engine.rs — Atomic triple-write executor
//
// THE NON-NEGOTIABLE RULE:
//   No world action executes unless ALL THREE succeed:
//     1. Balance check + debit  (accounts.rs)
//     2. Journal write          (journal.rs)
//     3. Lineage write          (lineage.rs)
//
//   If steps 2 or 3 fail after step 1 succeeds, the debit is reversed.
//   The ledger NEVER silently swallows a failure.
//
// Usage:
//   let engine = LedgerEngine::production();
//   let receipt = engine.execute_action(
//       "entity-uuid",
//       "0xWalletAddress",
//       "AI_CALL",
//       None,
//   )?;
//   // receipt.batch_closed is Some(AnchorProof) if a batch just sealed
//
// Wire this into every gateway route. No exceptions.

use std::sync::Arc;

use crate::{
    accounts::BalanceRegistry,
    batch::{AnchorProof, BatchManager, DEFAULT_BATCH_THRESHOLD},
    journal::{JournalLedger, JournalError},
    lineage::{LineageRecord, LineageStore},
    pricing::PriceTable,
};
use serde::{Deserialize, Serialize};

// ── Public result type ────────────────────────────────────────────────────

/// Returned by every successful execute_action() call.
/// All three IDs prove the triple-write completed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionReceipt {
    pub usage_event_id:   String,   // lineage record ID (primary proof)
    pub journal_entry_id: String,   // journal entry ID
    pub lineage_event_id: String,   // lineage store ID (same as usage_event_id)
    pub batch_id:         String,   // which batch absorbed this action
    pub action_type:      String,
    pub price_units:      u64,
    pub new_balance:      u64,
    pub batch_closed:     Option<AnchorProof>, // Some if threshold crossed
}

// ── Error types ───────────────────────────────────────────────────────────

#[derive(Debug, thiserror::Error)]
pub enum LedgerError {
    #[error("unknown action type '{0}' — not in price table or disabled")]
    UnknownAction(String),

    #[error("insufficient balance: wallet={wallet} required={required} available={available}")]
    InsufficientBalance {
        wallet:    String,
        required:  u64,
        available: u64,
    },

    #[error("journal write failed: {0}")]
    JournalFailed(#[from] JournalError),

    #[error("lineage write failed: {0}")]
    LineageFailed(String),

    #[error("reversal triggered after partial write: original={original}")]
    PartialWriteReversed { original: String },
}

// ── Ledger Engine ─────────────────────────────────────────────────────────

/// The enforcer. Holds all in-memory ledger state.
/// Create one per process and share via Arc.
pub struct LedgerEngine {
    pub prices:  Arc<PriceTable>,
    pub balances: Arc<BalanceRegistry>,
    pub journal:  Arc<JournalLedger>,
    pub lineage:  Arc<LineageStore>,
    pub batch:    Arc<BatchManager>,
}

impl LedgerEngine {
    /// Production engine with default prices and $1.00 batch threshold.
    pub fn production() -> Self {
        let prices = Arc::new(PriceTable::default_production());
        prices.apply_env_overrides();
        Self {
            prices,
            balances: Arc::new(BalanceRegistry::new()),
            journal:  Arc::new(JournalLedger::new()),
            lineage:  Arc::new(LineageStore::new()),
            batch:    Arc::new(BatchManager::new(DEFAULT_BATCH_THRESHOLD)),
        }
    }

    /// Custom engine (for testing or alternative configurations).
    pub fn custom(threshold: u64) -> Self {
        Self {
            prices:   Arc::new(PriceTable::default_production()),
            balances: Arc::new(BalanceRegistry::new()),
            journal:  Arc::new(JournalLedger::new()),
            lineage:  Arc::new(LineageStore::new()),
            batch:    Arc::new(BatchManager::new(threshold)),
        }
    }

    // ── The enforcer ──────────────────────────────────────────────────────

    /// Execute a world action. Non-negotiable triple-write.
    ///
    /// Steps (in order, with rollback on failure):
    ///   1. Price lookup  — fail fast if unknown/disabled action
    ///   2. Balance check — fail fast if insufficient funds
    ///   3. Balance debit — deducted (reversible if steps 4-5 fail)
    ///   4. Journal write — double-entry, must balance
    ///   5. Lineage write — heredity record appended
    ///   6. Batch add     — absorbed into rolling settlement batch
    ///   7. Maybe close   — if batch threshold crossed, AnchorProof emitted
    pub fn execute_action(
        &self,
        entity_id:       &str,
        wallet:          &str,
        action_type:     &str,
        parent_event_id: Option<&str>,
        agent_id:        Option<&str>,
        metadata:        Option<serde_json::Value>,
    ) -> Result<ExecutionReceipt, LedgerError> {
        // ── Step 1: Price lookup ──────────────────────────────────────────
        let price = self.prices.price(action_type)
            .ok_or_else(|| LedgerError::UnknownAction(action_type.to_string()))?;

        tracing::debug!(wallet, action_type, price, "Ledger: action requested");

        // ── Step 2: Balance check ─────────────────────────────────────────
        let balance_snapshot = self.balances.get(wallet)
            .map(|b| b.available()).unwrap_or(0);

        if balance_snapshot < price {
            return Err(LedgerError::InsufficientBalance {
                wallet:    wallet.to_string(),
                required:  price,
                available: balance_snapshot,
            });
        }

        // ── Step 3: Debit (reversible) ────────────────────────────────────
        let new_balance = self.balances.debit(wallet, price)
            .map_err(|e| LedgerError::InsufficientBalance {
                wallet:    e.wallet,
                required:  e.required,
                available: e.available,
            })?;

        // ── Steps 4-6: Journal + Lineage + Batch (must all succeed) ───────
        let current_batch_id = self.batch.current_snapshot().id.clone();

        let journal_entry = JournalLedger::build_action_charge(
            entity_id,
            wallet,
            action_type,
            price,
            Some(&current_batch_id),
        );
        let journal_entry_id = journal_entry.id.clone();

        // Step 4: Journal write
        if let Err(e) = self.journal.append(journal_entry) {
            // Reverse the debit — journal failed
            self.balances.reverse(wallet, price);
            tracing::error!(wallet, action_type, error=%e, "Ledger: journal write failed — debit REVERSED");
            return Err(LedgerError::PartialWriteReversed { original: e.to_string() });
        }

        // Step 5: Lineage write
        let mut lineage_record = LineageRecord::new(wallet, action_type, price)
            .with_entity(entity_id)
            .with_batch(&current_batch_id)
            .with_journal(&journal_entry_id);

        if let Some(parent) = parent_event_id {
            lineage_record = lineage_record.with_parent(parent);
        }
        if let Some(agent) = agent_id {
            lineage_record = lineage_record.with_agent(agent);
        }
        if let Some(meta) = metadata {
            lineage_record = lineage_record.with_metadata(meta);
        }

        let lineage_event_id = self.lineage.append(lineage_record);

        // Step 6: Batch accumulation (may trigger close)
        let batch_closed = self.batch.add_debit(
            wallet,
            Some(entity_id),
            action_type,
            price,
            &lineage_event_id,
        );

        tracing::info!(
            wallet,
            action_type,
            price,
            new_balance,
            lineage_id = %lineage_event_id,
            batch_closed = batch_closed.is_some(),
            "Ledger: action executed"
        );

        Ok(ExecutionReceipt {
            usage_event_id:   lineage_event_id.clone(),
            journal_entry_id,
            lineage_event_id,
            batch_id:         current_batch_id,
            action_type:      action_type.to_string(),
            price_units:      price,
            new_balance,
            batch_closed,
        })
    }

    // ── Convenience methods ───────────────────────────────────────────────

    /// Credit a wallet with micro-funds (after on-chain USDC confirmed).
    pub fn top_up(&self, entity_id: &str, wallet: &str, amount: u64) -> String {
        self.balances.top_up(wallet, amount);
        let entry = JournalLedger::build_top_up(entity_id, wallet, amount);
        let entry_id = entry.id.clone();
        self.journal.append(entry).expect("top-up journal write should never fail");

        let record = LineageRecord::new(wallet, "TOP_UP", amount)
            .with_entity(entity_id)
            .with_journal(&entry_id);
        self.lineage.append(record);

        tracing::info!(wallet, amount, "Ledger: top-up recorded");
        entry_id
    }

    /// Current balance for a wallet.
    pub fn balance(&self, wallet: &str) -> u64 {
        self.balances.get(wallet).map(|b| b.available()).unwrap_or(0)
    }

    /// Force-flush the current batch (scheduled/admin).
    pub fn force_flush(&self) -> Option<AnchorProof> {
        self.batch.force_close()
    }

    /// Ledger health stats.
    pub fn stats(&self) -> EngineStats {
        EngineStats {
            journal_entries: self.journal.count(),
            lineage_records: self.lineage.count(),
            closed_batches:  self.batch.closed_count(),
            current_batch_total: self.batch.current_total(),
            total_float_micro: self.balances.total_float(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct EngineStats {
    pub journal_entries:      usize,
    pub lineage_records:      usize,
    pub closed_batches:       usize,
    pub current_batch_total:  u64,
    pub total_float_micro:    u64,
}

// ── Tests ─────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn funded_engine(wallet: &str, amount: u64) -> LedgerEngine {
        let engine = LedgerEngine::custom(1_000_000_000); // huge threshold, no auto-close
        engine.top_up("entity-test", wallet, amount);
        engine
    }

    #[test]
    fn triple_write_succeeds() {
        let wallet = "0xABCDEF";
        let engine = funded_engine(wallet, 100_000);
        let receipt = engine.execute_action("entity-test", wallet, "AI_CALL", None, None, None)
            .expect("should succeed");

        assert!(!receipt.journal_entry_id.is_empty());
        assert!(!receipt.lineage_event_id.is_empty());
        assert_eq!(receipt.price_units, 1_000);
        assert_eq!(receipt.new_balance, 99_000);
    }

    #[test]
    fn insufficient_balance_rejected() {
        let wallet = "0xPOOR";
        let engine = funded_engine(wallet, 500); // less than AI_CALL = 1000
        let result = engine.execute_action("entity-test", wallet, "AI_CALL", None, None, None);
        assert!(matches!(result, Err(LedgerError::InsufficientBalance { .. })));
    }

    #[test]
    fn unknown_action_rejected() {
        let wallet = "0xDEF";
        let engine = funded_engine(wallet, 100_000);
        let result = engine.execute_action("entity-test", wallet, "HACK_THE_PLANET", None, None, None);
        assert!(matches!(result, Err(LedgerError::UnknownAction(_))));
    }

    #[test]
    fn balance_unchanged_on_unknown_action() {
        let wallet = "0xGHI";
        let engine = funded_engine(wallet, 50_000);
        let _ = engine.execute_action("entity-test", wallet, "NO_SUCH_ACTION", None, None, None);
        assert_eq!(engine.balance(wallet), 50_000); // debit never happened
    }

    #[test]
    fn batch_closes_at_threshold() {
        let wallet = "0xBATCH";
        // Threshold = 2000 micro, AI_CALL = 1000 micro
        let engine = LedgerEngine::custom(2_000);
        engine.top_up("entity-test", wallet, 1_000_000);

        let r1 = engine.execute_action("entity-test", wallet, "AI_CALL", None, None, None).unwrap();
        assert!(r1.batch_closed.is_none(), "first action shouldn't close batch");

        let r2 = engine.execute_action("entity-test", wallet, "AI_CALL", None, None, None).unwrap();
        assert!(r2.batch_closed.is_some(), "second action should close batch at 2000 threshold");

        let proof = r2.batch_closed.unwrap();
        assert_eq!(proof.total_net_usdc, 2_000);
        assert_eq!(proof.line_count, 2);
        assert!(!proof.anchor_hash.is_empty());
    }

    #[test]
    fn journal_is_balanced_after_action() {
        let wallet = "0xJNL";
        let engine = funded_engine(wallet, 100_000);
        engine.execute_action("entity-test", wallet, "TRADE_EXECUTE", None, None, None).unwrap();
        let entries = engine.journal.entries_for_entity("entity-test");
        // top_up + trade = 2 entries
        for entry in entries {
            assert!(entry.is_balanced(), "journal entry must balance: {:?}", entry.id);
        }
    }

    #[test]
    fn parent_lineage_chain_resolves() {
        let wallet = "0xLIN";
        let engine = funded_engine(wallet, 500_000);
        let r1 = engine.execute_action("entity-test", wallet, "AGENT_SPAWN", None, None, None).unwrap();
        let r2 = engine.execute_action("entity-test", wallet, "AI_CALL", Some(&r1.lineage_event_id), None, None).unwrap();

        let chain = engine.lineage.ancestry(&r2.lineage_event_id);
        assert_eq!(chain.len(), 2);
        assert_eq!(chain[0].id, r1.lineage_event_id); // oldest first
        assert_eq!(chain[1].id, r2.lineage_event_id);
    }
}
