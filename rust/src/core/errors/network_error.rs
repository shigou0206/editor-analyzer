use thiserror::Error;
use crate::core::errors::codes;

/// Network error
#[derive(Error, Debug, PartialEq, Eq)]
pub enum NetworkError {
    #[error("Connection timeout")]
    Timeout { code: &'static str },

    #[error("Connection refused")]
    ConnectionRefused { code: &'static str },

    #[error("DNS resolution failed")]
    DnsResolutionFailed { code: &'static str },

    #[error("HTTP error: {status}")]
    HttpError { code: &'static str, status: u16 },

    #[error("SSL/TLS error: {message}")]
    TlsError { code: &'static str, message: String },
}

impl NetworkError {
    /// 构造函数，自动填充 code
    pub fn timeout() -> Self {
        NetworkError::Timeout {
            code: codes::network::TIMEOUT,
        }
    }
    pub fn connection_refused() -> Self {
        NetworkError::ConnectionRefused {
            code: codes::network::ALL,
        }
    }
    pub fn dns_resolution_failed() -> Self {
        NetworkError::DnsResolutionFailed {
            code: codes::network::ALL,
        }
    }
    pub fn http_error(status: u16) -> Self {
        NetworkError::HttpError {
            code: codes::network::ALL,
            status,
        }
    }
    pub fn tls_error(message: String) -> Self {
        NetworkError::TlsError {
            code: codes::network::ALL,
            message,
        }
    }
    /// Get the error code
    pub fn code(&self) -> &'static str {
        match self {
            NetworkError::Timeout { code } => code,
            NetworkError::ConnectionRefused { code } => code,
            NetworkError::DnsResolutionFailed { code } => code,
            NetworkError::HttpError { code, .. } => code,
            NetworkError::TlsError { code, .. } => code,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::errors::codes;

    #[test]
    fn test_network_error() {
        let timeout = NetworkError::timeout();
        assert!(timeout.to_string().contains("Connection timeout"));
        assert_eq!(timeout.code(), codes::network::TIMEOUT);

        let connection_refused = NetworkError::connection_refused();
        assert!(connection_refused.to_string().contains("Connection refused"));
        assert_eq!(connection_refused.code(), codes::network::ALL);

        let dns_error = NetworkError::dns_resolution_failed();
        assert!(dns_error.to_string().contains("DNS resolution failed"));
        assert_eq!(dns_error.code(), codes::network::ALL);

        let http_error = NetworkError::http_error(404);
        assert!(http_error.to_string().contains("HTTP error"));
        assert!(http_error.to_string().contains("404"));
        assert_eq!(http_error.code(), codes::network::ALL);

        let tls_error = NetworkError::tls_error("Certificate error".to_string());
        assert!(tls_error.to_string().contains("SSL/TLS error"));
        assert!(tls_error.to_string().contains("Certificate error"));
        assert_eq!(tls_error.code(), codes::network::ALL);
    }
} 