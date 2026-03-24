// batch.rs — Settlement batch accumulation and anchor proof
//
// How batches work:
//   1. UsageEvents accumulate into the OPEN batch.
//   2. When batch crosses threshold (default $1.00 = 1_000_000 micro),
//      the batch is CLOSING.
//   3. AnchorProof is computed: keccak256(canonical serialization of all lines).
//   4. Summary is returned — caller posts to SettlementAnchor.sol on Polygon.
//   5. Batch moves to CLOSED.
//
// This is the ONLY Polygon interaction. No per-action gas.

use std::collections::HashMap; // used by BatchManager::wallet_totals if extended
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};
use uuid::Uuid;

/// Default batch close threshold: 1,000,000 micro = $1.00 USDC.
pub const DEFAULT_BATCH_THRESHOLD: u64 = 1_000_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BatchStatus {
    Open,
    Closing,
    Closed,
    Failed,
}

/// One line within a settlement batch.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchLine {
    pub wallet:       String,
    pub entity_id:    Option<String>,
    pub action_type:  String,
    pub amount_units: u64,
    pub direction:    String,   // "DEBIT" | "CREDIT"
    pub asset_type:   String,   // "MICRO"
    pub event_id:     String,   // source UsageEvent / LineageRecord ID
    pub created_at:   chrono::DateTime<chrono::Utc>,
}

/// Cryptographic proof that the batch is tamper-evident.
/// This is what gets posted to SettlementAnchor.sol.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnchorProof {
    pub batch_id:       String,
    pub anchor_hash:    String,     // hex keccak256 of canonical batch lines
    pub line_count:     u64,
    pub total_net_usdc: u64,        // micro-units (6 decimals)
    pub closed_at:      chrono::DateTime<chrono::Utc>,
}

/// A settlement batch.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettlementBatch {
    pub id:             String,
    pub seq:            u64,
    pub status:         BatchStatus,
    pub lines:          Vec<BatchLine>,
    pub total_debited:  u64,
    pub opened_at:      chrono::DateTime<chrono::Utc>,
    pub closed_at:      Option<chrono::DateTime<chrono::Utc>>,
    pub anchor_proof:   Option<AnchorProof>,
    pub on_chain_tx:    Option<String>,
}

impl SettlementBatch {
    fn new(seq: u64) -> Self {
        Self {
            id:            Uuid::new_v4().to_string(),
            seq,
            status:        BatchStatus::Open,
            lines:         Vec::new(),
            total_debited: 0,
            opened_at:     chrono::Utc::now(),
            closed_at:     None,
            anchor_proof:  None,
            on_chain_tx:   None,
        }
    }

    /// Add a usage event debit to the batch.
    pub fn add_debit(
        &mut self,
        wallet:      &str,
        entity_id:   Option<&str>,
        action_type: &str,
        amount:      u64,
        event_id:    &str,
    ) {
        self.lines.push(BatchLine {
            wallet:       wallet.to_string(),
            entity_id:    entity_id.map(str::to_string),
            action_type:  action_type.to_string(),
            amount_units: amount,
            direction:    "DEBIT".to_string(),
            asset_type:   "MICRO".to_string(),
            event_id:     event_id.to_string(),
            created_at:   chrono::Utc::now(),
        });
        self.total_debited += amount;
    }

    /// Compute the anchor hash for this batch.
    /// Format: keccak256(JSON-canonical serialization of sorted lines).
    pub fn compute_anchor_hash(&self) -> String {
        // Sort lines deterministically: by (wallet, created_at, event_id)
        let mut sorted = self.lines.clone();
        sorted.sort_by(|a, b| {
            a.wallet.cmp(&b.wallet)
                .then(a.created_at.cmp(&b.created_at))
                .then(a.event_id.cmp(&b.event_id))
        });

        // Canonical JSON
        let payload = serde_json::json!({
            "batchId":      self.id,
            "seq":          self.seq,
            "totalDebited": self.total_debited,
            "lineCount":    sorted.len(),
            "lines":        sorted,
        });
        let canonical = payload.to_string();

        let mut hasher = Keccak256::new();
        hasher.update(canonical.as_bytes());
        hex::encode(hasher.finalize())
    }

