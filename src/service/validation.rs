use crate::index::RepositoryIndex;
use crate::model::*;
use anyhow::Result;

pub fn validate(index: &RepositoryIndex) -> Result<Vec<ValidationIssue>> {
    index.validate()
}

pub fn validate_report(index: &RepositoryIndex) -> Result<ValidationReport> {
    index.validate_report()
}
