use crate::adapters::http::handlers::{account::get_me, health::health};
use crate::shared::state::AppState;
use axum::{Router, routing::get};
use tower_http::trace::TraceLayer;

/// Builds the Axum application router with all routes and middleware configured.
pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/me", get(get_me))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
