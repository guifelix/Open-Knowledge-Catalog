//! Structured error types for the Open Knowledge Catalog.
//!
//! This module provides a unified error hierarchy using `thiserror` for
//! machine-readable error codes and structured context.

use crate::config::ConfigError;
use crate::model::document::frontmatter::LimitError;
use std::path::PathBuf;
use thiserror::Error;

/// Result type alias using [`OkfError`].
pub type Result<T> = std::result::Result<T, OkfError>;

/// Main error type for the OKC library.
///
/// Each variant represents a distinct error category with structured context
/// for programmatic handling and user-friendly display.
#[derive(Error, Debug)]
pub enum OkfError {
    /// I/O error (file system, network, etc.)
    #[error("I/O error: {source}")]
    Io {
        #[source]
        source: std::io::Error,
        path: Option<PathBuf>,
    },

    /// Configuration error (validation, missing fields, etc.)
    #[error("Configuration error: {message}")]
    Config {
        message: String,
        field: Option<String>,
    },

    /// Configuration error from config module
    #[error("Configuration error: {0}")]
    ConfigError(#[from] ConfigError),

    /// Input validation error (user-provided data)
    #[error("Invalid input: {message}")]
    Validation {
        message: String,
        field: Option<String>,
        value: Option<String>,
    },

    /// Resource not found (document, file, directory)
    #[error("Not found: {resource}")]
    NotFound {
        resource: String,
        path: Option<PathBuf>,
    },

    /// Database/SQL error
    #[error("Database error: {message}")]
    Database {
        #[source]
        source: Option<rusqlite::Error>,
        message: String,
        query: Option<String>,
    },

    /// Serialization/deserialization error
    #[error("Serialization error: {message}")]
    Serde { message: String },

    /// Parsing error (markdown, YAML, frontmatter)
    #[error("Parse error: {message}")]
    Parse {
        message: String,
        path: Option<PathBuf>,
        line: Option<usize>,
    },

    /// Resource limit exceeded (file size, front-matter size, etc.)
    #[error("Limit exceeded: {0}")]
    Limit(LimitError),

    /// Internal/Invariant violation error (should not happen)
    #[error("Internal error: {message}")]
    Internal {
        message: String,
        context: Option<String>,
    },

    /// External service error (MCP transport, etc.)
    #[error("Transport error: {message}")]
    Transport { message: String, code: i32 },
}

impl OkfError {
    /// Create an I/O error with optional path context.
    pub fn io<E: Into<std::io::Error>>(source: E, path: Option<PathBuf>) -> Self {
        Self::Io {
            source: source.into(),
            path,
        }
    }

    /// Create a configuration error.
    pub fn config(message: impl Into<String>, field: Option<String>) -> Self {
        Self::Config {
            message: message.into(),
            field,
        }
    }

    /// Create a validation error.
    pub fn validation(
        message: impl Into<String>,
        field: Option<String>,
        value: Option<String>,
    ) -> Self {
        Self::Validation {
            message: message.into(),
            field,
            value,
        }
    }

    /// Create a not found error.
    pub fn not_found(resource: impl Into<String>, path: Option<PathBuf>) -> Self {
        Self::NotFound {
            resource: resource.into(),
            path,
        }
    }

    /// Create a database error.
    pub fn database(
        message: impl Into<String>,
        source: Option<rusqlite::Error>,
        query: Option<String>,
    ) -> Self {
        Self::Database {
            message: message.into(),
            source,
            query,
        }
    }

    /// Create a serialization error.
    pub fn serde(message: impl Into<String>) -> Self {
        Self::Serde {
            message: message.into(),
        }
    }

    /// Create a parse error.
    pub fn parse(message: impl Into<String>, path: Option<PathBuf>, line: Option<usize>) -> Self {
        Self::Parse {
            message: message.into(),
            path,
            line,
        }
    }

    /// Create an internal error.
    pub fn internal(message: impl Into<String>, context: Option<String>) -> Self {
        Self::Internal {
            message: message.into(),
            context,
        }
    }

    /// Create a transport error.
    pub fn transport(message: impl Into<String>, code: i32) -> Self {
        Self::Transport {
            message: message.into(),
            code,
        }
    }

