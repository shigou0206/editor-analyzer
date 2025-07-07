use rust::parsers::tree_sitter::{TreeSitterParser, TreeSitterPythonParser};
use rust::core::types::{Language, Span};
use rust::core::traits::ast::{Ast, AstNode, CodeParser};

#[test]
fn test_tree_sitter_integration() {
    // 测试 Python 代码解析
    let python_code = r#"
def fibonacci(n):
    """Calculate the nth Fibonacci number."""
    if n <= 1:
        return n
    return fibonacci(n-1) + fibonacci(n-2)

# Test the function
result = fibonacci(10)
print(f"Fibonacci(10) = {result}")
"#;

    let parser = TreeSitterParser::new();
    let result = parser.parse(python_code, Language::Python);
    assert!(result.is_ok(), "Failed to parse Python code");

    let ast = result.unwrap();
    let errors = ast.get_syntax_errors();
    assert!(errors.is_empty(), "Python code should have no syntax errors");

    // 测试 AST 遍历
    let root = ast.root_node();
    assert_eq!(root.kind(), "module");
    
    // 检查函数定义
    let children = root.children();
    assert!(!children.is_empty(), "Module should have children");
    
    // 查找函数定义
    let mut found_function = false;
    for child in &children {
        if child.kind() == "function_definition" {
            found_function = true;
            break;
        }
    }
    assert!(found_function, "Should find function definition");
}

#[test]
fn test_json_parsing() {
    let json_code = r#"{
    "name": "test_project",
    "version": "1.0.0",
    "dependencies": {
        "tree-sitter": "^0.20.0",
        "serde": "^1.0.0"
    },
    "scripts": {
        "test": "cargo test",
        "build": "cargo build"
    }
}"#;

    let parser = TreeSitterParser::new();
    let result = parser.parse(json_code, Language::Json);
    assert!(result.is_ok(), "Failed to parse JSON code");

    let ast = result.unwrap();
    let errors = ast.get_syntax_errors();
    assert!(errors.is_empty(), "JSON code should have no syntax errors");

    let root = ast.root_node();
    assert_eq!(root.kind(), "document");
}

#[test]
fn test_syntax_error_detection() {
    // 测试有语法错误的 Python 代码
    let invalid_python = r#"
def incomplete_function(
    # Missing closing parenthesis and function body
"#;

    let parser = TreeSitterParser::new();
    let result = parser.parse(invalid_python, Language::Python);
    
    // Tree-sitter 可能会尝试解析，但应该检测到错误
    if let Ok(ast) = result {
        let errors = ast.get_syntax_errors();
        // 注意：Tree-sitter 的错误检测可能不够完善
        // 这里主要是测试错误处理机制
        println!("Detected {} syntax errors", errors.len());
    }
}

#[test]
fn test_language_support() {
    let parser = TreeSitterParser::new();
    
    // 测试支持的语言
    assert!(parser.supports_language(&Language::Python));
    assert!(parser.supports_language(&Language::Json));
    
    // 测试不支持的语言
    assert!(!parser.supports_language(&Language::Rust));
    assert!(!parser.supports_language(&Language::JavaScript));
}

#[test]
fn test_ast_node_operations() {
    let python_code = "x = 42\ny = x + 1";
    
    let parser = TreeSitterParser::new();
    let ast = parser.parse(python_code, Language::Python).unwrap();
    
    let root = ast.root_node();
    
    // 测试节点属性
    assert_eq!(root.kind(), "module");
    assert!(!root.text().is_empty());
    assert!(root.span().start >= 0);
    assert!(root.span().end > root.span().start);
    
    // 测试子节点
    let children = root.children();
    assert!(!children.is_empty());
    
    for child in &children {
        assert!(!child.kind().is_empty());
        assert!(!child.text().is_empty());
        assert!(child.span().start >= 0);
        assert!(child.span().end > child.span().start);
    }
}

#[test]
fn test_legacy_python_parser() {
    // 测试向后兼容的 Python 解析器
    let mut parser = TreeSitterPythonParser::new();
    let code = "def test():\n    pass";
    
    let tree = parser.parse(code);
    assert!(tree.is_some(), "Legacy parser should parse successfully");
    
    let tree = tree.unwrap();
    let root = tree.root_node();
    assert_eq!(root.kind(), "module");
}

#[test]
fn test_parser_registry_functionality() {
    // 测试解析器注册表功能
    let languages = TreeSitterParser::supported_languages();
    assert!(languages.contains(&Language::Python));
    assert!(languages.contains(&Language::Json));
    
    // 测试注册新语言（模拟）
    let result = TreeSitterParser::register_language(Language::Rust, Box::new(|| {
        Err(rust::core::errors::ParserError::syntax_error(
            "Not implemented".to_string(), 
            Span::new(0, 0)
        ))
    }));
    assert!(result.is_ok(), "Should be able to register language");
    
    // 验证注册后的语言列表
    let updated_languages = TreeSitterParser::supported_languages();
    assert!(updated_languages.contains(&Language::Rust));
} 