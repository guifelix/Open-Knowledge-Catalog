//! Index validation and integrity checks.
//!
//! This module provides comprehensive validation of the repository index,
//! including:
//! - Broken link detection (internal and external)
//! - Scan error reporting
//! - Missing index file detection
//! - File encoding and size validation
//! - YAML front-matter validation
//! - Metadata completeness checks
//! - Duplicate concept/content detection
//! - Circular reference detection in the link graph

mod checks;

use crate::error::Result;
use std::collections::HashMap;

use super::database::RepositoryIndex;
use crate::index::traits::GraphStore;
use crate::model::document::{
    CheckResult, CheckStatus, ValidationIssue, ValidationReport, ValidationSummary,
};

/// Ordered list of validation check names.
const CHECKS: &[&str] = &[
    "broken_links",
    "scan_errors",
    "missing_index_files",
    "unsupported_encoding",
    "oversized_frontmatter",
    "invalid_yaml",
    "missing_type",
    "duplicate_concept",
    "duplicate_content",
    "circular_references",
    "reserved_file_frontmatter",
];

impl RepositoryIndex {
    /// Run all validation checks and return a flat list of issues.
    pub fn validate(&self) -> Result<Vec<ValidationIssue>> {
        Ok(self.validate_report()?.issues)
    }

    /// Run all validation checks and return a structured report.
    pub fn validate_report(&self) -> Result<ValidationReport> {
        let mut issues = Vec::new();

        if let Some(ref gs) = self.graph_store {
            issues.extend(gs.validate_links()?);
        }

        issues.extend(self.check_missing_index_files());

        issues.extend(self.validate_files());

        if let Some(ref gs) = self.graph_store {
            issues.extend(gs.detect_circular_references()?);
        }

        Ok(self.build_report(issues))
    }

    #[allow(dead_code)]
    /// Run incremental validation using previous content hashes.
    ///
    /// Only re-validates files that have changed since the last validation,
    /// using content hashes to detect modifications. Skips unchanged files
    /// for performance.
    pub fn validate_incremental(
        &self,
        previous_hashes: Option<&HashMap<String, String>>,
    ) -> Result<ValidationReport> {
        let mut issues = Vec::new();

        if let Some(ref gs) = self.graph_store {
            issues.extend(gs.validate_links()?);
            issues.extend(gs.detect_circular_references()?);
        }

        issues.extend(self.check_missing_index_files());

        issues.extend(self.validate_files_changed(previous_hashes));

        Ok(self.build_report(issues))
    }

    fn build_report(&self, issues: Vec<ValidationIssue>) -> ValidationReport {
        let total = issues.len();
        let errors = issues.iter().filter(|i| i.severity == "error").count();
        let warnings = issues.iter().filter(|i| i.severity == "warning").count();
        let infos = issues.iter().filter(|i| i.severity == "info").count();

        let checks: Vec<CheckResult> = CHECKS
            .iter()
            .map(|name| {
                let count = issues
                    .iter()
                    .filter(|i| i.category.as_str() == *name)
                    .count();
                CheckResult {
                    check_name: name.to_string(),
                    status: if count == 0 {
                        CheckStatus::Pass
                    } else if *name == "broken_links"
                        || *name == "missing_index_files"
                        || *name == "reserved_file_frontmatter"
                        || *name == "circular_references"
                    {
                        CheckStatus::Warn
                    } else {
                        CheckStatus::Fail
                    },
                    issue_count: count,
                }
            })
            .collect();

        ValidationReport {
            summary: ValidationSummary {
                total_issues: total,
                errors,
                warnings,
                infos,
                checks,
            },
            issues,
        }
    }
}
