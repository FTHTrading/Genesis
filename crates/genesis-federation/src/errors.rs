use thiserror::Error;

#[derive(Debug, Error)]
pub enum FederationError {
    #[error("Unknown organism: {0}")]
    UnknownOrganism(String),

    #[error("Handshake failed: {0}")]
    HandshakeFailed(String),

    #[error("Escrow insufficient: need {need:.2} ATP, have {have:.2}")]
    InsufficientEscrow { need: f64, have: f64 },

    #[error("Escrow already exists for pair ({0}, {1})")]
    EscrowExists(String, String),

    #[error("Gene module rejected: {0}")]
    GeneRejected(String),

    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
}
