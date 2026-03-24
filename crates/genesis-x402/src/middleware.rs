// middleware.rs — Axum x402 seller middleware
//
// Usage:
//   Router::new()
//       .route("/api/ai-call", get(ai_call_handler))
//       .layer(X402Layer::new(config, price_usdc))
//
// Flow for each request:
//   1. Check PAYMENT-SIGNATURE header.
//   2. If absent  → 402 + PAYMENT-REQUIRED header.
//   3. If present → verify with in-house ECDSA verifier (no HTTP call).
//   4. If valid   → record to lineage, pass through.
//   5. On-chain settlement is batched by genesis-ledger → SettlementAnchor.sol.
//   6. Attach PAYMENT-RESPONSE header on the way back.

use std::sync::Arc;

use axum::{
    body::Body,
    http::{header, HeaderValue, Request, Response, StatusCode},
    middleware::Next,
};
use serde_json::json;
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::{
    config::X402Config,
    facilitator::{FacilitatorError, InHouseVerifier, PaymentRequired},
    lineage::{LineageLedger, LineageRecord},
};

pub const HEADER_PAYMENT_REQUIRED:  &str = "PAYMENT-REQUIRED";
pub const HEADER_PAYMENT_SIGNATURE: &str = "PAYMENT-SIGNATURE";
pub const HEADER_PAYMENT_RESPONSE:  &str = "PAYMENT-RESPONSE";

// ── State shared across requests ──────────────────────────────────────────

#[derive(Clone)]
pub struct X402State {
    pub config:      Arc<X402Config>,
    pub verifier:    Arc<InHouseVerifier>,
    pub lineage:     Arc<LineageLedger>,
    pub price_usdc:  u64,    // atomic units for this route
    pub resource:    String,
    pub description: String,
}

impl X402State {
    pub fn new(
        config:      X402Config,
        price_usdc:  u64,
        resource:    impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        let resource    = resource.into();
        let description = description.into();
        let config      = Arc::new(config);
        let verifier    = Arc::new(InHouseVerifier::polygon_mainnet(&config));
        let lineage_path = config.lineage_path.clone();
        Self {
            verifier,
            config,
            lineage:     Arc::new(LineageLedger::open(lineage_path.into())),
            price_usdc,
            resource,
            description,
        }
    }
}

// ── Middleware function ───────────────────────────────────────────────────

/// Axum middleware that enforces x402 payment on the wrapped route.
pub async fn x402_gate(
    axum::extract::State(state): axum::extract::State<X402State>,
    req: Request<Body>,
    next: Next,
) -> Response<Body> {
    // Extract optional PAYMENT-SIGNATURE header.
    let sig_header = req
        .headers()
        .get(HEADER_PAYMENT_SIGNATURE)
        .and_then(|v: &axum::http::HeaderValue| v.to_str().ok())
        .map(|s: &str| s.to_string());

    let payment_req = PaymentRequired::for_endpoint(
        &state.config,
        state.price_usdc,
        &state.resource,
        &state.description,
    );

    match sig_header {
        None => {
            // Return 402 with payment instructions.
            warn!(resource = %state.resource, "x402: no payment header, returning 402");
            return payment_required_response(&payment_req);
        }
        Some(payload) => {
            // Verify in-house — no HTTP call, pure ECDSA recovery.
            let verification = match state.verifier.verify(&payload, state.price_usdc) {
                Ok(v)  => v,
                Err(FacilitatorError::AmountMismatch { expected, got }) => {
                    warn!("x402: amount mismatch expected={expected} got={got}");
                    return error_response(StatusCode::PAYMENT_REQUIRED, "amount mismatch");
                }
                Err(FacilitatorError::Expired { .. }) => {
                    warn!("x402: payment expired");
                    return error_response(StatusCode::PAYMENT_REQUIRED, "payment expired");
                }
                Err(FacilitatorError::Rejected { ref reason }) => {
                    warn!("x402: payment rejected — {reason}");
                    return error_response(StatusCode::PAYMENT_REQUIRED, reason);
                }
                Err(e) => {
                    error!("x402: verification error — {e}");
                    return error_response(StatusCode::BAD_REQUEST, "verification error");
                }
            };

            let payer = verification.payer.clone();
            info!(
                payer    = %payer,
                amount   = verification.amount_usdc,
                resource = %state.resource,
                "x402: payment verified in-house"
            );

            // No on-chain settle here. Settlement is batched by genesis-ledger
            // and anchored to SettlementAnchor.sol on Polygon per batch.

            // Append to lineage ledger.
            let record = LineageRecord {
                event_id:            Uuid::new_v4().to_string(),
                world_id:            crate::WORLD_ID.to_string(),
                wallet:              payer.clone(),
                agent_id:            None,
                payment_intent_id:   Uuid::new_v4().to_string(),
                authorization_hash:  payload.clone(),
                settlement_tx_hash:  None, // filled when batch settles on Polygon
                parent_event_id:     None,
                resource_id:         state.resource.clone(),
                entitlement_id:      None,
                revenue_split_id:    None,
                action_type:         state.resource.clone(),
                amount_usdc:         state.price_usdc,
                network:             state.config.network.clone(),
                timestamp:           chrono::Utc::now().to_rfc3339(),
            };
            if let Err(e) = state.lineage.append(&record) {
                error!("x402: lineage write failed — {e}");
            }

            // Pass through to the actual handler.
            let mut response: axum::response::Response<Body> = next.run(req).await;

            // Attach PAYMENT-RESPONSE header.
            let pr_json = json!({
                "payer":   payer,
                "amount":  payment_req.max_amount_required,
                "network": payment_req.network,
                "status":  "pending-batch-settlement",
            });
            if let Ok(hv) = HeaderValue::from_str(&pr_json.to_string()) {
                response.headers_mut().insert(HEADER_PAYMENT_RESPONSE, hv);
            }

            response
        }
    }
}

// ── 402 response builder ──────────────────────────────────────────────────

fn payment_required_response(payment_req: &PaymentRequired) -> Response<Body> {
    let header_val = payment_req.to_header_value();
    let body = json!({
        "error": "Payment required",
        "x402Version": payment_req.x402_version,
        "scheme": payment_req.scheme,
        "network": payment_req.network,
        "asset": payment_req.asset,
        "maxAmountRequired": payment_req.max_amount_required,
        "resource": payment_req.resource,
        "description": payment_req.description,
        "payTo": payment_req.pay_to,
    });

    Response::builder()
        .status(StatusCode::PAYMENT_REQUIRED)
        .header(header::CONTENT_TYPE, "application/json")
        .header(HEADER_PAYMENT_REQUIRED, header_val)
        .body(Body::from(body.to_string()))
        .unwrap()
}

fn error_response(status: StatusCode, msg: &str) -> Response<Body> {
    let body = json!({ "error": msg });
    Response::builder()
        .status(status)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(body.to_string()))
        .unwrap()
}
