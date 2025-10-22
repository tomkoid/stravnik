use args::Args;
use clap::Parser;

use crate::strava::client::StravaClient;

mod args;
mod credentials;
mod discord;
mod matrix;
mod meal_data;
mod ntfy;
mod services;
mod strava;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    dotenv::dotenv().ok();

    strava::env::init_env(); // setup environment variables needed for strava

    // setup logger
    pretty_env_logger::formatted_builder()
        .parse_filters(&std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()))
        .init();

    // create new strava client
    let mut sc = StravaClient::new();

    // fetch the correct s5url needed for the meal list API request
    sc.fetch_s5url().await;

    let meal_data = sc.get_meal_data().await?;

    match args.service {
        services::NotificationService::Matrix => {
            matrix::env::init_env(); // initialize environment variables and error if some are missing

            let credentials = credentials::init_matrix_credentials();

            if credentials.is_err() {
                return Err(anyhow::anyhow!("{:?}", credentials.unwrap_err()));
            }

            let m_client = matrix::sync::login_and_sync(credentials?).await?;
            matrix::message::send_meal_data(&m_client, meal_data.basic_fmt()).await?;
            matrix::sync::client_sync(&m_client).await?; // do a final sync
        }
        services::NotificationService::Ntfy => {
            ntfy::env::init_env();

            ntfy::send::send_ntfy_notification(meal_data.basic_fmt()).await?;
        }
        services::NotificationService::Discord => {
            discord::env::init_env();

            discord::send::send_discord_message(meal_data.discord_fmt()).await?
        }
    }

    Ok(())
}