    /// Close the batch and produce an AnchorProof.
    pub fn close(&mut self) -> AnchorProof {
        let closed_at   = chrono::Utc::now();
        let anchor_hash = self.compute_anchor_hash();

        let proof = AnchorProof {
            batch_id:       self.id.clone(),
            anchor_hash:    anchor_hash.clone(),
            line_count:     self.lines.len() as u64,
            total_net_usdc: self.total_debited,
            closed_at,
        };

        self.status     = BatchStatus::Closed;
        self.closed_at  = Some(closed_at);
        self.anchor_proof = Some(proof.clone());

        proof
    }
}

// ── Batch manager ─────────────────────────────────────────────────────────

/// Manages the rolling batch window. Thread-safe.
pub struct BatchManager {
    current:    Mutex<SettlementBatch>,
    history:    Mutex<Vec<SettlementBatch>>,
    seq:        Mutex<u64>,
    threshold:  u64,
}

impl BatchManager {
    pub fn new(threshold: u64) -> Self {
        Self {
            current:   Mutex::new(SettlementBatch::new(1)),
            history:   Mutex::new(Vec::new()),
            seq:       Mutex::new(1),
            threshold,
        }
    }

    /// Add a debit to the current batch.
    /// Returns Some(AnchorProof) if the batch closed due to threshold crossing.
    pub fn add_debit(
        &self,
        wallet:      &str,
        entity_id:   Option<&str>,
        action_type: &str,
        amount:      u64,
        event_id:    &str,
    ) -> Option<AnchorProof> {
        let mut batch = self.current.lock();
        batch.add_debit(wallet, entity_id, action_type, amount, event_id);

        if batch.total_debited >= self.threshold {
            let proof = batch.close();

            // Rotate to history, open new batch
            let mut seq = self.seq.lock();
            *seq += 1;
            let new_batch = SettlementBatch::new(*seq);
            let old_batch = std::mem::replace(&mut *batch, new_batch);
            self.history.lock().push(old_batch);

            tracing::info!(
                batch_id  = %proof.batch_id,
                hash      = %proof.anchor_hash,
                total     = proof.total_net_usdc,
                lines     = proof.line_count,
                "Batch closed — ready to anchor on Polygon"
            );
            Some(proof)
        } else {
            None
        }
    }

    /// Force-close the current batch (e.g. scheduled flush).
    pub fn force_close(&self) -> Option<AnchorProof> {
        let mut batch = self.current.lock();
        if batch.lines.is_empty() { return None; }

        let proof = batch.close();
        let mut seq = self.seq.lock();
        *seq += 1;
        let new_batch = SettlementBatch::new(*seq);
        let old_batch = std::mem::replace(&mut *batch, new_batch);
        self.history.lock().push(old_batch);

        tracing::info!(
            batch_id = %proof.batch_id,
            "Batch force-closed"
        );
        Some(proof)
    }

    /// Current batch state (snapshot).
    pub fn current_snapshot(&self) -> SettlementBatch {
        self.current.lock().clone()
    }

    /// Most recent N closed batches.
    pub fn recent_closed(&self, n: usize) -> Vec<SettlementBatch> {
        let h = self.history.lock();
        h.iter().rev().take(n).cloned().collect()
    }

    /// Net debited in current open batch.
    pub fn current_total(&self) -> u64 {
        self.current.lock().total_debited
    }

    /// Number of closed batches.
    pub fn closed_count(&self) -> usize {
        self.history.lock().len()
    }
}

impl Default for BatchManager {
    fn default() -> Self {
        Self::new(DEFAULT_BATCH_THRESHOLD)
    }
}
