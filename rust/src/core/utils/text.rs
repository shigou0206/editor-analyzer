// Text utilities
pub struct TextUtils;

impl TextUtils {
    /// Convert byte offset to line and column
    pub fn offset_to_position(text: &str, offset: usize) -> crate::core::types::Position {
        let mut line = 0;
        let mut column = 0;
        for (i, ch) in text.char_indices() {
            if i >= offset {
                break;
            }
            if ch == '\n' {
                line += 1;
                column = 0;
            } else {
                column += 1;
            }
        }
        crate::core::types::Position::new(line, column)
    }

    /// Convert line and column to byte offset
    pub fn position_to_offset(text: &str, position: &crate::core::types::Position) -> usize {
        let mut line = 0;
        let mut column = 0;
        for (i, ch) in text.char_indices() {
            if line == position.line && column == position.column {
                return i;
            }
            if ch == '\n' {
                line += 1;
                column = 0;
            } else {
                column += 1;
            }
        }
        text.len()
    }

    /// Get text slice by span
    pub fn get_text_slice<'a>(text: &'a str, span: &crate::core::types::Span) -> &'a str {
        let len = text.len();
        if span.start >= len || span.start >= span.end {
            return "";
        }
        let end = if span.end > len { len } else { span.end };
        &text[span.start..end]
    }

    /// Count lines in text
    pub fn count_lines(text: &str) -> usize {
        text.lines().count()
    }

    /// Get line at index
    pub fn get_line(text: &str, line_index: usize) -> Option<&str> {
        text.lines().nth(line_index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::Position;

    #[test]
    fn test_offset_to_position() {
        let text = "Hello\nWorld\nTest";
        let pos = TextUtils::offset_to_position(text, 6);
        assert_eq!(pos.line, 1);
        assert_eq!(pos.column, 0);
    }

    #[test]
    fn test_position_to_offset() {
        let text = "Hello\nWorld\nTest";
        let offset = TextUtils::position_to_offset(text, &Position::new(1, 0));
        assert_eq!(offset, 6);
    }

    #[test]
    fn test_count_lines() {
        let text = "Hello\nWorld\nTest";
        assert_eq!(TextUtils::count_lines(text), 3);
    }

    #[test]
    fn test_get_line() {
        let text = "Hello\nWorld\nTest";
        assert_eq!(TextUtils::get_line(text, 1), Some("World"));
    }

    #[test]
    fn test_get_text_slice() {
        let text = "Hello World";
        let span = crate::core::types::Span::new(0, 5);
        let slice = TextUtils::get_text_slice(text, &span);
        assert_eq!(slice, "Hello");
    }
}
