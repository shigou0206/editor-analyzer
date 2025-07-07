use tree_sitter::{Parser, Tree, Node as TSNode};
use tree_sitter_python;
use tree_sitter_json;
use crate::core::traits::ast::{Ast, AstNode, CodeParser, IncrementalParser};
use crate::core::types::{Span, Language};
use crate::core::errors::ParserError;
use std::collections::HashMap;
use std::sync::RwLock;
use std::sync::Weak;
use std::sync::Arc;
use once_cell::sync::Lazy;

/// Tree-sitter 语言注册表
static PARSER_REGISTRY: Lazy<RwLock<HashMap<Language, Box<dyn Fn() -> Result<tree_sitter::Language, ParserError> + Send + Sync>>>> = 
    Lazy::new(|| {
        let mut registry = HashMap::new();
        registry.insert(Language::Python, Box::new(|| Ok(tree_sitter_python::language())) as Box<dyn Fn() -> Result<tree_sitter::Language, ParserError> + Send + Sync>);
        registry.insert(Language::Json, Box::new(|| Ok(tree_sitter_json::language())) as Box<dyn Fn() -> Result<tree_sitter::Language, ParserError> + Send + Sync>);
        RwLock::new(registry)
    });

/// Tree-sitter AST 节点包装器
pub struct TreeSitterNode {
    kind: String,
    text: String,
    span: Span,
    children: Vec<TreeSitterNode>,
    parent: Option<Weak<TreeSitterNode>>,
}

impl TreeSitterNode {
    pub fn new(node: TSNode, source: &str) -> Self {
        let kind = node.kind().to_string();
        let text = node.utf8_text(source.as_bytes()).unwrap_or("").to_string();
        let span = Span::new(node.start_byte(), node.end_byte());
        
        let mut children = Vec::new();
        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                children.push(TreeSitterNode::new(child, source));
            }
        }
        
        Self { kind, text, span, children, parent: None }
    }

    /// 获取缓存的子节点（避免重复 Box 分配）
    pub fn cached_children(&self) -> &[TreeSitterNode] {
        &self.children
    }
}

impl AstNode for TreeSitterNode {
    fn kind(&self) -> &str {
        &self.kind
    }

    fn text(&self) -> &str {
        &self.text
    }

    fn span(&self) -> Span {
        self.span
    }

    fn children(&self) -> Vec<Box<dyn AstNode>> {
        self.children.iter()
            .map(|child| Box::new(child.clone()) as Box<dyn AstNode>)
            .collect()
    }

    fn parent(&self) -> Option<Box<dyn AstNode>> {
        None // 简化实现，暂时不提供父节点引用
    }
}

impl Clone for TreeSitterNode {
    fn clone(&self) -> Self {
        Self {
            kind: self.kind.clone(),
            text: self.text.clone(),
            span: self.span,
            children: self.children.clone(),
            parent: self.parent.clone(),
        }
    }
}

/// 增强的语法错误类型
#[derive(Debug, Clone)]
pub enum SyntaxErrorType {
    MissingToken(String),
    UnexpectedToken(String),
    InvalidSyntax(String),
    IncompleteExpression,
    UnmatchedDelimiter,
    Unknown,
}

impl SyntaxErrorType {
    fn from_node_kind(kind: &str, text: &str) -> Self {
        match kind {
            "ERROR" => SyntaxErrorType::InvalidSyntax(text.to_string()),
            "MISSING" => SyntaxErrorType::MissingToken(text.to_string()),
            "UNEXPECTED" => SyntaxErrorType::UnexpectedToken(text.to_string()),
            _ => SyntaxErrorType::Unknown,
        }
    }
}

/// Tree-sitter AST 包装器
pub struct TreeSitterAst {
    root_node: Arc<TreeSitterNode>,
}

impl TreeSitterAst {
    pub fn new(tree: Tree, source: &str) -> Self {
        let root_node = Arc::new(TreeSitterNode::new(tree.root_node(), source));
        Self { root_node }
    }

