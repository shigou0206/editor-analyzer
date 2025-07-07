use thiserror::Error;

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

    #[test]
    fn test_lsp_error() {
        let connection_error = LspError::ConnectionFailed {
            code: "connection_failed",
            message: "Connection refused".to_string(),
        };
        assert!(connection_error.to_string().contains("Connection failed"));
        assert!(connection_error.to_string().contains("Connection refused"));
        assert_eq!(connection_error.code(), "connection_failed");
        
        let init_error = LspError::InitializationFailed {
            code: "initialization_failed",
            message: "Init failed".to_string(),
        };
        assert!(init_error.to_string().contains("Initialization failed"));
        assert!(init_error.to_string().contains("Init failed"));
        assert_eq!(init_error.code(), "initialization_failed");
        
        let request_error = LspError::RequestFailed {
            code: "request_failed",
            message: "Request failed".to_string(),
        };
        assert!(request_error.to_string().contains("Request failed"));
        assert_eq!(request_error.code(), "request_failed");
        
        let response_error = LspError::ResponseError {
            code: "response_error",
            message: "Invalid response".to_string(),
        };
        assert!(response_error.to_string().contains("Response error"));
        assert!(response_error.to_string().contains("Invalid response"));
        assert_eq!(response_error.code(), "response_error");
        
        let server_error = LspError::ServerError {
            code: "server_error",
            message: "Server error".to_string(),
        };
        assert!(server_error.to_string().contains("Language server error"));
        assert!(server_error.to_string().contains("Server error"));
        assert_eq!(server_error.code(), "server_error");
    }
} 