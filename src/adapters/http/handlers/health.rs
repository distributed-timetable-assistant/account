use axum::Json;
use serde_json::{Value, json};

/// Handler for `GET /health`.
///
/// Unauthenticated endpoint providing service health status.
pub async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}
