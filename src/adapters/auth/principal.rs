use crate::core::domain::account::AuthenticatedPrincipal;
use crate::shared::error::AppError;
use crate::shared::state::AppState;
use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;

impl<S> FromRequestParts<S> for AuthenticatedPrincipal
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);
        let header_name = &app_state.config.auth.principal_header;
        let header_val = parts.headers.get(header_name.as_str()).ok_or_else(|| {
            AppError::Unauthorized(format!("Missing required header: {}", header_name))
        })?;

        let subject_str = header_val.to_str().map_err(|_| {
            AppError::Unauthorized(format!(
                "Header {} contains invalid characters",
                header_name
            ))
        })?;

        let trimmed = subject_str.trim();
        if trimmed.is_empty() {
            return Err(AppError::Unauthorized(format!(
                "Header {} cannot be empty",
                header_name
            )));
        }

        Ok(AuthenticatedPrincipal {
            subject: trimmed.to_string(),
        })
    }
}
