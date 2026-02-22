// Genesis Replay — Deterministic Epoch Replay Engine
//
// Given a seed and initial conditions, replays exact evolutionary
// trajectories for N epochs. Verifies anchor chain integrity during
// replay. Produces divergence reports if non-determinism is detected.
//
// This makes Genesis scientifically defensible — any epoch range
// can be independently reproduced and verified.

pub mod engine;
pub mod trajectory;
pub mod report;
pub mod errors;

pub use engine::ReplayEngine;
pub use trajectory::Trajectory;
pub use report::ReplayReport;
pub use errors::ReplayError;
