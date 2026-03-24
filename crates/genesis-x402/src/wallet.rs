// wallet.rs — Genesis EVM hot wallet
//
// Stores a secp256k1 private key in an AES-256-GCM encrypted vault file,
// derived using Argon2id KDF from a passphrase.
//
// On first use, generates a fresh key and writes the vault.
// Address is printed to stdout — fund it with USDC before enabling x402.
//
// Vault file format (JSON):
//   { "version": 1, "salt_hex": "...", "nonce_hex": "...", "ciphertext_hex": "..." }
// Vault plaintext (JSON):
//   { "private_key_hex": "...", "address": "0x...", "created_at": "..." }

use std::path::Path;

use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Nonce};
use argon2::{Argon2, Params};
use k256::ecdsa::{SigningKey, VerifyingKey};
use k256::elliptic_curve::sec1::ToEncodedPoint;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};
use zeroize::Zeroize;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum WalletError {
    #[error("vault I/O: {0}")]
    Io(#[from] std::io::Error),
    #[error("vault JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("vault decrypt failed")]
    DecryptFailed,
    #[error("invalid private key")]
    InvalidKey,
    #[error("no passphrase provided — set GENESIS_VAULT_PASSPHRASE")]
    NoPassphrase,
    #[error("argon2: {0}")]
    Kdf(String),
}

/// On-disk vault envelope.
#[derive(Serialize, Deserialize)]
struct VaultEnvelope {
    version:        u32,
    salt_hex:       String,
    nonce_hex:      String,
    ciphertext_hex: String,
}

/// Plaintext inside the vault.
#[derive(Serialize, Deserialize)]
struct VaultPlaintext {
    private_key_hex: String,
    address:         String,
    created_at:      String,
}

/// The Genesis EVM hot wallet.
#[derive(Clone)]
pub struct GenesisWallet {
    /// secp256k1 signing key (private key).
    signing_key: SigningKey,
    /// Checksummed EVM address string.
    pub address: String,
}

impl GenesisWallet {
    /// Load from vault file, or generate a new key if vault doesn't exist.
    ///
    /// Prints the EVM address on first run — fund it with USDC before use.
    pub fn load_or_generate(vault_path: &str, passphrase: Option<&str>) -> Result<Self, WalletError> {
        if Path::new(vault_path).exists() {
            Self::load(vault_path, passphrase)
        } else {
            let pass = passphrase.ok_or(WalletError::NoPassphrase)?;
            let wallet = Self::generate();
            wallet.save(vault_path, pass)?;
            tracing::info!(
                address = %wallet.address,
                vault = vault_path,
                "Generated new Genesis EVM wallet — fund with Polygon USDC before enabling x402"
            );
            println!("  Genesis wallet: {}", wallet.address);
            println!("  Fund with Polygon USDC at: {}", wallet.address);
            Ok(wallet)
        }
    }

    /// Generate a new random EVM wallet.
    pub fn generate() -> Self {
        let mut rng = rand::thread_rng();
        let signing_key = SigningKey::random(&mut rng);
        let address = derive_address(signing_key.verifying_key());
        Self { signing_key, address }
    }

    /// Load wallet from encrypted vault file.
    pub fn load(vault_path: &str, passphrase: Option<&str>) -> Result<Self, WalletError> {
        let pass = passphrase.ok_or(WalletError::NoPassphrase)?;
        let raw = std::fs::read_to_string(vault_path)?;
        let envelope: VaultEnvelope = serde_json::from_str(&raw)?;

        let salt        = hex::decode(&envelope.salt_hex).map_err(|_| WalletError::DecryptFailed)?;
        let nonce_bytes = hex::decode(&envelope.nonce_hex).map_err(|_| WalletError::DecryptFailed)?;
        let ciphertext  = hex::decode(&envelope.ciphertext_hex).map_err(|_| WalletError::DecryptFailed)?;

        let mut aes_key = derive_aes_key(pass.as_bytes(), &salt)?;
        let cipher = Aes256Gcm::new_from_slice(&aes_key).map_err(|_| WalletError::DecryptFailed)?;
        aes_key.zeroize();

        let nonce = Nonce::from_slice(&nonce_bytes);
        let plaintext = cipher.decrypt(nonce, ciphertext.as_ref())
            .map_err(|_| WalletError::DecryptFailed)?;

        let vp: VaultPlaintext = serde_json::from_slice(&plaintext)?;
        let key_bytes = hex::decode(&vp.private_key_hex).map_err(|_| WalletError::InvalidKey)?;
        let signing_key = SigningKey::from_bytes(key_bytes.as_slice().into())
            .map_err(|_| WalletError::InvalidKey)?;

        let address = vp.address.clone();
        Ok(Self { signing_key, address })
    }

    /// Encrypt and save wallet to vault file.
    pub fn save(&self, vault_path: &str, passphrase: &str) -> Result<(), WalletError> {
        let mut rng = rand::thread_rng();

        // Random salt (16 bytes) and nonce (12 bytes)
        let mut salt  = [0u8; 16];
        let mut nonce = [0u8; 12];
        rng.fill_bytes(&mut salt);
        rng.fill_bytes(&mut nonce);

        let mut aes_key = derive_aes_key(passphrase.as_bytes(), &salt)?;
        let cipher = Aes256Gcm::new_from_slice(&aes_key).map_err(|_| WalletError::DecryptFailed)?;
        aes_key.zeroize();

        let vp = VaultPlaintext {
            private_key_hex: hex::encode(self.signing_key.to_bytes()),
            address:         self.address.clone(),
            created_at:      chrono::Utc::now().to_rfc3339(),
        };
        let plaintext = serde_json::to_vec(&vp)?;

        let nonce_ref = Nonce::from_slice(&nonce);
        let ciphertext = cipher.encrypt(nonce_ref, plaintext.as_ref())
            .map_err(|_| WalletError::DecryptFailed)?;

        let envelope = VaultEnvelope {
            version:        1,
            salt_hex:       hex::encode(salt),
            nonce_hex:      hex::encode(nonce),
            ciphertext_hex: hex::encode(ciphertext),
        };

        if let Some(parent) = Path::new(vault_path).parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(vault_path, serde_json::to_vec_pretty(&envelope)?)?;
        Ok(())
    }

    /// Return the raw 32-byte private key (for EIP-3009 signing).
    pub(crate) fn signing_key(&self) -> &SigningKey {
        &self.signing_key
    }
}

/// Derive an AES-256 key (32 bytes) from passphrase + salt using Argon2id.
fn derive_aes_key(passphrase: &[u8], salt: &[u8]) -> Result<[u8; 32], WalletError> {
    let params = Params::new(65536, 3, 1, Some(32))
        .map_err(|e| WalletError::Kdf(e.to_string()))?;
    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);

    let mut key = [0u8; 32];
    argon2.hash_password_into(passphrase, salt, &mut key)
        .map_err(|e| WalletError::Kdf(e.to_string()))?;
    Ok(key)
}

