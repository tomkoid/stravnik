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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_credentials_new() {
        let creds = MatrixCredentials::new(
            "https://matrix.org".to_string(),
            "user".to_string(),
            "pass123".to_string(),
        );
        assert_eq!(creds.homeserver, "https://matrix.org");
        assert_eq!(creds.username, "user");
        assert_eq!(creds.password, "pass123");
    }

    #[test]
    fn test_matrix_credentials_empty() {
        let creds = MatrixCredentials::new(
            "".to_string(),
            "".to_string(),
            "".to_string(),
        );
        assert_eq!(creds.homeserver, "");
        assert_eq!(creds.username, "");
        assert_eq!(creds.password, "");
    }
}
