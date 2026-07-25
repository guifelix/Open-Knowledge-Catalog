//! Storage trait abstractions for testing and alternative backends.
//!
//! This module defines the core traits that abstract the storage layer:
//! - [`DocumentStore`] - CRUD operations for documents, tags, headings, links, metadata
//! - [`SearchIndex`] - Full-text search operations
//! - [`GraphStore`] - Graph link storage and traversal
//!
//! The default implementation uses SQLite (see [`SqliteDocumentStore`], [`SqliteSearchIndex`],
//! [`SqliteGraphStore`]). These traits enable in-memory implementations for testing
//! and future alternative storage backends.

#![allow(dead_code)]

use crate::model::document::{
    HeadingInfo, IndexStats, Link, LinkInfo, MetadataQueryResponse, ParseError, SearchResponse,
    ValidationIssue,
};
use crate::model::graph::TraverseResponse;
use std::collections::{BTreeMap, HashMap};

/// Document suitable for full-text search indexing.
pub struct SearchableDocument {
    /// Document path (relative to repository root)
    pub path: String,
    /// Optional document title from front-matter
    pub title: Option<String>,
    /// Optional description from front-matter
    pub description: Option<String>,
    /// Concatenated heading text for search
    pub headings: String,
    /// Document body text (markdown content)
    pub body: String,
    /// Optional concept type from front-matter
    pub concept_type: Option<String>,
}

/// Filters for search queries.
pub struct SearchFilters {
    /// Optional path prefix to restrict search scope
    pub path_prefix: Option<String>,
    /// Optional concept types to filter by
    pub concept_types: Option<Vec<String>>,
    /// Optional tags to filter by
    pub tags: Option<Vec<String>>,
}

/// Type alias for storage operation results.
pub type Result<T> = std::result::Result<T, anyhow::Error>;

/// Trait for document storage operations.
///
/// Provides CRUD operations for documents and associated data:
/// tags, headings, links, metadata fields, and scan errors.
pub trait DocumentStore: Send + Sync {
    /// Initialize the storage schema.
    fn init(&self) -> Result<()>;

    /// Insert or update a document record.
    fn upsert_document(&self, doc: &DocumentRecord) -> Result<()>;
    /// Retrieve a document by path.
    fn get_document(&self, path: &str) -> Result<Option<DocumentRecord>>;
    /// Delete a document by path.
    fn delete_document(&self, path: &str) -> Result<()>;
    /// List documents with optional path prefix filter.
    fn list_documents(
        &self,
        path_prefix: Option<&str>,
        limit: usize,
    ) -> Result<Vec<DocumentRecord>>;

    /// Insert tags for a document.
    fn insert_tags(&self, doc_id: i64, tags: &[String]) -> Result<()>;
    /// Get tags for a document.
    fn get_tags(&self, doc_id: i64) -> Result<Vec<String>>;
    /// Delete all tags for a document.
    fn delete_tags(&self, doc_id: i64) -> Result<()>;

    /// Insert headings for a document.
    fn insert_headings(&self, doc_id: i64, headings: &[HeadingInfo]) -> Result<()>;
    /// Get headings for a document.
    fn get_headings(&self, doc_id: i64) -> Result<Vec<HeadingInfo>>;
    /// Delete all headings for a document.
    fn delete_headings(&self, doc_id: i64) -> Result<()>;

    /// Insert links for a document.
    fn insert_links(&self, doc_id: i64, links: &[LinkInfo]) -> Result<()>;
    /// Get links for a document.
    fn get_links(&self, doc_id: i64) -> Result<Vec<LinkInfo>>;
    /// Delete all links for a document.
    fn delete_links(&self, doc_id: i64) -> Result<()>;

    /// Insert metadata fields for a document.
    fn insert_metadata_fields(
        &self,
        doc_id: i64,
        fields: &BTreeMap<String, serde_json::Value>,
    ) -> Result<()>;
    /// Get metadata fields for a document.
    fn get_metadata_fields(&self, doc_id: i64) -> Result<BTreeMap<String, serde_json::Value>>;
    /// Delete all metadata fields for a document.
    fn delete_metadata_fields(&self, doc_id: i64) -> Result<()>;

    /// Insert scan errors for a document.
    fn insert_scan_errors(&self, path: &str, errors: &[ParseError]) -> Result<()>;
    /// Get scan errors for a document.
    fn get_scan_errors(&self, path: &str) -> Result<Vec<ParseError>>;
    /// Delete scan errors for a document.
    fn delete_scan_errors(&self, path: &str) -> Result<()>;

    /// Query metadata with filters.
    fn query_metadata(
        &self,
        filters: &HashMap<String, String>,
        select: &[String],
        limit: usize,
    ) -> Result<MetadataQueryResponse>;
    /// Get index statistics.
    fn get_stats(&self) -> Result<IndexStats>;
}

/// Document record as stored in the database.
#[derive(Debug, Clone)]
pub struct DocumentRecord {
    /// Primary key
    pub id: i64,
    /// Document path (relative to repository root)
    pub path: String,
    /// Parent directory path
    pub parent_path: String,
    /// Optional title from front-matter
    pub title: Option<String>,
    /// Optional concept type from front-matter
    pub concept_type: Option<String>,
    /// Optional description from front-matter
    pub description: Option<String>,
    /// Full document body text
    pub body_text: String,
    /// File size in bytes
    pub file_size: u64,
    /// Last modified timestamp (Unix epoch seconds)
    pub modified_at: i64,
    /// Blake3 content hash for change detection
    pub content_hash: String,
    /// Parse status: "ok", "partial", or "failed"
    pub parse_status: String,
}

/// Trait for full-text search operations.
pub trait SearchIndex: Send + Sync {
    /// Initialize the search index schema.
    fn init(&self) -> Result<()>;

    /// Index a document for full-text search.
    fn index_document(&self, doc: &SearchableDocument) -> Result<()>;
    /// Remove a document from the search index.
    fn remove_document(&self, path: &str) -> Result<()>;
    /// Search the index with query and filters.
    fn search(&self, query: &str, filters: &SearchFilters, limit: usize) -> Result<SearchResponse>;

    /// Get search index statistics.
    fn stats(&self) -> Result<IndexStats>;
}

/// Trait for graph storage and traversal operations.
pub trait GraphStore: Send + Sync {
    /// Initialize the graph store schema.
    fn init(&self) -> Result<()>;

    /// Store links for a source document.
    fn store_links(&self, source_path: &str, links: &[Link]) -> Result<()>;
    /// Remove all links for a source document.
    fn remove_links(&self, source_path: &str) -> Result<()>;
    /// Get forward links from a document.
    fn get_links(&self, path: &str) -> Result<Vec<LinkInfo>>;
    /// Get backlinks to a document.
    fn get_backlinks(&self, path: &str, limit: usize) -> Result<Vec<LinkInfo>>;

    /// Traverse the graph from a starting node.
    fn traverse(
        &self,
        start: &str,
        relations: &[String],
        max_depth: usize,
        max_nodes: usize,
    ) -> Result<TraverseResponse>;

    /// Validate all links in the graph.
    fn validate_links(&self) -> Result<Vec<ValidationIssue>>;
    /// Detect circular references in the graph.
    fn detect_circular_references(&self) -> Result<Vec<ValidationIssue>>;
}
