// ATP Escrow — Inter-organism ATP exchange via escrow.
//
// When two organisms agree to exchange resources (e.g., gene modules
// for ATP), an escrow holds the ATP until the exchange is confirmed
// by both sides. This prevents fraud without governance injection.

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};

use crate::errors::FederationError;

/// State of an escrow transaction.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EscrowState {
    /// Created, awaiting deposit.
    Pending,
    /// Deposit received, awaiting fulfillment.
    Funded,
    /// Both sides fulfilled — releasing.
    Releasing,
    /// Completed successfully.
    Completed,
    /// Timed out or cancelled.
    Cancelled,
}

/// An inter-organism ATP escrow.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtpEscrow {
    /// Unique escrow ID.
    pub escrow_id: String,
    /// Organism paying ATP.
    pub payer_organism: String,
    /// Organism receiving ATP.
    pub payee_organism: String,
    /// ATP amount in escrow.
    pub amount: f64,
    /// What the payee is providing in return.
    pub consideration: String,
    /// Current state.
    pub state: EscrowState,
    /// Created at.
    pub created_at: DateTime<Utc>,
    /// Expires at (auto-cancel if not fulfilled).
    pub expires_at: DateTime<Utc>,
    /// Payer confirmed fulfillment.
    pub payer_confirmed: bool,
    /// Payee confirmed fulfillment.
    pub payee_confirmed: bool,
}

impl AtpEscrow {
    /// Create a new escrow between two organisms.
    pub fn new(
        payer: impl Into<String>,
        payee: impl Into<String>,
        amount: f64,
        consideration: impl Into<String>,
        ttl_seconds: i64,
    ) -> Self {
        let payer = payer.into();
        let payee = payee.into();

        let mut hasher = Sha256::new();
        hasher.update(payer.as_bytes());
        hasher.update(payee.as_bytes());
        hasher.update(amount.to_le_bytes());
        hasher.update(Utc::now().to_rfc3339().as_bytes());
        let escrow_id = hex::encode(&hasher.finalize()[..16]);

        Self {
            escrow_id,
            payer_organism: payer,
            payee_organism: payee,
            amount,
            consideration: consideration.into(),
            state: EscrowState::Pending,
            created_at: Utc::now(),
            expires_at: Utc::now() + chrono::Duration::seconds(ttl_seconds),
            payer_confirmed: false,
            payee_confirmed: false,
        }
    }

    /// Fund the escrow (payer deposits ATP).
    pub fn fund(&mut self) -> Result<(), FederationError> {
        if self.state != EscrowState::Pending {
            return Err(FederationError::HandshakeFailed(
                "Escrow not in pending state".into(),
            ));
        }
        self.state = EscrowState::Funded;
        Ok(())
    }

    /// Confirm fulfillment from payer side.
    pub fn confirm_payer(&mut self) {
        self.payer_confirmed = true;
        self.check_release();
    }

    /// Confirm fulfillment from payee side.
    pub fn confirm_payee(&mut self) {
        self.payee_confirmed = true;
        self.check_release();
    }

    /// Check if both sides confirmed — if so, transition to Releasing.
    fn check_release(&mut self) {
        if self.payer_confirmed && self.payee_confirmed && self.state == EscrowState::Funded {
            self.state = EscrowState::Releasing;
        }
    }

    /// Complete the escrow — release ATP to payee.
    pub fn complete(&mut self) -> Result<f64, FederationError> {
        if self.state != EscrowState::Releasing {
            return Err(FederationError::HandshakeFailed(
                "Escrow not ready for release".into(),
            ));
        }
        self.state = EscrowState::Completed;
        Ok(self.amount)
    }

    /// Cancel the escrow — return ATP to payer.
    pub fn cancel(&mut self) {
        self.state = EscrowState::Cancelled;
    }

    /// Check if the escrow has expired.
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escrow_lifecycle() {
        let mut escrow = AtpEscrow::new(
            "organism-a",
            "organism-b",
            50.0,
            "Gene module XYZ",
            3600,
        );
        assert_eq!(escrow.state, EscrowState::Pending);

        escrow.fund().unwrap();
        assert_eq!(escrow.state, EscrowState::Funded);

        escrow.confirm_payer();
        assert_eq!(escrow.state, EscrowState::Funded); // still funded

        escrow.confirm_payee();
        assert_eq!(escrow.state, EscrowState::Releasing);

        let amount = escrow.complete().unwrap();
        assert_eq!(amount, 50.0);
        assert_eq!(escrow.state, EscrowState::Completed);
    }

    #[test]
    fn test_escrow_cancel() {
        let mut escrow = AtpEscrow::new("a", "b", 10.0, "test", 60);
        escrow.fund().unwrap();
        escrow.cancel();
        assert_eq!(escrow.state, EscrowState::Cancelled);
    }
}
