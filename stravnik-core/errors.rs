#[derive(thiserror::Error, Debug)]
pub enum MealClientError {
    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Meal not found")]
    MealNotFound,

    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Invalid config: {0}")]
    InvalidConfig(String),
}
