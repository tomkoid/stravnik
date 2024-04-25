pub struct Credentials {
    pub homeserver: String,
    pub username: String,
    pub password: String,
}

pub fn init_credentials() -> anyhow::Result<Credentials> {
    // get username and password from environment variables
    let homeserver = std::env::var("MATRIX_HOMESERVER").unwrap_or("https://matrix.org".to_string());
    let username = std::env::var("MATRIX_USERNAME").unwrap_or("".to_string());
    let password = std::env::var("MATRIX_PASSWORD").unwrap_or("".to_string());

    if homeserver.is_empty() {
        return Err(anyhow::anyhow!(
            "Missing matrix homeserver (MATRIX_HOMESERVER)"
        ));
    }

    if username.is_empty() || password.is_empty() {
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
