use gtnkr::UPPERCASE_PACKAGE_NAME;
use std::env;
use tracing::{subscriber, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    setup_debugging();

    gtnkr::cli::run().await?;

    Ok(())
}

fn setup_debugging() {
    let mut tracing_max_level = Level::INFO;
    let debug_env_var_key = format!("{}_DEBUG", UPPERCASE_PACKAGE_NAME.as_str());

    if let Ok(debug) = env::var(debug_env_var_key) {
        if debug == "1" {
            tracing_max_level = Level::TRACE;
        }
    }

    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing_max_level)
        .finish();

    subscriber::set_global_default(subscriber)
        .expect("Failed to set the global default tracing subscriber");
}
