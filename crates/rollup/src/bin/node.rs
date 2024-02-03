use anyhow::Error;
use tracing::info;

use sov_modules_rollup_blueprint::Rollup;
#[cfg(feature = "mock_da")]
use sov_rollup_starter::mock_rollup::MockRollup;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Initializing logging
    tracing_subscriber::fmt().init();

    info!("Hello world");

    Ok(())
}

#[cfg(feature = "mock_da")]
async fn new_rollup() -> Result<Rollup<MockRollup>, anyhow::Error> {
    let mock_rollup = MockRollup {};

    todo!();
}
