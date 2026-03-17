//! # Genesis Multiverse
//!
//! Public crate namespace for the Genesis Protocol simulation ecosystem.
//!
//! This crate is a **canonical entry point**, not the full engine. It publishes
//! the project's authoritative metadata constants and links to the complete
//! 13-crate workspace where the actual simulation runs.
//!
//! ## What This Crate Contains
//!
//! - Canonical experiment totals (worlds, configurations, tests, crates)
//! - Version and namespace ownership for the `genesis-multiverse` package name
//! - Repository and documentation linkage
//!
//! ## What This Crate Does NOT Contain
//!
//! The simulation engine, experiment runner, deterministic replay system,
//! econometric analysis, and all other runtime code live in the full workspace:
//!
//! <https://github.com/FTHTrading/Genesis>
//!
//! ## Current Scale (full engine)
//!
//! - 6,820 parallel world-runs across 44 configurations
//! - 13 internal crates, 403 tests, 26,581 source lines
//! - Zero collapses at P_floor=3 (phase transition at floors 5–10)
//! - DOI: `10.5281/zenodo.18729652`
//!
//! ## Replication Challenge
//!
//! The replication challenge is open. Clone the repo, run the experiments,
//! compare hashes. See the repository for the full protocol.

/// Current version of this crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Total number of world-runs in the canonical experiment corpus.
pub const ENGINE_WORLDS: u32 = 6820;

/// Total number of distinct experiment configurations.
pub const ENGINE_EXPERIMENTS: u32 = 44;

/// Total number of Rust crates in the full engine.
pub const ENGINE_CRATES: u32 = 13;

/// Total test count across all engine crates.
pub const ENGINE_TESTS: u32 = 403;
