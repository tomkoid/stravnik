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
