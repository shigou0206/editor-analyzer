use thiserror::Error;
use crate::core::errors::codes;

/// LSP error
#[derive(Error, Debug, PartialEq, Eq)]
pub enum LspError {
    #[error("Connection failed: {message}")]
    ConnectionFailed { code: &'static str, message: String },
    
    #[error("Initialization failed: {message}")]
    InitializationFailed { code: &'static str, message: String },
    
    #[error("Request failed: {message}")]
    RequestFailed { code: &'static str, message: String },
    
    #[error("Response error: {message}")]
    ResponseError { code: &'static str, message: String },
    
    #[error("Language server error: {message}")]
    ServerError { code: &'static str, message: String },
}

impl LspError {
    /// 构造函数，自动填充 code
    pub fn connection_failed(message: String) -> Self {
        LspError::ConnectionFailed {
            code: codes::lsp::CONNECTION_FAILED,
            message,
        }
    }
    pub fn initialization_failed(message: String) -> Self {
        LspError::InitializationFailed {
            code: codes::lsp::ALL,
            message,
        }
    }
    pub fn request_failed(message: String) -> Self {
        LspError::RequestFailed {
            code: codes::lsp::INVALID_REQUEST,
            message,
        }
    }
    pub fn response_error(message: String) -> Self {
        LspError::ResponseError {
            code: codes::lsp::ALL,
            message,
        }
    }
    pub fn server_error(message: String) -> Self {
        LspError::ServerError {
            code: codes::lsp::ALL,
            message,
        }
    }
    /// Get the error code
    pub fn code(&self) -> &'static str {
        match self {
            LspError::ConnectionFailed { code, .. } => code,
            LspError::InitializationFailed { code, .. } => code,
            LspError::RequestFailed { code, .. } => code,
            LspError::ResponseError { code, .. } => code,
            LspError::ServerError { code, .. } => code,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::errors::codes;

    #[test]
    fn test_lsp_error() {
        let connection_error = LspError::connection_failed("Connection refused".to_string());
        assert!(connection_error.to_string().contains("Connection failed"));
        assert!(connection_error.to_string().contains("Connection refused"));
        assert_eq!(connection_error.code(), codes::lsp::CONNECTION_FAILED);
        
        let init_error = LspError::initialization_failed("Init failed".to_string());
        assert!(init_error.to_string().contains("Initialization failed"));
        assert!(init_error.to_string().contains("Init failed"));
        assert_eq!(init_error.code(), codes::lsp::ALL);
        
        let request_error = LspError::request_failed("Request failed".to_string());
        assert!(request_error.to_string().contains("Request failed"));
        assert_eq!(request_error.code(), codes::lsp::INVALID_REQUEST);
        
        let response_error = LspError::response_error("Invalid response".to_string());
        assert!(response_error.to_string().contains("Response error"));
        assert!(response_error.to_string().contains("Invalid response"));
        assert_eq!(response_error.code(), codes::lsp::ALL);
        
        let server_error = LspError::server_error("Server error".to_string());
        assert!(server_error.to_string().contains("Language server error"));
        assert!(server_error.to_string().contains("Server error"));
        assert_eq!(server_error.code(), codes::lsp::ALL);
    }
} 