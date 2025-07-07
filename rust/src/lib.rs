use crate::core::traits::{Cache, Config, ObjectPool};

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

/// Test function to verify core abstraction layer
pub fn test_core_abstraction() -> String {
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
        let result = test_core_abstraction();
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
    fn test_core_traits_integration() {
        // Test SymbolTable
        let mut symbol_table = traits::SymbolTable::new();
        let file_id = types::FileId::new("test.py");
        let symbol = types::Symbol::new(
            "test_func".to_string(),
            "test_func".to_string(),
            types::SymbolKind::Function,
            types::Span::new(0, 10),
            file_id.clone(),
        );
        symbol_table.add_symbol(symbol);
        assert_eq!(symbol_table.symbols.len(), 1);

        // Test Scope
        let scope = traits::Scope::new("global".to_string(), types::Span::new(0, 100));
        symbol_table.add_scope(scope);
        assert_eq!(symbol_table.scopes.len(), 1);
    }

    #[test]
    fn test_core_common_integration() {
        // Test MemoryCache
        let cache: common::MemoryCache<String, i32> = common::MemoryCache::new();
        assert!(cache.set("test_key".to_string(), 42).is_ok());
        assert_eq!(cache.get(&"test_key".to_string()).unwrap(), Some(42));

        // Test MemoryConfig
        let config = common::MemoryConfig::new();
        assert!(config.set("test_config", "test_value").is_ok());
        assert_eq!(config.get::<String>("test_config").unwrap(), "test_value");

        // Test SimpleObjectPool
        let pool = common::SimpleObjectPool::new(|| String::new());
        // Initially pool is empty
        assert!(pool.acquire().is_none());
        // Release an item
        pool.release("test_item".to_string());
        // Now should be able to acquire
        let item = pool.acquire();
        assert!(item.is_some());
        assert_eq!(item.unwrap(), "test_item");
    }

    #[test]
    fn test_core_errors_integration() {
        // Test error creation and conversion
        let parse_error = errors::ParserError::SyntaxError {
            code: "syntax_error",
            message: "Test error".to_string(),
            span: types::Span::new(0, 10),
        };
        let core_error: errors::CoreError = parse_error.into();
        match core_error {
            errors::CoreError::ParseError { message, .. } => {
                assert!(message.contains("Test error"));
            }
            _ => panic!("Expected ParseError"),
        }

        // Test result aliases
        let _: errors::CoreResult<()> = Ok(());
        let _: errors::ParserResult<()> = Ok(());
        let _: errors::SemanticResult<()> = Ok(());
    }

    #[test]
    fn test_text_utils_integration() {
        let text = "Hello\nWorld\nTest";
        
        // Test position conversion
        let pos = utils::TextUtils::offset_to_position(text, 6);
        assert_eq!(pos.line, 1);
        assert_eq!(pos.column, 0);
        
        let offset = utils::TextUtils::position_to_offset(text, &types::Position::new(1, 0));
        assert_eq!(offset, 6);
        
        // Test text slicing
        let span = types::Span::new(0, 5);
        let slice = utils::TextUtils::get_text_slice(text, &span);
        assert_eq!(slice, "Hello");
        
        // Test line counting
        assert_eq!(utils::TextUtils::count_lines(text), 3);
        assert_eq!(utils::TextUtils::get_line(text, 1), Some("World"));
    }

    #[test]
    fn test_validation_utils_integration() {
        let text = "Hello World";
        let span = types::Span::new(0, 5);
        let position = types::Position::new(0, 0);
        
        // Test span validation
        assert!(utils::ValidationUtils::validate_span(&span, text.len()));
        
        // Test position validation
        assert!(utils::ValidationUtils::validate_position(&position, text));
        
        // Test file ID validation
        assert!(utils::ValidationUtils::validate_file_id("test.py"));
        assert!(!utils::ValidationUtils::validate_file_id(""));
    }

    #[test]
    fn test_hash_utils_integration() {
        let text = "Hello World";
        
        // Test text hashing
        let hash1 = utils::HashUtils::hash_text(text);
        let hash2 = utils::HashUtils::hash_text(text);
        assert_eq!(hash1, hash2);
        
        // Test file content hashing
        let file_hash = utils::HashUtils::hash_file_content(text, &types::Language::Python);
        assert!(!file_hash.is_empty());
        assert!(file_hash.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_performance_timer_integration() {
        let timer = common::PerformanceTimer::start();
        
        // Do some minimal work
        std::thread::sleep(std::time::Duration::from_millis(1));
        
        let elapsed = timer.elapsed();
        assert!(elapsed.as_millis() >= 1);
        
        let elapsed_millis = timer.elapsed_millis();
        assert!(elapsed_millis >= 1);
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
        
        let context = traits::AiContext::new(source_code.clone(), file_context.clone());
        
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
        let ai_context = traits::AiContext::new(source_code, file_context);
        
        let request_type = traits::AiRequestType::CodeGeneration {
            prompt: "Generate a function".to_string(),
        };
        
        let request = traits::AiRequest::new(request_type, ai_context);
        
        match &request.request_type {
            traits::AiRequestType::CodeGeneration { prompt } => {
                assert_eq!(prompt, "Generate a function");
            }
            _ => panic!("Expected CodeGeneration"),
        }
        
        let response = traits::AiResponse::new(
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
    fn test_severity_ordering_integration() {
        let severities = vec![
            types::Severity::Info,
            types::Severity::Error,
            types::Severity::Hint,
            types::Severity::Warning,
        ];
        let mut sorted = severities.clone();
        sorted.sort();
        
        assert_eq!(sorted, vec![
            types::Severity::Error,
            types::Severity::Warning,
            types::Severity::Info,
            types::Severity::Hint,
        ]);
    }
} 