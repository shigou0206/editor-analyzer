// 核心抽象层
pub mod core;
pub use core::{traits, types, errors, common, utils};

// 代码解析模块
pub mod parsers;
// TODO: Export when modules have content
// pub use parsers::tree_sitter;

// 代码分析模块
pub mod analysis;
// TODO: Export when modules have content
// pub use analysis::semantic;

// AI 交互模块
pub mod ai;
// TODO: Export when modules have content
// pub use ai::providers;

// LSP 支持模块
pub mod lsp;
// TODO: Export when modules have content
// pub use lsp::client;

// 平台桥接层
pub mod bridge;
// TODO: Export when modules have content
// pub use bridge::flutter;

// Re-export for flutter_rust_bridge
pub use flutter_rust_bridge::*;

// Re-export main types for convenience
pub use core::*;

/// Initialize the core abstraction layer
pub fn initialize() -> String {
    // Test basic types
    let file_id = types::FileId::new("test.py");
    let span = types::Span::new(0, 10);
    let _position = types::Position::new(0, 0);
    let language = types::Language::Python;
    
    // Test symbol table
    let mut symbol_table = traits::SymbolTable::new();
    let symbol = types::Symbol::new(
        "test_func".to_string(),
        "test_func".to_string(),
        types::SymbolKind::Function,
        span,
        file_id.clone(),
    );
    symbol_table.add_symbol(symbol);
    
    // Test cache
    let cache: common::MemoryCache<String, i32> = common::MemoryCache::new();
    let _ = cache.set("test_key".to_string(), 42);
    
    // Test config
    let config = common::MemoryConfig::new();
    let _ = config.set("test_config", "test_value");
    
    // Test utils
    let text = "Hello\nWorld";
    let _pos = utils::TextUtils::offset_to_position(text, 6);
    let _hash = utils::HashUtils::hash_text(text);
    let _valid = utils::ValidationUtils::validate_span(&span, text.len());
    
    format!(
        "Core abstraction layer initialized successfully\n\
         File: {}\n\
         Language: {:?}\n\
         Symbol count: {}\n\
         Cache size: {}\n\
         Config has test_config: {}",
        file_id.0,
        language,
        symbol_table.symbols.len(),
        cache.len(),
        config.has("test_config")
    )
}

