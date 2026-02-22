use thiserror::Error;

#[derive(Debug, Error)]
pub enum AnchorError {
    #[error("Chain integrity violated at epoch {epoch}: expected {expected}, got {actual}")]
    ChainIntegrity {
        epoch: u64,
        expected: String,
        actual: String,
    },

    #[error("Anchor not found for epoch {0}")]
    NotFound(u64),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Merkle proof verification failed for leaf {0}")]
    ProofFailed(String),
}
