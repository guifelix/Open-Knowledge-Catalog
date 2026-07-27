//! Statistics types for the Open Knowledge Catalog.
//!
//! This module contains types for index statistics, scan results,
//! and processing outcomes.

use serde::{Deserialize, Serialize};

/// Index statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexStats {
    /// Total documents indexed.
    pub document_count: usize,
    /// Documents with parse errors.
    pub error_count: usize,
    /// Total links in index.
    pub link_count: usize,
    /// Total headings in index.
    pub heading_count: usize,
}

/// Result of a full repository scan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    /// Total files discovered.
    pub total_files: usize,
    /// Files added in this scan.
    pub added: usize,
    /// Files modified in this scan.
    pub modified: usize,
    /// Files deleted since last scan.
    pub deleted: usize,
    /// Files with parse failures.
    pub parse_failures: usize,
    /// Broken internal links found.
    pub broken_links: usize,
    /// Total links processed.
    pub total_links: usize,
    /// Scan duration in seconds.
    pub duration_secs: f64,
}

/// Result of incremental change processing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessChangesResult {
    /// Files added.
    pub files_added: usize,
    /// Files modified.
    pub files_modified: usize,
    /// Files deleted.
    pub files_deleted: usize,
    /// Parse failures encountered.
    pub parse_failures: usize,
    /// Broken links found.
    pub broken_links: usize,
    /// Total links processed.
    pub total_links: usize,
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
