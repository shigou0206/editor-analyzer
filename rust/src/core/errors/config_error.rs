use thiserror::Error;
use crate::core::errors::codes;

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
    /// 构造函数，自动填充 code
    pub fn config_not_found(path: String) -> Self {
        ConfigError::ConfigNotFound {
            code: codes::config::CONFIG_NOT_FOUND,
            path,
        }
    }
    pub fn parse_failed(message: String) -> Self {
        ConfigError::ParseFailed {
            code: codes::config::INVALID_FORMAT,
            message,
        }
    }
    pub fn validation_failed(message: String) -> Self {
        ConfigError::ValidationFailed {
            code: codes::config::ALL,
            message,
        }
    }
    pub fn missing_required(key: String) -> Self {
        ConfigError::MissingRequired {
            code: codes::CONFIG_KEY_NOT_FOUND,
            key,
        }
    }
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
    use crate::core::errors::codes;

    #[test]
    fn test_config_error() {
        let not_found = ConfigError::config_not_found("/config.json".to_string());
        assert!(not_found.to_string().contains("Configuration file not found"));
        assert!(not_found.to_string().contains("/config.json"));
        assert_eq!(not_found.code(), codes::config::CONFIG_NOT_FOUND);
        
        let parse_failed = ConfigError::parse_failed("Invalid JSON".to_string());
        assert!(parse_failed.to_string().contains("Configuration parse failed"));
        assert!(parse_failed.to_string().contains("Invalid JSON"));
        assert_eq!(parse_failed.code(), codes::config::INVALID_FORMAT);
        
        let validation_failed = ConfigError::validation_failed("Invalid value".to_string());
        assert!(validation_failed.to_string().contains("Configuration validation failed"));
        assert!(validation_failed.to_string().contains("Invalid value"));
        assert_eq!(validation_failed.code(), codes::config::ALL);
        
        let missing_required = ConfigError::missing_required("api_key".to_string());
        assert!(missing_required.to_string().contains("Missing required configuration"));
        assert!(missing_required.to_string().contains("api_key"));
        assert_eq!(missing_required.code(), codes::CONFIG_KEY_NOT_FOUND);
    }
} 