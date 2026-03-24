// journal.rs — Double-entry journal
//
// Every action that touches value produces:
//   JournalEntry  — the transaction header
//   JournalLine[] — the debit/credit legs (minimum 2, must balance)
//
// Invariant: sum(debits) == sum(credits) within every JournalEntry.
// No silent state changes. If the journal write fails, the action fails.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use parking_lot::RwLock;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntryType {
    ActionCharge,       // world action debit
    TopUp,              // patron credit purchase
    Settlement,         // batch close
    Reversal,           // credit returned
    Emission,           // WorldToken emission
    FeeIncome,          // revenue split incoming
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalEntry {
    pub id:           String,
    pub entry_type:   EntryType,
    pub description:  String,
    pub entity_id:    Option<String>,
    pub batch_id:     Option<String>,
    pub created_at:   chrono::DateTime<chrono::Utc>,
    pub reversed_at:  Option<chrono::DateTime<chrono::Utc>>,
    pub lines:        Vec<JournalLine>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LineDirection {
    Debit,
    Credit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalLine {
    pub id:             String,
    pub entry_id:       String,
    pub direction:      LineDirection,
    pub account_name:   String,         // e.g. "MicroCredit:0xabc", "Revenue:Treasury"
    pub amount_units:   u64,            // always positive
    pub asset_type:     String,         // "USDC" | "WORLD" | "MICRO"
    pub entity_id:      Option<String>,
}

impl JournalEntry {
    /// Validate that the entry balances (debits == credits per asset type).
    pub fn is_balanced(&self) -> bool {
        let mut totals: HashMap<String, (u64, u64)> = HashMap::new();
        for line in &self.lines {
            let e = totals.entry(line.asset_type.clone()).or_default();
            match line.direction {
                LineDirection::Debit  => e.0 += line.amount_units,
                LineDirection::Credit => e.1 += line.amount_units,
            }
        }
        totals.values().all(|(d, c)| d == c)
    }
}

/// In-memory journal ledger. Append-only.
pub struct JournalLedger {
    entries: RwLock<Vec<JournalEntry>>,
}

impl JournalLedger {
    pub fn new() -> Self {
        Self { entries: RwLock::new(Vec::new()) }
    }

    /// Append a balanced journal entry. Rejects unbalanced entries.
    pub fn append(&self, entry: JournalEntry) -> Result<(), JournalError> {
        if !entry.is_balanced() {
            return Err(JournalError::Unbalanced {
                entry_id: entry.id.clone(),
            });
        }
        self.entries.write().push(entry);
        Ok(())
    }

    /// Build a standard action-charge entry (2-legged: wallet debit + revenue credit).
    pub fn build_action_charge(
        entity_id:    &str,
        wallet:       &str,
        action_type:  &str,
        amount_units: u64,
        batch_id:     Option<&str>,
    ) -> JournalEntry {
        let entry_id = Uuid::new_v4().to_string();
        let now      = chrono::Utc::now();

        let lines = vec![
            JournalLine {
                id:           Uuid::new_v4().to_string(),
                entry_id:     entry_id.clone(),
                direction:    LineDirection::Debit,
                account_name: format!("MicroCredit:{}", wallet),
                amount_units,
                asset_type:   "MICRO".to_string(),
                entity_id:    Some(entity_id.to_string()),
            },
            JournalLine {
                id:           Uuid::new_v4().to_string(),
                entry_id:     entry_id.clone(),
                direction:    LineDirection::Credit,
                account_name: "Revenue:WorldFee".to_string(),
                amount_units,
                asset_type:   "MICRO".to_string(),
                entity_id:    None,
            },
        ];

        JournalEntry {
            id:          entry_id,
            entry_type:  EntryType::ActionCharge,
            description: format!("{} charge for {}", action_type, wallet),
            entity_id:   Some(entity_id.to_string()),
            batch_id:    batch_id.map(str::to_string),
            created_at:  now,
            reversed_at: None,
            lines,
        }
    }

    /// Build a top-up entry (patron deposits USDC → micro-credit balance).
    pub fn build_top_up(
        entity_id:    &str,
        wallet:       &str,
        amount_units: u64,
    ) -> JournalEntry {
        let entry_id = Uuid::new_v4().to_string();
        let now      = chrono::Utc::now();

        let lines = vec![
            JournalLine {
                id:           Uuid::new_v4().to_string(),
                entry_id:     entry_id.clone(),
                direction:    LineDirection::Debit,
                account_name: "Asset:UsdcReserve".to_string(),
                amount_units,
                asset_type:   "USDC".to_string(),
                entity_id:    None,
            },
            JournalLine {
                id:           Uuid::new_v4().to_string(),
                entry_id:     entry_id.clone(),
                direction:    LineDirection::Credit,
                account_name: format!("Liability:MicroCredit:{}", wallet),
                amount_units,
                asset_type:   "USDC".to_string(),
                entity_id:    Some(entity_id.to_string()),
            },
        ];

        JournalEntry {
            id:          entry_id,
            entry_type:  EntryType::TopUp,
            description: format!("Top-up {} micro-credits for {}", amount_units, wallet),
            entity_id:   Some(entity_id.to_string()),
            batch_id:    None,
            created_at:  now,
            reversed_at: None,
            lines,
        }
    }

    /// Return entries for a given entity.
    pub fn entries_for_entity(&self, entity_id: &str) -> Vec<JournalEntry> {
        self.entries.read()
            .iter()
            .filter(|e| e.entity_id.as_deref() == Some(entity_id))
            .cloned()
            .collect()
    }

    /// Total count of entries.
    pub fn count(&self) -> usize {
        self.entries.read().len()
    }
}

impl Default for JournalLedger {
    fn default() -> Self { Self::new() }
}

// ── Errors ────────────────────────────────────────────────────────────────

#[derive(Debug, thiserror::Error)]
pub enum JournalError {
    #[error("unbalanced journal entry: {entry_id}")]
    Unbalanced { entry_id: String },
}
