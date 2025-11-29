use args::Args;
use clap::Parser;

use crate::services::pick_service;

mod args;
mod env;
mod services;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    eyre::install()?;
    let args = Args::parse();

    // setup logger
    pretty_env_logger::formatted_builder()
        .parse_filters(&std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()))
        .init();

    let date = chrono::Local::now();
    pick_service(args, date).await?;

    Ok(())
}
