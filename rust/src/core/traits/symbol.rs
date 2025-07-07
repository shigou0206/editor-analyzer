use crate::core::types::*;
use std::collections::HashMap;

/// Scope information
#[derive(Debug, Clone)]
pub struct Scope {
    pub id: String,
    pub parent_id: Option<String>,
    pub symbols: HashMap<String, String>, // name -> symbol_id
    pub span: Span,
}

impl Scope {
    pub fn new(id: String, span: Span) -> Self {
        Self {
            id,
            parent_id: None,
            symbols: HashMap::new(),
            span,
        }
    }

    pub fn with_parent(mut self, parent_id: String) -> Self {
        self.parent_id = Some(parent_id);
        self
    }

    pub fn add_symbol(&mut self, name: String, symbol_id: String) {
        self.symbols.insert(name, symbol_id);
    }
}

/// Symbol table
#[derive(Debug, Clone)]
pub struct SymbolTable {
    pub symbols: HashMap<String, Symbol>,
    pub scopes: HashMap<String, Scope>,
    pub scope_chain: Vec<String>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            scopes: HashMap::new(),
            scope_chain: Vec::new(),
        }
    }

    pub fn add_symbol(&mut self, symbol: Symbol) {
        self.symbols.insert(symbol.id.clone(), symbol);
    }

    pub fn add_scope(&mut self, scope: Scope) {
        self.scopes.insert(scope.id.clone(), scope);
    }

    pub fn push_scope(&mut self, scope_id: String) {
        self.scope_chain.push(scope_id);
    }

    pub fn pop_scope(&mut self) -> Option<String> {
        self.scope_chain.pop()
    }

    pub fn current_scope(&self) -> Option<&Scope> {
        self.scope_chain.last().and_then(|id| self.scopes.get(id))
    }

    pub fn find_symbol(&self, name: &str) -> Option<&Symbol> {
        // Search from current scope upwards
        for scope_id in self.scope_chain.iter().rev() {
            if let Some(scope) = self.scopes.get(scope_id) {
                if let Some(symbol_id) = scope.symbols.get(name) {
                    return self.symbols.get(symbol_id);
                }
            }
        }
        None
    }
}

/// Semantic analyzer interface
pub trait SemanticAnalyzer<A: crate::core::traits::ast::Ast> {
    type Context;
    type Error;
    
    fn analyze(&self, ast: &A) -> Result<Self::Context, Self::Error>;
    fn get_symbols(&self, context: &Self::Context) -> Vec<Symbol>;
    fn get_references(&self, context: &Self::Context, symbol: &Symbol) -> Vec<Reference>;
    fn get_symbol_table(&self, context: &Self::Context) -> &SymbolTable;
    fn get_scope_chain(&self, context: &Self::Context) -> Vec<&Scope>;
}