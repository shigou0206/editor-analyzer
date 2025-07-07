use crate::core::types::*;
use std::collections::HashMap;
use std::sync::Arc;

/// Symbol handle trait - 提供符号的统一接口
pub trait SymbolHandle: Send + Sync {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn kind(&self) -> &SymbolKind;
    fn span(&self) -> &Span;
    fn scope_id(&self) -> Option<&str>;
    fn is_exported(&self) -> bool;
    fn is_mutable(&self) -> bool;
}

/// Scope handle trait - 提供作用域的统一接口
pub trait ScopeHandle: Send + Sync {
    fn id(&self) -> &str;
    fn parent_id(&self) -> Option<&str>;
    fn span(&self) -> &Span;
    fn symbol_count(&self) -> usize;
    fn contains_symbol(&self, name: &str) -> bool;
}

/// Symbol table handle trait - 提供符号表的统一接口
pub trait SymbolTableHandle: Send + Sync {
    type Symbol: SymbolHandle;
    type Scope: ScopeHandle;
    
    fn symbol_count(&self) -> usize;
    fn scope_count(&self) -> usize;
    fn find_symbol(&self, name: &str) -> Option<&Self::Symbol>;
    fn find_scope(&self, id: &str) -> Option<&Self::Scope>;
    fn current_scope(&self) -> Option<&Self::Scope>;
    fn scope_chain(&self) -> Vec<&str>;
}

/// Scope information
#[derive(Debug, Clone)]
pub struct Scope {
    pub id: Arc<str>,
    pub parent_id: Option<Arc<str>>,
    pub symbols: HashMap<Arc<str>, Arc<str>>, // name -> symbol_id
    pub span: Span,
}

impl Scope {
    pub fn new(id: impl Into<Arc<str>>, span: Span) -> Self {
        Self {
            id: id.into(),
            parent_id: None,
            symbols: HashMap::new(),
            span,
        }
    }

    pub fn with_parent(mut self, parent_id: impl Into<Arc<str>>) -> Self {
        self.parent_id = Some(parent_id.into());
        self
    }

    pub fn add_symbol(&mut self, name: impl Into<Arc<str>>, symbol_id: impl Into<Arc<str>>) {
        self.symbols.insert(name.into(), symbol_id.into());
    }
}

impl ScopeHandle for Scope {
    fn id(&self) -> &str {
        &self.id
    }
    
    fn parent_id(&self) -> Option<&str> {
        self.parent_id.as_deref()
    }
    
    fn span(&self) -> &Span {
        &self.span
    }
    
    fn symbol_count(&self) -> usize {
        self.symbols.len()
    }
    
    fn contains_symbol(&self, name: &str) -> bool {
        let name_arc: Arc<str> = name.into();
        self.symbols.contains_key(&name_arc)
    }
}

/// Symbol table
#[derive(Debug, Clone)]
pub struct SymbolTable {
    pub symbols: HashMap<Arc<str>, Symbol>,
    pub scopes: HashMap<Arc<str>, Scope>,
    pub scope_chain: Vec<Arc<str>>,
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
        self.symbols.insert(symbol.id.clone().into(), symbol);
    }

    pub fn add_scope(&mut self, scope: Scope) {
        self.scopes.insert(scope.id.clone(), scope);
    }

    pub fn push_scope(&mut self, scope_id: impl Into<Arc<str>>) {
        self.scope_chain.push(scope_id.into());
    }

    pub fn pop_scope(&mut self) -> Option<Arc<str>> {
        self.scope_chain.pop()
    }

    pub fn current_scope(&self) -> Option<&Scope> {
        self.scope_chain.last().and_then(|id| self.scopes.get(id))
    }

    pub fn find_symbol(&self, name: &str) -> Option<&Symbol> {
        // Search from current scope upwards
        for scope_id in self.scope_chain.iter().rev() {
            if let Some(scope) = self.scopes.get(scope_id) {
                // Convert &str to Arc<str> for HashMap lookup
                let name_arc: Arc<str> = name.into();
                if let Some(symbol_id) = scope.symbols.get(&name_arc) {
                    return self.symbols.get(symbol_id);
                }
            }
        }
        None
    }
}

impl SymbolTableHandle for SymbolTable {
    type Symbol = Symbol;
    type Scope = Scope;
    
    fn symbol_count(&self) -> usize {
        self.symbols.len()
    }
    
    fn scope_count(&self) -> usize {
        self.scopes.len()
    }
    
    fn find_symbol(&self, name: &str) -> Option<&Self::Symbol> {
        self.find_symbol(name)
    }
    
    fn find_scope(&self, id: &str) -> Option<&Self::Scope> {
        let id_arc: Arc<str> = id.into();
        self.scopes.get(&id_arc)
    }
    
    fn current_scope(&self) -> Option<&Self::Scope> {
        self.current_scope()
    }
    
    fn scope_chain(&self) -> Vec<&str> {
        self.scope_chain.iter().map(|s| s.as_ref()).collect()
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
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