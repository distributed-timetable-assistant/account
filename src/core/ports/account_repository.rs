use crate::core::domain::account::Account;
use crate::core::domain::error::DomainError;
use async_trait::async_trait;

/// Abstract port for account persistence/retrieval.
///
/// Implemented by `KratosClient` in the adapters layer. Using this trait
/// decouples the use cases from the HTTP/Kratos implementation details and
/// makes the use cases independently testable with mock implementations.
#[async_trait]
pub trait AccountRepository: Send + Sync {
    /// Retrieves the account associated with the given Kratos identity ID.
    ///
    /// Returns `DomainError::AccountNotFound` if no identity exists with that ID.
    /// Returns `DomainError::RepositoryError` for upstream communication failures.
    async fn get_account_by_id(&self, id: &str) -> Result<Account, DomainError>;
}
