// Federation Protocol — Message exchange between sovereign organisms.
//
// Organisms can handshake, exchange telemetry snapshots, and share
// gene modules — all without merging or losing sovereignty.

use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};

use crate::identity::OrganismIdentity;
use crate::errors::FederationError;

/// A handshake between two organisms — mutual identity verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Handshake {
    pub initiator: OrganismIdentity,
    pub responder: Option<OrganismIdentity>,
    pub challenge: String,
    pub response: Option<String>,
    pub established_at: Option<DateTime<Utc>>,
    pub valid: bool,
}

impl Handshake {
    /// Create a new handshake as initiator.
    pub fn initiate(identity: OrganismIdentity) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(identity.organism_id.as_bytes());
        hasher.update(Utc::now().to_rfc3339().as_bytes());
        let challenge = hex::encode(hasher.finalize());

        Self {
            initiator: identity,
            responder: None,
            challenge,
            response: None,
            established_at: None,
            valid: false,
        }
    }

    /// Respond to a handshake as the responder.
    pub fn respond(&mut self, identity: OrganismIdentity) {
        let mut hasher = Sha256::new();
        hasher.update(self.challenge.as_bytes());
        hasher.update(identity.organism_id.as_bytes());
        let response = hex::encode(hasher.finalize());

        self.responder = Some(identity);
        self.response = Some(response);
        self.established_at = Some(Utc::now());
        self.valid = true;
    }

    /// Verify the handshake is valid.
    pub fn verify(&self) -> bool {
        if let (Some(resp), Some(resp_id)) = (&self.response, &self.responder) {
            let mut hasher = Sha256::new();
            hasher.update(self.challenge.as_bytes());
            hasher.update(resp_id.organism_id.as_bytes());
            let expected = hex::encode(hasher.finalize());
            expected == *resp
        } else {
            false
        }
    }
}

/// Federation message types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FederationMessage {
    /// Request handshake from another organism.
    HandshakeRequest(Handshake),
    /// Response to handshake request.
    HandshakeResponse(Handshake),
    /// Share telemetry snapshot.
    TelemetryExchange(TelemetrySnapshot),
    /// Offer a gene module for cross-organism transfer.
    GeneOffer(GeneExchangeOffer),
    /// Accept a gene module offer.
    GeneAccept { offer_id: String },
    /// Share cross-organism statistics.
    StatsPublication(CrossOrganismStats),
}

/// Read-only telemetry snapshot for exchange.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetrySnapshot {
    pub organism_id: String,
    pub epoch: u64,
    pub population: usize,
    pub total_supply: f64,
    pub treasury_reserve: f64,
    pub mean_fitness: f64,
    pub role_distribution: HashMap<String, usize>,
    pub season: String,
    pub uptime_seconds: u64,
    pub at: DateTime<Utc>,
}

/// A gene module offered for cross-organism exchange.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneExchangeOffer {
    pub offer_id: String,
    pub from_organism: String,
    pub gene_hash: String,
    pub gene_data: Vec<u8>,
    pub fitness_impact: f64,
    pub atp_price: f64,
    pub offered_at: DateTime<Utc>,
}

/// Cross-organism comparative statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossOrganismStats {
    pub organisms: Vec<OrganismSummary>,
    pub compiled_at: DateTime<Utc>,
}

/// Summary of a single organism for cross-organism comparison.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganismSummary {
    pub organism_id: String,
    pub name: String,
    pub epoch: u64,
    pub population: usize,
    pub mean_fitness: f64,
    pub total_supply: f64,
}

/// The federation protocol manager — tracks known organisms and handshakes.
pub struct FederationProtocol {
    /// This organism's identity.
    pub identity: OrganismIdentity,
    /// Known peer organisms.
    pub peers: HashMap<String, OrganismIdentity>,
    /// Completed handshakes.
    pub handshakes: Vec<Handshake>,
    /// Pending gene offers.
    pub gene_offers: HashMap<String, GeneExchangeOffer>,
}

impl FederationProtocol {
    /// Create a new federation protocol instance.
    pub fn new(identity: OrganismIdentity) -> Self {
        Self {
            identity,
            peers: HashMap::new(),
            handshakes: Vec::new(),
            gene_offers: HashMap::new(),
        }
    }

