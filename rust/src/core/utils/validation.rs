// Validation utilities
pub struct ValidationUtils;

impl ValidationUtils {
    /// Validate span is within text bounds
    pub fn validate_span(span: &crate::core::types::Span, text_len: usize) -> bool {
        span.start <= span.end && span.end <= text_len
    }

    /// Validate position is within text bounds
    pub fn validate_position(position: &crate::core::types::Position, text: &str) -> bool {
        let line_count = crate::core::utils::text::TextUtils::count_lines(text);
        position.line < line_count
    }

    /// Validate file ID format
    pub fn validate_file_id(file_id: &str) -> bool {
        !file_id.is_empty() && !file_id.contains('\0')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_span() {
        let text = "Hello World";
        let span = crate::core::types::Span::new(0, 5);
        
        assert!(ValidationUtils::validate_span(&span, text.len()));
    }

    #[test]
    fn test_validate_position() {
        let text = "Hello World";
        let position = crate::core::types::Position::new(0, 0);
        
        assert!(ValidationUtils::validate_position(&position, text));
    }

    #[test]
    fn test_validate_file_id() {
        assert!(ValidationUtils::validate_file_id("test.py"));
        assert!(!ValidationUtils::validate_file_id(""));
    }

    #[test]
    fn test_validation_edge_cases() {
        // Test span validation
        let text = "Hello";
        let valid_span = crate::core::types::Span::new(0, 5);
        let invalid_span1 = crate::core::types::Span::new(10, 15); // Out of bounds
        let invalid_span2 = crate::core::types::Span::new(5, 3); // Start > end
        
        assert!(ValidationUtils::validate_span(&valid_span, text.len()));
        assert!(!ValidationUtils::validate_span(&invalid_span1, text.len()));
        assert!(!ValidationUtils::validate_span(&invalid_span2, text.len()));
        
        // Test position validation
        let multi_line_text = "Line 1\nLine 2\nLine 3";
        let valid_pos = crate::core::types::Position::new(1, 2);
        let invalid_pos = crate::core::types::Position::new(10, 0); // Line out of bounds
        
        assert!(ValidationUtils::validate_position(&valid_pos, multi_line_text));
        assert!(!ValidationUtils::validate_position(&invalid_pos, multi_line_text));
        
        // Test file ID validation
        assert!(ValidationUtils::validate_file_id("test.py"));
        assert!(ValidationUtils::validate_file_id("path/to/file.rs"));
        assert!(!ValidationUtils::validate_file_id("")); // Empty
        assert!(!ValidationUtils::validate_file_id("file\0with\0nulls")); // Contains null bytes
    }
}
