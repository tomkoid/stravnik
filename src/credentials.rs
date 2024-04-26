use log::error;

#[derive(Debug)]
pub struct Credentials {
    pub homeserver: String,
    pub username: String,
    pub password: String,
}

pub fn init_credentials() -> anyhow::Result<Credentials> {
    // get username and password from environment variables
    let homeserver = std::env::var("MATRIX_HOMESERVER").unwrap_or("https://matrix.org".to_string());
    let username = std::env::var("MATRIX_USERNAME").unwrap_or(String::new());
    let password = std::env::var("MATRIX_PASSWORD").unwrap_or(String::new());

    if homeserver.is_empty() {
        error!("Missing matrix homeserver (MATRIX_HOMESERVER)");

        return Err(anyhow::anyhow!(
            "Missing matrix homeserver (MATRIX_HOMESERVER)"
        ));
    }

    if username.is_empty() || password.is_empty() {
        error!("Missing username (MATRIX_USERNAME) or password (MATRIX_PASSWORD)");

        return Err(anyhow::anyhow!(
            "Missing username (MATRIX_USERNAME) or password (MATRIX_PASSWORD)"
        ));
    }

    Ok(Credentials {
        homeserver,
        username,
        password,
    })
}
