// Organism Identity — Sovereign identity for a Genesis instance.
//
// Each organism has a unique identity hash derived from its genesis
// block (epoch 0 state). This identity is stable across restarts
// and allows other organisms to verify provenance.

use sha2::{Sha256, Digest};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

/// Sovereign identity of a Genesis organism instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganismIdentity {
    /// SHA-256 hash of the organism's genesis state (epoch 0).
    pub organism_id: String,
    /// Human-readable name.
    pub name: String,
    /// Protocol version.
    pub version: String,
    /// Genesis epoch timestamp.
    pub genesis_time: DateTime<Utc>,
    /// Current epoch (updated on telemetry exchange).
    pub current_epoch: u64,
    /// Current population.
    pub population: usize,
    /// Total ATP supply.
    pub total_supply: f64,
    /// Network address (host:port) for federation protocol.
    pub address: Option<String>,
}

impl OrganismIdentity {
    /// Create a new identity from genesis state.
    pub fn new(
        name: impl Into<String>,
        version: impl Into<String>,
        genesis_state_bytes: &[u8],
    ) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(b"genesis-organism-id-v1:");
        hasher.update(genesis_state_bytes);
        let organism_id = hex::encode(hasher.finalize());

        Self {
            organism_id,
            name: name.into(),
            version: version.into(),
            genesis_time: Utc::now(),
            current_epoch: 0,
            population: 0,
            total_supply: 0.0,
            address: None,
        }
    }

    /// Update live stats.
    pub fn update(&mut self, epoch: u64, population: usize, total_supply: f64) {
        self.current_epoch = epoch;
        self.population = population;
        self.total_supply = total_supply;
    }

    /// Short ID for display (first 16 hex chars).
    pub fn short_id(&self) -> &str {
        &self.organism_id[..self.organism_id.len().min(16)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_creation() {
        let id = OrganismIdentity::new(
            "Genesis Alpha",
            "1.2.0",
            b"primordial-state-bytes",
        );
        assert!(!id.organism_id.is_empty());
        assert_eq!(id.name, "Genesis Alpha");
        assert_eq!(id.short_id().len(), 16);
    }

    #[test]
    fn test_deterministic_id() {
        let id1 = OrganismIdentity::new("A", "1.0", b"same-state");
        let id2 = OrganismIdentity::new("B", "1.0", b"same-state");
        assert_eq!(id1.organism_id, id2.organism_id);
    }

    #[test]
    fn test_different_state_different_id() {
        let id1 = OrganismIdentity::new("A", "1.0", b"state-alpha");
        let id2 = OrganismIdentity::new("A", "1.0", b"state-beta");
        assert_ne!(id1.organism_id, id2.organism_id);
    }
}
