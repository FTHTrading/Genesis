use thiserror::Error;

#[derive(Debug, Error)]
pub enum HomeostasisError {
    #[error("Insufficient history: need at least {need} epochs, have {have}")]
    InsufficientHistory { need: usize, have: usize },
}
