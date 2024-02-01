use tracing::info;

fn main() {
    // Initializing logging
    tracing_subscriber::fmt().init();

    info!("Hello world");
}