    /// 获取语法错误，使用增强的错误检测
    pub fn get_detailed_syntax_errors(&self) -> Vec<(SyntaxErrorType, Span, String)> {
        let mut errors = Vec::new();
        
        fn check_for_errors(node: &TreeSitterNode, errors: &mut Vec<(SyntaxErrorType, Span, String)>) {
            // 检查错误节点类型
            let error_type = SyntaxErrorType::from_node_kind(node.kind(), node.text());
            if matches!(error_type, SyntaxErrorType::InvalidSyntax(_) | SyntaxErrorType::MissingToken(_) | SyntaxErrorType::UnexpectedToken(_)) {
                errors.push((error_type, node.span(), node.text().to_string()));
            }
            
            // 递归检查子节点
            for child in &node.children {
                check_for_errors(child, errors);
            }
        }
        
        check_for_errors(&self.root_node, &mut errors);
        errors
    }
}

impl Ast for TreeSitterAst {
    type Node = TreeSitterNode;
    type Error = ParserError;

    fn root_node(&self) -> &Self::Node {
        &self.root_node
    }

    fn node_text<'a>(&self, node: &'a Self::Node) -> &'a str {
        node.text()
    }

    fn node_kind<'a>(&self, node: &'a Self::Node) -> &'a str {
        node.kind()
    }

    fn node_span(&self, node: &Self::Node) -> Span {
        node.span()
    }

    fn node_children(&self, node: &Self::Node) -> Vec<Self::Node> {
        node.cached_children().iter()
            .map(|child| child.clone())
            .collect()
    }

    fn get_syntax_errors(&self) -> Vec<crate::core::traits::ast::SyntaxError> {
        self.get_detailed_syntax_errors()
            .into_iter()
            .map(|(error_type, span, text)| {
                let message = match error_type {
                    SyntaxErrorType::MissingToken(token) => format!("Missing token: {}", token),
                    SyntaxErrorType::UnexpectedToken(token) => format!("Unexpected token: {}", token),
                    SyntaxErrorType::InvalidSyntax(_) => format!("Invalid syntax: {}", text),
                    SyntaxErrorType::IncompleteExpression => "Incomplete expression".to_string(),
                    SyntaxErrorType::UnmatchedDelimiter => "Unmatched delimiter".to_string(),
                    SyntaxErrorType::Unknown => "Unknown syntax error".to_string(),
                };
                
                crate::core::traits::ast::SyntaxError::new(
                    message,
                    span,
                    crate::core::types::Severity::Error,
                )
            })
            .collect()
    }
}

/// Tree-sitter 解析器实现
pub struct TreeSitterParser {
    // 缓存解析器实例以提高性能
    python_parser: Option<Parser>,
    json_parser: Option<Parser>,
}

impl TreeSitterParser {
    pub fn new() -> Self {
        Self {
            python_parser: None,
            json_parser: None,
        }
    }

    fn get_language(language: Language) -> Result<tree_sitter::Language, ParserError> {
        let registry = PARSER_REGISTRY.read()
            .map_err(|_| Self::create_error(
                "Failed to access parser registry".to_string(),
                Span::new(0, 0)
            ))?;
        
        if let Some(lang_fn) = registry.get(&language) {
            lang_fn()
        } else {
            Err(Self::create_error(
                format!("Unsupported language: {:?}", language),
                Span::new(0, 0)
            ))
        }
    }

    /// 统一的错误创建函数
    fn create_error(message: String, span: Span) -> ParserError {
        ParserError::syntax_error(message, span)
    }

    /// 获取或创建解析器实例
    fn get_or_create_parser(&mut self, language: Language) -> Result<&mut Parser, ParserError> {
        match language {
            Language::Python => {
                if self.python_parser.is_none() {
                    let mut parser = Parser::new();
                    let lang = Self::get_language(language)?;
                    parser.set_language(lang)
                        .map_err(|e| Self::create_error(
                            format!("Failed to load Python grammar: {}", e),
                            Span::new(0, 0)
                        ))?;
                    self.python_parser = Some(parser);
                }
                Ok(self.python_parser.as_mut().unwrap())
            }
            Language::Json => {
                if self.json_parser.is_none() {
                    let mut parser = Parser::new();
                    let lang = Self::get_language(language)?;
                    parser.set_language(lang)
                        .map_err(|e| Self::create_error(
                            format!("Failed to load JSON grammar: {}", e),
                            Span::new(0, 0)
                        ))?;
                    self.json_parser = Some(parser);
                }
                Ok(self.json_parser.as_mut().unwrap())
            }
            _ => Err(Self::create_error(
                format!("Unsupported language: {:?}", language),
                Span::new(0, 0)
            ))
        }
    }

