use log::info;
use services::get_notification_services;

mod credentials;
mod matrix;
mod meals;
mod services;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    // setup logger
    pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    // get notification services for notifications to send
    let services = get_notification_services();

    if services.is_empty() {
        info!("No notification services enabled, please enable at least one in `.env` and try again, exiting...");
        return Ok(());
    }

    for service in services {
        match service {
            services::Service::Matrix => {
                matrix::env::init_env(); // initialize environment variables and error if some are missing

                let credentials = credentials::init_matrix_credentials();

                if credentials.is_err() {
                    return Err(anyhow::anyhow!("{:?}", credentials.unwrap_err()));
                }

                matrix::sync::login_and_sync(credentials?).await?;
            }
            services::Service::Ntfy => {
                info!("Sending notification to ntfy...");
                // ntfy::send_notification().await?;
            }
        }
    }

    Ok(())
}
