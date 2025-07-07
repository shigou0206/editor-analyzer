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
        let mut count = 0;
        for line_text in text.lines() {
            if !line_text.trim().is_empty() {
                count += 1;
            }
        }
        count
    }

    /// Get line at index
    pub fn get_line(text: &str, line_index: usize) -> Option<&str> {
        text.lines().nth(line_index)
    }

    /// Convert UTF-8 position to UTF-16 position (for LSP compatibility)
    pub fn position_utf8_to_utf16(text: &str, position: &crate::core::types::Position) -> crate::core::types::Position {
        let mut utf16_line = 0;
        let mut utf16_column = 0;
        for (line_idx, line) in text.lines().enumerate() {
            if line_idx == position.line {
                // Found the target line, now count UTF-16 characters
                for (char_idx, ch) in line.char_indices() {
                    if char_idx == position.column {
                        break;
                    }
                    utf16_column += ch.len_utf16();
                }
                break;
            }
            utf16_line += 1;
        }
        
        crate::core::types::Position::new(utf16_line, utf16_column)
    }

    /// Convert UTF-16 position to UTF-8 position (for LSP compatibility)
    pub fn position_utf16_to_utf8(text: &str, position: &crate::core::types::Position) -> crate::core::types::Position {
        let mut utf8_line = 0;
        let mut utf8_column = 0;
        let mut utf16_column = 0;
        
        for (line_idx, line) in text.lines().enumerate() {
            if line_idx == position.line {
                // Found the target line, now count UTF-8 characters
                for (char_idx, ch) in line.char_indices() {
                    if utf16_column >= position.column {
                        break;
                    }
                    utf8_column = char_idx;
                    utf16_column += ch.len_utf16();
                }
                break;
            }
            utf8_line += 1;
        }
        
        crate::core::types::Position::new(utf8_line, utf8_column)
    }

    /// Convert UTF-8 span to UTF-16 span (for LSP compatibility)
    pub fn span_utf8_to_utf16(text: &str, span: &crate::core::types::Span) -> crate::core::types::Span {
        let start_pos = Self::offset_to_position(text, span.start);
        let end_pos = Self::offset_to_position(text, span.end);
        
        let start_utf16 = Self::position_utf8_to_utf16(text, &start_pos);
        let end_utf16 = Self::position_utf8_to_utf16(text, &end_pos);
        
        // Convert positions back to offsets for UTF-16 span
        let start_offset = Self::position_to_offset_utf16(text, &start_utf16);
        let end_offset = Self::position_to_offset_utf16(text, &end_utf16);
        
        crate::core::types::Span::new(start_offset, end_offset)
    }

    /// Convert UTF-16 span to UTF-8 span (for LSP compatibility)
    pub fn span_utf16_to_utf8(text: &str, span: &crate::core::types::Span) -> crate::core::types::Span {
        let start_pos = Self::offset_to_position_utf16(text, span.start);
        let end_pos = Self::offset_to_position_utf16(text, span.end);
        
        let start_utf8 = Self::position_utf16_to_utf8(text, &start_pos);
        let end_utf8 = Self::position_utf16_to_utf8(text, &end_pos);
        
        // Convert positions back to offsets for UTF-8 span
        let start_offset = Self::position_to_offset(text, &start_utf8);
        let end_offset = Self::position_to_offset(text, &end_utf8);
        
        crate::core::types::Span::new(start_offset, end_offset)
    }

    /// Convert line and column to UTF-16 byte offset
    pub fn position_to_offset_utf16(text: &str, position: &crate::core::types::Position) -> usize {
        let mut utf16_offset = 0;
        
        for (line_idx, line_text) in text.lines().enumerate() {
            if line_idx == position.line {
                // Found the target line, count UTF-16 characters
                for (char_idx, ch) in line_text.char_indices() {
                    if char_idx == position.column {
                        break;
                    }
                    utf16_offset += ch.len_utf16();
                }
                break;
            }
            utf16_offset += line_text.chars().map(|ch| ch.len_utf16()).sum::<usize>();
        }
        
        utf16_offset
    }

    /// Convert UTF-16 byte offset to line and column
    pub fn offset_to_position_utf16(text: &str, offset: usize) -> crate::core::types::Position {
        let mut line = 0;
        let mut column = 0;
        let mut utf16_offset = 0;
        
        for line_text in text.lines() {
            for (char_idx, ch) in line_text.char_indices() {
                if utf16_offset >= offset {
                    return crate::core::types::Position::new(line, column);
                }
                column = char_idx;
                utf16_offset += ch.len_utf16();
            }
            line += 1;
            column = 0;
        }
        
        crate::core::types::Position::new(line, column)
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
