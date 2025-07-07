use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::OnceLock;

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
    Custom(String), // 支持自定义语言
    Unknown,
}

impl Language {
    /// 从字符串创建语言
    pub fn from_string(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "python" | "py" => Language::Python,
            "json" => Language::Json,
            "yaml" | "yml" => Language::Yaml,
            "markdown" | "md" => Language::Markdown,
            "rust" | "rs" => Language::Rust,
            "javascript" | "js" => Language::JavaScript,
            "typescript" | "ts" => Language::TypeScript,
            _ => Language::Custom(s.to_string()),
        }
    }

    /// 获取语言标识符
    pub fn as_string(&self) -> String {
        match self {
            Language::Python => "python".to_string(),
            Language::Json => "json".to_string(),
            Language::Yaml => "yaml".to_string(),
            Language::Markdown => "markdown".to_string(),
            Language::Rust => "rust".to_string(),
            Language::JavaScript => "javascript".to_string(),
            Language::TypeScript => "typescript".to_string(),
            Language::Custom(name) => name.clone(),
            Language::Unknown => "unknown".to_string(),
        }
    }

    /// 检查是否为内置语言
    pub fn is_builtin(&self) -> bool {
        !matches!(self, Language::Custom(_))
    }

    /// 检查是否为自定义语言
    pub fn is_custom(&self) -> bool {
        matches!(self, Language::Custom(_))
    }
}

/// Language configuration for dynamic mapping
#[derive(Debug, Clone)]
pub struct LanguageConfig {
    pub extensions: HashMap<String, Language>,
    pub filenames: HashMap<String, Language>,
    pub shebangs: HashMap<String, Vec<String>>,
    pub language_id: Option<String>,
}

impl LanguageConfig {
    pub fn new() -> Self {
        let mut config = Self {
            extensions: HashMap::new(),
            filenames: HashMap::new(),
            shebangs: HashMap::new(),
            language_id: None,
        };
        
        // Initialize with default mappings
        config.add_extension("py", Language::Python);
        config.add_extension("json", Language::Json);
        config.add_extension("yaml", Language::Yaml);
        config.add_extension("yml", Language::Yaml);
        config.add_extension("md", Language::Markdown);
        config.add_extension("markdown", Language::Markdown);
        config.add_extension("rs", Language::Rust);
        config.add_extension("js", Language::JavaScript);
        config.add_extension("ts", Language::TypeScript);
        config.add_extension("tsx", Language::TypeScript);
        
        // Add filename mappings
        config.add_filename("Dockerfile", Language::Yaml);
        config.add_filename("Makefile", Language::Unknown);
        config.add_filename("README", Language::Markdown);
        
        config
    }
    
    pub fn add_extension(&mut self, ext: &str, language: Language) {
        self.extensions.insert(ext.to_lowercase(), language);
    }
    
    pub fn add_filename(&mut self, filename: &str, language: Language) {
        self.filenames.insert(filename.to_string(), language);
    }

    /// 动态注册新的语言映射
    pub fn register_custom_language(&mut self, name: &str, extensions: &[&str], filenames: &[&str]) {
        let custom_lang = Language::Custom(name.to_string());
        
        // 注册扩展名
        for ext in extensions {
            self.add_extension(ext, custom_lang.clone());
        }
        
        // 注册文件名
        for filename in filenames {
            self.add_filename(filename, custom_lang.clone());
        }
    }
    
    pub fn from_extension(&self, ext: &str) -> Language {
        self.extensions.get(&ext.to_lowercase()).cloned().unwrap_or(Language::Unknown)
    }
    
    pub fn from_filename(&self, filename: &str) -> Language {
        // Check for exact filename match first
        if let Some(lang) = self.filenames.get(filename) {
            return lang.clone();
        }
        
        // Then check extension
        if let Some(ext) = filename.split('.').next_back() {
            return self.from_extension(ext);
        }
        
        Language::Unknown
    }

