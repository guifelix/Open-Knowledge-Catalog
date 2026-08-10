//! Search and metadata query service.
//!
//! Provides full-text search with filtering and structured metadata queries.

use std::collections::HashMap;

use crate::error::Result;
use crate::model::document::{MetadataQueryResponse, SearchResponse, SearchResult};
use crate::service::OkcService;

impl OkcService {
    /// Get headings for a document by path, filtered by depth and count.
    ///
    /// Returns empty vec if:
    /// - Document not found
    /// - Document has no body/headings
    /// - `heading_depth` is 0
    /// - No headings match the depth filter
    pub fn get_document_headings(
        &self,
        path: &str,
        heading_depth: u32,
        max_headings: usize,
    ) -> Vec<String> {
        // Return empty for edge cases per AC #2, #3
        if heading_depth == 0 || max_headings == 0 {
            return Vec::new();
        }

        self.index
            .get_headings_by_path(path, heading_depth, max_headings)
            .unwrap_or_default()
    }

    /// Full-text search across indexed documents.
    ///
    /// - `query`: Search query string (FTS5 syntax supported)
    /// - `path_prefix`: Optional path prefix to restrict search scope
    /// - `types`: Optional concept types to filter by
    /// - `tags`: Optional tags to filter by
    /// - `limit`: Maximum results to return
    /// - `max_headings`: Maximum headings per result (uses config default if None)
    /// - `heading_depth`: Maximum heading depth to include (uses config default if None)
    /// - `root_id`: Optional root ID to filter by (for multi-root repositories)
    #[allow(clippy::too_many_arguments)]
    pub fn search(
        &self,
        query: &str,
        path_prefix: Option<&str>,
        types: Option<&[String]>,
        tags: Option<&[String]>,
        limit: usize,
        max_headings: Option<usize>,
        heading_depth: Option<u32>,
        root_id: Option<i64>,
    ) -> Result<SearchResponse> {
        // Apply fallback chain: per-request > config > hard default (1)
        let max_headings = max_headings.unwrap_or(self.index.config.search.max_headings);
        let heading_depth = heading_depth.unwrap_or(self.index.config.search.heading_depth);

        let mut response = self
            .index
            .search(query, path_prefix, types, tags, limit, root_id)?;

        // Populate headings for each result
        for result in &mut response.results {
            result.headings = self.get_document_headings(&result.path, heading_depth, max_headings);
        }

        Ok(response)
    }

    /// Structured metadata query with filtering and projection.
    ///
    /// - `filters`: Key-value pairs to match against front-matter fields
    /// - `select`: Fields to return (empty = all default fields)
    /// - `limit`: Maximum rows to return
    pub fn query_metadata(
        &self,
        filters: &HashMap<String, serde_json::Value>,
        select: &[String],
        limit: usize,
    ) -> Result<MetadataQueryResponse> {
        self.index.query_metadata(filters, select, limit)
    }
}
