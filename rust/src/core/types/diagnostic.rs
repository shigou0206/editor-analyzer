use serde::{Deserialize, Serialize};
use super::span::Span;

/// 诊断严重程度
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Severity {
    Error,
    Warning,
    Info,
    Hint,
}

/// 诊断信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Diagnostic {
    pub severity: Severity,
    pub message: String,
    pub span: Span,
    pub code: Option<String>,
    pub fixable: bool,
    pub suggestions: Vec<String>,
}

impl Diagnostic {
    pub fn new(severity: Severity, message: String, span: Span) -> Self {
        Self {
            severity,
            message,
            span,
            code: None,
            fixable: false,
            suggestions: Vec::new(),
        }
    }

    pub fn with_code(mut self, code: String) -> Self {
        self.code = Some(code);
        self
    }

    pub fn with_fixable(mut self, fixable: bool) -> Self {
        self.fixable = fixable;
        self
    }

    pub fn with_suggestions(mut self, suggestions: Vec<String>) -> Self {
        self.suggestions = suggestions;
        self
    }
}

/// 修复命令
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixCommand {
    pub title: String,
    pub kind: FixKind,
    pub edits: Vec<TextEdit>,
}

impl FixCommand {
    pub fn new(title: String, kind: FixKind, edits: Vec<TextEdit>) -> Self {
        Self {
            title,
            kind,
            edits,
        }
    }
}

/// 修复类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FixKind {
    Replace,
    Insert,
    Delete,
    Refactor,
}

/// 文本编辑
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextEdit {
    pub span: Span,
    pub new_text: String,
}

impl TextEdit {
    pub fn new(span: Span, new_text: String) -> Self {
        Self {
            span,
            new_text,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagnostic() {
        let span = Span::new(0, 10);
        let diagnostic = Diagnostic::new(
            Severity::Error,
            "Test error".to_string(),
            span,
        );
        
        assert_eq!(diagnostic.severity, Severity::Error);
        assert_eq!(diagnostic.message, "Test error");
        assert_eq!(diagnostic.span, span);
        assert!(!diagnostic.fixable);
        
        let diagnostic_with_code = diagnostic.with_code("E001".to_string());
        assert_eq!(diagnostic_with_code.code, Some("E001".to_string()));
        
        let diagnostic_fixable = diagnostic_with_code.with_fixable(true);
        assert!(diagnostic_fixable.fixable);
    }

    #[test]
    fn test_severity_ordering() {
        let severities = vec![
            Severity::Info,
            Severity::Error,
            Severity::Hint,
            Severity::Warning,
        ];
        let mut sorted = severities.clone();
        sorted.sort();
        
        assert_eq!(sorted, vec![
            Severity::Error,
            Severity::Warning,
            Severity::Info,
            Severity::Hint,
        ]);
    }

    #[test]
    fn test_fix_command() {
        let span = Span::new(0, 10);
        let edit = TextEdit::new(span, "new code".to_string());
        let fix_command = FixCommand::new(
            "Fix error".to_string(),
            FixKind::Replace,
            vec![edit.clone()],
        );
        
        assert_eq!(fix_command.title, "Fix error");
        assert_eq!(fix_command.kind, FixKind::Replace);
        assert_eq!(fix_command.edits.len(), 1);
        assert_eq!(fix_command.edits[0], edit);
    }

    #[test]
    fn test_text_edit() {
        let span = Span::new(0, 10);
        let edit = TextEdit::new(span, "new text".to_string());
        
        assert_eq!(edit.span, span);
        assert_eq!(edit.new_text, "new text");
    }
} 