/// Get supported languages
pub fn get_supported_languages() -> Vec<String> {
    vec![
        "Python".to_string(),
        "JSON".to_string(),
        "Rust".to_string(),
        "JavaScript".to_string(),
        "TypeScript".to_string(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::traits::{Cache, Config, ObjectPool};

    #[test]
    fn test_core_abstraction_function() {
        let result = initialize();
        assert!(result.contains("Core abstraction layer initialized successfully"));
        assert!(result.contains("test.py"));
        assert!(result.contains("Python"));
    }

    #[test]
    fn test_get_supported_languages() {
        let languages = get_supported_languages();
        assert!(languages.contains(&"Python".to_string()));
        assert!(languages.contains(&"JSON".to_string()));
        assert!(languages.contains(&"Rust".to_string()));
        assert!(languages.contains(&"JavaScript".to_string()));
        assert!(languages.contains(&"TypeScript".to_string()));
        assert_eq!(languages.len(), 5);
    }

    #[test]
    fn test_core_types_integration() {
        // Test FileId
        let file_id = types::FileId::new("test.py");
        assert_eq!(file_id.0, "test.py");

        // Test Span
        let span = types::Span::new(0, 10);
        assert_eq!(span.start, 0);
        assert_eq!(span.end, 10);
        assert_eq!(span.len(), 10);

        // Test Position
        let pos = types::Position::new(1, 5);
        assert_eq!(pos.line, 1);
        assert_eq!(pos.column, 5);

        // Test Language
        let language = types::Language::Python;
        assert_eq!(language, types::Language::from_extension("py"));
    }

    #[test]
    fn test_ai_context_integration() {
        let file_id = types::FileId::new("test.py");
        let source_code = types::SourceCode::new(
            "def hello(): pass".to_string(),
            types::Language::Python,
            file_id.clone(),
        );
        let file_context = types::FileContext::new(file_id.clone());
        
        let context = traits::ConcreteAiContext::new(source_code.clone(), file_context.clone());
        
        assert_eq!(context.source_code, source_code);
        assert_eq!(context.file_context, file_context);
        assert!(!context.trace_id.is_empty());
        assert!(context.symbols.is_empty());
        assert!(context.diagnostics.is_empty());
    }

    #[test]
    fn test_ai_request_response_integration() {
        let file_id = types::FileId::new("test.py");
        let source_code = types::SourceCode::new(
            "def hello(): pass".to_string(),
            types::Language::Python,
            file_id.clone(),
        );
        let file_context = types::FileContext::new(file_id.clone());
        let ai_context = traits::ConcreteAiContext::new(source_code, file_context);
        
        let request_type = "code_generation".to_string();
        let request = traits::ConcreteAiRequest::new(request_type, ai_context);
        
        assert_eq!(request.request_type, "code_generation");
        
        let response = traits::ConcreteAiResponse::new(
            "Generated code".to_string(),
            "trace_123".to_string(),
        );
        
        assert_eq!(response.content, "Generated code");
        assert_eq!(response.trace_id, "trace_123");
    }

    #[test]
    fn test_diagnostic_integration() {
        let span = types::Span::new(0, 10);
        let diagnostic = types::Diagnostic::new(
            types::Severity::Error,
            "Test error".to_string(),
            span,
        );
        
        assert_eq!(diagnostic.severity, types::Severity::Error);
        assert_eq!(diagnostic.message, "Test error");
        assert_eq!(diagnostic.span, span);
        assert!(!diagnostic.fixable);
        
        let diagnostic_with_code = diagnostic.with_code("E001".to_string());
        assert_eq!(diagnostic_with_code.code, Some("E001".to_string()));
        
        let diagnostic_fixable = diagnostic_with_code.with_fixable(true);
        assert!(diagnostic_fixable.fixable);
    }

    #[test]
    fn test_symbol_reference_integration() {
        let file_id = types::FileId::new("test.py");
        let span = types::Span::new(0, 10);
        
        let symbol = types::Symbol::new(
            "test_func".to_string(),
            "test_func".to_string(),
            types::SymbolKind::Function,
            span,
            file_id.clone(),
        );
        
        assert_eq!(symbol.name, "test_func");
        assert_eq!(symbol.kind, types::SymbolKind::Function);
        assert_eq!(symbol.span, span);
        assert_eq!(symbol.file_id, file_id);
        
        let reference = types::Reference::new(
            "test_func".to_string(),
            span,
            file_id.clone(),
            true,
        );
        
        assert_eq!(reference.symbol_id, "test_func");
        assert_eq!(reference.span, span);
        assert_eq!(reference.file_id, file_id);
        assert!(reference.is_definition);
    }

    #[test]
    fn test_text_document_integration() {
        let file_id = types::FileId::new("test.py");
        let content = "def hello(): pass";
        let doc = types::TextDocument::new(file_id.clone(), content.to_string(), types::Language::Python);
        
        assert_eq!(doc.file_id, file_id);
        assert_eq!(doc.content, content);
        assert_eq!(doc.language, types::Language::Python);
        assert_eq!(doc.version, 1);
        
        let doc_with_version = doc.with_version(5);
        assert_eq!(doc_with_version.version, 5);
    }

    #[test]
    fn test_language_detection_integration() {
        // Test file extension detection
        assert_eq!(types::Language::from_extension("py"), types::Language::Python);
        assert_eq!(types::Language::from_extension("json"), types::Language::Json);
        assert_eq!(types::Language::from_extension("yaml"), types::Language::Yaml);
        assert_eq!(types::Language::from_extension("yml"), types::Language::Yaml);
        assert_eq!(types::Language::from_extension("md"), types::Language::Markdown);
        assert_eq!(types::Language::from_extension("rs"), types::Language::Rust);
        assert_eq!(types::Language::from_extension("js"), types::Language::JavaScript);
        assert_eq!(types::Language::from_extension("ts"), types::Language::TypeScript);
        assert_eq!(types::Language::from_extension("unknown"), types::Language::Unknown);
        
        // Test filename detection
        assert_eq!(types::Language::from_filename("test.py"), types::Language::Python);
        assert_eq!(types::Language::from_filename("config.json"), types::Language::Json);
        assert_eq!(types::Language::from_filename("README.md"), types::Language::Markdown);
        assert_eq!(types::Language::from_filename("no_extension"), types::Language::Unknown);
    }

    #[test]
    fn test_cache_integration() {
        let cache: common::MemoryCache<String, i32> = common::MemoryCache::new();
        
        // Test set and get
        cache.set("key1".to_string(), 42).unwrap();
        let value = cache.get(&"key1".to_string()).unwrap();
        assert_eq!(value, Some(42));
        
        // Test remove
        cache.remove(&"key1".to_string()).unwrap();
        let value = cache.get(&"key1".to_string()).unwrap();
        assert_eq!(value, None);
        
        // Test clear
        cache.set("key2".to_string(), 100).unwrap();
        cache.clear().unwrap();
        let value = cache.get(&"key2".to_string()).unwrap();
        assert_eq!(value, None);
    }

    #[test]
    fn test_object_pool_integration() {
        let pool = common::SimpleObjectPool::new(|| String::new());
        
        // Initially pool is empty, but acquire will create new items
        let item1 = pool.acquire().unwrap();
        assert_eq!(item1, "");
        
        // Release some items first
        pool.release("item1".to_string()).unwrap();
        pool.release("item2".to_string()).unwrap();
        
        // Now we can acquire items
        let item1 = pool.acquire().unwrap();
        assert_eq!(item1, "item2"); // LIFO order
        
        let item2 = pool.acquire().unwrap();
        assert_eq!(item2, "item1"); // LIFO order
        
        // Test available count
        assert_eq!(pool.available_count(), 0);
        
        // Release again
        pool.release("item3".to_string()).unwrap();
        assert_eq!(pool.available_count(), 1);
        
        // Test clear
        pool.clear().unwrap();
        assert_eq!(pool.available_count(), 0);
    }

    #[test]
    fn test_config_integration() {
        let config = common::MemoryConfig::new();
        
        // Test set and get
        config.set("test_key", "test_value").unwrap();
        let value: String = config.get("test_key").unwrap();
        assert_eq!(value, "test_value");
        
        // Test has
        assert!(config.has("test_key"));
        assert!(!config.has("nonexistent_key"));
        
        // Test remove
        config.remove("test_key").unwrap();
        assert!(!config.has("test_key"));
    }

    #[test]
    fn test_utils_integration() {
        let text = "Hello\nWorld\nTest";
        
        // Test text utils
        let pos = utils::TextUtils::offset_to_position(text, 6);
        assert_eq!(pos.line, 1);
        assert_eq!(pos.column, 0);
        
        let offset = utils::TextUtils::position_to_offset(text, &types::Position::new(1, 0));
        assert_eq!(offset, 6);
        
        let line_count = utils::TextUtils::count_lines(text);
        assert_eq!(line_count, 3);
        
        let line = utils::TextUtils::get_line(text, 1);
        assert_eq!(line, Some("World"));
        
        // Test hash utils
        let hash1 = utils::HashUtils::hash_text("Hello");
        let hash2 = utils::HashUtils::hash_text("Hello");
        let hash3 = utils::HashUtils::hash_text("World");
        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
        
        // Test validation utils
        let span = types::Span::new(0, 5);
        let valid = utils::ValidationUtils::validate_span(&span, text.len());
        assert!(valid);
        
        let invalid_span = types::Span::new(0, 100);
        let invalid = utils::ValidationUtils::validate_span(&invalid_span, text.len());
        assert!(!invalid);
    }
} 