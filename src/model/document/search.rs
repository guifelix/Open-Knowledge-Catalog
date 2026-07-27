//! Search-related types for the Open Knowledge Catalog.
//!
//! This module contains types for search requests, responses,
//! and metadata query results.

use serde::{Deserialize, Serialize};

/// Search result entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// Document path.
    pub path: String,
    /// Optional title.
    pub title: Option<String>,
    /// Optional concept type.
    #[serde(rename = "type")]
    pub concept_type: Option<String>,
    /// Relevance score.
    pub score: f64,
    /// Matching section if applicable.
    pub matching_section: Option<String>,
    /// Text excerpt around match.
    pub excerpt: String,
}

/// Search response with results and metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    /// Matching documents.
    pub results: Vec<SearchResult>,
    /// Total matches (may exceed results due to limit).
    pub total_matches: usize,
    /// Whether results were truncated.
    pub truncated: bool,
}

/// Metadata query response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataQueryResponse {
    /// Query results as JSON values.
    pub results: Vec<serde_json::Value>,
    /// Total matches.
    pub total_matches: usize,
    /// Whether results were truncated.
    pub truncated: bool,
}
