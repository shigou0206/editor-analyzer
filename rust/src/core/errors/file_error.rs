use thiserror::Error;
use crate::core::errors::codes;

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
    /// 构造函数，自动填充 code
    pub fn file_not_found(path: String) -> Self {
        FileError::FileNotFound {
            code: codes::file::FILE_NOT_FOUND,
            path,
        }
    }
    pub fn read_failed(path: String) -> Self {
        FileError::ReadFailed {
            code: codes::file::ALL,
            path,
        }
    }
    pub fn write_failed(path: String) -> Self {
        FileError::WriteFailed {
            code: codes::file::ALL,
            path,
        }
    }
    pub fn permission_denied(path: String) -> Self {
        FileError::PermissionDenied {
            code: codes::file::PERMISSION_DENIED,
            path,
        }
    }
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
    use crate::core::errors::codes;

    #[test]
    fn test_file_error() {
        let not_found = FileError::file_not_found("/test/file.txt".to_string());
        assert!(not_found.to_string().contains("File not found"));
        assert!(not_found.to_string().contains("/test/file.txt"));
        assert_eq!(not_found.code(), codes::file::FILE_NOT_FOUND);

        let read_failed = FileError::read_failed("/test/file.txt".to_string());
        assert!(read_failed.to_string().contains("File read failed"));
        assert!(read_failed.to_string().contains("/test/file.txt"));
        assert_eq!(read_failed.code(), codes::file::ALL);

        let write_failed = FileError::write_failed("/test/file.txt".to_string());
        assert!(write_failed.to_string().contains("File write failed"));
        assert!(write_failed.to_string().contains("/test/file.txt"));
        assert_eq!(write_failed.code(), codes::file::ALL);

        let permission_denied = FileError::permission_denied("/test/file.txt".to_string());
        assert!(permission_denied.to_string().contains("Permission denied"));
        assert!(permission_denied.to_string().contains("/test/file.txt"));
        assert_eq!(permission_denied.code(), codes::file::PERMISSION_DENIED);
    }
} 