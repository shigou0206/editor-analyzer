use thiserror::Error;

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

    #[test]
    fn test_semantic_error() {
        let symbol_error = SemanticError::SymbolNotFound {
            code: "symbol_not_found",
            symbol_name: "test_func".to_string(),
        };
        assert!(symbol_error.to_string().contains("Symbol not found"));
        assert!(symbol_error.to_string().contains("test_func"));
        assert_eq!(symbol_error.code(), "symbol_not_found");
        
        let scope_error = SemanticError::ScopeError {
            code: "scope_error",
            message: "Invalid scope".to_string(),
        };
        assert!(scope_error.to_string().contains("Scope error"));
        assert!(scope_error.to_string().contains("Invalid scope"));
        assert_eq!(scope_error.code(), "scope_error");
        
        let type_error = SemanticError::TypeError {
            code: "type_error",
            message: "Type mismatch".to_string(),
        };
        assert!(type_error.to_string().contains("Type error"));
        assert!(type_error.to_string().contains("Type mismatch"));
        assert_eq!(type_error.code(), "type_error");
        
        let circular_error = SemanticError::CircularDependency {
            code: "circular_dependency",
            message: "Circular import".to_string(),
        };
        assert!(circular_error.to_string().contains("Circular dependency"));
        assert!(circular_error.to_string().contains("Circular import"));
        assert_eq!(circular_error.code(), "circular_dependency");
    }
} 