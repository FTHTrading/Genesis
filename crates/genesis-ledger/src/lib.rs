// genesis-ledger — In-house micropayment ledger for Genesis Protocol
//
// Non-negotiable rule:
//   No world action executes unless balance check, journal write,
//   and lineage write all succeed — or all are rolled back.
//
// Architecture:
//   pricing.rs   — action price table and lookup
//   accounts.rs  — wallet balance management
//   journal.rs   — double-entry journal (JournalEntry + JournalLine)
//   lineage.rs   — immutable heredity chain (in-memory mirror of JSONL ledger)
//   engine.rs    — atomic triple-write executor (the enforcer)
//   batch.rs     — settlement batch accumulation, anchor hash, Polygon proof

pub mod accounts;
pub mod batch;
pub mod engine;
pub mod journal;
pub mod lineage;
pub mod pricing;

pub use engine::{LedgerEngine, ExecutionReceipt, LedgerError};
pub use pricing::{PriceTable, ActionCategory};
pub use accounts::{Account, AccountType, Balance};
pub use batch::{SettlementBatch, BatchStatus, AnchorProof};
pub use journal::{JournalEntry, JournalLine, EntryType};
pub use lineage::{LineageRecord, LineageChain};

/// World identifier stamped on every ledger record.
pub const WORLD_ID: &str = "genesis-1";

/// Micro-unit conversion: 1_000_000 micro = $1.00 USDC (6 decimals)
pub const MICRO_PER_USDC: u64 = 1_000_000;
