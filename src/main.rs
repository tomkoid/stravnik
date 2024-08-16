mod credentials;
mod env;
mod meals;
mod sync;

use crate::env::init_env;
use crate::sync::login_and_sync;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // setup logger
    pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    init_env(); // initialize environment variables and error if some are missing

    let credentials = credentials::init_credentials();

    if credentials.is_err() {
        return Err(anyhow::anyhow!("{:?}", credentials.unwrap_err()));
    }

    login_and_sync(credentials?).await?;

    Ok(())
}
