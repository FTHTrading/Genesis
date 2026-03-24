// facilitator.rs — In-house ECDSA payment verifier
//
// Replaces the CDP external facilitator entirely.
// No HTTP calls. No third-party dependency on verification.
//
// HOW IT WORKS:
//   1. Client sends PAYMENT-SIGNATURE header = base64(JSON EIP-3009 authorization)
//   2. We decode and parse the authorization in-process
//   3. We reconstruct the EIP-712 TypedData digest from first principles:
//        domainSeparator = keccak256(abi.encode(typehash, name, version, chainId, contract))
//        structHash      = keccak256(abi.encode(typehash, from, to, value, nonce, validAfter, validBefore))
//        digest          = keccak256("\x19\x01" || domainSeparator || structHash)
//   4. We recover the signer address from the 65-byte EIP-712 signature using k256
//   5. Recovered address == from field in the authorization → VALID
//   6. No external `/verify` call. No external `/settle` call.
//
// Settlement is handled by genesis-ledger (batched) → SettlementAnchor.sol on Polygon.
// Polygon is the ONLY external rail.

use base64::Engine;
use k256::ecdsa::{RecoveryId, Signature, VerifyingKey};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};
use thiserror::Error;

use crate::config::X402Config;
use crate::eip3009::Eip3009Authorization;

// ── Error types ───────────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum FacilitatorError {
    #[error("invalid base64 payload: {0}")]
    Base64(String),

    #[error("invalid JSON in payment payload: {0}")]
    Json(String),

    #[error("signature decode failed: {0}")]
    SignatureDecode(String),

    #[error("signer recovery failed: {0}")]
    Recovery(String),

    #[error("authorization rejected: {reason}")]
    Rejected { reason: String },

    #[error("expired: validBefore={valid_before} now={now}")]
    Expired { valid_before: u64, now: u64 },

    #[error("not yet valid: validAfter={valid_after} now={now}")]
    NotYetValid { valid_after: u64, now: u64 },

    #[error("amount mismatch: expected={expected} got={got}")]
    AmountMismatch { expected: u64, got: u64 },

    #[error("wrong recipient: expected={expected} got={got}")]
    WrongRecipient { expected: String, got: String },
}

// ── PaymentRequired (kept for middleware compatibility) ───────────────────

/// What a seller declares when returning a 402.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentRequired {
    pub scheme:       String,               // "exact"
    pub network:      String,               // "eip155:137"
    pub asset:        String,               // USDC contract on Polygon
    pub max_amount_required: String,        // atomic USDC units as string
    pub resource:     String,
    pub description:  String,
    pub mime_type:    Option<String>,
    pub output_schema: Option<serde_json::Value>,
    pub estimated_processing_time_ms: Option<u64>,
    pub expires:      Option<u64>,
    pub pay_to:       String,               // 0x address that receives USDC
    pub requires_beneficiary: bool,
    pub x402_version: u32,
}

impl PaymentRequired {
    pub fn for_endpoint(
        config:      &X402Config,
        amount_usdc: u64,
        resource:    &str,
        description: &str,
    ) -> Self {
        Self {
            scheme:      "exact".to_string(),
            network:     config.network.clone(),
            asset:       config.usdc_contract.clone(),
            max_amount_required: amount_usdc.to_string(),
            resource:    resource.to_string(),
            description: description.to_string(),
            mime_type:   Some("application/json".to_string()),
            output_schema: None,
            estimated_processing_time_ms: Some(500),
            expires:     Some(unix_now() + 300),
            pay_to:      config.pay_to.clone(),
            requires_beneficiary: false,
            x402_version: crate::X402_VERSION,
        }
    }

    pub fn to_header_value(&self) -> String {
        let json = serde_json::to_string(self).unwrap_or_default();
        base64::engine::general_purpose::STANDARD.encode(json.as_bytes())
    }
}

// ── Verification result ───────────────────────────────────────────────────

/// Successful verification result — contains the recovered payer address.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    pub payer:         String,      // recovered EVM address
    pub amount_usdc:   u64,
    pub valid_before:  u64,
    pub nonce:         String,      // 0x hex bytes32
    pub asset:         String,
    pub authorization: Eip3009Authorization,
}

// ── In-house verifier ─────────────────────────────────────────────────────

/// Verifies an EIP-3009 USDC authorization locally using k256 ECDSA recovery.
/// No HTTP calls. No external service.
pub struct InHouseVerifier {
    pub usdc_contract: String,
    pub pay_to:        String,
    pub chain_id:      u64,
}

