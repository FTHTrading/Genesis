// genesis-x402 — x402 payment layer for Genesis Protocol
//
// Architecture:
//   config.rs     — X402Config::from_env() for runtime configuration
//   wallet.rs     — EVM key vault (AES-256-GCM + Argon2id), address derivation
//   eip3009.rs    — EIP-712 / EIP-3009 signing for USDC transferWithAuthorization
//   facilitator.rs — In-house ECDSA verifier (replaces CDP; no external HTTP)
//   middleware.rs  — Axum x402 payment middleware (Genesis as seller)
//   lineage.rs    — Transaction heredity ledger (JSONL, full ancestry chain)
//   settlement.rs — Milestone settlement events (agent birth/death/anchor)
//
// Payment flow (Genesis as seller):
//   Client → GET /api/ai-call
//   Genesis → 402 + PAYMENT-REQUIRED header (USDC amount, payTo, network)
//   Client → GET /api/ai-call + PAYMENT-SIGNATURE header (signed EIP-3009 auth)
//   Genesis → InHouseVerifier::verify() → local k256 ECDSA recovery → payer address
//   Genesis → 200 + resource + PAYMENT-RESPONSE header (status: pending-batch-settlement)
//   Genesis → lineage.append(LineageRecord { ... })
//   Genesis → genesis-ledger.execute_action() → triple-write (balance+journal+lineage)
//   Genesis → batch accumulates → SettlementAnchor.sol on Polygon (ONE tx per batch)
//
// Token layers:
//   GENESIS (Polygon ERC-20, 10B) — internal economy, identity, incentives
//   USDC (Polygon, EIP-3009)      — real x402 payment settlement
//
// External dependencies: ONLY Polygon mainnet. No CDP. No third-party facilitator.

pub mod config;
pub mod eip3009;
pub mod facilitator;
pub mod lineage;
pub mod middleware;
pub mod settlement;
pub mod wallet;

pub use config::X402Config;
pub use lineage::{LineageRecord, LineageLedger};
pub use middleware::{x402_gate, X402State, HEADER_PAYMENT_REQUIRED, HEADER_PAYMENT_SIGNATURE, HEADER_PAYMENT_RESPONSE};
pub use settlement::{SettlementEvent, start_settlement_loop};
pub use wallet::GenesisWallet;

/// x402 library version — sent in payment headers.
pub const X402_VERSION: u32 = 2;

/// USDC contract address on Polygon mainnet.
pub const USDC_POLYGON: &str = "0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359";

/// Polygon mainnet CAIP-2 network identifier.
pub const NETWORK_POLYGON: &str = "eip155:137";

/// CDP facilitator mainnet endpoint.
pub const CDP_FACILITATOR_URL: &str = "https://api.cdp.coinbase.com/platform/v2/x402";

/// World identifier embedded in every lineage record.
pub const WORLD_ID: &str = "genesis-1";
