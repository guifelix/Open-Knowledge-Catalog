//! Query and search operations for RepositoryIndex.
//!
//! Public API: browse_directory, get_document, get_section, search,
//! query_metadata, get_recently_modified, get_stats.

pub mod browse;
pub mod document;
pub mod metadata;
pub mod search;
pub mod stats;

use crate::model::directory::{BrowseResponse, DirectoryDocument};
use crate::model::document::{
    DocumentDetail, DocumentMetadata, DocumentSummary, HeadingInfo, IndexStats,
    MetadataQueryResponse, ParseError, SearchResponse, SearchResult,
};

impl super::database::RepositoryIndex {
    /// Browse a directory in the knowledge base.
    pub fn browse_directory(
        &self,
        path: &str,
        depth: usize,
        limit: usize,
    ) -> Result<BrowseResponse, anyhow::Error> {
        browse::browse_directory(self, path, depth, limit)
    }

    /// Get a document by path with optional section inclusion and truncation.
    pub fn get_document(
        &self,
        doc_path: &str,
        include: &[String],
        max_body_chars: usize,
    ) -> Result<DocumentDetail, anyhow::Error> {
        document::get_document(self, doc_path, include, max_body_chars)
    }

    /// Get a specific section from a document by heading title or anchor slug.
    pub fn get_section(
        &self,
        doc_path: &str,
        heading: &str,
        max_chars: usize,
    ) -> Result<Option<(String, String)>, anyhow::Error> {
        document::get_section(self, doc_path, heading, max_chars)
    }

    /// Full-text search across indexed documents.
    pub fn search(
        &self,
        query: &str,
        path_prefix: Option<&str>,
        types: Option<&[String]>,
        tags: Option<&[String]>,
        limit: usize,
    ) -> Result<SearchResponse, anyhow::Error> {
        search::search(self, query, path_prefix, types, tags, limit)
    }

    /// Structured metadata query with filtering and projection.
    pub fn query_metadata(
        &self,
        filters: &std::collections::HashMap<String, serde_json::Value>,
        limit: usize,
    ) -> Result<MetadataQueryResponse, anyhow::Error> {
        metadata::query_metadata(self, filters, limit)
    }

    /// Get recently modified documents.
    pub fn get_recently_modified(
        &self,
        limit: usize,
    ) -> Result<Vec<DocumentSummary>, anyhow::Error> {
        stats::get_recently_modified(self, limit)
    }

    /// Get index statistics.
    pub fn get_stats(&self) -> Result<IndexStats, anyhow::Error> {
        stats::get_stats(self)
    }
}

fn slugify(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == ' ' || *c == '-' || *c == '_')
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("-")
}