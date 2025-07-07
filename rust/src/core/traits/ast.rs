use crate::core::types::*;

/// AST Node interface
pub trait AstNode {
    fn kind(&self) -> &str;
    fn text(&self) -> &str;
    fn span(&self) -> Span;
    fn children(&self) -> Vec<Box<dyn AstNode>>;
    fn parent(&self) -> Option<Box<dyn AstNode>>;
}

/// AST abstraction interface
pub trait Ast {
    type Node: AstNode;
    type Error;
    
    fn root_node(&self) -> &Self::Node;
    fn node_text<'a>(&self, node: &'a Self::Node) -> &'a str;
    fn node_kind<'a>(&self, node: &'a Self::Node) -> &'a str;
    fn node_span(&self, node: &Self::Node) -> Span;
    fn node_children(&self, node: &Self::Node) -> Vec<Self::Node>;
    fn get_syntax_errors(&self) -> Vec<SyntaxError>;
}

/// AST Visitor pattern
pub trait AstVisitor {
    type Ast: Ast;
    type Result;
    
    fn visit_node(&mut self, node: &<Self::Ast as Ast>::Node) -> Self::Result;
    fn visit_children(&mut self, node: &<Self::Ast as Ast>::Node) -> Self::Result;
}

/// Syntax error
#[derive(Debug, Clone)]
pub struct SyntaxError {
    pub message: String,
    pub span: Span,
    pub severity: Severity,
}

impl SyntaxError {
    pub fn new(message: String, span: Span, severity: Severity) -> Self {
        Self {
            message,
            span,
            severity,
        }
    }
}

/// Code parser interface
pub trait CodeParser {
    type Ast: Ast;
    type Error;
    
    fn parse(&self, source: &str, language: Language) -> Result<Self::Ast, Self::Error>;
    fn parse_incremental(&self, source: &str, old_ast: &Self::Ast) -> Result<Self::Ast, Self::Error>;
    fn get_syntax_errors(&self, ast: &Self::Ast) -> Vec<SyntaxError>;
    fn supports_language(&self, language: &Language) -> bool;
}

/// Incremental parser interface
pub trait IncrementalParser: CodeParser {
    fn compute_diff(&self, old_source: &str, new_source: &str) -> Diff;
    fn apply_diff(&self, ast: &Self::Ast, diff: &Diff) -> Result<Self::Ast, Self::Error>;
}

/// Diff information
#[derive(Debug, Clone)]
pub struct Diff {
    pub changes: Vec<Change>,
}

#[derive(Debug, Clone)]
pub enum Change {
    Insert { position: usize, text: String },
    Delete { start: usize, end: usize },
    Replace { start: usize, end: usize, text: String },
} 