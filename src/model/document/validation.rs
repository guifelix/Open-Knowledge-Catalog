//! Validation types for the Open Knowledge Catalog.
//!
//! This module contains types for document validation issues,
//! reports, and check results.

use serde::{Deserialize, Serialize};

/// A validation issue found during index validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationIssue {
    /// Document path where issue was found.
    pub path: String,
    /// Severity: "error", "warning", or "info".
    pub severity: String,
    /// Issue category.
    pub category: String,
    /// Human-readable description.
    pub message: String,
    /// Optional line number.
    pub line: Option<usize>,
}

/// Complete validation report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    /// Summary statistics.
    pub summary: ValidationSummary,
    /// All issues found.
    pub issues: Vec<ValidationIssue>,
}

/// Validation summary statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationSummary {
    /// Total number of issues.
    pub total_issues: usize,
    /// Number of errors.
    pub errors: usize,
    /// Number of warnings.
    pub warnings: usize,
    /// Number of infos.
    pub infos: usize,
    /// Per-check results.
    pub checks: Vec<CheckResult>,
}

/// Result of a single validation check.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckResult {
    /// Check name.
    pub check_name: String,
    /// Check status.
    pub status: CheckStatus,
    /// Number of issues found by this check.
    pub issue_count: usize,
}

/// Status of a validation check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckStatus {
    /// Check passed (no issues).
    Pass,
    /// Check found warnings.
    Warn,
    /// Check found errors.
    Fail,
}

impl std::fmt::Display for CheckStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CheckStatus::Pass => write!(f, "pass"),
            CheckStatus::Warn => write!(f, "warn"),
            CheckStatus::Fail => write!(f, "fail"),
        }
    }
}
