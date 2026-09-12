use thiserror::Error;

/// Domain-level errors for the account domain.
///
/// These are framework-agnostic and translated to HTTP responses at the adapter
/// layer via `AppError`.
#[derive(Error, Debug)]
pub enum DomainError {
    /// The requested account was not found in Ory Kratos.
    #[error("Account not found")]
    AccountNotFound,

    /// The authenticated principal is missing or invalid.
    #[error("Invalid principal: {0}")]
    InvalidPrincipal(String),

    /// An error occurred communicating with the Kratos Admin API.
    #[error("Repository error: {0}")]
    RepositoryError(String),
}
