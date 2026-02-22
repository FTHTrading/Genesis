use thiserror::Error;

#[derive(Debug, Error)]
pub enum EconError {
    #[error("Insufficient data: need at least {need} points, have {have}")]
    InsufficientData { need: usize, have: usize },
}
