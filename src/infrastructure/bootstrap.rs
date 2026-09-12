use crate::adapters::external::kratos::KratosClient;
use crate::adapters::http::routes::create_router;
use crate::core::usecases::get_account::GetAccountUseCase;
use crate::infrastructure::{cli::Cli, config_loader, telemetry};
use crate::shared::config::Config;
use crate::shared::error::AppResult;
use crate::shared::state::AppState;
use clap::Parser;
use std::sync::Arc;
use tracing::info;

/// Bootstraps the application according to the DiTA Service Bootstrap Guideline.
///
/// Orchestrates:
/// 1. Telemetry initialization
/// 2. CLI parsing
/// 3. Configuration loading
/// 4. Adapter & use case instantiation
/// 5. Application state construction
/// 6. Axum router initialization
/// 7. HTTP server listener binding and serving
pub async fn start() -> AppResult<()> {
    telemetry::init();
    info!("Starting account service...");

    let cli = Cli::parse();
    let conf_path = cli.config;
    let config: Config = config_loader::load(&conf_path);

    let kratos_client = KratosClient::new(config.kratos.admin_url.clone(), None);
    let get_account_usecase = Arc::new(GetAccountUseCase::new(Arc::new(kratos_client)));

    let state = AppState {
        config: config.clone(),
        get_account_usecase,
    };

    let app = create_router(state);

    let listener = tokio::net::TcpListener::bind(&config.listen_addr).await?;

    info!("Listening -> SUCCESS: listen_addr=({})", config.listen_addr);

    axum::serve(listener, app).await?;

    info!(
        "Axum Serve -> SUCCESS: listen_addr=({})",
        config.listen_addr
    );

    Ok(())
}
