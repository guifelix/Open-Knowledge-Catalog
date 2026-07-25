//! Document retrieval service.
//!
//! Provides detailed document access including full content, specific sections,
//! and recently modified listings.

use crate::model::document::{DocumentDetail, DocumentSummary};
use crate::service::OkcService;

impl OkcService {
    /// Get a document by path with optional section inclusion and truncation.
    ///
    /// - `include`: Section names to include (empty = all)
    /// - `max_chars`: Maximum characters for body content
    pub fn get_document(
        &self,
        path: &str,
        include: &[String],
        max_chars: usize,
    ) -> Result<DocumentDetail, anyhow::Error> {
        self.index.get_document(path, include, max_chars)
    }

    /// Get a specific section from a document by heading title or anchor.
    ///
    /// Returns `(heading_title, section_content)` if found.
    pub fn get_section(
        &self,
        path: &str,
        heading: &str,
        max_chars: usize,
    ) -> Result<Option<(String, String)>, anyhow::Error> {
        self.index.get_section(path, heading, max_chars)
    }

    /// Get recently modified documents.
    ///
    /// Returns lightweight summaries sorted by modification time (newest first).
    #[allow(dead_code)]
    pub fn get_recently_modified(
        &self,
        limit: usize,
    ) -> Result<Vec<DocumentSummary>, anyhow::Error> {
        self.index.get_recently_modified(limit)
    }
}
