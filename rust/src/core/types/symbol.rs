use serde::{Deserialize, Serialize};
use super::span::Span;
use super::document::FileId;

/// 符号类型
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SymbolKind {
    Function,
    Class,
    Variable,
    Module,
    Import,
    Comment,
    String,
    Number,
    Keyword,
    Operator,
    Unknown,
}

/// 符号信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Symbol {
    pub id: String,
    pub name: String,
    pub kind: SymbolKind,
    pub span: Span,
    pub file_id: FileId,
    pub scope_id: Option<String>,
}

impl Symbol {
    pub fn new(id: String, name: String, kind: SymbolKind, span: Span, file_id: FileId) -> Self {
        Self {
            id,
            name,
            kind,
            span,
            file_id,
            scope_id: None,
        }
    }

    pub fn with_scope(mut self, scope_id: String) -> Self {
        self.scope_id = Some(scope_id);
        self
    }
}

/// 引用信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reference {
    pub symbol_id: String,
    pub span: Span,
    pub file_id: FileId,
    pub is_definition: bool,
}

impl Reference {
    pub fn new(symbol_id: String, span: Span, file_id: FileId, is_definition: bool) -> Self {
        Self {
            symbol_id,
            span,
            file_id,
            is_definition,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol() {
        let file_id = FileId::new("test.py");
        let span = Span::new(0, 10);
        let symbol = Symbol::new(
            "func1".to_string(),
            "func1".to_string(),
            SymbolKind::Function,
            span,
            file_id.clone(),
        );
        
        assert_eq!(symbol.id, "func1");
        assert_eq!(symbol.name, "func1");
        assert_eq!(symbol.kind, SymbolKind::Function);
        assert_eq!(symbol.span, span);
        assert_eq!(symbol.file_id, file_id);
        assert!(symbol.scope_id.is_none());
        
        let symbol_with_scope = symbol.with_scope("global".to_string());
        assert_eq!(symbol_with_scope.scope_id, Some("global".to_string()));
    }

    #[test]
    fn test_reference() {
        let file_id = FileId::new("test.py");
        let span = Span::new(0, 10);
        let reference = Reference::new(
            "func1".to_string(),
            span,
            file_id.clone(),
            true,
        );
        
        assert_eq!(reference.symbol_id, "func1");
        assert_eq!(reference.span, span);
        assert_eq!(reference.file_id, file_id);
        assert!(reference.is_definition);
    }
} 