pub mod span;
pub mod language;
pub mod symbol;
pub mod diagnostic;
pub mod document;

// Re-export all types from submodules
pub use span::{Position, TextRange, Span};
pub use language::{Language, LanguageConfig};
pub use symbol::{Symbol, SymbolKind, Reference};
pub use diagnostic::{Diagnostic, Severity, FixCommand, FixKind, TextEdit};
pub use document::{FileId, TextDocument, SourceCode, FileContext}; 