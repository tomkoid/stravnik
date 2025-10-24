use args::Args;
use clap::Parser;

use crate::services::pick_service;

mod args;
mod credentials;
mod discord;
mod icanteen;
mod matrix;
mod meal_data;
mod ntfy;
mod services;
mod strava;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // setup logger
    pretty_env_logger::formatted_builder()
        .parse_filters(&std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()))
        .init();

    pick_service(args).await?;

    Ok(())
}
