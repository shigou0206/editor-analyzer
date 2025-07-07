use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// 配置验证错误
#[derive(Debug, Clone, thiserror::Error)]
pub enum ConfigValidationError {
    #[error("Invalid value for key '{key}': {message}")]
    InvalidValue { key: String, message: String },
    
    #[error("Missing required key: {key}")]
    MissingKey { key: String },
    
    #[error("Schema validation failed: {message}")]
    SchemaError { message: String },
    
    #[error("Type mismatch for key '{key}': expected {expected}, got {actual}")]
    TypeMismatch { key: String, expected: String, actual: String },
}

/// 配置 schema 定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigSchema {
    pub properties: HashMap<String, PropertySchema>,
    pub required: Vec<String>,
    pub additional_properties: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertySchema {
    pub r#type: String,
    pub description: Option<String>,
    pub default: Option<serde_json::Value>,
    pub required: bool,
    pub enum_values: Option<Vec<serde_json::Value>>,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub pattern: Option<String>,
}

/// 配置 trait - 支持外部注入和 schema 校验
pub trait Config: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;
    
    /// 获取配置值
    fn get<T: serde::de::DeserializeOwned>(&self, key: &str) -> Result<T, Self::Error>;
    
    /// 设置配置值
    fn set<T: serde::Serialize>(&self, key: &str, value: T) -> Result<(), Self::Error>;
    
    /// 检查配置键是否存在
    fn has(&self, key: &str) -> bool;
    
    /// 移除配置键
    fn remove(&self, key: &str) -> Result<(), Self::Error>;
    
    /// 获取所有配置键
    fn keys(&self) -> Vec<String>;
    
    /// 获取配置的原始值
    fn get_raw(&self, key: &str) -> Option<serde_json::Value>;
    
    /// 设置配置的原始值
    fn set_raw(&self, key: &str, value: serde_json::Value) -> Result<(), Self::Error>;
    
    /// 加载配置文件
    fn load_from_file(&self, path: &PathBuf) -> Result<(), Self::Error>;
    
    /// 保存配置到文件
    fn save_to_file(&self, path: &PathBuf) -> Result<(), Self::Error>;
    
    /// 加载环境变量
    fn load_from_env(&self, prefix: &str) -> Result<(), Self::Error>;
    
    /// 验证配置
    fn validate(&self, schema: &ConfigSchema) -> Result<(), ConfigValidationError>;
    
    /// 获取配置 schema
    fn schema(&self) -> Option<&ConfigSchema>;
    
    /// 设置配置 schema
    fn set_schema(&self, schema: ConfigSchema) -> Result<(), Self::Error>;
    
    /// 重置配置到默认值
    fn reset_to_defaults(&self) -> Result<(), Self::Error>;
    
    /// 获取配置统计信息
    fn stats(&self) -> ConfigStats;
}

/// 配置统计信息
#[derive(Debug, Clone)]
pub struct ConfigStats {
    pub total_keys: usize,
    pub loaded_files: Vec<PathBuf>,
    pub last_modified: Option<std::time::SystemTime>,
    pub validation_errors: Vec<ConfigValidationError>,
}

impl Default for ConfigStats {
    fn default() -> Self {
        Self {
            total_keys: 0,
            loaded_files: Vec::new(),
            last_modified: None,
            validation_errors: Vec::new(),
        }
    }
}

/// 配置监听器 trait
pub trait ConfigListener: Send + Sync {
    type Error;
    
    /// 配置值改变时的回调
    fn on_config_changed(&self, key: &str, old_value: Option<serde_json::Value>, new_value: serde_json::Value) -> Result<(), Self::Error>;
    
    /// 配置重新加载时的回调
    fn on_config_reloaded(&self) -> Result<(), Self::Error>;
}

/// 配置提供者 trait - 支持多种配置源
pub trait ConfigProvider: Send + Sync {
    type Error;
    
    /// 从提供者加载配置
    fn load(&self) -> Result<HashMap<String, serde_json::Value>, Self::Error>;
    
    /// 保存配置到提供者
    fn save(&self, config: &HashMap<String, serde_json::Value>) -> Result<(), Self::Error>;
    
    /// 检查提供者是否可用
    fn is_available(&self) -> bool;
    
    /// 获取提供者名称
    fn name(&self) -> &str;
}

/// 文件配置提供者
pub struct FileConfigProvider {
    path: PathBuf,
    format: ConfigFormat,
}

#[derive(Debug, Clone)]
pub enum ConfigFormat {
    Json,
    Yaml,
    Toml,
    Ini,
}

impl FileConfigProvider {
    pub fn new(path: PathBuf, format: ConfigFormat) -> Self {
        Self { path, format }
    }
}

impl ConfigProvider for FileConfigProvider {
    type Error = std::io::Error;
    
    fn load(&self) -> Result<HashMap<String, serde_json::Value>, Self::Error> {
        let content = std::fs::read_to_string(&self.path)?;
        
        let config: HashMap<String, serde_json::Value> = match self.format {
            ConfigFormat::Json => serde_json::from_str(&content)?,
            ConfigFormat::Yaml => {
                // 简单的 YAML 解析（仅支持基本格式）
                let mut config = HashMap::new();
                for line in content.lines() {
                    if let Some((key, value)) = line.split_once(':') {
                        config.insert(key.trim().to_string(), serde_json::Value::String(value.trim().to_string()));
                    }
                }
                config
            }
            ConfigFormat::Toml => {
                // 简单的 TOML 解析（仅支持基本格式）
                let mut config = HashMap::new();
                for line in content.lines() {
                    if let Some((key, value)) = line.split_once('=') {
                        config.insert(key.trim().to_string(), serde_json::Value::String(value.trim().to_string()));
                    }
                }
                config
            }
            ConfigFormat::Ini => {
                // 简单的 INI 解析
                let mut config = HashMap::new();
                for line in content.lines() {
                    if let Some((key, value)) = line.split_once('=') {
                        config.insert(key.trim().to_string(), serde_json::Value::String(value.trim().to_string()));
                    }
                }
                config
            }
        };
        
        Ok(config)
    }
    
    fn save(&self, config: &HashMap<String, serde_json::Value>) -> Result<(), Self::Error> {
        let content = match self.format {
            ConfigFormat::Json => serde_json::to_string_pretty(config)?,
            ConfigFormat::Yaml => {
                // 简单的 YAML 序列化
                config.iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect::<Vec<_>>()
                    .join("\n")
            }
            ConfigFormat::Toml => {
                // 简单的 TOML 序列化
                config.iter()
                    .map(|(k, v)| format!("{} = {}", k, v))
                    .collect::<Vec<_>>()
                    .join("\n")
            }
            ConfigFormat::Ini => {
                // 简单的 INI 序列化
                config.iter()
                    .map(|(k, v)| format!("{}={}", k, v))
                    .collect::<Vec<_>>()
                    .join("\n")
            }
        };
        
        std::fs::write(&self.path, content)?;
        Ok(())
    }
    
    fn is_available(&self) -> bool {
        self.path.exists()
    }
    
    fn name(&self) -> &str {
        "file"
    }
} 