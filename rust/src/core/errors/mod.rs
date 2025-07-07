pub mod ai_error;
pub mod config_error;
pub mod core_error;
pub mod file_error;
pub mod lsp_error;
pub mod network_error;
pub mod parser_error;
pub mod semantic_error;
pub mod codes;

pub use core_error::*;
pub use ai_error::*;
pub use config_error::*;
pub use file_error::*;
pub use lsp_error::*;
pub use network_error::*;
pub use parser_error::*;
pub use semantic_error::*;

/// 统一错误类型 - 包含所有模块的错误
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Core error: {0}")]
    Core(#[from] core_error::CoreError),
    
    #[error("AI error: {0}")]
    Ai(#[from] ai_error::AiError),
    
    #[error("Config error: {0}")]
    Config(#[from] config_error::ConfigError),
    
    #[error("File error: {0}")]
    File(#[from] file_error::FileError),
    
    #[error("LSP error: {0}")]
    Lsp(#[from] lsp_error::LspError),
    
    #[error("Network error: {0}")]
    Network(#[from] network_error::NetworkError),
    
    #[error("Parser error: {0}")]
    Parser(#[from] parser_error::ParserError),
    
    #[error("Semantic error: {0}")]
    Semantic(#[from] semantic_error::SemanticError),
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl AppError {
    /// 获取错误代码
    pub fn code(&self) -> &str {
        match self {
            AppError::Core(e) => e.code(),
            AppError::Ai(e) => e.code(),
            AppError::Config(e) => e.code(),
            AppError::File(e) => e.code(),
            AppError::Lsp(e) => e.code(),
            AppError::Network(e) => e.code(),
            AppError::Parser(e) => e.code(),
            AppError::Semantic(e) => e.code(),
            AppError::Unknown(_) => "UNKNOWN_ERROR",
        }
    }
    
    /// 获取错误来源模块
    pub fn module(&self) -> &str {
        match self {
            AppError::Core(_) => "core",
            AppError::Ai(_) => "ai",
            AppError::Config(_) => "config",
            AppError::File(_) => "file",
            AppError::Lsp(_) => "lsp",
            AppError::Network(_) => "network",
            AppError::Parser(_) => "parser",
            AppError::Semantic(_) => "semantic",
            AppError::Unknown(_) => "unknown",
        }
    }
    

}

// Unified Result type aliases
pub type CoreResult<T> = Result<T, core_error::CoreError>;
pub type ParserResult<T> = Result<T, parser_error::ParserError>;
pub type SemanticResult<T> = Result<T, semantic_error::SemanticError>;
pub type AiResult<T> = Result<T, ai_error::AiError>;
pub type LspResult<T> = Result<T, lsp_error::LspError>;
pub type FileResult<T> = Result<T, file_error::FileError>;
pub type ConfigResult<T> = Result<T, config_error::ConfigError>;
pub type NetworkResult<T> = Result<T, network_error::NetworkError>;

// 统一结果类型
pub type AppResult<T> = Result<T, AppError>;

// Unified error type for trait objects
pub type UnifiedError = Box<dyn std::error::Error + Send + Sync>;
pub type UnifiedResult<T> = Result<T, UnifiedError>; 