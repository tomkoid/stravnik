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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_error_display() {
        let error = MealClientError::ParseError("Invalid JSON".to_string());
        assert_eq!(error.to_string(), "Parse error: Invalid JSON");
    }

    #[test]
    fn test_meal_not_found_display() {
        let error = MealClientError::MealNotFound;
        assert_eq!(error.to_string(), "Meal not found");
    }

    #[test]
    fn test_invalid_config_display() {
        let error = MealClientError::InvalidConfig("Missing URL".to_string());
        assert_eq!(error.to_string(), "Invalid config: Missing URL");
    }
}
