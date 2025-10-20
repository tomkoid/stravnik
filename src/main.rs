use args::Args;
use clap::Parser;

use crate::meals::StravaClient;

mod args;
mod credentials;
mod env;
mod matrix;
mod meals;
mod ntfy;
mod services;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    dotenv::dotenv().ok();

    env::init_env(); // setup environment variables needed for any service

    // setup logger
    pretty_env_logger::formatted_builder()
        .parse_filters(&std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()))
        .init();

    match args.service {
        services::Service::Matrix => {
            matrix::env::init_env(); // initialize environment variables and error if some are missing

            let credentials = credentials::init_matrix_credentials();

            if credentials.is_err() {
                return Err(anyhow::anyhow!("{:?}", credentials.unwrap_err()));
            }

            matrix::sync::login_and_sync(credentials?).await?;
        }
        services::Service::Ntfy => {
            ntfy::env::init_env();

            let mut sc = StravaClient::new();
            sc.fetch_s5url().await;
            let meal_data = sc.get_meal_data().await?;

            ntfy::send::send_notification(meal_data).await?;
        }
    }

    Ok(())
}