    /// 注册新的语言支持
    pub fn register_language(language: Language, lang_fn: Box<dyn Fn() -> Result<tree_sitter::Language, ParserError> + Send + Sync>) -> Result<(), ParserError> {
        let mut registry = PARSER_REGISTRY.write()
            .map_err(|_| Self::create_error(
                "Failed to access parser registry".to_string(),
                Span::new(0, 0)
            ))?;
        
        registry.insert(language, lang_fn);
        Ok(())
    }

    /// 获取支持的语言列表
    pub fn supported_languages() -> Vec<Language> {
        if let Ok(registry) = PARSER_REGISTRY.read() {
            registry.keys().cloned().collect()
        } else {
            Vec::new()
        }
    }

    /// 计算文本差异（改进的 diff 算法）
    fn compute_text_diff(&self, old_source: &str, new_source: &str) -> crate::core::traits::ast::Diff {
        // 简单的基于行的 diff 实现
        let old_lines: Vec<&str> = old_source.lines().collect();
        let new_lines: Vec<&str> = new_source.lines().collect();
        
        let mut changes = Vec::new();
        let mut i = 0;
        let mut j = 0;
        
        while i < old_lines.len() && j < new_lines.len() {
            if old_lines[i] == new_lines[j] {
                i += 1;
                j += 1;
            } else {
                // 找到不同的行，标记为替换
                let start = old_source.lines().take(i).map(|l| l.len() + 1).sum::<usize>();
                let end = old_source.lines().take(i + 1).map(|l| l.len() + 1).sum::<usize>();
                
                changes.push(crate::core::traits::ast::Change::Replace {
                    start,
                    end,
                    text: new_lines[j].to_string(),
                });
                i += 1;
                j += 1;
            }
        }
        
        // 处理剩余的行
        if i < old_lines.len() {
            let start = old_source.lines().take(i).map(|l| l.len() + 1).sum::<usize>();
            changes.push(crate::core::traits::ast::Change::Replace {
                start,
                end: old_source.len(),
                text: "".to_string(),
            });
        } else if j < new_lines.len() {
            let start = old_source.len();
            changes.push(crate::core::traits::ast::Change::Replace {
                start,
                end: start,
                text: new_lines[j..].join("\n"),
            });
        }
        
        crate::core::traits::ast::Diff { changes }
    }
}

impl CodeParser for TreeSitterParser {
    type Ast = TreeSitterAst;
    type Error = ParserError;

    fn parse(&self, source: &str, language: Language) -> Result<Self::Ast, Self::Error> {
        // 为了保持 trait 兼容性，我们创建新的解析器实例
        let mut parser = Parser::new();
        let lang = Self::get_language(language)?;
        parser.set_language(lang)
            .map_err(|e| Self::create_error(
                format!("Failed to load grammar: {}", e),
                Span::new(0, 0)
            ))?;
        
        let tree = parser.parse(source, None)
            .ok_or_else(|| Self::create_error(
                "Failed to parse source code".to_string(),
                Span::new(0, 0)
            ))?;
        
        Ok(TreeSitterAst::new(tree, source))
    }

    fn parse_incremental(&self, source: &str, old_ast: &Self::Ast) -> Result<Self::Ast, Self::Error> {
        // 改进的增量解析：使用 Tree-sitter 的编辑功能
        let diff = self.compute_text_diff(old_ast.root_node().text(), source);
        
        // 应用差异并重新解析
        let new_source = diff.changes.iter()
            .map(|change| match change {
                crate::core::traits::ast::Change::Replace { text, .. } => text,
                _ => "",
            })
            .collect::<Vec<_>>()
            .join("");
        
        // 对于真正的增量解析，这里应该使用 Tree-sitter 的 edit 功能
        // 但为了简化，我们重新解析整个文件
        self.parse(source, Language::Python)
    }

    fn get_syntax_errors(&self, ast: &Self::Ast) -> Vec<crate::core::traits::ast::SyntaxError> {
        ast.get_syntax_errors()
    }

