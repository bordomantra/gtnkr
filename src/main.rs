use std::env;
use tracing::{subscriber, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut tracing_max_level = Level::INFO;

    if let Ok(gtnkr_debug) = env::var("GTNKR_DEBUG") {
        if gtnkr_debug == "1" {
            tracing_max_level = Level::TRACE;
        }
    }

    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing_max_level)
        .finish();

    subscriber::set_global_default(subscriber)
        .expect("Failed to set the global default tracing subscriber");

    Ok(())
}