/// Derive an Ethereum address from a secp256k1 public key.
/// Address = "0x" + lowercase hex of keccak256(uncompressed_pubkey_64_bytes)[12..]
pub fn derive_address(verifying_key: &VerifyingKey) -> String {
    let point = verifying_key.to_encoded_point(false); // uncompressed
    let pubkey_bytes = &point.as_bytes()[1..]; // strip 04 prefix → 64 bytes

    let mut hasher = Keccak256::new();
    hasher.update(pubkey_bytes);
    let hash = hasher.finalize();

    let addr_bytes = &hash[12..]; // last 20 bytes
    format!("0x{}", hex::encode(addr_bytes))
}

/// Compute keccak256 of arbitrary bytes.
pub fn keccak256(data: &[u8]) -> [u8; 32] {
    let mut h = Keccak256::new();
    h.update(data);
    h.finalize().into()
}

/// ABI-encode a static type as 32 bytes (left-pad for address/uint256, as-is for bytes32).
pub fn abi_encode_address(addr_hex: &str) -> [u8; 32] {
    // Parse the 20-byte address (strip 0x)
    let clean = addr_hex.trim_start_matches("0x");
    let bytes = hex::decode(clean).unwrap_or_default();
    let mut padded = [0u8; 32];
    if bytes.len() <= 20 {
        padded[32 - bytes.len()..].copy_from_slice(&bytes);
    }
    padded
}

pub fn abi_encode_u256(value: u128) -> [u8; 32] {
    let mut padded = [0u8; 32];
    let bytes = value.to_be_bytes(); // 16 bytes
    padded[16..].copy_from_slice(&bytes);
    padded
}

pub fn abi_encode_u64(value: u64) -> [u8; 32] {
    let mut padded = [0u8; 32];
    let bytes = value.to_be_bytes(); // 8 bytes
    padded[24..].copy_from_slice(&bytes);
    padded
}