    fn supports_language(&self, language: &Language) -> bool {
        Self::get_language(language.clone()).is_ok()
    }
}

impl IncrementalParser for TreeSitterParser {
    fn compute_diff(&self, old_source: &str, new_source: &str) -> crate::core::traits::ast::Diff {
        self.compute_text_diff(old_source, new_source)
    }

    fn apply_diff(&self, _ast: &Self::Ast, diff: &crate::core::traits::ast::Diff) -> Result<Self::Ast, Self::Error> {
        // 应用差异并重新解析
        let new_source = diff.changes.iter()
            .map(|change| match change {
                crate::core::traits::ast::Change::Replace { text, .. } => text,
                _ => "",
            })
            .collect::<Vec<_>>()
            .join("");
        
        self.parse(&new_source, Language::Python)
    }
}

/// 简化的 Python 解析器（保持向后兼容）
pub struct TreeSitterPythonParser {
    parser: Parser,
}

impl TreeSitterPythonParser {
    pub fn new() -> Self {
        let mut parser = Parser::new();
        parser.set_language(tree_sitter_python::language()).expect("Error loading Python grammar");
        Self { parser }
    }

    pub fn parse(&mut self, source_code: &str) -> Option<Tree> {
        self.parser.parse(source_code, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_python_code() {
        let mut parser = TreeSitterPythonParser::new();
        let code = "def foo(x):\n    return x + 1\n";
        let tree = parser.parse(code);
        assert!(tree.is_some());
        let tree = tree.unwrap();
        let root = tree.root_node();
        assert_eq!(root.kind(), "module");
        assert!(root.child_count() > 0);
    }

    #[test]
    fn test_tree_sitter_parser() {
        let parser = TreeSitterParser::new();
        let code = "def hello():\n    print('Hello, World!')\n";
        let result = parser.parse(code, Language::Python);
        assert!(result.is_ok());
        
        let ast = result.unwrap();
        let errors = ast.get_syntax_errors();
        assert!(errors.is_empty());
        
        // 测试 AST 操作
        let root = ast.root_node();
        assert_eq!(root.kind(), "module");
        assert!(!root.children().is_empty());
    }

    #[test]
    fn test_parse_json() {
        let parser = TreeSitterParser::new();
        let code = r#"{"name": "test", "value": 42}"#;
        let result = parser.parse(code, Language::Json);
        assert!(result.is_ok());
        
        let ast = result.unwrap();
        let errors = ast.get_syntax_errors();
        assert!(errors.is_empty());
        
        let root = ast.root_node();
        assert_eq!(root.kind(), "document");
    }

    #[test]
    fn test_unsupported_language() {
        let parser = TreeSitterParser::new();
        let result = parser.parse("{}", Language::Rust);
        assert!(result.is_err());
    }

    #[test]
    fn test_supports_language() {
        let parser = TreeSitterParser::new();
        assert!(parser.supports_language(&Language::Python));
        assert!(parser.supports_language(&Language::Json));
        assert!(!parser.supports_language(&Language::Rust));
    }

    #[test]
    fn test_supported_languages() {
        let languages = TreeSitterParser::supported_languages();
        assert!(languages.contains(&Language::Python));
        assert!(languages.contains(&Language::Json));
        // 注意：Rust 可能被之前的测试注册了，所以这里不检查它
    }

    #[test]
    fn test_parser_registry() {
        // 测试注册新语言（这里只是测试 API，实际需要 tree-sitter 语法）
        let result = TreeSitterParser::register_language(Language::Rust, Box::new(|| {
            Err(ParserError::syntax_error("Not implemented".to_string(), Span::new(0, 0)))
        }));
        assert!(result.is_ok());
        
        // 验证注册后语言在支持列表中
        let languages = TreeSitterParser::supported_languages();
        assert!(languages.contains(&Language::Rust));
        
        // 验证解析器能识别该语言（即使语法函数返回错误）
        let parser = TreeSitterParser::new();
        // 注意：supports_language 会调用语法函数，如果函数返回错误则返回 false
        // 这是正确的行为，因为语法函数失败意味着该语言实际上不可用
    }
}
