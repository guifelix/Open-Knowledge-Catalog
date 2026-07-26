//! Storage layer for the Open Knowledge Catalog.
//!
//! This module provides the persistence layer including:
//!
//! - [`database`] - [`RepositoryIndex`]: Main SQLite-backed storage for documents, links, and metadata
//! - [`document_store`] - Document storage and retrieval operations
//! - [`graph`] - Graph data structures and algorithms
//! - [`graph_store`] - [`SqliteGraphStore`]: Graph persistence (links, traversal)
//! - [`search_index`] - Full-text search index using SQLite FTS5
//! - [`traits`] - Storage trait abstractions for testing and alternative backends
//! - [`queries`] - Complex query operations (search, metadata, graph traversal)
//! - [`validate`] - Index validation and integrity checks
//! - [`export`] - JSON export functionality
//! - [`migrations`] - Database schema migrations
//! - [`parser`] - Document parsing (front-matter, markdown, links)

pub mod content_hash;
pub mod database;
pub mod document_store;
pub mod export;
pub mod graph;
pub mod graph_store;
pub mod migrations;
pub mod parser;
pub mod queries;
pub mod search_index;
pub mod traits;
pub mod validate;

pub use database::RepositoryIndex;
