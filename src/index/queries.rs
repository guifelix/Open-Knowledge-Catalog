//! Query and search operations for RepositoryIndex.
//!
//! Public API: browse_directory, get_document, get_section, search,
//! query_metadata, get_recently_modified, get_stats.

pub mod browse;
pub mod document;
pub mod metadata;
pub mod stats;
pub mod suggest;

use crate::error::Result;
use crate::index::traits::{SearchFilters, SearchIndex};
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
    ) -> Result<BrowseResponse> {
        browse::browse_directory(self, path, depth, limit)
    }

    /// Check whether a document exists at the given path.
    pub fn document_exists(&self, doc_path: &str) -> Result<bool> {
        document::document_exists(self, doc_path)
    }

    /// Get a document by path with optional section inclusion and truncation.
    pub fn get_document(
        &self,
        doc_path: &str,
        include: &[String],
        max_body_chars: usize,
    ) -> Result<DocumentDetail> {
        document::get_document(self, doc_path, include, max_body_chars)
    }

    /// Get a specific section from a document by heading title or anchor slug.
    pub fn get_section(
        &self,
        doc_path: &str,
        heading: &str,
        max_chars: usize,
    ) -> Result<Option<(String, String)>> {
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
    ) -> Result<SearchResponse> {
        self.search_index.search(
            query,
            &SearchFilters {
                path_prefix: path_prefix.map(str::to_string),
                concept_types: types.map(<[String]>::to_vec),
                tags: tags.map(<[String]>::to_vec),
            },
            limit,
        )
    }

    /// Structured metadata query with filtering and projection.
    pub fn query_metadata(
        &self,
        filters: &std::collections::HashMap<String, serde_json::Value>,
        select: &[String],
        limit: usize,
    ) -> Result<MetadataQueryResponse> {
        metadata::query_metadata(self, filters, select, limit)
    }

    /// Get recently modified documents.
    pub fn get_recently_modified(&self, limit: usize) -> Result<Vec<DocumentSummary>> {
        stats::get_recently_modified(self, limit)
    }

    /// Get index statistics.
    pub fn get_stats(&self) -> Result<IndexStats> {
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
