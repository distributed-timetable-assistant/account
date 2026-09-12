use crate::adapters::external::kratos::KratosClient;
use crate::core::usecases::get_account::GetAccountUseCase;
use crate::shared::config::Config;
use std::sync::Arc;

/// Shared application runtime state.
///
/// Constructed once during bootstrap and injected into the Axum router.
/// All fields are read-only after construction.
#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub get_account_usecase: Arc<GetAccountUseCase<KratosClient>>,
}
