//! Search and metadata query service.
//!
//! Provides full-text search with filtering and structured metadata queries.

use std::collections::HashMap;

use crate::model::document::{MetadataQueryResponse, SearchResponse};
use crate::service::OkcService;

impl OkcService {
    /// Full-text search across indexed documents.
    ///
    /// - `query`: Search query string (FTS5 syntax supported)
    /// - `path_prefix`: Optional path prefix to restrict search scope
    /// - `types`: Optional concept types to filter by
    /// - `tags`: Optional tags to filter by
    /// - `limit`: Maximum results to return
    pub fn search(
        &self,
        query: &str,
        path_prefix: Option<&str>,
        types: Option<&[String]>,
        tags: Option<&[String]>,
        limit: usize,
    ) -> Result<SearchResponse, anyhow::Error> {
        self.index.search(query, path_prefix, types, tags, limit)
    }

    /// Structured metadata query with filtering.
    ///
    /// - `filters`: Key-value pairs to match against front-matter fields
    /// - `limit`: Maximum rows to return
    pub fn query_metadata(
        &self,
        filters: &HashMap<String, serde_json::Value>,
        limit: usize,
    ) -> Result<MetadataQueryResponse, anyhow::Error> {
        self.index.query_metadata(filters, limit)
    }
}
