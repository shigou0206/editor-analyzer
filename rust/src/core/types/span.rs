use serde::{Deserialize, Serialize};

/// 文本位置 (行, 列)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

/// 文本范围 (起始位置到结束位置)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextRange {
    pub start: Position,
    pub end: Position,
}

impl TextRange {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }

    pub fn contains(&self, position: &Position) -> bool {
        position >= &self.start && position <= &self.end
    }
}

/// 字节范围 (用于 Tree-sitter)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position() {
        let pos = Position::new(1, 5);
        assert_eq!(pos.line, 1);
        assert_eq!(pos.column, 5);
        
        let pos2 = Position::new(1, 10);
        assert!(pos < pos2);
    }

    #[test]
    fn test_text_range() {
        let start = Position::new(1, 0);
        let end = Position::new(1, 10);
        let range = TextRange::new(start.clone(), end.clone());
        
        assert_eq!(range.start, start);
        assert_eq!(range.end, end);
        
        let inside = Position::new(1, 5);
        let outside = Position::new(2, 0);
        
        assert!(range.contains(&inside));
        assert!(!range.contains(&outside));
    }

    #[test]
    fn test_span() {
        let span = Span::new(0, 10);
        assert_eq!(span.start, 0);
        assert_eq!(span.end, 10);
        assert_eq!(span.len(), 10);
        assert!(!span.is_empty());
        
        let empty_span = Span::new(5, 5);
        assert!(empty_span.is_empty());
        assert_eq!(empty_span.len(), 0);
    }
} 