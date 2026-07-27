//! Front-matter parsing types for the Open Knowledge Catalog.
//!
//! This module contains types related to parsing and representing
//! YAML front-matter from markdown documents.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Parsed front-matter metadata from a document.
#[derive(Debug, Clone)]
pub struct FrontMatter {
    /// Document concept type (e.g., "concept", "reference", "tutorial").
    pub concept_type: Option<String>,
    /// Document title.
    pub title: Option<String>,
    /// Brief description.
    pub description: Option<String>,
    /// User-defined tags.
    pub tags: Vec<String>,
    /// Custom front-matter fields not recognized as standard keys.
    pub custom: BTreeMap<String, serde_json::Value>,
    /// Raw YAML content for debugging/re-processing.
    #[allow(dead_code)]
    pub raw_yaml: String,
}

/// Result of front-matter and markdown parsing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParseStatus {
    /// Parsed successfully with no errors.
    Ok,
    /// Parsed with warnings (e.g., invalid YAML but front-matter extracted).
    Partial,
    /// Failed to parse (e.g., missing closing delimiter).
    Failed,
}

/// A parse error with context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseError {
    /// Parsing stage where error occurred (e.g., "frontmatter", "yaml", "markdown").
    pub stage: String,
    /// Human-readable error message.
    pub message: String,
    /// Optional line number in source.
    pub line: Option<usize>,
}

/// A resource limit violation error with structured details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitError {
    /// The name of the limit that was exceeded (e.g., "max_file_size", "max_front_matter_size").
    pub limit_name: String,
    /// The configured limit value.
    pub limit_value: String,
    /// The actual value that exceeded the limit.
    pub actual_value: Option<String>,
    /// Human-readable error message.
    pub message: String,
    /// Error code for programmatic handling.
    pub code: String,
}

impl LimitError {
    /// Create a new limit error.
    pub fn new(limit_name: &str, limit_value: &str, message: &str) -> Self {
        Self {
            limit_name: limit_name.to_string(),
            limit_value: limit_value.to_string(),
            actual_value: None,
            message: message.to_string(),
            code: format!("LIMIT_EXCEEDED_{}", limit_name.to_uppercase()),
        }
    }

    /// Set the actual value that exceeded the limit.
    pub fn with_actual(mut self, actual: &str) -> Self {
        self.actual_value = Some(actual.to_string());
        self
    }
}

impl std::fmt::Display for LimitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} (limit: {}, code: {})",
            self.message, self.limit_value, self.code
        )
    }
}

impl std::error::Error for LimitError {}
