use rust::core::{common, traits, types};
use rust::traits::Cache;

#[test]
fn test_memory_cache_integration() {
    let cache: common::MemoryCache<String, i32> = common::MemoryCache::new();
    assert!(cache.set("integration_key".to_string(), 123).is_ok());
    assert_eq!(cache.get(&"integration_key".to_string()).unwrap(), Some(123));
}

#[test]
fn test_symbol_table_integration() {
    let mut symbol_table = traits::SymbolTable::new();
    let file_id = types::FileId::new("integration.py");
    let symbol = types::Symbol::new(
        "func".to_string(),
        "func".to_string(),
        types::SymbolKind::Function,
        types::Span::new(0, 10),
        file_id,
    );
    symbol_table.add_symbol(symbol);
    assert_eq!(symbol_table.symbols.len(), 1);
} 