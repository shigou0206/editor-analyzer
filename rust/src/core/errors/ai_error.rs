use thiserror::Error;

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

    #[test]
    fn test_ai_error() {
        let api_error = AiError::ApiCallFailed {
            code: "api_call_failed",
            message: "API call failed".to_string(),
        };
        assert!(api_error.to_string().contains("API call failed"));
        assert_eq!(api_error.code(), "api_call_failed");
        
        let auth_error = AiError::AuthenticationFailed {
            code: "authentication_failed",
            message: "Invalid token".to_string(),
        };
        assert!(auth_error.to_string().contains("Authentication failed"));
        assert!(auth_error.to_string().contains("Invalid token"));
        assert_eq!(auth_error.code(), "authentication_failed");
        
        let quota_error = AiError::QuotaExceeded {
            code: "quota_exceeded",
            message: "Rate limit exceeded".to_string(),
        };
        assert!(quota_error.to_string().contains("Quota exceeded"));
        assert!(quota_error.to_string().contains("Rate limit exceeded"));
        assert_eq!(quota_error.code(), "quota_exceeded");
        
        let parse_error = AiError::ResponseParseFailed {
            code: "response_parse_failed",
            message: "Invalid JSON".to_string(),
        };
        assert!(parse_error.to_string().contains("Response parse failed"));
        assert!(parse_error.to_string().contains("Invalid JSON"));
        assert_eq!(parse_error.code(), "response_parse_failed");
        
        let timeout_error = AiError::Timeout {
            code: "timeout",
            message: "Request timeout".to_string(),
        };
        assert!(timeout_error.to_string().contains("Timeout"));
        assert!(timeout_error.to_string().contains("Request timeout"));
        assert_eq!(timeout_error.code(), "timeout");
        
        let streaming_error = AiError::StreamingError {
            code: "streaming_error",
            message: "Stream interrupted".to_string(),
        };
        assert!(streaming_error.to_string().contains("Streaming error"));
        assert!(streaming_error.to_string().contains("Stream interrupted"));
        assert_eq!(streaming_error.code(), "streaming_error");
    }
} 