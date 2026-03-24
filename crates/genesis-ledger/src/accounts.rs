// accounts.rs — Wallet balance management
//
// Tracks micro-credit balances per wallet (EVM address).
// All balances in micro-units (1,000,000 = $1.00 USDC).
//
// Thread-safe: RwLock over the balance map.
// Balance check + debit is done under a write lock to prevent races.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use parking_lot::RwLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccountType {
    Asset,
    Liability,
    Equity,
    Revenue,
    Expense,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    pub wallet:           String,    // EVM address
    pub micro_credit:     u64,       // available prepaid balance
    pub total_deposited:  u64,       // cumulative top-up amount
    pub total_spent:      u64,       // cumulative debit amount
    pub total_reversed:   u64,       // cumulative reversal credits
    pub last_updated_at:  chrono::DateTime<chrono::Utc>,
}

impl Balance {
    pub fn new(wallet: impl Into<String>) -> Self {
        Self {
            wallet:          wallet.into(),
            micro_credit:    0,
            total_deposited: 0,
            total_spent:     0,
            total_reversed:  0,
            last_updated_at: chrono::Utc::now(),
        }
    }

    pub fn available(&self) -> u64 {
        self.micro_credit
    }

    pub fn has_funds(&self, amount: u64) -> bool {
        self.micro_credit >= amount
    }
}

// A WorldAccount entry in the chart of accounts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id:           String,
    pub account_type: AccountType,
    pub name:         String,
    pub balance:      i128,     // signed for double-entry (debits positive, credits negative)
    pub currency:     String,   // "USDC" | "WORLD" | "MICRO"
}

/// Thread-safe wallet balance registry.
pub struct BalanceRegistry {
    balances: RwLock<HashMap<String, Balance>>,
}

impl BalanceRegistry {
    pub fn new() -> Self {
        Self { balances: RwLock::new(HashMap::new()) }
    }

    /// Get a snapshot of a wallet's balance (read-only).
    pub fn get(&self, wallet: &str) -> Option<Balance> {
        self.balances.read().get(wallet).cloned()
    }

    /// Top up a wallet's micro-credit balance. Called after on-chain USDC deposit confirmed.
    pub fn top_up(&self, wallet: &str, amount: u64) {
        let mut map = self.balances.write();
        let entry = map.entry(wallet.to_string()).or_insert_with(|| Balance::new(wallet));
        entry.micro_credit    += amount;
        entry.total_deposited += amount;
        entry.last_updated_at  = chrono::Utc::now();
        tracing::info!(wallet, amount, balance = entry.micro_credit, "Ledger: top-up");
    }

    /// Atomically check balance and debit if sufficient.
    /// Returns Ok(new_balance) on success, Err if insufficient.
    pub fn debit(&self, wallet: &str, amount: u64) -> Result<u64, InsufficientFunds> {
        let mut map = self.balances.write();
        let entry = map.entry(wallet.to_string()).or_insert_with(|| Balance::new(wallet));

        if entry.micro_credit < amount {
            return Err(InsufficientFunds {
                wallet:    wallet.to_string(),
                required:  amount,
                available: entry.micro_credit,
            });
        }

        entry.micro_credit   -= amount;
        entry.total_spent    += amount;
        entry.last_updated_at = chrono::Utc::now();
        Ok(entry.micro_credit)
    }

    /// Reverse a debit (for failed/disputed transactions).
    pub fn reverse(&self, wallet: &str, amount: u64) {
        let mut map = self.balances.write();
        let entry = map.entry(wallet.to_string()).or_insert_with(|| Balance::new(wallet));
        entry.micro_credit    += amount;
        entry.total_reversed  += amount;
        entry.last_updated_at  = chrono::Utc::now();
        tracing::warn!(wallet, amount, "Ledger: reversal applied");
    }

    /// All wallet balances snapshot (for reporting).
    pub fn snapshot(&self) -> Vec<Balance> {
        self.balances.read().values().cloned().collect()
    }

    /// Total prepaid micro-credit across all wallets.
    pub fn total_float(&self) -> u64 {
        self.balances.read().values().map(|b| b.micro_credit).sum()
    }
}

impl Default for BalanceRegistry {
    fn default() -> Self { Self::new() }
}

// ── Error types ───────────────────────────────────────────────────────────

#[derive(Debug, thiserror::Error)]
#[error("insufficient funds: wallet={wallet} required={required} available={available}")]
pub struct InsufficientFunds {
    pub wallet:    String,
    pub required:  u64,
    pub available: u64,
}
