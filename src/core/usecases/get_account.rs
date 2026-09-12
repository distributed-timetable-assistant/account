use crate::core::domain::account::{Account, AuthenticatedPrincipal};
use crate::core::domain::error::DomainError;
use crate::core::ports::account_repository::AccountRepository;
use std::sync::Arc;

/// Use case: retrieve account information for an authenticated principal.
///
/// This is the primary account-domain operation. It accepts a trusted
/// `AuthenticatedPrincipal` (sourced from the ingress layer, not from any JWT
/// processed by this service) and retrieves the corresponding account from
/// the `AccountRepository` port.
///
/// Contains no HTTP, framework, or Kratos-specific logic. Those concerns belong
/// in the adapter layer.
pub struct GetAccountUseCase<R: AccountRepository> {
    repository: Arc<R>,
}

impl<R: AccountRepository> GetAccountUseCase<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    /// Execute the use case.
    ///
    /// Validates the principal is non-empty and retrieves the account by ID.
    pub async fn execute(
        &self,
        principal: &AuthenticatedPrincipal,
    ) -> Result<Account, DomainError> {
        if principal.subject.is_empty() {
            return Err(DomainError::InvalidPrincipal(
                "Authenticated subject must not be empty.".to_string(),
            ));
        }

        self.repository.get_account_by_id(&principal.subject).await
    }
}