impl InHouseVerifier {
    pub fn polygon_mainnet(config: &X402Config) -> Self {
        Self {
            usdc_contract: config.usdc_contract.clone(),
            pay_to:        config.pay_to.clone(),
            chain_id:      137,
        }
    }

    /// Verify a base64-encoded EIP-3009 authorization.
    pub fn verify(
        &self,
        payment_payload: &str,
        expected_amount: u64,
    ) -> Result<VerificationResult, FacilitatorError> {
        // 1. Decode base64 → JSON
        let json_bytes = base64::engine::general_purpose::STANDARD
            .decode(payment_payload)
            .map_err(|e| FacilitatorError::Base64(e.to_string()))?;

        // 2. Parse EIP-3009 authorization
        let auth: Eip3009Authorization = serde_json::from_slice(&json_bytes)
            .map_err(|e| FacilitatorError::Json(e.to_string()))?;

        // Parse string fields to numeric
        let value: u64 = auth.value.parse()
            .map_err(|_| FacilitatorError::Json(format!("invalid value: {}", auth.value)))?;
        let valid_after: u64 = auth.valid_after.parse()
            .map_err(|_| FacilitatorError::Json(format!("invalid validAfter: {}", auth.valid_after)))?;
        let valid_before: u64 = auth.valid_before.parse()
            .map_err(|_| FacilitatorError::Json(format!("invalid validBefore: {}", auth.valid_before)))?;

        // Parse nonce from 0x hex string to [u8; 32]
        let nonce_bytes = {
            let clean = auth.nonce.trim_start_matches("0x");
            let b = hex::decode(clean)
                .map_err(|e| FacilitatorError::Json(format!("invalid nonce: {}", e)))?;
            if b.len() != 32 {
                return Err(FacilitatorError::Json(format!("nonce must be 32 bytes, got {}", b.len())));
            }
            let mut arr = [0u8; 32];
            arr.copy_from_slice(&b);
            arr
        };

        // 3. Timestamp window
        let now = unix_now();
        if now < valid_after {
            return Err(FacilitatorError::NotYetValid { valid_after, now });
        }
        if now >= valid_before {
            return Err(FacilitatorError::Expired { valid_before, now });
        }

        // 4. Amount
        if value != expected_amount {
            return Err(FacilitatorError::AmountMismatch { expected: expected_amount, got: value });
        }

        // 5. Recipient
        if auth.to.to_lowercase() != self.pay_to.to_lowercase() {
            return Err(FacilitatorError::WrongRecipient {
                expected: self.pay_to.clone(), got: auth.to.clone(),
            });
        }

        // 6. Reconstruct EIP-712 digest (adapts to string fields)
        let digest = self.compute_eip712_digest_raw(
            &auth.from, &auth.to, value, valid_after, valid_before, &nonce_bytes,
        )?;

        // 7. Recover signer from v, r, s
        let payer = self.recover_signer_vrs(&digest, auth.v, &auth.r, &auth.s)?;

        // 8. Confirm recovered == stated from
        if payer.to_lowercase() != auth.from.to_lowercase() {
            return Err(FacilitatorError::Rejected {
                reason: format!("signature mismatch: recovered={} stated={}", payer, auth.from),
            });
        }

        tracing::info!(
            payer     = %payer,
            amount    = value,
            "Payment authorization verified in-house"
        );

        Ok(VerificationResult {
            payer,
            amount_usdc:   value,
            valid_before,
            nonce:         auth.nonce.clone(),
            asset:         self.usdc_contract.clone(),
            authorization: auth,
        })
    }

