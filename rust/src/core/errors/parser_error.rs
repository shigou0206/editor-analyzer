use thiserror::Error;

/// Parser error
#[derive(Error, Debug, PartialEq, Eq)]
pub enum ParserError {
    #[error("Syntax error: {message} at {span:?}")]
    SyntaxError { code: &'static str, message: String, span: crate::core::types::Span },
    
    #[error("Unsupported language: {language}")]
    UnsupportedLanguage { code: &'static str, language: String },
    
    #[error("Parse failed: {message}")]
    ParseFailed { code: &'static str, message: String },
    
    #[error("Incremental parse error: {message}")]
    IncrementalParseError { code: &'static str, message: String },
}

impl ParserError {
    /// Get the error code
    pub fn code(&self) -> &'static str {
        match self {
            ParserError::SyntaxError { code, .. } => code,
            ParserError::UnsupportedLanguage { code, .. } => code,
            ParserError::ParseFailed { code, .. } => code,
            ParserError::IncrementalParseError { code, .. } => code,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_error() {
        let span = crate::core::types::Span::new(0, 10);
        let syntax_error = ParserError::SyntaxError {
            code: "syntax_error",
            message: "Unexpected token".to_string(),
            span,
        };
        assert!(syntax_error.to_string().contains("Syntax error"));
        assert!(syntax_error.to_string().contains("Unexpected token"));
        assert_eq!(syntax_error.code(), "syntax_error");
        
        let unsupported_error = ParserError::UnsupportedLanguage {
            code: "unsupported_language",
            language: "Unknown".to_string(),
        };
        assert!(unsupported_error.to_string().contains("Unsupported language"));
        assert!(unsupported_error.to_string().contains("Unknown"));
        assert_eq!(unsupported_error.code(), "unsupported_language");
        
        let parse_failed = ParserError::ParseFailed {
            code: "parse_failed",
            message: "Parse failed".to_string(),
        };
        assert!(parse_failed.to_string().contains("Parse failed"));
        assert_eq!(parse_failed.code(), "parse_failed");
        
        let incremental_error = ParserError::IncrementalParseError {
            code: "incremental_parse_error",
            message: "Incremental parse error".to_string(),
        };
        assert!(incremental_error.to_string().contains("Incremental parse error"));
        assert_eq!(incremental_error.code(), "incremental_parse_error");
    }
} 