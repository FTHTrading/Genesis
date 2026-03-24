// lineage.rs — JSONL heredity ledger
//
// Every payment, settlement, and entitlement is appended as a single JSON line.
// Each record may carry `parent_event_id` to form an ancestry chain:
//   TopUp → ActionCreditBurn → BatchSettle → TreasuryJournal
//
// The ledger is append-only. Reads are by full scan (small enough for nightly ETL).
// For production query patterns, ETL this file into the Prisma `JournalLine` table.

use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::PathBuf,
    sync::Mutex,
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LineageError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
}

// ── Record ────────────────────────────────────────────────────────────────

/// One immutable entry in the lineage ledger.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageRecord {
    /// Globally unique event ID (UUID v4).
    pub event_id: String,
    /// World namespace (e.g. "genesis-1").
    pub world_id: String,
    /// Payer EVM address (0x…).
    pub wallet: String,
    /// AI agent genome hex prefix or UUID, if the action was triggered by an agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    /// Unique intent ID for the payment round-trip.
    pub payment_intent_id: String,
    /// Raw PAYMENT-SIGNATURE payload (base64).
    pub authorization_hash: String,
    /// On-chain settlement tx hash, if already settled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_tx_hash: Option<String>,
    /// Parent event (e.g. the TopUp that funded this action).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_event_id: Option<String>,
    /// HTTP resource path (e.g. "/api/ai-call").
    pub resource_id: String,
    /// Off-chain entitlement ID granted after payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlement_id: Option<String>,
    /// DB revenue split record ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revenue_split_id: Option<String>,
    /// High-level action type label (mirrors x402PaymentAdapter.ActionType).
    pub action_type: String,
    /// Amount settled in USDC atomic units (6 decimals).
    pub amount_usdc: u64,
    /// CAIP-2 network identifier, e.g. "eip155:137".
    pub network: String,
    /// RFC 3339 timestamp.
    pub timestamp: String,
}

// ── Ledger ────────────────────────────────────────────────────────────────

/// Thread-safe append-only JSONL lineage ledger.
pub struct LineageLedger {
    path: PathBuf,
    lock: Mutex<()>,
}

impl LineageLedger {
    /// Open (or create) the ledger at `path`.
    pub fn open(path: PathBuf) -> Self {
        // Ensure parent directories exist.
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).ok();
        }
        Self {
            path,
            lock: Mutex::new(()),
        }
    }

    /// Append a single record as one JSON line.
    pub fn append(&self, record: &LineageRecord) -> Result<(), LineageError> {
        let _guard = self.lock.lock().expect("lineage mutex poisoned");
        let mut f = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;
        let mut line = serde_json::to_string(record)?;
        line.push('\n');
        f.write_all(line.as_bytes())?;
        Ok(())
    }

    /// Read all records (for ETL / reporting). Returns in append order.
    pub fn all(&self) -> Result<Vec<LineageRecord>, LineageError> {
        let _guard = self.lock.lock().expect("lineage mutex poisoned");
        let f = match File::open(&self.path) {
            Ok(f) => f,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(vec![]),
            Err(e) => return Err(LineageError::Io(e)),
        };
        let mut records = Vec::new();
        for line in BufReader::new(f).lines() {
            let line = line?;
            let trimmed = line.trim();
            if trimmed.is_empty() { continue; }
            let r: LineageRecord = serde_json::from_str(trimmed)?;
            records.push(r);
        }
        Ok(records)
    }

    /// Filter by wallet address.
    pub fn by_wallet(&self, wallet: &str) -> Result<Vec<LineageRecord>, LineageError> {
        Ok(self.all()?.into_iter().filter(|r| r.wallet == wallet).collect())
    }

    /// Filter by agent ID.
    pub fn by_agent(&self, agent_id: &str) -> Result<Vec<LineageRecord>, LineageError> {
        Ok(self.all()?.into_iter()
            .filter(|r| r.agent_id.as_deref() == Some(agent_id))
            .collect())
    }

    /// Reconstruct the full ancestry chain for a given event ID.
    pub fn ancestry(&self, event_id: &str) -> Result<Vec<LineageRecord>, LineageError> {
        let all = self.all()?;
        let mut chain = Vec::new();
        let mut current = event_id.to_string();
        loop {
            let found = all.iter().find(|r| r.event_id == current);
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
        Ok(chain)
    }

    /// Total USDC settled (for a given wallet, or all wallets if None).
    pub fn total_settled(&self, wallet: Option<&str>) -> Result<u64, LineageError> {
        let records = match wallet {
            Some(w) => self.by_wallet(w)?,
            None    => self.all()?,
        };
        Ok(records.iter()
            .filter(|r| r.settlement_tx_hash.is_some())
            .map(|r| r.amount_usdc)
            .sum())
    }
}

// ── Record builder helpers ────────────────────────────────────────────────

impl LineageRecord {
    pub fn new(
        wallet:       impl Into<String>,
        action_type:  impl Into<String>,
        resource_id:  impl Into<String>,
        amount_usdc:  u64,
        network:      impl Into<String>,
    ) -> Self {
        Self {
            event_id:           uuid::Uuid::new_v4().to_string(),
            world_id:           crate::WORLD_ID.to_string(),
            wallet:             wallet.into(),
            agent_id:           None,
            payment_intent_id:  uuid::Uuid::new_v4().to_string(),
            authorization_hash: String::new(),
            settlement_tx_hash: None,
            parent_event_id:    None,
            resource_id:        resource_id.into(),
            entitlement_id:     None,
            revenue_split_id:   None,
            action_type:        action_type.into(),
            amount_usdc,
            network:            network.into(),
            timestamp:          chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn with_agent(mut self, agent_id: impl Into<String>) -> Self {
        self.agent_id = Some(agent_id.into());
        self
    }

    pub fn with_parent(mut self, parent_event_id: impl Into<String>) -> Self {
        self.parent_event_id = Some(parent_event_id.into());
        self
    }

    pub fn with_settlement_tx(mut self, tx_hash: impl Into<String>) -> Self {
        self.settlement_tx_hash = Some(tx_hash.into());
        self
    }

    pub fn with_auth(mut self, auth_hash: impl Into<String>) -> Self {
        self.authorization_hash = auth_hash.into();
        self
    }
}
