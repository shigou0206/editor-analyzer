pub mod core_error;
pub mod parser_error;
pub mod semantic_error;
pub mod ai_error;
pub mod lsp_error;
pub mod file_error;
pub mod config_error;
pub mod network_error;

pub use core_error::*;
pub use parser_error::*;
pub use semantic_error::*;
pub use ai_error::*;
pub use lsp_error::*;
pub use file_error::*;
pub use config_error::*;
pub use network_error::*;

// Unified Result type aliases
pub type CoreResult<T> = Result<T, core_error::CoreError>;
pub type ParserResult<T> = Result<T, parser_error::ParserError>;
pub type SemanticResult<T> = Result<T, semantic_error::SemanticError>;
pub type AiResult<T> = Result<T, ai_error::AiError>;
pub type LspResult<T> = Result<T, lsp_error::LspError>;
pub type FileResult<T> = Result<T, file_error::FileError>;
pub type ConfigResult<T> = Result<T, config_error::ConfigError>;
pub type NetworkResult<T> = Result<T, network_error::NetworkError>; 