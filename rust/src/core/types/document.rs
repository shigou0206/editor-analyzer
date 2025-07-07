use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::language::Language;

/// 文件标识符
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct FileId(pub String);

impl FileId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl From<&str> for FileId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<String> for FileId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

/// 文本文档
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDocument {
    pub file_id: FileId,
    pub content: String,
    pub language: Language,
    pub version: u64,
}

impl TextDocument {
    pub fn new(file_id: FileId, content: String, language: Language) -> Self {
        Self {
            file_id,
            content,
            language,
            version: 1,
        }
    }

    pub fn with_version(mut self, version: u64) -> Self {
        self.version = version;
        self
    }
}

/// 文件上下文
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FileContext {
    pub file_id: FileId,
    pub project_root: Option<String>,
    pub dependencies: Vec<String>,
    pub metadata: HashMap<String, serde_json::Value>,
}

impl FileContext {
    pub fn new(file_id: FileId) -> Self {
        Self {
            file_id,
            project_root: None,
            dependencies: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn with_project_root(mut self, project_root: String) -> Self {
        self.project_root = Some(project_root);
        self
    }

    pub fn with_dependencies(mut self, dependencies: Vec<String>) -> Self {
        self.dependencies = dependencies;
        self
    }
}

/// 源代码
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceCode {
    pub content: String,
    pub language: Language,
    pub file_id: FileId,
}

impl SourceCode {
    pub fn new(content: String, language: Language, file_id: FileId) -> Self {
        Self {
            content,
            language,
            file_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_id() {
        let file_id = FileId::new("test.py");
        assert_eq!(file_id.0, "test.py");
        
        let file_id_from_str = FileId::from("test.rs");
        assert_eq!(file_id_from_str.0, "test.rs");
        
        let file_id_from_string = FileId::from("test.js".to_string());
        assert_eq!(file_id_from_string.0, "test.js");
    }

    #[test]
    fn test_text_document() {
        let file_id = FileId::new("test.py");
        let doc = TextDocument::new(
            file_id.clone(),
            "def hello(): pass".to_string(),
            Language::Python,
        );
        
        assert_eq!(doc.file_id, file_id);
        assert_eq!(doc.content, "def hello(): pass");
        assert_eq!(doc.language, Language::Python);
        assert_eq!(doc.version, 1);
        
        let doc_with_version = doc.with_version(5);
        assert_eq!(doc_with_version.version, 5);
    }

    #[test]
    fn test_file_context() {
        let file_id = FileId::new("test.py");
        let context = FileContext::new(file_id.clone());
        
        assert_eq!(context.file_id, file_id);
        assert!(context.project_root.is_none());
        assert!(context.dependencies.is_empty());
        assert!(context.metadata.is_empty());
        
        let context_with_root = context.with_project_root("/path/to/project".to_string());
        assert_eq!(context_with_root.project_root, Some("/path/to/project".to_string()));
        
        let context_with_deps = context_with_root.with_dependencies(vec!["dep1".to_string(), "dep2".to_string()]);
        assert_eq!(context_with_deps.dependencies.len(), 2);
    }

    #[test]
    fn test_source_code() {
        let file_id = FileId::new("test.py");
        let source_code = SourceCode::new(
            "def hello(): pass".to_string(),
            Language::Python,
            file_id.clone(),
        );
        
        assert_eq!(source_code.content, "def hello(): pass");
        assert_eq!(source_code.language, Language::Python);
        assert_eq!(source_code.file_id, file_id);
    }
} 