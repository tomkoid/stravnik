mod credentials;
mod matrix;
mod meals;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    // setup logger
    pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    matrix::env::init_env(); // initialize environment variables and error if some are missing

    let credentials = credentials::init_matrix_credentials();

    if credentials.is_err() {
        return Err(anyhow::anyhow!("{:?}", credentials.unwrap_err()));
    }

    matrix::sync::login_and_sync(credentials?).await?;

    Ok(())
}
