use crate::core::traits::ast::Ast;
use crate::core::traits::symbol::SemanticAnalyzer;
use crate::core::types::*;

pub trait DiagnosticProvider<A: Ast, S: SemanticAnalyzer<A>> {
    type Diagnostic;
    type Error;
    
    fn analyze(&self, ast: &A, analyzer: &S) -> Result<Vec<Self::Diagnostic>, Self::Error>;
    fn get_quick_fixes(&self, diagnostic: &Self::Diagnostic) -> Vec<FixCommand>;
    fn get_suggestions(&self, diagnostic: &Self::Diagnostic) -> Vec<String>;
} 