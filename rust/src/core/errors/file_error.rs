use thiserror::Error;

/// File system error
#[derive(Error, Debug, PartialEq, Eq)]
pub enum FileError {
    #[error("File not found: {path}")]
    FileNotFound { code: &'static str, path: String },

    #[error("File read failed: {path}")]
    ReadFailed { code: &'static str, path: String },

    #[error("File write failed: {path}")]
    WriteFailed { code: &'static str, path: String },

    #[error("Permission denied: {path}")]
    PermissionDenied { code: &'static str, path: String },
}

impl FileError {
    /// Get the error code
    pub fn code(&self) -> &'static str {
        match self {
            FileError::FileNotFound { code, .. } => code,
            FileError::ReadFailed { code, .. } => code,
            FileError::WriteFailed { code, .. } => code,
            FileError::PermissionDenied { code, .. } => code,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_error() {
        let not_found = FileError::FileNotFound {
            code: "file_not_found",
            path: "/test/file.txt".to_string(),
        };
        assert!(not_found.to_string().contains("File not found"));
        assert!(not_found.to_string().contains("/test/file.txt"));
        assert_eq!(not_found.code(), "file_not_found");

        let read_failed = FileError::ReadFailed {
            code: "read_failed",
            path: "/test/file.txt".to_string(),
        };
        assert!(read_failed.to_string().contains("File read failed"));
        assert!(read_failed.to_string().contains("/test/file.txt"));
        assert_eq!(read_failed.code(), "read_failed");

        let write_failed = FileError::WriteFailed {
            code: "write_failed",
            path: "/test/file.txt".to_string(),
        };
        assert!(write_failed.to_string().contains("File write failed"));
        assert!(write_failed.to_string().contains("/test/file.txt"));
        assert_eq!(write_failed.code(), "write_failed");

        let permission_denied = FileError::PermissionDenied {
            code: "permission_denied",
            path: "/test/file.txt".to_string(),
        };
        assert!(permission_denied.to_string().contains("Permission denied"));
        assert!(permission_denied.to_string().contains("/test/file.txt"));
        assert_eq!(permission_denied.code(), "permission_denied");
    }
} 