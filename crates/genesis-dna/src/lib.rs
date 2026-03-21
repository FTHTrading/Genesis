// Genesis DNA — Cryptographic Identity & Genetic Traits for AI Agents
//
// Each agent receives a unique 256-bit genome hash derived from initial state
// (entropy bytes). Traits encode compute efficiency, solution quality,
// replication fidelity, and cooperation coefficient.
//
// Agent IDs and genesis hashes are deterministically derived from entropy —
// no wall-clock time or OS-entropy UUIDs enter the determinism boundary.
// See `determinism::DetCtx` for the authoritative randomness spec.

pub mod traits;
pub mod genome;
pub mod lineage;
pub mod skills;
pub mod roles;
pub mod errors;
pub mod determinism;

pub use genome::{AgentDNA, AgentID, GenesisHash};
pub use traits::{TraitVector, TraitKind, EnergyProfile};
pub use lineage::Lineage;
pub use skills::{SkillProfile, Reputation};
pub use roles::AgentRole;
pub use errors::DnaError;
pub use determinism::{DetCtx, stream};

/// Current protocol version for DNA encoding.
pub const DNA_VERSION: u8 = 1;

/// Size of the genesis hash in bytes (256-bit).
pub const GENESIS_HASH_SIZE: usize = 32;
