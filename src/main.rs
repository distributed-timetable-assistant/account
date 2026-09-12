use mimalloc::MiMalloc;
use account::infrastructure::bootstrap;
use account::shared::error::AppResult;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() -> AppResult<()> {
    bootstrap::start().await
}
