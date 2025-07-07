use thiserror::Error;

/// Configuration error
#[derive(Error, Debug, PartialEq, Eq)]
pub enum ConfigError {
    #[error("Configuration file not found: {path}")]
    ConfigNotFound { code: &'static str, path: String },
    
    #[error("Configuration parse failed: {message}")]
    ParseFailed { code: &'static str, message: String },
    
    #[error("Configuration validation failed: {message}")]
    ValidationFailed { code: &'static str, message: String },
    
    #[error("Missing required configuration: {key}")]
    MissingRequired { code: &'static str, key: String },
}

impl ConfigError {
    /// Get the error code
    pub fn code(&self) -> &'static str {
        match self {
            ConfigError::ConfigNotFound { code, .. } => code,
            ConfigError::ParseFailed { code, .. } => code,
            ConfigError::ValidationFailed { code, .. } => code,
            ConfigError::MissingRequired { code, .. } => code,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_error() {
        let not_found = ConfigError::ConfigNotFound {
            code: "config_not_found",
            path: "/config.json".to_string(),
        };
        assert!(not_found.to_string().contains("Configuration file not found"));
        assert!(not_found.to_string().contains("/config.json"));
        assert_eq!(not_found.code(), "config_not_found");
        
        let parse_failed = ConfigError::ParseFailed {
            code: "parse_failed",
            message: "Invalid JSON".to_string(),
        };
        assert!(parse_failed.to_string().contains("Configuration parse failed"));
        assert!(parse_failed.to_string().contains("Invalid JSON"));
        assert_eq!(parse_failed.code(), "parse_failed");
        
        let validation_failed = ConfigError::ValidationFailed {
            code: "validation_failed",
            message: "Invalid value".to_string(),
        };
        assert!(validation_failed.to_string().contains("Configuration validation failed"));
        assert!(validation_failed.to_string().contains("Invalid value"));
        assert_eq!(validation_failed.code(), "validation_failed");
        
        let missing_required = ConfigError::MissingRequired {
            code: "missing_required",
            key: "api_key".to_string(),
        };
        assert!(missing_required.to_string().contains("Missing required configuration"));
        assert!(missing_required.to_string().contains("api_key"));
        assert_eq!(missing_required.code(), "missing_required");
    }
} 