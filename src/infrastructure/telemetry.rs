/// Initializes tracing subscriber for structured logging.
pub fn init() {
    let _ = tracing_subscriber::fmt::try_init();
}
