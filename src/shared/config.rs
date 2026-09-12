use serde::Deserialize;
use std::fmt;
use std::net::SocketAddr;

/// Application configuration model.
///
/// Loaded by `infrastructure::config_loader`. All values can be overridden via
/// configuration files or environment variables with the `APP_` prefix.
///
/// The `Debug` implementation is manually written to redact any secrets that
/// may be added to `KratosConfig` in the future, preventing accidental secret
/// exposure through `tracing`.
#[derive(Clone, Deserialize)]
pub struct Config {
    pub listen_addr: SocketAddr,
    pub auth: AuthConfig,
    pub kratos: KratosConfig,
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Config")
            .field("listen_addr", &self.listen_addr)
            .field("auth", &self.auth)
            .field("kratos", &self.kratos)
            .finish()
    }
}

/// Authentication configuration consumed by the principal extractor.
#[derive(Clone, Deserialize, Debug)]
pub struct AuthConfig {
    /// Name of the HTTP request header that carries the authenticated subject.
    ///
    /// This header is injected by the trusted ingress layer (ingress-nginx from
    /// the oauth2-proxy auth response). It is never accepted directly from an
    /// untrusted client.
    ///
    /// Default: `X-Authenticated-Subject`
    #[serde(default = "default_principal_header")]
    pub principal_header: String,
}

fn default_principal_header() -> String {
    "X-Authenticated-Subject".to_string()
}

/// Ory Kratos integration configuration.
///
/// The `Debug` implementation redacts any future secret fields.
#[derive(Clone, Deserialize)]
pub struct KratosConfig {
    /// Internal cluster URL for the Ory Kratos Admin API.
    ///
    /// Example: `http://kratos:4434`
    pub admin_url: String,
}

impl fmt::Debug for KratosConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("KratosConfig")
            .field("admin_url", &self.admin_url)
            .finish()
    }
}