    pub fn detect_language(&self, filename: &str) -> Option<Language> {
        // 检查文件扩展名
        if let Some(ext) = filename.split('.').next_back() {
            if let Some(lang) = self.extensions.get(ext) {
                return Some(lang.clone());
            }
        }
        
        // 检查文件名
        if let Some(lang) = self.filenames.get(filename) {
            return Some(lang.clone());
        }
        
        None
    }

    /// 获取所有支持的语言
    pub fn get_supported_languages(&self) -> Vec<Language> {
        let mut languages = Vec::new();
        
        // 添加内置语言
        languages.extend_from_slice(&[
            Language::Python,
            Language::Json,
            Language::Yaml,
            Language::Markdown,
            Language::Rust,
            Language::JavaScript,
            Language::TypeScript,
        ]);
        
        // 添加自定义语言
        for lang in self.extensions.values() {
            if lang.is_custom() && !languages.contains(lang) {
                languages.push(lang.clone());
            }
        }
        
        languages
    }
}

impl Default for LanguageConfig {
    fn default() -> Self {
        Self::new()
    }
}

// Global language configuration
static LANGUAGE_CONFIG: OnceLock<LanguageConfig> = OnceLock::new();

fn get_language_config() -> &'static LanguageConfig {
    LANGUAGE_CONFIG.get_or_init(LanguageConfig::new)
}

impl Language {
    pub fn from_extension(ext: &str) -> Self {
        get_language_config().from_extension(ext)
    }

    pub fn from_filename(filename: &str) -> Self {
        get_language_config().from_filename(filename)
    }
    
    /// Register a new language mapping
    /// 
    /// # Note
    /// This is currently a stub implementation that does nothing.
    /// For runtime language registration, this would require interior mutability
    /// (e.g., RwLock<LanguageConfig>) instead of the current static configuration.
    /// 
    /// # Panics
    /// This method currently does nothing and will not panic.
    /// 
    /// # Future Implementation
    /// To implement this properly:
    /// 1. Change static LANGUAGE_CONFIG to use RwLock<LanguageConfig>
    /// 2. Implement thread-safe dynamic registration
    /// 3. Consider plugin system for language registration
    pub fn register_extension(_ext: &str, _language: Language) {
        // TODO: Implement runtime language registration with interior mutability
        // This would require changing the static LANGUAGE_CONFIG to use RwLock
        // and implementing proper thread-safe dynamic registration
        //
        // Current implementation is intentionally empty to prevent accidental use
        // of the stub implementation in production code.
    }

    /// 动态注册自定义语言
    pub fn register_custom_language(name: &str, extensions: &[&str], filenames: &[&str]) {
        // TODO: 实现线程安全的动态语言注册
        // 这需要将静态 LANGUAGE_CONFIG 改为使用 RwLock<LanguageConfig>
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

    #[test]
    fn test_language_from_string() {
        assert_eq!(Language::from_string("python"), Language::Python);
        assert_eq!(Language::from_string("PYTHON"), Language::Python);
        assert_eq!(Language::from_string("custom_lang"), Language::Custom("custom_lang".to_string()));
    }

    #[test]
    fn test_language_as_string() {
        assert_eq!(Language::Python.as_string(), "python");
        assert_eq!(Language::Custom("my_lang".to_string()).as_string(), "my_lang");
    }

    #[test]
    fn test_language_properties() {
        assert!(Language::Python.is_builtin());
        assert!(!Language::Python.is_custom());
        assert!(Language::Custom("test".to_string()).is_custom());
        assert!(!Language::Custom("test".to_string()).is_builtin());
    }

    #[test]
    fn test_custom_language_registration() {
        let mut config = LanguageConfig::new();
        config.register_custom_language("my_lang", &["ml", "mylang"], &["MyFile.ml"]);
        
        assert_eq!(config.from_extension("ml"), Language::Custom("my_lang".to_string()));
        assert_eq!(config.from_filename("MyFile.ml"), Language::Custom("my_lang".to_string()));
        
        let languages = config.get_supported_languages();
        assert!(languages.contains(&Language::Custom("my_lang".to_string())));
    }
} 