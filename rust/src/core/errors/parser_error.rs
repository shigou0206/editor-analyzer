use thiserror::Error;
use crate::core::errors::codes;

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
    /// 构造函数，自动填充 code
    pub fn syntax_error(message: String, span: crate::core::types::Span) -> Self {
        ParserError::SyntaxError {
            code: codes::parser::SYNTAX_ERROR,
            message,
            span,
        }
    }
    pub fn unsupported_language(language: String) -> Self {
        ParserError::UnsupportedLanguage {
            code: codes::parser::UNSUPPORTED_LANGUAGE,
            language,
        }
    }
    pub fn parse_failed(message: String) -> Self {
        ParserError::ParseFailed {
            code: codes::parser::ALL,
            message,
        }
    }
    pub fn incremental_parse_error(message: String) -> Self {
        ParserError::IncrementalParseError {
            code: codes::parser::ALL,
            message,
        }
    }
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
    use crate::core::errors::codes;

    #[test]
    fn test_parser_error() {
        let span = crate::core::types::Span::new(0, 10);
        let syntax_error = ParserError::syntax_error("Unexpected token".to_string(), span);
        assert!(syntax_error.to_string().contains("Syntax error"));
        assert!(syntax_error.to_string().contains("Unexpected token"));
        assert_eq!(syntax_error.code(), codes::parser::SYNTAX_ERROR);
        
        let unsupported_error = ParserError::unsupported_language("Unknown".to_string());
        assert!(unsupported_error.to_string().contains("Unsupported language"));
        assert!(unsupported_error.to_string().contains("Unknown"));
        assert_eq!(unsupported_error.code(), codes::parser::UNSUPPORTED_LANGUAGE);
        
        let parse_failed = ParserError::parse_failed("Parse failed".to_string());
        assert!(parse_failed.to_string().contains("Parse failed"));
        assert_eq!(parse_failed.code(), codes::parser::ALL);
        
        let incremental_error = ParserError::incremental_parse_error("Incremental parse error".to_string());
        assert!(incremental_error.to_string().contains("Incremental parse error"));
        assert_eq!(incremental_error.code(), codes::parser::ALL);
    }
} 