#[derive(thiserror::Error, Debug)]
pub enum NotificationClientError {
    #[error("Parse error: {0}")]
    ParseError(#[from] serde_json::Error),

    #[error("Room {0} not found")]
    RoomNotFound(String),

    #[cfg(any(feature = "matrix", feature = "discord"))]
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[cfg(feature = "matrix")]
    #[error("Matrix error: {0}")]
    MatrixError(#[from] matrix_sdk::Error),

    #[cfg(feature = "matrix")]
    #[error("Matrix client build error: {0}")]
    MatrixClientBuildError(#[from] matrix_sdk::ClientBuildError),

    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Request error: {0}")]
    RequestError(String),

    #[error("Invalid config: {0}")]
    InvalidConfig(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_room_not_found_display() {
        let error = NotificationClientError::RoomNotFound("test-room".to_string());
        assert_eq!(error.to_string(), "Room test-room not found");
    }

    #[test]
    fn test_request_error_display() {
        let error = NotificationClientError::RequestError("Connection failed".to_string());
        assert_eq!(error.to_string(), "Request error: Connection failed");
    }

    #[test]
    fn test_invalid_config_display() {
        let error = NotificationClientError::InvalidConfig("Missing API key".to_string());
        assert_eq!(error.to_string(), "Invalid config: Missing API key");
    }

    #[test]
    fn test_io_error_conversion() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let error: NotificationClientError = io_error.into();
        assert!(error.to_string().contains("IO error"));
    }
}
