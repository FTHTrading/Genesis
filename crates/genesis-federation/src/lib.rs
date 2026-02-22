// Genesis Federation — Multi-Organism Communication Protocol
//
// Allows multiple sovereign Genesis instances to communicate without
// merging. Each organism retains full sovereignty — federation is
// observational exchange, not governance injection.
//
// Capabilities:
// - Instance handshake (identity exchange)
// - Telemetry sharing (read-only organism stats)
// - Gene module exchange (horizontal transfer between organisms)
// - ATP escrow (inter-organism resource exchange)
// - Cross-organism statistics publication

pub mod identity;
pub mod protocol;
pub mod escrow;
pub mod errors;

pub use identity::OrganismIdentity;
pub use protocol::{FederationProtocol, FederationMessage, Handshake};
pub use escrow::AtpEscrow;
pub use errors::FederationError;
