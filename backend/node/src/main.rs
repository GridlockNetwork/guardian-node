use anyhow::Result;
use node::logging::GridlockLogInitializer;

#[tokio::main]
async fn main() -> Result<()> {
    GridlockLogInitializer::init();
    Ok(())
}
