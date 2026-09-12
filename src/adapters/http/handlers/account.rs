use crate::core::domain::account::{Account, AuthenticatedPrincipal};
use crate::shared::error::AppResult;
use crate::shared::state::AppState;
use axum::Json;
use axum::extract::State;

/// Handler for `GET /me`.
///
/// Retrieves account information for the authenticated principal.
/// The principal is extracted from the trusted header injected by ingress-nginx.
pub async fn get_me(
    State(state): State<AppState>,
    principal: AuthenticatedPrincipal,
) -> AppResult<Json<Account>> {
    let account = state.get_account_usecase.execute(&principal).await?;
    Ok(Json(account))
}
