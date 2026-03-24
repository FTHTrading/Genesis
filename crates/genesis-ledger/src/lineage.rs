// lineage.rs — In-memory heredity chain
//
// Mirrors the JSONL file ledger as a queryable in-memory structure.
// Every payment, action, top-up, and settlement creates one LineageRecord.
// Records are immutable once written. Parent chain forms ancestry tree.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use parking_lot::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageRecord {
    pub id:                 String,
    pub world_id:           String,
    pub wallet:             String,
    pub entity_id:          Option<String>,
    pub agent_id:           Option<String>,
    pub parent_event_id:    Option<String>,
    pub action_type:        String,
    pub amount_units:       u64,
    pub resource_id:        Option<String>,
    pub batch_id:           Option<String>,
    pub journal_entry_id:   Option<String>,
    pub settlement_tx_hash: Option<String>,
    pub metadata:           Option<serde_json::Value>,
    pub created_at:         chrono::DateTime<chrono::Utc>,
}

impl LineageRecord {
    pub fn new(
        wallet:      impl Into<String>,
        action_type: impl Into<String>,
        amount_units: u64,
    ) -> Self {
        Self {
            id:                 Uuid::new_v4().to_string(),
            world_id:           crate::WORLD_ID.to_string(),
            wallet:             wallet.into(),
            entity_id:          None,
            agent_id:           None,
            parent_event_id:    None,
            action_type:        action_type.into(),
            amount_units,
            resource_id:        None,
            batch_id:           None,
            journal_entry_id:   None,
            settlement_tx_hash: None,
            metadata:           None,
            created_at:         chrono::Utc::now(),
        }
    }

    pub fn with_entity(mut self, entity_id: impl Into<String>) -> Self {
        self.entity_id = Some(entity_id.into()); self
    }
    pub fn with_agent(mut self, agent_id: impl Into<String>) -> Self {
        self.agent_id = Some(agent_id.into()); self
    }
    pub fn with_parent(mut self, parent_id: impl Into<String>) -> Self {
        self.parent_event_id = Some(parent_id.into()); self
    }
    pub fn with_resource(mut self, resource_id: impl Into<String>) -> Self {
        self.resource_id = Some(resource_id.into()); self
    }
    pub fn with_batch(mut self, batch_id: impl Into<String>) -> Self {
        self.batch_id = Some(batch_id.into()); self
    }
    pub fn with_journal(mut self, journal_entry_id: impl Into<String>) -> Self {
        self.journal_entry_id = Some(journal_entry_id.into()); self
    }
    pub fn with_tx(mut self, tx_hash: impl Into<String>) -> Self {
        self.settlement_tx_hash = Some(tx_hash.into()); self
    }
    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata); self
    }
}

/// Full ancestry chain for a lineage record (oldest-first).
pub type LineageChain = Vec<LineageRecord>;

/// Thread-safe in-memory lineage store. Append-only.
pub struct LineageStore {
    records: RwLock<Vec<LineageRecord>>,
}

impl LineageStore {
    pub fn new() -> Self {
        Self { records: RwLock::new(Vec::new()) }
    }

    /// Append a new lineage record. Returns its ID.
    pub fn append(&self, record: LineageRecord) -> String {
        let id = record.id.clone();
        self.records.write().push(record);
        id
    }

    /// All records for a wallet address.
    pub fn by_wallet(&self, wallet: &str) -> Vec<LineageRecord> {
        self.records.read().iter()
            .filter(|r| r.wallet == wallet)
            .cloned().collect()
    }

    /// All records for an agent.
    pub fn by_agent(&self, agent_id: &str) -> Vec<LineageRecord> {
        self.records.read().iter()
            .filter(|r| r.agent_id.as_deref() == Some(agent_id))
            .cloned().collect()
    }

    /// Reconstruct ancestry chain for a record ID.
    pub fn ancestry(&self, event_id: &str) -> LineageChain {
        let records = self.records.read();
        let mut chain = Vec::new();
        let mut current = event_id.to_string();
        loop {
            let found = records.iter().find(|r| r.id == current);
            match found {
                None => break,
                Some(r) => {
                    chain.push(r.clone());
                    match &r.parent_event_id {
                        None => break,
                        Some(p) => current = p.clone(),
                    }
                }
            }
        }
        chain.reverse(); // oldest first
        chain
    }

    /// Total amount spent by a wallet.
    pub fn total_spent(&self, wallet: &str) -> u64 {
        self.records.read().iter()
            .filter(|r| r.wallet == wallet)
            .map(|r| r.amount_units)
            .sum()
    }

    /// Count of all records.
    pub fn count(&self) -> usize {
        self.records.read().len()
    }

    /// Recent N records, newest first.
    pub fn recent(&self, n: usize) -> Vec<LineageRecord> {
        let r = self.records.read();
        r.iter().rev().take(n).cloned().collect()
    }
}

impl Default for LineageStore {
    fn default() -> Self { Self::new() }
}
