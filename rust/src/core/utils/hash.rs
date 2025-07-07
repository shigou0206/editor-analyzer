// Hash utilities
pub struct HashUtils;

impl HashUtils {
    /// Generate hash for text content
    pub fn hash_text(text: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        text.hash(&mut hasher);
        hasher.finish()
    }

    /// Generate hash for file content (for caching)
    pub fn hash_file_content(content: &str, _language: &crate::core::types::Language) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_text() {
        let text1 = "Hello World";
        let text2 = "Hello World";
        let text3 = "Different text";
        
        let hash1 = HashUtils::hash_text(text1);
        let hash2 = HashUtils::hash_text(text2);
        let hash3 = HashUtils::hash_text(text3);
        
        assert_eq!(hash1, hash2); // Same text should have same hash
        assert_ne!(hash1, hash3); // Different text should have different hash
    }

    #[test]
    fn test_hash_file_content() {
        let text = "Hello World";
        let file_hash1 = HashUtils::hash_file_content(text, &crate::core::types::Language::Python);
        let file_hash2 = HashUtils::hash_file_content(text, &crate::core::types::Language::Python);
        let file_hash3 = HashUtils::hash_file_content(text, &crate::core::types::Language::JavaScript);
        
        assert_eq!(file_hash1, file_hash2); // Same content should have same hash
        assert_eq!(file_hash1, file_hash3); // Language shouldn't affect hash (currently)
        assert!(!file_hash1.is_empty());
        assert!(file_hash1.chars().all(|c| c.is_ascii_hexdigit()));
    }
}
