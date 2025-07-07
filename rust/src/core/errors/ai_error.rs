use thiserror::Error;
use crate::core::errors::codes;

/// AI service error
#[derive(Error, Debug, PartialEq, Eq)]
pub enum AiError {
    #[error("API call failed: {message}")]
    ApiCallFailed { code: &'static str, message: String },
    
    #[error("Authentication failed: {message}")]
    AuthenticationFailed { code: &'static str, message: String },
    
    #[error("Quota exceeded: {message}")]
    QuotaExceeded { code: &'static str, message: String },
    
    #[error("Response parse failed: {message}")]
    ResponseParseFailed { code: &'static str, message: String },
    
    #[error("Timeout: {message}")]
    Timeout { code: &'static str, message: String },
    
    #[error("Streaming error: {message}")]
    StreamingError { code: &'static str, message: String },
}

impl AiError {
    /// 构造函数，自动填充 code
    pub fn api_call_failed(message: String) -> Self {
        AiError::ApiCallFailed {
            code: codes::ai::API_CALL_FAILED,
            message,
        }
    }
    pub fn authentication_failed(message: String) -> Self {
        AiError::AuthenticationFailed {
            code: codes::ai::ALL,
            message,
        }
    }
    pub fn quota_exceeded(message: String) -> Self {
        AiError::QuotaExceeded {
            code: codes::ai::ALL,
            message,
        }
    }
    pub fn response_parse_failed(message: String) -> Self {
        AiError::ResponseParseFailed {
            code: codes::ai::INVALID_RESPONSE,
            message,
        }
    }
    pub fn timeout(message: String) -> Self {
        AiError::Timeout {
            code: codes::ai::ALL,
            message,
        }
    }
    pub fn streaming_error(message: String) -> Self {
        AiError::StreamingError {
            code: codes::ai::ALL,
            message,
        }
    }
    /// Get the error code
    pub fn code(&self) -> &'static str {
        match self {
            AiError::ApiCallFailed { code, .. } => code,
            AiError::AuthenticationFailed { code, .. } => code,
            AiError::QuotaExceeded { code, .. } => code,
            AiError::ResponseParseFailed { code, .. } => code,
            AiError::Timeout { code, .. } => code,
            AiError::StreamingError { code, .. } => code,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::errors::codes;

    #[test]
    fn test_ai_error() {
        let api_error = AiError::api_call_failed("API call failed".to_string());
        assert!(api_error.to_string().contains("API call failed"));
        assert_eq!(api_error.code(), codes::ai::API_CALL_FAILED);
        
        let auth_error = AiError::authentication_failed("Invalid token".to_string());
        assert!(auth_error.to_string().contains("Authentication failed"));
        assert!(auth_error.to_string().contains("Invalid token"));
        assert_eq!(auth_error.code(), codes::ai::ALL);
        
        let quota_error = AiError::quota_exceeded("Rate limit exceeded".to_string());
        assert!(quota_error.to_string().contains("Quota exceeded"));
        assert!(quota_error.to_string().contains("Rate limit exceeded"));
        assert_eq!(quota_error.code(), codes::ai::ALL);
        
        let parse_error = AiError::response_parse_failed("Invalid JSON".to_string());
        assert!(parse_error.to_string().contains("Response parse failed"));
        assert!(parse_error.to_string().contains("Invalid JSON"));
        assert_eq!(parse_error.code(), codes::ai::INVALID_RESPONSE);
        
        let timeout_error = AiError::timeout("Request timeout".to_string());
        assert!(timeout_error.to_string().contains("Timeout"));
        assert!(timeout_error.to_string().contains("Request timeout"));
        assert_eq!(timeout_error.code(), codes::ai::ALL);
        
        let streaming_error = AiError::streaming_error("Stream interrupted".to_string());
        assert!(streaming_error.to_string().contains("Streaming error"));
        assert!(streaming_error.to_string().contains("Stream interrupted"));
        assert_eq!(streaming_error.code(), codes::ai::ALL);
    }
} 