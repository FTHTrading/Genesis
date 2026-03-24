// facilitator.rs — CDP x402 Facilitator client
//
// Implements the two-call x402 flow:
//   POST /verify  — confirm the payment proof is valid
//   POST /settle  — broadcast the USDC transfer on-chain
//
// CDP Mainnet endpoint:  https://api.cdp.coinbase.com/platform/v2/x402
// x402.org testnet:      https://x402.org/facilitator
//
// Both verify and settle use the same payload format.
// CDP requires API key auth; x402.org testnet is open.

use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::config::X402Config;

#[derive(Debug, Error)]
pub enum FacilitatorError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Facilitator rejected: {reason}")]
    Rejected { reason: String },
    #[error("Settlement failed: {reason}")]
    SettlementFailed { reason: String },
    #[error("Timeout")]
    Timeout,
}

// ── Verify request/response ───────────────────────────────────────────────

/// The payment payload that was in the client's PAYMENT-SIGNATURE header.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyRequest {
    /// The raw PAYMENT-SIGNATURE header value (base64-encoded JSON).
    pub payment_payload: String,
    /// The payment requirements we declared in our 402 response.
    pub payment_required: PaymentRequired,
}

/// What a seller declares when returning a 402.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentRequired {
    pub scheme:       String,               // "exact"
    pub network:      String,               // "eip155:137" for Polygon
    pub asset:        String,               // USDC contract address
    pub max_amount_required: String,        // USDC atomic units as string
    pub resource:     String,               // URI of the resource being purchased
    pub description:  String,
    pub mime_type:    Option<String>,
    pub output_schema: Option<serde_json::Value>,
    pub estimated_processing_time_ms: Option<u64>,
    pub expires:      Option<u64>,          // unix timestamp
    pub pay_to:       String,               // 0x address that receives USDC
    pub requires_beneficiary: bool,
    pub x402_version: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyResponse {
    pub is_valid:  bool,
    pub error:     Option<String>,
    pub payer:     Option<String>,    // resolved EVM address after verification
}

// ── Settle request/response ───────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettleRequest {
    pub payment_payload:  String,
    pub payment_required: PaymentRequired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettleResponse {
    pub success:    bool,
    pub error:      Option<String>,
    pub tx_hash:    Option<String>,     // on-chain transaction hash
    pub network:    Option<String>,
    pub payer:      Option<String>,
    pub amount:     Option<String>,     // amount settled (atomic units)
}

// ── Client ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct FacilitatorClient {
    client:       Client,
    base_url:     String,
    cdp_api_key:  Option<String>,
    cdp_api_secret: Option<String>,
}

impl FacilitatorClient {
    pub fn new(config: &X402Config) -> Self {
        let client = Client::builder()
            .timeout(config.facilitator_timeout)
            .user_agent("genesis-protocol/x402 (Rust)")
            .build()
            .expect("HTTP client build failed");

        Self {
            client,
            base_url:      config.facilitator_url.clone(),
            cdp_api_key:   config.cdp_api_key.clone(),
            cdp_api_secret: config.cdp_api_secret.clone(),
        }
    }

    /// Verify a payment proof. Returns the resolved payer address on success.
    pub async fn verify(
        &self,
        payment_payload:  &str,
        payment_required: &PaymentRequired,
    ) -> Result<String, FacilitatorError> {
        let url = format!("{}/verify", self.base_url);
        let body = VerifyRequest {
            payment_payload:  payment_payload.to_string(),
            payment_required: payment_required.clone(),
        };

        let mut req = self.client.post(&url).json(&body);
        req = self.add_auth(req);

        let resp = req.send().await.map_err(|e| {
            if e.is_timeout() { FacilitatorError::Timeout }
            else { FacilitatorError::Http(e) }
        })?;

        let v: VerifyResponse = resp.json().await?;
        if v.is_valid {
            Ok(v.payer.unwrap_or_default())
        } else {
            Err(FacilitatorError::Rejected {
                reason: v.error.unwrap_or_else(|| "unknown".to_string()),
            })
        }
    }

    /// Settle a payment proof on-chain. Returns transaction hash.
    pub async fn settle(
        &self,
        payment_payload:  &str,
        payment_required: &PaymentRequired,
    ) -> Result<SettleResponse, FacilitatorError> {
        let url = format!("{}/settle", self.base_url);
        let body = SettleRequest {
            payment_payload:  payment_payload.to_string(),
            payment_required: payment_required.clone(),
        };

        let mut req = self.client.post(&url).json(&body);
        req = self.add_auth(req);

        let resp = req.send().await.map_err(|e| {
            if e.is_timeout() { FacilitatorError::Timeout }
            else { FacilitatorError::Http(e) }
        })?;

        let s: SettleResponse = resp.json().await?;
        if s.success {
            Ok(s)
        } else {
            Err(FacilitatorError::SettlementFailed {
                reason: s.error.unwrap_or_else(|| "unknown".to_string()),
            })
        }
    }

    fn add_auth(&self, req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        match (&self.cdp_api_key, &self.cdp_api_secret) {
            (Some(key), Some(_secret)) => {
                // CDP uses Bearer token auth.
                // For production, use proper JWT/HMAC signing per CDP docs.
                req.header("Authorization", format!("Bearer {}", key))
            }
            _ => req,
        }
    }
}

// ── Payment required builder ──────────────────────────────────────────────

impl PaymentRequired {
    /// Build a payment requirement for a given endpoint using world config.
    pub fn for_endpoint(
        config:      &X402Config,
        amount_usdc: u64,      // atomic units (6 decimals)
        resource:    &str,     // e.g. "/api/ai-call"
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
            expires:     Some(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs() + 300 // 5 minute window
            ),
            pay_to:      config.pay_to.clone(),
            requires_beneficiary: false,
            x402_version: crate::X402_VERSION,
        }
    }

    /// Serialize to the PAYMENT-REQUIRED header value (base64 JSON).
    pub fn to_header_value(&self) -> String {
        let json = serde_json::to_string(self).unwrap_or_default();
        use base64::Engine;
        base64::engine::general_purpose::STANDARD.encode(json.as_bytes())
    }
}
