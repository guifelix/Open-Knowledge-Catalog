//! Index validation and statistics service.
//!
//! Runs comprehensive validation checks and provides index statistics.

use crate::model::document::{IndexStats, ValidationIssue, ValidationReport};
use crate::service::OkcService;

impl OkcService {
    /// Run all validation checks and return flat list of issues.
    pub fn validate(&self) -> Result<Vec<ValidationIssue>, anyhow::Error> {
        self.index.validate()
    }

    /// Run all validation checks and return structured report with summary.
    pub fn validate_report(&self) -> Result<ValidationReport, anyhow::Error> {
        self.index.validate_report()
    }

    /// Get index statistics (document count, link count, etc.).
    pub fn get_stats(&self) -> Result<IndexStats, anyhow::Error> {
        self.index.get_stats()
    }
}
