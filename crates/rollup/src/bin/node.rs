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
async fn _new_rollup() -> Result<Rollup<MockRollup>, anyhow::Error> {
    let _mock_rollup = MockRollup {};

    todo!();
}
