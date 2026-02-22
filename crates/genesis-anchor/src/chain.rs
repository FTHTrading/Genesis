// Anchor Chain — Verification of anchor chain integrity
//
// Loads persisted anchors and verifies the chain from genesis
// to the latest epoch. Detects any breaks in cryptographic lineage.

use crate::anchor::EpochAnchor;
use crate::errors::AnchorError;

use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};

/// Result of a chain verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainVerification {
    /// Total anchors in the chain.
    pub total_anchors: usize,
    /// Whether the entire chain is valid.
    pub valid: bool,
    /// First epoch in the chain.
    pub first_epoch: u64,
    /// Last epoch in the chain.
    pub last_epoch: u64,
    /// If invalid, the epoch where the break occurred.
    pub break_at: Option<u64>,
    /// Human-readable message.
    pub message: String,
}

/// In-memory anchor chain for verification and querying.
pub struct AnchorChain {
    /// Anchors ordered by epoch.
    pub anchors: Vec<EpochAnchor>,
}

impl AnchorChain {
    /// Create an empty chain.
    pub fn new() -> Self {
        Self {
            anchors: Vec::new(),
        }
    }

    /// Load chain from a directory of epoch_N.json files.
    pub fn load_from_dir(dir: &str) -> Result<Self, AnchorError> {
        let mut anchors = Vec::new();

        let entries = std::fs::read_dir(dir)?;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.extension().map(|e| e == "json").unwrap_or(false) {
                let data = std::fs::read_to_string(&path)?;
                let anchor: EpochAnchor = serde_json::from_str(&data)?;
                anchors.push(anchor);
            }
        }

        anchors.sort_by_key(|a| a.epoch);
        Ok(Self { anchors })
    }

    /// Load chain from a filechain (anchor.log, one JSON per line).
    pub fn load_from_log(path: &str) -> Result<Self, AnchorError> {
        let data = std::fs::read_to_string(path)?;
        let mut anchors = Vec::new();

        for line in data.lines() {
            if line.trim().is_empty() {
                continue;
            }
            let anchor: EpochAnchor = serde_json::from_str(line)?;
            anchors.push(anchor);
        }

        anchors.sort_by_key(|a| a.epoch);
        Ok(Self { anchors })
    }

    /// Add an anchor to the chain (in-memory).
    pub fn push(&mut self, anchor: EpochAnchor) {
        self.anchors.push(anchor);
    }

    /// Verify cryptographic chain integrity.
    ///
    /// For each consecutive pair (A_i, A_{i+1}):
    ///   A_{i+1}.previous_root == A_i.epoch_root
    ///
    /// Also re-derives each epoch_root from its components to detect tampering.
    pub fn verify(&self) -> ChainVerification {
        if self.anchors.is_empty() {
            return ChainVerification {
                total_anchors: 0,
                valid: true,
                first_epoch: 0,
                last_epoch: 0,
                break_at: None,
                message: "Empty chain — trivially valid".into(),
            };
        }

        let first_epoch = self.anchors[0].epoch;
        let last_epoch = self.anchors.last().unwrap().epoch;

        for i in 0..self.anchors.len() {
            let anchor = &self.anchors[i];

            // Re-derive epoch_root to detect tampering
            let mut hasher = Sha256::new();
            hasher.update(anchor.epoch.to_le_bytes());
            hasher.update(anchor.ledger_root.as_bytes());
            hasher.update(anchor.world_root.as_bytes());
            let recomputed = hex::encode(hasher.finalize());

            if recomputed != anchor.epoch_root {
                return ChainVerification {
                    total_anchors: self.anchors.len(),
                    valid: false,
                    first_epoch,
                    last_epoch,
                    break_at: Some(anchor.epoch),
                    message: format!(
                        "Epoch {} root tampered: stored {} != recomputed {}",
                        anchor.epoch,
                        &anchor.epoch_root[..16],
                        &recomputed[..16]
                    ),
                };
            }

            // Chain linkage: previous_root must match prior anchor's epoch_root
            if i > 0 {
                let prev = &self.anchors[i - 1];
                if anchor.previous_root != prev.epoch_root {
                    return ChainVerification {
                        total_anchors: self.anchors.len(),
                        valid: false,
                        first_epoch,
                        last_epoch,
                        break_at: Some(anchor.epoch),
                        message: format!(
                            "Chain break at epoch {}: previous_root {} != epoch_{} root {}",
                            anchor.epoch,
                            &anchor.previous_root[..16],
                            prev.epoch,
                            &prev.epoch_root[..16]
                        ),
                    };
                }
            }
        }

        ChainVerification {
            total_anchors: self.anchors.len(),
            valid: true,
            first_epoch,
            last_epoch,
            break_at: None,
            message: format!(
                "Chain valid: {} anchors, epochs {}-{}",
                self.anchors.len(),
                first_epoch,
                last_epoch
            ),
        }
    }

    /// Get anchor by epoch number.
    pub fn get(&self, epoch: u64) -> Option<&EpochAnchor> {
        self.anchors.iter().find(|a| a.epoch == epoch)
    }

    /// Get the latest anchor.
    pub fn latest(&self) -> Option<&EpochAnchor> {
        self.anchors.last()
    }

    /// Total number of anchors.
    pub fn len(&self) -> usize {
        self.anchors.len()
    }

    /// Whether the chain is empty.
    pub fn is_empty(&self) -> bool {
        self.anchors.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::anchor::{AnchorEngine, AnchorMode, WorldSummary};

    fn make_summary(epoch: u64, pop: usize) -> WorldSummary {
        WorldSummary {
            epoch,
            population: pop,
            total_supply: pop as f64 * 50.0,
            treasury_reserve: 25.0,
            mean_fitness: 0.5,
            total_births: pop as u64,
            total_deaths: 0,
            role_counts: vec![],
        }
    }

    #[test]
    fn test_chain_verification() {
        let mut engine = AnchorEngine::new(100, AnchorMode::Local, "test");
        let balances = vec![("a".into(), 100.0), ("b".into(), 50.0)];

        let mut chain = AnchorChain::new();
        for epoch in (100..=500).step_by(100) {
            let summary = make_summary(epoch, 10);
            let anchor = engine.anchor(epoch, &balances, &summary);
            chain.push(anchor);
        }

        let result = chain.verify();
        assert!(result.valid, "{}", result.message);
        assert_eq!(result.total_anchors, 5);
    }

    #[test]
    fn test_detect_tamper() {
        let mut engine = AnchorEngine::new(100, AnchorMode::Local, "test");
        let balances = vec![("a".into(), 100.0)];

        let mut chain = AnchorChain::new();
        let anchor1 = engine.anchor(100, &balances, &make_summary(100, 5));
        chain.push(anchor1);

        let mut anchor2 = engine.anchor(200, &balances, &make_summary(200, 5));
        // Tamper with epoch_root
        anchor2.epoch_root = "deadbeef".repeat(8);
        chain.push(anchor2);

        let result = chain.verify();
        assert!(!result.valid);
        assert_eq!(result.break_at, Some(200));
    }
}
