//! Directory browsing data structures.
//!
//! Types for representing directory hierarchies and browsing responses
//! in the knowledge base.

use serde::{Deserialize, Serialize};

/// A node in the directory tree.
///
/// Represents a directory with its index document (if any), child directories,
/// and contained documents.
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryNode {
    /// Relative path of this directory from repository root.
    pub path: String,
    /// Path to the index document (e.g., `index.md`) if present.
    pub index_document: Option<String>,
    /// Child directory paths.
    pub child_directories: Vec<String>,
    /// Documents directly in this directory.
    pub documents: Vec<DirectoryDocument>,
}

/// A document entry within a directory listing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryDocument {
    /// Document path relative to repository root.
    pub path: String,
    /// Optional title from front-matter.
    pub title: Option<String>,
    /// Optional concept type from front-matter.
    #[serde(rename = "type")]
    pub concept_type: Option<String>,
    /// Optional description from front-matter.
    pub description: Option<String>,
}

/// Response for directory browse operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowseResponse {
    /// The browsed directory path.
    pub path: String,
    /// Path to the directory's index/summary document if present.
    pub summary_document: Option<String>,
    /// Subdirectory paths.
    pub directories: Vec<String>,
    /// Documents in this directory.
    pub documents: Vec<DirectoryDocument>,
    /// Whether results were truncated due to limit.
    pub truncated: bool,
}
