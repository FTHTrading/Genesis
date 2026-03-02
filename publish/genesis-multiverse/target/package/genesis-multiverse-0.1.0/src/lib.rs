//! # Genesis Multiverse
//!
//! Multiverse-scale systemic experiment engine.
//!
//! Deterministic simulation across thousands of parallel worlds.
//! Phase transition detection. Collapse boundary mapping.
//! Sensitivity sweeps. Econometric modeling.
//!
//! ## What This Is
//!
//! Genesis Multiverse is a research-grade simulation ecosystem built in Rust.
//! It runs deterministic macroeconomic experiments across large parameter spaces,
//! identifies systemic phase transitions, and maps collapse boundaries with
//! statistical precision.
//!
//! ## Current Scale
//!
//! - 6,820 parallel world-runs
//! - 44 distinct configurations
//! - 13 internal crates
//! - 396 tests
//! - Zero collapses at P_floor=3
//! - Phase transition identified at floors 5-10
//!
//! ## Full Engine
//!
//! The complete simulation engine is at:
//! <https://github.com/FTHTrading/Genesis>
//!
//! ## Replication Challenge
//!
//! The replication challenge is open. Canonical data is publicly available.
//! See the repository for details.

/// Current version of this crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Total number of world-runs in the canonical experiment corpus.
pub const ENGINE_WORLDS: u32 = 6820;

/// Total number of distinct experiment configurations.
pub const ENGINE_EXPERIMENTS: u32 = 44;

/// Total number of Rust crates in the full engine.
pub const ENGINE_CRATES: u32 = 13;

/// Total test count across all engine crates.
pub const ENGINE_TESTS: u32 = 396;
