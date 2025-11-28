#[derive(Debug)]
pub struct MatrixCredentials {
    pub homeserver: String,
    pub username: String,
    pub password: String,
}

impl MatrixCredentials {
    pub fn new(homeserver: String, username: String, password: String) -> Self {
        Self {
            homeserver,
            username,
            password,
        }
    }
}
