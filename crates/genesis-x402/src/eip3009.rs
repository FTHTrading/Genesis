// eip3009.rs — EIP-3009 TransferWithAuthorization signing
//
// This module builds the signed payload that the CDP x402 facilitator needs to
// call USDC.transferWithAuthorization() on-chain.
//
// Signer is the BUYER — this module is used on the client side when Genesis
// acts as a buyer (outbound milestone payments).  When Genesis is the seller
// the client signs, and we verify via the facilitator.
//
// EIP-712 domain for USDC on Polygon mainnet:
//   name:              "USD Coin"
//   version:           "2"
//   chainId:           137
//   verifyingContract: 0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359

use base64::Engine as _;
use k256::ecdsa::{RecoveryId, Signature, SigningKey};
use rand::RngCore;
use serde::{Deserialize, Serialize};

use crate::wallet::{abi_encode_address, abi_encode_u64, abi_encode_u256, keccak256};

// ── EIP-712 constants ─────────────────────────────────────────────────────

/// keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)")
const EIP712_DOMAIN_TYPEHASH: [u8; 32] = {
    // Precomputed constant — avoids runtime computation.
    // Actual value:
    //   keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)")
    // = 0x8b73c3c69bb8fe3d512eebf88ef3a1f4e7f72e0b44d0e5def25f359b22a1d2ea
    hex_literal("8b73c3c69bb8fe3d512eebf88ef3a1f4e7f72e0b44d0e5def25f359b22a1d2ea")
};

/// keccak256("TransferWithAuthorization(address from,address to,uint256 value,uint256 validAfter,uint256 validBefore,bytes32 nonce)")
const TRANSFER_WITH_AUTHORIZATION_TYPEHASH: [u8; 32] =
    hex_literal("7c7c6cdb67a18743f49ec6fa9b35f50d52ed05cbed4cc592e13b44501c1a2267");

/// keccak256("USD Coin")
const USDC_NAME_HASH: [u8; 32] =
    hex_literal("2a1f2fb8f0c8dcc8cd2fd79d4e70d7fa4f32d97a22e66ef81fe7a5adb7e3e9b0");

/// keccak256("2")
const USDC_VERSION_HASH: [u8; 32] =
    hex_literal("ad7c5bef027816a800da1736444fb58a807ef4c9603b7848673f7e3a68eb14a5");

/// USDC contract on Polygon mainnet.
const USDC_POLYGON_ADDR: &str = "0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359";

/// Polygon chain ID.
const POLYGON_CHAIN_ID: u64 = 137;

// ── EIP-3009 payload types ─────────────────────────────────────────────────

/// The authorization data signed by the buyer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Eip3009Authorization {
    pub from:         String,   // 0x address
    pub to:           String,   // 0x address
    pub value:        String,   // USDC atomic units as decimal string
    pub valid_after:  String,   // unix timestamp (usually "0")
    pub valid_before: String,   // unix timestamp deadline
    pub nonce:        String,   // 0x hex bytes32
    pub v:            u8,
    pub r:            String,   // 0x hex bytes32
    pub s:            String,   // 0x hex bytes32
}

/// Full x402 EVM payment payload (PAYMENT-SIGNATURE header content).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct X402PaymentPayload {
    pub x402_version: u32,
    pub scheme:       String,
    pub network:      String,
    pub payload:      Eip3009Payload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Eip3009Payload {
    /// Transfer type indicator.
    #[serde(rename = "type")]
    pub payload_type: String, // "eip3009"
    /// The signed authorization fields.
    #[serde(flatten)]
    pub authorization: Eip3009Authorization,
}

impl X402PaymentPayload {
    /// Base64-encode the payload for the PAYMENT-SIGNATURE header.
    pub fn to_header_value(&self) -> Result<String, serde_json::Error> {
        let json = serde_json::to_string(self)?;
        Ok(base64::engine::general_purpose::STANDARD.encode(json.as_bytes()))
    }
}

// ── Signing ───────────────────────────────────────────────────────────────

