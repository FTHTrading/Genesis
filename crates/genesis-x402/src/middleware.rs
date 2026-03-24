// middleware.rs — Axum x402 seller middleware
//
// Usage:
//   Router::new()
//       .route("/api/ai-call", get(ai_call_handler))
//       .layer(X402Layer::new(config, facilitator, lineage, price_usdc))
//
// Flow for each request:
//   1. Check PAYMENT-SIGNATURE header.
//   2. If absent  → 402 + PAYMENT-REQUIRED header.
//   3. If present → verify with facilitator.
//   4. If valid   → settle on-chain, record to lineage, pass through.
//   5. Record PAYMENT-RESPONSE header on the way back.

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
    facilitator::{FacilitatorClient, FacilitatorError, PaymentRequired},
    lineage::{LineageLedger, LineageRecord},
};

pub const HEADER_PAYMENT_REQUIRED:  &str = "PAYMENT-REQUIRED";
pub const HEADER_PAYMENT_SIGNATURE: &str = "PAYMENT-SIGNATURE";
pub const HEADER_PAYMENT_RESPONSE:  &str = "PAYMENT-RESPONSE";

// ── State shared across requests ──────────────────────────────────────────

#[derive(Clone)]
pub struct X402State {
    pub config:      Arc<X402Config>,
    pub facilitator: Arc<FacilitatorClient>,
    pub lineage:     Arc<LineageLedger>,
    pub price_usdc:  u64,    // atomic units for this route
    pub resource:    String, // e.g. "/api/ai-call"
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
        let facilitator = Arc::new(FacilitatorClient::new(&config));
        let lineage_path = config.lineage_path.clone();
        Self {
            facilitator,
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
            // Verify with CDP facilitator.
            let payer: String = match state.facilitator.verify(&payload, &payment_req).await {
                Ok(addr) => addr,
                Err(FacilitatorError::Rejected { reason }) => {
                    warn!("x402: payment rejected — {reason}");
                    return error_response(StatusCode::PAYMENT_REQUIRED, &reason);
                }
                Err(FacilitatorError::Timeout) => {
                    error!("x402: facilitator timeout");
                    return error_response(
                        StatusCode::SERVICE_UNAVAILABLE,
                        "facilitator timeout",
                    );
                }
                Err(e) => {
                    error!("x402: facilitator error — {e}");
                    return error_response(StatusCode::BAD_GATEWAY, "facilitator error");
                }
            };

            // Settle on-chain.
            let settlement = match state.facilitator.settle(&payload, &payment_req).await {
                Ok(s) => s,
                Err(e) => {
                    error!("x402: settlement failed — {e}");
                    return error_response(StatusCode::BAD_GATEWAY, "settlement failed");
                }
            };

            let tx_hash = settlement.tx_hash.clone().unwrap_or_default();
            info!(
                payer = %payer,
                tx_hash = %tx_hash,
                amount = ?settlement.amount,
                resource = %state.resource,
                "x402: payment settled"
            );

            // Append to lineage ledger.
            let record = LineageRecord {
                event_id:     Uuid::new_v4().to_string(),
                world_id:     crate::WORLD_ID.to_string(),
                wallet:       payer.clone(),
                agent_id:     None,
                payment_intent_id:   Uuid::new_v4().to_string(),
                authorization_hash:  payload.clone(),
                settlement_tx_hash:  Some(tx_hash.clone()),
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
                "txHash":  tx_hash,
                "payer":   payer,
                "amount":  payment_req.max_amount_required,
                "network": payment_req.network,
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
