//! Search-related types for the Open Knowledge Catalog.
//!
//! This module contains types for search requests, responses,
//! and metadata query results.

use serde::{Deserialize, Serialize};

/// Derive a human-readable display title from a concept's path.
///
/// Used when the frontmatter `title` field is absent. Per the OKF v0.2 spec (§4.1):
/// "If omitted, consumers MAY derive a title from the filename."
///
/// The derivation takes the filename (last path segment), strips `.md`,
/// replaces `-` and `_` with spaces, and capitalises the first letter.
pub fn derive_display_title(path: &str, title: Option<&str>) -> String {
    match title {
        Some(t) if !t.is_empty() => t.to_string(),
        _ => {
            // Strip .md extension, get last segment, clean it up
            let stem = path.strip_suffix(".md").unwrap_or(path);
            let filename = stem.split('/').next_back().unwrap_or(stem);
            let cleaned: String = filename
                .chars()
                .map(|c| if c == '-' || c == '_' { ' ' } else { c })
                .collect();
            // Capitalize first letter
            let mut chars = cleaned.chars();
            match chars.next() {
                None => path.to_string(),
                Some(first) => first.to_uppercase().to_string() + chars.as_str(),
            }
        }
    }
}

/// Search result entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// Document path.
    pub path: String,
    /// Optional title from frontmatter.
    pub title: Option<String>,
    /// Human-readable display name. Derived from `title` when available,
    /// otherwise derived from the filename per OKF v0.2 §4.1.
    pub display_title: String,
    /// Optional concept type.
    #[serde(rename = "type")]
    pub concept_type: Option<String>,
    /// Relevance score.
    pub score: f64,
    /// Matching section if applicable.
    pub matching_section: Option<String>,
    /// Text excerpt around match.
    pub excerpt: String,
    /// Heading titles for the document, filtered by depth and capped at max_headings.
    /// Empty when no headings found or document has no body.
    #[serde(default)]
    pub headings: Vec<String>,
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
