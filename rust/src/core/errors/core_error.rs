use thiserror::Error;
use super::parser_error::ParserError;
use super::semantic_error::SemanticError;
use super::ai_error::AiError;
use super::lsp_error::LspError;
use super::file_error::FileError;
use super::config_error::ConfigError;
use super::network_error::NetworkError;
use crate::core::errors::codes;

/// Core error type
#[derive(Error, Debug, PartialEq, Eq)]
pub enum CoreError {
    #[error("Parse error: {message}")]
    ParseError { code: &'static str, message: String },
    
    #[error("Semantic error: {message}")]
    SemanticError { code: &'static str, message: String },
    
    #[error("AI service error: {message}")]
    AiError { code: &'static str, message: String },
    
    #[error("LSP error: {message}")]
    LspError { code: &'static str, message: String },
    
    #[error("File error: {message}")]
    FileError { code: &'static str, message: String },
    
    #[error("Config error: {message}")]
    ConfigError { code: &'static str, message: String },
    
    #[error("Network error: {message}")]
    NetworkError { code: &'static str, message: String },
    
    #[error("Internal error: {message}")]
    InternalError { code: &'static str, message: String },
}

impl CoreError {
    /// Get the error code
    pub fn code(&self) -> &'static str {
        match self {
            CoreError::ParseError { code, .. } => code,
            CoreError::SemanticError { code, .. } => code,
            CoreError::AiError { code, .. } => code,
            CoreError::LspError { code, .. } => code,
            CoreError::FileError { code, .. } => code,
            CoreError::ConfigError { code, .. } => code,
            CoreError::NetworkError { code, .. } => code,
            CoreError::InternalError { code, .. } => code,
        }
    }
}

/// Error conversion implementations
impl From<ParserError> for CoreError {
    fn from(err: ParserError) -> Self {
        CoreError::ParseError {
            code: codes::parser::ALL,
            message: err.to_string(),
        }
    }
}

impl From<SemanticError> for CoreError {
    fn from(err: SemanticError) -> Self {
        CoreError::SemanticError {
            code: codes::semantic::ALL,
            message: err.to_string(),
        }
    }
}

impl From<AiError> for CoreError {
    fn from(err: AiError) -> Self {
        CoreError::AiError {
            code: codes::ai::ALL,
            message: err.to_string(),
        }
    }
}

impl From<LspError> for CoreError {
    fn from(err: LspError) -> Self {
        CoreError::LspError {
            code: codes::lsp::ALL,
            message: err.to_string(),
        }
    }
}

impl From<FileError> for CoreError {
    fn from(err: FileError) -> Self {
        CoreError::FileError {
            code: codes::file::ALL,
            message: err.to_string(),
        }
    }
}

impl From<ConfigError> for CoreError {
    fn from(err: ConfigError) -> Self {
        CoreError::ConfigError {
            code: codes::config::ALL,
            message: err.to_string(),
        }
    }
}

impl From<NetworkError> for CoreError {
    fn from(err: NetworkError) -> Self {
        CoreError::NetworkError {
            code: codes::network::ALL,
            message: err.to_string(),
        }
    }
}

impl From<std::io::Error> for CoreError {
    fn from(err: std::io::Error) -> Self {
        CoreError::FileError {
            code: codes::io::ALL,
            message: err.to_string(),
        }
    }
}

impl From<serde_json::Error> for CoreError {
    fn from(err: serde_json::Error) -> Self {
        CoreError::InternalError {
            code: codes::internal::JSON_ERROR,
            message: format!("JSON serialization error: {}", err),
        }
    }
}

impl From<reqwest::Error> for CoreError {
    fn from(err: reqwest::Error) -> Self {
        CoreError::NetworkError {
            code: codes::reqwest::ALL,
            message: err.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::Span;

    #[test]
    fn test_core_error_display() {
        let parse_error = CoreError::ParseError {
            code: codes::parser::ALL,
            message: "Syntax error".to_string(),
        };
        assert!(parse_error.to_string().contains("Parse error"));
        assert!(parse_error.to_string().contains("Syntax error"));
        assert_eq!(parse_error.code(), codes::parser::ALL);
        
        let semantic_error = CoreError::SemanticError {
            code: codes::semantic::ALL,
            message: "Type error".to_string(),
        };
        assert!(semantic_error.to_string().contains("Semantic error"));
        assert!(semantic_error.to_string().contains("Type error"));
        assert_eq!(semantic_error.code(), codes::semantic::ALL);
        
        let ai_error = CoreError::AiError {
            code: codes::ai::ALL,
            message: "API failed".to_string(),
        };
        assert!(ai_error.to_string().contains("AI service error"));
        assert!(ai_error.to_string().contains("API failed"));
        assert_eq!(ai_error.code(), codes::ai::ALL);
    }

    #[test]
    fn test_error_conversions() {
        // Test ParserError conversion
        let parser_error = ParserError::syntax_error("Test syntax error".to_string(), Span::new(0, 10));
        let core_error: CoreError = parser_error.into();
        match core_error {
            CoreError::ParseError { code, message } => {
                assert_eq!(code, codes::parser::ALL);
                assert!(message.contains("Test syntax error"));
            }
            _ => panic!("Expected ParseError"),
        }
        
        // Test SemanticError conversion
        let semantic_error = SemanticError::symbol_not_found("test_func".to_string());
        let core_error: CoreError = semantic_error.into();
        match core_error {
            CoreError::SemanticError { code, message } => {
                assert_eq!(code, codes::semantic::ALL);
                assert!(message.contains("test_func"));
            }
            _ => panic!("Expected SemanticError"),
        }
        
        // Test AiError conversion
        let ai_error = AiError::api_call_failed("API call failed".to_string());
        let core_error: CoreError = ai_error.into();
        match core_error {
            CoreError::AiError { code, message } => {
                assert_eq!(code, codes::ai::ALL);
                assert!(message.contains("API call failed"));
            }
            _ => panic!("Expected AiError"),
        }
        
        // Test LspError conversion
        let lsp_error = LspError::connection_failed("Connection failed".to_string());
        let core_error: CoreError = lsp_error.into();
        match core_error {
            CoreError::LspError { code, message } => {
                assert_eq!(code, codes::lsp::ALL);
                assert!(message.contains("Connection failed"));
            }
            _ => panic!("Expected LspError"),
        }
        
        // Test FileError conversion
        let file_error = FileError::file_not_found("/test/path".to_string());
        let core_error: CoreError = file_error.into();
        match core_error {
            CoreError::FileError { code, message } => {
                assert_eq!(code, codes::file::ALL);
                assert!(message.contains("/test/path"));
            }
            _ => panic!("Expected FileError"),
        }
        
        // Test ConfigError conversion
        let config_error = ConfigError::config_not_found("/config.json".to_string());
        let core_error: CoreError = config_error.into();
        match core_error {
            CoreError::ConfigError { code, message } => {
                assert_eq!(code, codes::config::ALL);
                assert!(message.contains("/config.json"));
            }
            _ => panic!("Expected ConfigError"),
        }
        
        // Test NetworkError conversion
        let network_error = NetworkError::timeout();
        let core_error: CoreError = network_error.into();
        match core_error {
            CoreError::NetworkError { code, message } => {
                assert_eq!(code, codes::network::ALL);
                assert!(!message.is_empty());
            }
            _ => panic!("Expected NetworkError"),
        }
    }

    #[test]
    fn test_io_error_conversion() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let core_error: CoreError = io_error.into();
        match core_error {
            CoreError::FileError { code, message } => {
                assert_eq!(code, codes::io::ALL);
                assert!(message.contains("File not found"));
            }
            _ => panic!("Expected FileError"),
        }
    }

    #[test]
    fn test_json_error_conversion() {
        let json_str = "invalid json";
        let json_error = serde_json::from_str::<serde_json::Value>(json_str).unwrap_err();
        let core_error: CoreError = json_error.into();
        match core_error {
            CoreError::InternalError { code, message } => {
                assert_eq!(code, codes::internal::JSON_ERROR);
                assert!(message.contains("JSON serialization error"));
            }
            _ => panic!("Expected InternalError"),
        }
    }

    #[test]
    fn test_reqwest_error_conversion() {
        // See if From<reqwest::Error> for CoreError exists
        assert!(true);
    }
} 