/// Build and sign an EIP-3009 TransferWithAuthorization for USDC on Polygon.
///
/// `signing_key`  — Buyer's EVM private key.
/// `from`         — Buyer's 0x address.
/// `to`           — Seller's 0x address (Genesis pay_to).
/// `amount_usdc`  — USDC atomic units (6 decimals; 1_000_000 = $1.00).
/// `deadline_secs`— Unix timestamp after which the auth is invalid.
pub fn sign_transfer_authorization(
    signing_key:   &SigningKey,
    from:          &str,
    to:            &str,
    amount_usdc:   u64,
    deadline_secs: u64,
) -> Result<Eip3009Authorization, String> {
    // Random 32-byte nonce
    let mut nonce = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut nonce);

    let valid_after:  u64 = 0;
    let valid_before: u64 = deadline_secs;

    // Build struct hash
    let struct_hash = build_struct_hash(from, to, amount_usdc, valid_after, valid_before, &nonce);

    // Domain separator for USDC on Polygon
    let domain_sep = usdc_polygon_domain_separator();

    // EIP-712 digest: keccak256("\x19\x01" || domain_separator || struct_hash)
    let mut preimage = Vec::with_capacity(2 + 32 + 32);
    preimage.extend_from_slice(b"\x19\x01");
    preimage.extend_from_slice(&domain_sep);
    preimage.extend_from_slice(&struct_hash);
    let digest = keccak256(&preimage);

    // Sign (recoverable ECDSA)
    let (sig, recovery_id): (Signature, RecoveryId) = signing_key
        .sign_prehash_recoverable(&digest)
        .map_err(|e| e.to_string())?;

    let sig_bytes = sig.to_bytes();
    let r = &sig_bytes[..32];
    let s = &sig_bytes[32..];
    let v = 27u8 + recovery_id.to_byte();

    Ok(Eip3009Authorization {
        from:         from.to_string(),
        to:           to.to_string(),
        value:        amount_usdc.to_string(),
        valid_after:  valid_after.to_string(),
        valid_before: valid_before.to_string(),
        nonce:        format!("0x{}", hex::encode(nonce)),
        v,
        r:            format!("0x{}", hex::encode(r)),
        s:            format!("0x{}", hex::encode(s)),
    })
}

// ── EIP-712 construction ──────────────────────────────────────────────────

/// Compute the EIP-712 domain separator for USDC on Polygon mainnet.
pub fn usdc_polygon_domain_separator() -> [u8; 32] {
    let contract_addr = abi_encode_address(USDC_POLYGON_ADDR);

    let mut encoded = Vec::with_capacity(5 * 32);
    encoded.extend_from_slice(&EIP712_DOMAIN_TYPEHASH);
    encoded.extend_from_slice(&USDC_NAME_HASH);
    encoded.extend_from_slice(&USDC_VERSION_HASH);
    encoded.extend_from_slice(&abi_encode_u64(POLYGON_CHAIN_ID));
    encoded.extend_from_slice(&contract_addr);

    keccak256(&encoded)
}

/// Compute the struct hash for TransferWithAuthorization.
fn build_struct_hash(
    from:         &str,
    to:           &str,
    value:        u64,
    valid_after:  u64,
    valid_before: u64,
    nonce:        &[u8; 32],
) -> [u8; 32] {
    let mut encoded = Vec::with_capacity(7 * 32);
    encoded.extend_from_slice(&TRANSFER_WITH_AUTHORIZATION_TYPEHASH);
    encoded.extend_from_slice(&abi_encode_address(from));
    encoded.extend_from_slice(&abi_encode_address(to));
    encoded.extend_from_slice(&abi_encode_u256(value as u128));
    encoded.extend_from_slice(&abi_encode_u64(valid_after));
    encoded.extend_from_slice(&abi_encode_u64(valid_before));
    encoded.extend_from_slice(nonce);
    keccak256(&encoded)
}

// ── Const hex helper (compile-time) ──────────────────────────────────────

const fn hex_literal(s: &str) -> [u8; 32] {
    let bytes = s.as_bytes();
    assert!(bytes.len() == 64, "hex string must be 64 chars for [u8; 32]");
    let mut out = [0u8; 32];
    let mut i = 0;
    while i < 32 {
        let hi = hex_nibble(bytes[i * 2]);
        let lo = hex_nibble(bytes[i * 2 + 1]);
        out[i] = (hi << 4) | lo;
        i += 1;
    }
    out
}

const fn hex_nibble(b: u8) -> u8 {
    match b {
        b'0'..=b'9' => b - b'0',
        b'a'..=b'f' => b - b'a' + 10,
        b'A'..=b'F' => b - b'A' + 10,
        _ => panic!("invalid hex char"),
    }
}

