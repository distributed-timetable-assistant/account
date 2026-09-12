use clap::Parser;

/// Command-line interface for the DiTA Account Service.
///
/// Complies with the DiTA Service Bootstrap Guideline.
#[derive(Parser, Debug)]
#[command(name = "account")]
#[command(about = "DiTA Account Service", long_about = None)]
pub struct Cli {
    /// Path to an optional configuration file.
    #[arg(short, long, global = true)]
    pub config: Option<String>,
}
