use serde::{Deserialize, Serialize};

/// 支持的编程语言
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    Python,
    Json,
    Yaml,
    Markdown,
    Rust,
    JavaScript,
    TypeScript,
    Unknown,
}

impl Language {
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "py" => Self::Python,
            "json" => Self::Json,
            "yaml" | "yml" => Self::Yaml,
            "md" | "markdown" => Self::Markdown,
            "rs" => Self::Rust,
            "js" => Self::JavaScript,
            "ts" | "tsx" => Self::TypeScript,
            _ => Self::Unknown,
        }
    }

    pub fn from_filename(filename: &str) -> Self {
        if let Some(ext) = filename.split('.').last() {
            Self::from_extension(ext)
        } else {
            Self::Unknown
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language() {
        // Test file extension detection
        assert_eq!(Language::from_extension("py"), Language::Python);
        assert_eq!(Language::from_extension("json"), Language::Json);
        assert_eq!(Language::from_extension("yaml"), Language::Yaml);
        assert_eq!(Language::from_extension("yml"), Language::Yaml);
        assert_eq!(Language::from_extension("md"), Language::Markdown);
        assert_eq!(Language::from_extension("rs"), Language::Rust);
        assert_eq!(Language::from_extension("js"), Language::JavaScript);
        assert_eq!(Language::from_extension("ts"), Language::TypeScript);
        assert_eq!(Language::from_extension("unknown"), Language::Unknown);
        
        // Test filename detection
        assert_eq!(Language::from_filename("test.py"), Language::Python);
        assert_eq!(Language::from_filename("config.json"), Language::Json);
        assert_eq!(Language::from_filename("README.md"), Language::Markdown);
        assert_eq!(Language::from_filename("no_extension"), Language::Unknown);
    }
} 