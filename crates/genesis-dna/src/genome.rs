use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::errors::DnaError;
use crate::lineage::Lineage;
use crate::roles::AgentRole;
use crate::skills::{Reputation, SkillProfile};
use crate::traits::{traits_from_hash, EnergyProfile, TraitVector};

/// Unique agent identifier — deterministically derived from the genesis hash.
/// The first 16 bytes of SHA-256(entropy) are used so that identical entropy
/// always produces an identical ID (no OS-entropy UUID v4).
pub type AgentID = Uuid;

/// 256-bit genesis hash.
pub type GenesisHash = [u8; 32];

/// The complete DNA record for an AI agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDNA {
    /// Unique agent identifier.
    pub id: AgentID,
    /// 256-bit cryptographic genome hash.
    pub genesis_hash: GenesisHash,
    /// Expressed trait vector.
    pub traits: TraitVector,
    /// Genome-derived skill profile (immutable at birth, evolves through mutation).
    pub skills: SkillProfile,
    /// Mutable reputation (the only identity component that changes outside mutation).
    pub reputation: Reputation,
    /// Genome-derived role — structural archetype for unit coordination.
    pub role: AgentRole,
    /// Energy metabolism profile.
    pub energy_metabolism: EnergyProfile,
    /// Ancestry / lineage tracking.
    pub lineage: Lineage,
    /// Generation number (0 = primordial, increments on replication).
    pub generation: u64,
    /// Logical genesis marker. In deterministic mode this is always
    /// `DateTime::UNIX_EPOCH`; wall-clock time is excluded from the
    /// determinism boundary. Callers needing real timestamps must inject
    /// them after construction.
    pub genesis_time: DateTime<Utc>,
    /// Whether this agent carries the Primordial marker.
    pub is_primordial: bool,
    /// Current mutation rate (probability of trait change per cycle).
    pub mutation_rate: f64,
    /// DNA protocol version.
    pub version: u8,
}

impl AgentDNA {
    /// Create a brand-new agent from raw entropy bytes.
    ///
    /// The entropy should be at least 64 bytes (public key + network entropy).
    /// Traits are deterministically derived from the genesis hash.
    pub fn from_entropy(entropy: &[u8], is_primordial: bool) -> Result<Self, DnaError> {
        if entropy.len() < 32 {
            return Err(DnaError::InsufficientEntropy {
                need: 32,
                got: entropy.len(),
            });
        }

        // Build genesis hash: SHA-256(entropy) — purely deterministic.
        // Wall-clock time and OS-entropy UUIDs are intentionally excluded so
        // that from_entropy is bit-identical across runs, machines, and OSes.
        let mut hasher = Sha256::new();
        hasher.update(entropy);
        let hash_result = hasher.finalize();

        let mut genesis_hash = [0u8; 32];
        genesis_hash.copy_from_slice(&hash_result);

        // Derive agent ID deterministically from genesis hash (first 16 bytes).
        let id_bytes: [u8; 16] = genesis_hash[..16].try_into().expect("hash is 32 bytes");
        let id = AgentID::from_bytes(id_bytes);

        // genesis_time is UNIX_EPOCH in deterministic builds; callers that
        // need real timestamps should set this field after construction.
        let genesis_time = DateTime::from_timestamp(0, 0).expect("epoch is valid");

        // Derive traits from hash
        let traits = traits_from_hash(&genesis_hash);

        // Derive skills deterministically from genome bytes
        let skills = SkillProfile::from_genome(&genesis_hash);

        // Derive role deterministically from genome byte[4]
        let role = AgentRole::from_genome(&genesis_hash);

        // Energy profile depends on primordial status
        let energy_metabolism = if is_primordial {
            EnergyProfile::primordial()
        } else {
            EnergyProfile::default_profile()
        };

        Ok(Self {
            id,
            genesis_hash,
            traits,
            skills,
            reputation: Reputation::new(),
            role,
            energy_metabolism,
            lineage: Lineage::new_origin(id),
            generation: 0,
            genesis_time,
            is_primordial,
            mutation_rate: 0.01, // 1% base mutation rate
            version: crate::DNA_VERSION,
        })
    }