    fn compute_eip712_digest_raw(
        &self,
        from:         &str,
        to:           &str,
        value:        u64,
        valid_after:  u64,
        valid_before: u64,
        nonce:        &[u8; 32],
    ) -> Result<[u8; 32], FacilitatorError> {
        let domain_typehash = keccak256(
            b"EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"
        );
        let transfer_typehash = keccak256(
            b"TransferWithAuthorization(address from,address to,uint256 value,uint256 validAfter,uint256 validBefore,bytes32 nonce)"
        );

        // Domain separator
        let domain_separator = {
            let name_hash    = keccak256(b"USD Coin");
            let version_hash = keccak256(b"2");
            let mut enc      = [0u8; 160];
            enc[0..32].copy_from_slice(&domain_typehash);
            enc[32..64].copy_from_slice(&name_hash);
            enc[64..96].copy_from_slice(&version_hash);
            enc[88..96].copy_from_slice(&self.chain_id.to_be_bytes());
            let addr = decode_address(&self.usdc_contract)?;
            enc[108..128].copy_from_slice(&addr);
            keccak256(&enc)
        };

        // Struct hash
        let struct_hash = {
            let from_b = decode_address(from)?;
            let to_b   = decode_address(to)?;
            let mut enc = [0u8; 224];
            enc[0..32].copy_from_slice(&transfer_typehash);
            enc[44..64].copy_from_slice(&from_b);
            enc[76..96].copy_from_slice(&to_b);
            write_u64_be(&mut enc[96..128], value);
            write_u64_be(&mut enc[128..160], valid_after);
            write_u64_be(&mut enc[160..192], valid_before);
            enc[192..224].copy_from_slice(nonce);
            keccak256(&enc)
        };

        // "\x19\x01" || domainSeparator || structHash
        let mut payload = [0u8; 66];
        payload[0] = 0x19;
        payload[1] = 0x01;
        payload[2..34].copy_from_slice(&domain_separator);
        payload[34..66].copy_from_slice(&struct_hash);
        Ok(keccak256(&payload))
    }

    /// Recover EVM address from EIP-2 v/r/s components.
    fn recover_signer_vrs(
        &self,
        digest: &[u8; 32],
        v:      u8,
        r_hex:  &str,
        s_hex:  &str,
    ) -> Result<String, FacilitatorError> {
        let r = hex::decode(r_hex.trim_start_matches("0x"))
            .map_err(|e| FacilitatorError::SignatureDecode(format!("r: {}", e)))?;
        let s = hex::decode(s_hex.trim_start_matches("0x"))
            .map_err(|e| FacilitatorError::SignatureDecode(format!("s: {}", e)))?;
        if r.len() != 32 || s.len() != 32 {
            return Err(FacilitatorError::SignatureDecode(
                format!("r/s must be 32 bytes each, got r={} s={}", r.len(), s.len())
            ));
        }

        let recovery_id = match v {
            0 | 27 => RecoveryId::new(false, false),
            1 | 28 => RecoveryId::new(true, false),
            _      => return Err(FacilitatorError::SignatureDecode(format!("invalid v: {}", v))),
        };

        let mut rs = [0u8; 64];
        rs[..32].copy_from_slice(&r);
        rs[32..].copy_from_slice(&s);
        let sig = Signature::try_from(rs.as_slice())
            .map_err(|e| FacilitatorError::SignatureDecode(e.to_string()))?;

        let vk = VerifyingKey::recover_from_prehash(digest, &sig, recovery_id)
            .map_err(|e| FacilitatorError::Recovery(e.to_string()))?;

        let uncompressed = vk.to_encoded_point(false);
        let full_hash = keccak256(&uncompressed.as_bytes()[1..]);
        Ok(format!("0x{}", hex::encode(&full_hash[12..])))
    }
}

// ── Utilities ─────────────────────────────────────────────────────────────

fn keccak256(data: &[u8]) -> [u8; 32] {
    let mut h = Keccak256::new();
    h.update(data);
    h.finalize().into()
}

fn decode_address(addr: &str) -> Result<[u8; 20], FacilitatorError> {
    let clean = addr.trim_start_matches("0x");
    if clean.len() != 40 {
        return Err(FacilitatorError::Rejected { reason: format!("invalid address: {}", addr) });
    }
    let b = hex::decode(clean)
        .map_err(|e| FacilitatorError::Rejected { reason: e.to_string() })?;
    let mut out = [0u8; 20];
    out.copy_from_slice(&b);
    Ok(out)
}

fn write_u64_be(buf: &mut [u8], val: u64) {
    let bytes = val.to_be_bytes();
    let start = buf.len() - 8;
    buf[start..].copy_from_slice(&bytes);
}

fn unix_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

// ── Tests ─────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keccak256_empty() {
        // keccak256("") = c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470
        let h = keccak256(b"");
        assert_eq!(h[0], 0xc5);
    }

    #[test]
    fn address_decode_valid() {
        let b = decode_address("0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359").unwrap();
        assert_eq!(b.len(), 20);
    }

    #[test]
    fn address_decode_rejects_short() {
        assert!(decode_address("0xshort").is_err());
    }
}