    /// Initiate a handshake with a remote organism.
    pub fn initiate_handshake(&self) -> Handshake {
        Handshake::initiate(self.identity.clone())
    }

    /// Process an incoming handshake request — respond with our identity.
    pub fn process_handshake(&mut self, mut handshake: Handshake) -> Handshake {
        handshake.respond(self.identity.clone());
        // Register the initiator as a known peer
        self.peers.insert(
            handshake.initiator.organism_id.clone(),
            handshake.initiator.clone(),
        );
        self.handshakes.push(handshake.clone());
        handshake
    }

    /// Complete a handshake after receiving a response.
    pub fn complete_handshake(&mut self, handshake: Handshake) -> Result<(), FederationError> {
        if !handshake.verify() {
            return Err(FederationError::HandshakeFailed(
                "Handshake verification failed".into(),
            ));
        }
        if let Some(ref responder) = handshake.responder {
            self.peers.insert(
                responder.organism_id.clone(),
                responder.clone(),
            );
        }
        self.handshakes.push(handshake);
        Ok(())
    }

    /// Create a telemetry snapshot for exchange.
    pub fn create_snapshot(
        &self,
        epoch: u64,
        population: usize,
        total_supply: f64,
        treasury_reserve: f64,
        mean_fitness: f64,
        role_distribution: HashMap<String, usize>,
        season: String,
        uptime_seconds: u64,
    ) -> TelemetrySnapshot {
        TelemetrySnapshot {
            organism_id: self.identity.organism_id.clone(),
            epoch,
            population,
            total_supply,
            treasury_reserve,
            mean_fitness,
            role_distribution,
            season,
            uptime_seconds,
            at: Utc::now(),
        }
    }

    /// Process incoming telemetry — update our view of the peer.
    pub fn process_telemetry(&mut self, snapshot: TelemetrySnapshot) {
        if let Some(peer) = self.peers.get_mut(&snapshot.organism_id) {
            peer.update(snapshot.epoch, snapshot.population, snapshot.total_supply);
        }
    }

    /// Number of known peers.
    pub fn peer_count(&self) -> usize {
        self.peers.len()
    }

    /// List known peer IDs.
    pub fn peer_ids(&self) -> Vec<&str> {
        self.peers.keys().map(|s| s.as_str()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_identity(name: &str) -> OrganismIdentity {
        OrganismIdentity::new(name, "1.2.0", name.as_bytes())
    }

    #[test]
    fn test_handshake_flow() {
        let id_a = make_identity("Organism Alpha");
        let id_b = make_identity("Organism Beta");

        let mut proto_a = FederationProtocol::new(id_a);
        let mut proto_b = FederationProtocol::new(id_b);

        // A initiates handshake
        let handshake = proto_a.initiate_handshake();
        assert!(!handshake.valid);

        // B responds
        let completed = proto_b.process_handshake(handshake);
        assert!(completed.valid);
        assert!(completed.verify());

        // A completes
        proto_a.complete_handshake(completed).unwrap();

        assert_eq!(proto_a.peer_count(), 1);
        assert_eq!(proto_b.peer_count(), 1);
    }

    #[test]
    fn test_telemetry_exchange() {
        let id_a = make_identity("Alpha");
        let id_b = make_identity("Beta");

        let mut proto_a = FederationProtocol::new(id_a.clone());
        let mut proto_b = FederationProtocol::new(id_b.clone());

        // Handshake first
        let h = proto_a.initiate_handshake();
        let h = proto_b.process_handshake(h);
        proto_a.complete_handshake(h).unwrap();

        // A sends telemetry to B
        let snapshot = proto_a.create_snapshot(
            1000, 50, 2500.0, 100.0, 0.65,
            HashMap::new(), "Autumn".into(), 3600,
        );
        proto_b.process_telemetry(snapshot);

        // B should have updated view of A
        let peer = proto_b.peers.get(&id_a.organism_id).unwrap();
        assert_eq!(peer.current_epoch, 1000);
        assert_eq!(peer.population, 50);
    }
}
