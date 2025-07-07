use thiserror::Error;

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

    #[test]
    fn test_network_error() {
        let timeout = NetworkError::Timeout { code: "timeout" };
        assert!(timeout.to_string().contains("Connection timeout"));
        assert_eq!(timeout.code(), "timeout");

        let connection_refused = NetworkError::ConnectionRefused { code: "connection_refused" };
        assert!(connection_refused.to_string().contains("Connection refused"));
        assert_eq!(connection_refused.code(), "connection_refused");

        let dns_error = NetworkError::DnsResolutionFailed { code: "dns_resolution_failed" };
        assert!(dns_error.to_string().contains("DNS resolution failed"));
        assert_eq!(dns_error.code(), "dns_resolution_failed");

        let http_error = NetworkError::HttpError { code: "http_error", status: 404 };
        assert!(http_error.to_string().contains("HTTP error"));
        assert!(http_error.to_string().contains("404"));
        assert_eq!(http_error.code(), "http_error");

        let tls_error = NetworkError::TlsError {
            code: "tls_error",
            message: "Certificate error".to_string(),
        };
        assert!(tls_error.to_string().contains("SSL/TLS error"));
        assert!(tls_error.to_string().contains("Certificate error"));
        assert_eq!(tls_error.code(), "tls_error");
    }
} 