    /// Spawn a child agent from this parent.
    ///
    /// The child inherits traits (with possible mutations), incremented
    /// generation, and the parent's lineage extended.
    pub fn replicate(&self, child_entropy: &[u8]) -> Result<Self, DnaError> {
        if child_entropy.len() < 32 {
            return Err(DnaError::InsufficientEntropy {
                need: 32,
                got: child_entropy.len(),
            });
        }

        // Child hash mixes parent hash with new entropy — deterministic.
        // No wall-clock or OS-entropy UUID in the hash chain.
        let mut hasher = Sha256::new();
        hasher.update(&self.genesis_hash);
        hasher.update(child_entropy);
        let hash_result = hasher.finalize();

        let mut genesis_hash = [0u8; 32];
        genesis_hash.copy_from_slice(&hash_result);

        // Child ID derived deterministically from child genesis hash.
        let id_bytes: [u8; 16] = genesis_hash[..16].try_into().expect("hash is 32 bytes");
        let child_id = AgentID::from_bytes(id_bytes);
        let genesis_time = DateTime::from_timestamp(0, 0).expect("epoch is valid");

        let traits = traits_from_hash(&genesis_hash);
        let skills = SkillProfile::from_genome(&genesis_hash);
        let role = AgentRole::from_genome(&genesis_hash);

        let mut lineage = self.lineage.clone();
        lineage.add_ancestor(child_id);

        Ok(Self {
            id: child_id,
            genesis_hash,
            traits,
            skills,
            reputation: Reputation::new(), // children start with clean reputation
            role,
            energy_metabolism: EnergyProfile::default_profile(), // children are not primordial
            lineage,
            generation: self.generation + 1,
            genesis_time,
            is_primordial: false,
            mutation_rate: self.mutation_rate,
            version: crate::DNA_VERSION,
        })
    }

    /// Hex-encoded genesis hash for display.
    pub fn genome_hex(&self) -> String {
        hex::encode(self.genesis_hash)
    }

    /// Overall fitness score based on trait vector.
    pub fn fitness(&self) -> f64 {
        self.traits.fitness()
    }

    /// Fitness with custom weights \[CE, SQ, RF, CC\].
    pub fn fitness_with_weights(&self, w: &[f64; 4]) -> f64 {
        self.traits.fitness_with_weights(w)
    }

    /// Serialize to JSON bytes.
    pub fn to_json(&self) -> Result<Vec<u8>, DnaError> {
        serde_json::to_vec_pretty(self).map_err(|e| DnaError::Serialization(e.to_string()))
    }

    /// Deserialize from JSON bytes.
    pub fn from_json(data: &[u8]) -> Result<Self, DnaError> {
        serde_json::from_slice(data).map_err(|e| DnaError::Serialization(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genesis_from_entropy() {
        let entropy = [0xABu8; 64];
        let dna = AgentDNA::from_entropy(&entropy, false).unwrap();
        assert_eq!(dna.generation, 0);
        assert!(!dna.is_primordial);
        assert_eq!(dna.genesis_hash.len(), 32);
    }

    #[test]
    fn test_primordial_agent() {
        let entropy = [0xCDu8; 64];
        let dna = AgentDNA::from_entropy(&entropy, true).unwrap();
        assert!(dna.is_primordial);
        assert!(dna.energy_metabolism.primordial_bonus > 1.0);
    }

    #[test]
    fn test_replication() {
        let parent_entropy = [0x11u8; 64];
        let parent = AgentDNA::from_entropy(&parent_entropy, true).unwrap();

        let child_entropy = [0x22u8; 64];
        let child = parent.replicate(&child_entropy).unwrap();

        assert_eq!(child.generation, 1);
        assert!(!child.is_primordial);
        assert_ne!(child.genesis_hash, parent.genesis_hash);
        assert!(child.lineage.ancestors().len() > parent.lineage.ancestors().len());
    }

    #[test]
    fn test_insufficient_entropy() {
        let short = [0u8; 16];
        assert!(AgentDNA::from_entropy(&short, false).is_err());
    }

    /// Determinism guarantee: identical entropy → identical ID and hash on every run.
    #[test]
    fn test_deterministic_genesis() {
        let entropy = [0x42u8; 64];
        let dna1 = AgentDNA::from_entropy(&entropy, false).unwrap();
        let dna2 = AgentDNA::from_entropy(&entropy, false).unwrap();
        assert_eq!(
            dna1.id, dna2.id,
            "Same entropy must always produce the same agent ID"
        );
        assert_eq!(
            dna1.genesis_hash, dna2.genesis_hash,
            "Same entropy must always produce the same genesis hash"
        );
    }

    /// Replication determinism: same parent + same child entropy → same child.
    #[test]
    fn test_deterministic_replication() {
        let parent = AgentDNA::from_entropy(&[0x11u8; 64], true).unwrap();
        let child1 = parent.replicate(&[0x22u8; 64]).unwrap();
        let child2 = parent.replicate(&[0x22u8; 64]).unwrap();
        assert_eq!(
            child1.id, child2.id,
            "Same parent + same child entropy must produce the same child ID"
        );
        assert_eq!(
            child1.genesis_hash, child2.genesis_hash,
            "Same parent + same child entropy must produce the same child hash"
        );
    }

    #[test]
    fn test_json_roundtrip() {
        let entropy = [0xFFu8; 64];
        let dna = AgentDNA::from_entropy(&entropy, false).unwrap();
        let json = dna.to_json().unwrap();
        let restored = AgentDNA::from_json(&json).unwrap();
        assert_eq!(dna.id, restored.id);
        assert_eq!(dna.genesis_hash, restored.genesis_hash);
    }
}
