// src/interfaces/api.rs

use axum::{
    extract::Json,
    http::StatusCode,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use chrono::Utc;

/// The expected structure of the incoming request body (n8n "directive" style)
#[derive(Debug, Deserialize)]
pub struct IngressBody {
    #[serde(rename = "directive")]
    pub action: Option<String>,
    #[serde(rename = "origin_node")]
    pub origin_node: Option<String>,
    pub idempotency_key: Option<String>, // fallback if header is missing
    pub data: Option<serde_json::Value>,
}

/// The response payload structure returned to the caller
#[derive(Debug, Serialize)]
#[serde(tag = "status", rename_all = "UPPERCASE")]
pub enum IngressResponse {
    Accepted { data: SanitizedPayload },
    Rejected { reason: String },
}

#[derive(Debug, Serialize)]
pub struct SanitizedPayload {
    pub timestamp: String,
    pub idempotency_key: String,
    pub source: String,
    pub action: String,
    pub parameters: serde_json::Value,
}

/// POST /submit
/// Enforces structural contracts on the payload and headers.
pub async fn submit_handler(
    headers: axum::http::HeaderMap,
    Json(body): Json<IngressBody>,
) -> (StatusCode, Json<IngressResponse>) {
    // Extract idempotency key from header, fallback to body.idempotency_key
    let idempotency_key = headers
        .get("x-idempotency-key")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string())
        .or(body.idempotency_key.clone());

    // Extract action from directive field, fallback to top-level "action"
    let action = body.action.clone();

    if idempotency_key.is_none() || action.is_none() {
        let response = IngressResponse::Rejected {
            reason: "ERR_MALFORMED_PACKET: Guillotine discard reflex engaged. Missing critical routing headers or directive.".to_string(),
        };
        return (StatusCode::BAD_REQUEST, Json(response));
    }

    let sanitized = SanitizedPayload {
        timestamp: headers
            .get("x-timestamp")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string())
            .unwrap_or_else(|| Utc::now().to_rfc3339()),
        idempotency_key: idempotency_key.unwrap(),
        source: body.origin_node.clone().unwrap_or_else(|| "UNKNOWN_NODE".to_string()),
        action: action.unwrap(),
        parameters: body.data.clone().unwrap_or(serde_json::json!({})),
    };

    (StatusCode::OK, Json(IngressResponse::Accepted { data: sanitized }))
}

pub fn router() -> Router {
    Router::new().route("/submit", post(submit_handler))
}
