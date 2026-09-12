use crate::core::domain::error::DomainError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use thiserror::Error;
use tracing::error;

pub type AppResult<T> = Result<T, AppError>;

/// Unified application error type.
///
/// Converts domain and infrastructure errors into structured HTTP responses.
/// Error details safe for external exposure are included; sensitive internals
/// are logged server-side only and never returned to the caller.
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Account not found")]
    NotFound,

    #[error("Upstream Kratos error: {0}")]
    Upstream(String),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl From<DomainError> for AppError {
    fn from(err: DomainError) -> Self {
        match err {
            DomainError::AccountNotFound => AppError::NotFound,
            DomainError::InvalidPrincipal(msg) => AppError::Unauthorized(msg),
            DomainError::RepositoryError(msg) => AppError::Upstream(msg),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match &self {
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED", msg.clone()),
            AppError::NotFound => (
                StatusCode::NOT_FOUND,
                "ACCOUNT_NOT_FOUND",
                "The authenticated account was not found.".to_string(),
            ),
            AppError::Upstream(_) => {
                error!("Upstream Kratos error: {:?}", self);
                (
                    StatusCode::BAD_GATEWAY,
                    "UPSTREAM_ERROR",
                    "An upstream service error occurred.".to_string(),
                )
            }
            AppError::Internal(_) | AppError::Io(_) => {
                error!("Internal error: {:?}", self);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_ERROR",
                    "An internal server error occurred.".to_string(),
                )
            }
        };

        let body = json!({
            "error": {
                "code": code,
                "message": message,
            }
        });

        (status, axum::Json(body)).into_response()
    }
}
