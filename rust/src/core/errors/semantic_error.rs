use thiserror::Error;
use crate::core::errors::codes;

/// Semantic analysis error
#[derive(Error, Debug, PartialEq, Eq)]
pub enum SemanticError {
    #[error("Symbol not found: {symbol_name}")]
    SymbolNotFound { code: &'static str, symbol_name: String },
    
    #[error("Scope error: {message}")]
    ScopeError { code: &'static str, message: String },
    
    #[error("Type error: {message}")]
    TypeError { code: &'static str, message: String },
    
    #[error("Circular dependency: {message}")]
    CircularDependency { code: &'static str, message: String },
}

impl SemanticError {
    /// 构造函数，自动填充 code
    pub fn symbol_not_found(symbol_name: String) -> Self {
        SemanticError::SymbolNotFound {
            code: codes::semantic::SYMBOL_NOT_FOUND,
            symbol_name,
        }
    }
    pub fn scope_error(message: String) -> Self {
        SemanticError::ScopeError {
            code: codes::semantic::ALL,
            message,
        }
    }
    pub fn type_error(message: String) -> Self {
        SemanticError::TypeError {
            code: codes::semantic::TYPE_MISMATCH,
            message,
        }
    }
    pub fn circular_dependency(message: String) -> Self {
        SemanticError::CircularDependency {
            code: codes::semantic::ALL,
            message,
        }
    }
    /// Get the error code
    pub fn code(&self) -> &'static str {
        match self {
            SemanticError::SymbolNotFound { code, .. } => code,
            SemanticError::ScopeError { code, .. } => code,
            SemanticError::TypeError { code, .. } => code,
            SemanticError::CircularDependency { code, .. } => code,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::errors::codes;

    #[test]
    fn test_semantic_error() {
        let symbol_error = SemanticError::symbol_not_found("test_func".to_string());
        assert!(symbol_error.to_string().contains("Symbol not found"));
        assert!(symbol_error.to_string().contains("test_func"));
        assert_eq!(symbol_error.code(), codes::semantic::SYMBOL_NOT_FOUND);
        
        let scope_error = SemanticError::scope_error("Invalid scope".to_string());
        assert!(scope_error.to_string().contains("Scope error"));
        assert!(scope_error.to_string().contains("Invalid scope"));
        assert_eq!(scope_error.code(), codes::semantic::ALL);
        
        let type_error = SemanticError::type_error("Type mismatch".to_string());
        assert!(type_error.to_string().contains("Type error"));
        assert!(type_error.to_string().contains("Type mismatch"));
        assert_eq!(type_error.code(), codes::semantic::TYPE_MISMATCH);
        
        let circular_error = SemanticError::circular_dependency("Circular import".to_string());
        assert!(circular_error.to_string().contains("Circular dependency"));
        assert!(circular_error.to_string().contains("Circular import"));
        assert_eq!(circular_error.code(), codes::semantic::ALL);
    }
} 