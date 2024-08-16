#[derive(Debug)]
pub struct Credentials {
    pub homeserver: String,
    pub username: String,
    pub password: String,
}

pub fn init_credentials() -> anyhow::Result<Credentials> {
    // get username and password from environment variables
    let homeserver = std::env::var("MATRIX_HOMESERVER").unwrap_or("https://matrix.org".to_string());
    let username = std::env::var("MATRIX_USERNAME")?;
    let password = std::env::var("MATRIX_PASSWORD")?;

    Ok(Credentials {
        homeserver,
        username,
        password,
    })
}
