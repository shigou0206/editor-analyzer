pub mod types;
pub mod traits;
pub mod errors;
pub mod common;
pub mod utils;

// 按功能分区的公共 API 导出
// 避免深层 re-export，只导出必要的接口

// 1. 基础类型导出
pub use types::{
    Span,
    Position,
    TextRange,
    Language,
    FileId,
    TextDocument,
    SourceCode,
    FileContext,
};

// 2. 符号系统导出
pub use types::{
    Symbol,
    SymbolKind,
    Reference,
};

// 3. 诊断系统导出
pub use types::{
    Diagnostic,
    Severity,
    FixCommand,
    FixKind,
    TextEdit,
};

// 4. 核心 trait 导出 - 只导出主要接口
pub use traits::ast::{Ast, AstNode, CodeParser};
pub use traits::symbol::{SymbolTable, SemanticAnalyzer};
pub use traits::ai::{AiProvider, ConcreteAiContext, ConcreteAiRequest, ConcreteAiResponse};
pub use traits::diagnostic::DiagnosticProvider;
pub use traits::cache::Cache;
pub use traits::object_pool::ObjectPool;
pub use traits::config::Config;

// 5. 错误类型导出
pub use errors::{AppError, AppResult, CoreResult, UnifiedResult, UnifiedError};

// 6. 工具类导出
pub use common::{MemoryCache, SimpleObjectPool, MemoryConfig, PerformanceTimer};
pub use utils::{TextUtils, HashUtils, ValidationUtils};

// 7. 预定义的结果类型别名
pub type ParserResult<T> = errors::ParserResult<T>;
pub type SemanticResult<T> = errors::SemanticResult<T>;
pub type AiResult<T> = errors::AiResult<T>;
pub type LspResult<T> = errors::LspResult<T>;
pub type FileResult<T> = errors::FileResult<T>;
pub type ConfigResult<T> = errors::ConfigResult<T>;
pub type NetworkResult<T> = errors::NetworkResult<T>; 