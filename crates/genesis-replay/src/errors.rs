use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReplayError {
    #[error("Divergence at epoch {epoch}: {detail}")]
    Divergence { epoch: u64, detail: String },

    #[error("Anchor verification failed at epoch {0}")]
    AnchorMismatch(u64),

    #[error("Seed must be non-zero")]
    InvalidSeed,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
}
