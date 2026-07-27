//! Document record types for the Open Knowledge Catalog.
//!
//! This module contains types representing documents at different
//! levels of detail: file records from scanning, summaries for lists,
//! and detailed views for individual documents.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// File record from filesystem scanning.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileRecord {
    /// Relative path from repository root.
    pub path: String,
    /// Absolute filesystem path.
    pub absolute_path: String,
    /// File size in bytes.
    pub size: u64,
    /// Last modified timestamp (Unix epoch seconds).
    pub modified_at: i64,
}

/// Lightweight document summary for listings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentSummary {
    /// Document path.
    pub path: String,
    /// Optional title from front-matter.
    pub title: Option<String>,
    /// Optional concept type.
    #[serde(rename = "type")]
    pub concept_type: Option<String>,
    /// Optional description.
    pub description: Option<String>,
    /// Document tags.
    pub tags: Vec<String>,
}

/// Detailed document view for API responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentDetail {
    /// Document path.
    pub path: String,
    /// Full metadata.
    pub metadata: DocumentMetadata,
    /// Heading hierarchy.
    pub headings: Vec<crate::model::document::content::HeadingInfo>,
    /// Optional body content (may be truncated).
    pub body: Option<String>,
    /// Whether body was truncated.
    pub truncated: bool,
    /// Any parse errors.
    pub errors: Vec<crate::model::document::frontmatter::ParseError>,
}

/// Document metadata for API responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetadata {
    /// Document title.
    pub title: Option<String>,
    /// Concept type.
    #[serde(rename = "type")]
    pub concept_type: Option<String>,
    /// Description.
    pub description: Option<String>,
    /// Tags.
    pub tags: Vec<String>,
    /// Custom front-matter fields.
    pub custom: BTreeMap<String, serde_json::Value>,
    /// File size in bytes.
    pub file_size: u64,
    /// Last modified timestamp.
    pub modified_at: i64,
    /// Parse status string.
    pub parse_status: String,
}