    /// Get the MCP error code for this error variant.
    pub fn mcp_code(&self) -> i32 {
        match self {
            OkfError::Io { .. } => -32603,         // Internal error
            OkfError::Config { .. } => -32602,     // Invalid params
            OkfError::ConfigError(_) => -32602,    // Invalid params
            OkfError::Validation { .. } => -32602, // Invalid params
            OkfError::NotFound { .. } => -32602,   // Invalid params / Not found
            OkfError::Database { .. } => -32603,   // Internal error
            OkfError::Serde { .. } => -32603,      // Internal error
            OkfError::Parse { .. } => -32602,      // Invalid params
            OkfError::Limit(_) => -32602,          // Invalid params
            OkfError::Internal { .. } => -32603,   // Internal error
            OkfError::Transport { code, .. } => *code,
        }
    }
}

// Conversion from common error types

impl From<std::io::Error> for OkfError {
    fn from(err: std::io::Error) -> Self {
        OkfError::io(err, None)
    }
}

impl From<r2d2::Error> for OkfError {
    fn from(err: r2d2::Error) -> Self {
        OkfError::internal(err.to_string(), None)
    }
}

impl From<rusqlite::Error> for OkfError {
    fn from(err: rusqlite::Error) -> Self {
        OkfError::database(err.to_string(), Some(err), None)
    }
}

impl From<serde_json::Error> for OkfError {
    fn from(err: serde_json::Error) -> Self {
        OkfError::serde(err.to_string())
    }
}

impl From<LimitError> for OkfError {
    fn from(err: LimitError) -> Self {
        OkfError::Limit(err)
    }
}

impl From<std::num::TryFromIntError> for OkfError {
    fn from(err: std::num::TryFromIntError) -> Self {
        OkfError::internal(err.to_string(), None)
    }
}

impl From<anyhow::Error> for OkfError {
    fn from(err: anyhow::Error) -> Self {
        // Try to downcast to known types
        if let Some(io_err) = err.downcast_ref::<std::io::Error>() {
            return OkfError::io(std::io::Error::new(io_err.kind(), io_err.to_string()), None);
        }
        if let Some(sql_err) = err.downcast_ref::<rusqlite::Error>() {
            return OkfError::database(sql_err.to_string(), None, None);
        }
        if let Some(ser_err) = err.downcast_ref::<serde_json::Error>() {
            return OkfError::serde(ser_err.to_string());
        }
        OkfError::internal(err.to_string(), None)
    }
}

// Conversion from String (for legacy code migration)
impl From<String> for OkfError {
    fn from(err: String) -> Self {
        OkfError::internal(err, None)
    }
}

impl From<&str> for OkfError {
    fn from(err: &str) -> Self {
        OkfError::internal(err.to_string(), None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    use std::num::TryFromIntError;

    #[test]
    fn test_error_construction() {
        let err = OkfError::validation(
            "invalid field",
            Some("name".to_string()),
            Some("value".to_string()),
        );
        assert!(matches!(err, OkfError::Validation { .. }));
    }

    #[test]
    fn test_mcp_codes() {
        assert_eq!(OkfError::validation("x", None, None).mcp_code(), -32602);
        assert_eq!(OkfError::not_found("doc", None).mcp_code(), -32602);
        assert_eq!(OkfError::internal("x", None).mcp_code(), -32603);
        assert_eq!(OkfError::config("x", None).mcp_code(), -32602);
        assert_eq!(OkfError::database("x", None, None).mcp_code(), -32603);
        assert_eq!(OkfError::serde("x").mcp_code(), -32603);
        assert_eq!(OkfError::parse("x", None, None).mcp_code(), -32602);
        assert_eq!(
            OkfError::Limit(crate::model::document::frontmatter::LimitError::new(
                "max_file_size",
                "100",
                "too big"
            ))
            .mcp_code(),
            -32602
        );
        assert_eq!(OkfError::transport("x", -32000).mcp_code(), -32000);
    }

    #[test]
    fn test_from_string() {
        let err: OkfError = "test error".into();
        assert!(matches!(err, OkfError::Internal { .. }));
    }

    #[test]
    fn test_from_io_error() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let err: OkfError = io_err.into();
        assert!(matches!(err, OkfError::Io { .. }));
    }

    #[test]
    fn test_from_rusqlite_error() {
        // Can't easily construct rusqlite::Error, test the conversion path exists
        // by checking the From impl compiles
    }

    #[test]
    fn test_from_serde_json_error() {
        let json_err = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
        let err: OkfError = json_err.into();
        assert!(matches!(err, OkfError::Serde { .. }));
    }

    #[test]
    fn test_from_limit_error() {
        let limit_err = crate::model::document::frontmatter::LimitError::new(
            "max_file_size",
            "100",
            "file too large",
        )
        .with_actual("200");
        let err: OkfError = limit_err.into();
        assert!(matches!(err, OkfError::Limit(_)));
    }

    #[test]
    fn test_from_try_from_int_error() {
        let int_err = u32::try_from(-1i32).unwrap_err();
        let err: OkfError = int_err.into();
        assert!(matches!(err, OkfError::Internal { .. }));
    }

    #[test]
    fn test_error_display() {
        let err = OkfError::validation(
            "bad input",
            Some("field".to_string()),
            Some("value".to_string()),
        );
        let display = err.to_string();
        assert!(display.contains("Invalid input: bad input"));
    }

    #[test]
    fn test_error_debug() {
        let err = OkfError::not_found("document", Some(std::path::PathBuf::from("/test.md")));
        let debug = format!("{:?}", err);
        assert!(debug.contains("NotFound"));
        assert!(debug.contains("/test.md"));
    }

    #[test]
    fn test_config_error_conversion() {
        let config_err = crate::config::ConfigError::ValidationError("invalid".to_string());
        let err: OkfError = config_err.into();
        assert!(matches!(err, OkfError::ConfigError(_)));
    }
}
