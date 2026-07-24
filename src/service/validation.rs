use crate::model::document::{IndexStats, ValidationIssue, ValidationReport};
use crate::service::OkcService;

impl OkcService {
    pub fn validate(&self) -> Result<Vec<ValidationIssue>, anyhow::Error> {
        self.index.validate()
    }

    pub fn validate_report(&self) -> Result<ValidationReport, anyhow::Error> {
        self.index.validate_report()
    }

    pub fn get_stats(&self) -> Result<IndexStats, anyhow::Error> {
        self.index.get_stats()
    }
}
