// Merkle Tree — Binary hash tree for ATP ledger state proofs
//
// Builds a binary Merkle tree from agent balance data, producing
// a root hash and inclusion proofs for individual agents.

use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};

/// A leaf in the Merkle tree: agent_id || balance serialized and hashed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleLeaf {
    pub key: String,
    pub hash: [u8; 32],
}

/// Direction in a Merkle proof path.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProofDirection {
    Left,
    Right,
}

/// A Merkle inclusion proof: the path from a leaf to the root.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleProof {
    pub leaf_key: String,
    pub leaf_hash: String,
    pub path: Vec<(ProofDirection, String)>,
    pub root: String,
}

impl MerkleProof {
    /// Verify this proof against a given root hash.
    pub fn verify(&self) -> bool {
        let mut current = hex_to_bytes(&self.leaf_hash);
        for (direction, sibling_hex) in &self.path {
            let sibling = hex_to_bytes(sibling_hex);
            let mut hasher = Sha256::new();
            match direction {
                ProofDirection::Left => {
                    hasher.update(&sibling);
                    hasher.update(&current);
                }
                ProofDirection::Right => {
                    hasher.update(&current);
                    hasher.update(&sibling);
                }
            }
            current = hasher.finalize().into();
        }
        hex::encode(current) == self.root
    }
}

/// Binary Merkle tree built from sorted key-value pairs.
#[derive(Debug, Clone)]
pub struct MerkleTree {
    /// Leaf nodes (sorted by key).
    pub leaves: Vec<MerkleLeaf>,
    /// Internal nodes stored level-by-level (bottom-up).
    levels: Vec<Vec<[u8; 32]>>,
    /// Root hash.
    pub root: [u8; 32],
}

impl MerkleTree {
    /// Build a Merkle tree from key-value pairs.
    /// Keys should be unique. Values are arbitrary bytes.
    pub fn build(mut entries: Vec<(String, Vec<u8>)>) -> Self {
        // Sort for deterministic ordering
        entries.sort_by(|a, b| a.0.cmp(&b.0));

        if entries.is_empty() {
            return Self {
                leaves: Vec::new(),
                levels: Vec::new(),
                root: [0u8; 32],
            };
        }

        // Hash leaves: H(key || value)
        let leaves: Vec<MerkleLeaf> = entries
            .iter()
            .map(|(key, value)| {
                let mut hasher = Sha256::new();
                hasher.update(key.as_bytes());
                hasher.update(value);
                MerkleLeaf {
                    key: key.clone(),
                    hash: hasher.finalize().into(),
                }
            })
            .collect();

        let mut current_level: Vec<[u8; 32]> = leaves.iter().map(|l| l.hash).collect();
        let mut levels = vec![current_level.clone()];

        // Build tree bottom-up
        while current_level.len() > 1 {
            let mut next_level = Vec::new();
            for chunk in current_level.chunks(2) {
                let mut hasher = Sha256::new();
                hasher.update(&chunk[0]);
                if chunk.len() > 1 {
                    hasher.update(&chunk[1]);
                } else {
                    // Odd node: hash with itself
                    hasher.update(&chunk[0]);
                }
                next_level.push(hasher.finalize().into());
            }
            levels.push(next_level.clone());
            current_level = next_level;
        }

        let root = current_level[0];

        Self {
            leaves,
            levels,
            root,
        }
    }

    /// Get the hex-encoded root hash.
    pub fn root_hex(&self) -> String {
        hex::encode(self.root)
    }

    /// Generate an inclusion proof for a given key.
    pub fn proof(&self, key: &str) -> Option<MerkleProof> {
        let leaf_idx = self.leaves.iter().position(|l| l.key == key)?;

        let mut path = Vec::new();
        let mut idx = leaf_idx;

        for level in &self.levels[..self.levels.len().saturating_sub(1)] {
            let sibling_idx = if idx % 2 == 0 { idx + 1 } else { idx - 1 };
            let sibling_hash = if sibling_idx < level.len() {
                level[sibling_idx]
            } else {
                // Odd: sibling is self
                level[idx]
            };

            let direction = if idx % 2 == 0 {
                ProofDirection::Right
            } else {
                ProofDirection::Left
            };

            path.push((direction, hex::encode(sibling_hash)));
            idx /= 2;
        }

        Some(MerkleProof {
            leaf_key: key.to_string(),
            leaf_hash: hex::encode(self.leaves[leaf_idx].hash),
            path,
            root: self.root_hex(),
        })
    }
}

fn hex_to_bytes(s: &str) -> [u8; 32] {
    let bytes = hex::decode(s).unwrap_or_else(|_| vec![0u8; 32]);
    let mut arr = [0u8; 32];
    let len = bytes.len().min(32);
    arr[..len].copy_from_slice(&bytes[..len]);
    arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_tree() {
        let tree = MerkleTree::build(vec![]);
        assert_eq!(tree.root, [0u8; 32]);
        assert!(tree.leaves.is_empty());
    }

    #[test]
    fn test_single_leaf() {
        let tree = MerkleTree::build(vec![
            ("agent-1".into(), b"100.0".to_vec()),
        ]);
        assert_ne!(tree.root, [0u8; 32]);
        let proof = tree.proof("agent-1").unwrap();
        assert!(proof.verify());
    }

    #[test]
    fn test_multiple_leaves() {
        let entries = vec![
            ("agent-a".into(), b"50.0".to_vec()),
            ("agent-b".into(), b"75.0".to_vec()),
            ("agent-c".into(), b"100.0".to_vec()),
            ("agent-d".into(), b"25.0".to_vec()),
        ];
        let tree = MerkleTree::build(entries);
        assert_ne!(tree.root, [0u8; 32]);

        // Verify proofs for each leaf
        for leaf in &tree.leaves {
            let proof = tree.proof(&leaf.key).unwrap();
            assert!(proof.verify(), "Proof failed for {}", leaf.key);
        }
    }

    #[test]
    fn test_deterministic() {
        let entries = vec![
            ("z".into(), b"1".to_vec()),
            ("a".into(), b"2".to_vec()),
            ("m".into(), b"3".to_vec()),
        ];
        let tree1 = MerkleTree::build(entries.clone());
        let tree2 = MerkleTree::build(entries);
        assert_eq!(tree1.root, tree2.root);
    }

    #[test]
    fn test_tamper_detection() {
        let entries = vec![
            ("agent-1".into(), b"100.0".to_vec()),
            ("agent-2".into(), b"200.0".to_vec()),
        ];
        let tree = MerkleTree::build(entries);
        let mut proof = tree.proof("agent-1").unwrap();
        // Tamper with the leaf hash
        proof.leaf_hash = hex::encode([0xFFu8; 32]);
        assert!(!proof.verify());
    }
}
