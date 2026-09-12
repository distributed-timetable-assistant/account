use crate::core::domain::account::{Account, KratosIdentity};
use crate::core::domain::error::DomainError;
use crate::core::ports::account_repository::AccountRepository;
use async_trait::async_trait;
use reqwest_middleware::ClientWithMiddleware;

/// Client for communicating with the Ory Kratos Admin API.
///
/// Implements the `AccountRepository` port. This adapter connects directly to
/// the Kratos Admin API (`/admin/identities/{id}`) as an internal, trusted
/// server-to-server interaction. It does not perform public self-service flows
/// and does not fall back to other endpoints.
#[derive(Clone)]
pub struct KratosClient {
    admin_url: String,
    client: ClientWithMiddleware,
}

impl KratosClient {
    /// Creates a new `KratosClient`.
    ///
    /// If `client` is `None`, a default `reqwest::Client` wrapped with
    /// `reqwest_tracing::TracingMiddleware` is constructed.
    pub fn new(admin_url: String, client: Option<ClientWithMiddleware>) -> Self {
        let client = client.unwrap_or_else(|| {
            let reqwest_client = reqwest::Client::new();
            reqwest_middleware::ClientBuilder::new(reqwest_client)
                .with(reqwest_tracing::TracingMiddleware::default())
                .build()
        });
        let admin_url = admin_url.trim_end_matches('/').to_string();
        Self { admin_url, client }
    }
}

#[async_trait]
impl AccountRepository for KratosClient {
    async fn get_account_by_id(&self, id: &str) -> Result<Account, DomainError> {
        let url = format!("{}/identities/{}", self.admin_url, id);

        let res = self.client.get(&url).send().await.map_err(|e| {
            DomainError::RepositoryError(format!("Failed to connect to Kratos: {}", e))
        })?;

        let status = res.status();
        if status == reqwest::StatusCode::NOT_FOUND {
            return Err(DomainError::AccountNotFound);
        }

        if !status.is_success() {
            let err_body = res.text().await.unwrap_or_default();
            return Err(DomainError::RepositoryError(format!(
                "Kratos returned error status {}: {}",
                status, err_body
            )));
        }

        let kratos_identity: KratosIdentity = res.json().await.map_err(|e| {
            DomainError::RepositoryError(format!("Failed to parse Kratos identity response: {}", e))
        })?;

        Ok(kratos_identity.into_account())
    }
}
