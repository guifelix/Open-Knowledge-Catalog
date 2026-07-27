//! BM25 search configuration for FTS5 relevance ranking.
//!
//! Controls field weights and BM25 algorithm parameters (k1, b).
//! Field weights determine the relative importance of each column in the FTS5 index.
//! Higher weight = more important for relevance scoring.
//!
//! Default weights follow the ADR-002 specification:
//! - title: 10.0 (most important)
//! - description: 5.0
//! - headings: 2.0
//! - body: 1.0 (baseline)
//! - concept_type: 0.0 (not used for relevance)

use serde::{Deserialize, Serialize};

/// BM25 search configuration for FTS5 relevance ranking.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Bm25Config {
    /// Weight for the title field. Default: 10.0
    pub title_weight: f64,
    /// Weight for the description field. Default: 5.0
    pub description_weight: f64,
    /// Weight for the headings field. Default: 2.0
    pub headings_weight: f64,
    /// Weight for the body field. Default: 1.0
    pub body_weight: f64,
    /// Weight for the concept_type field. Default: 0.0 (ignored for relevance)
    pub concept_type_weight: f64,
    /// BM25 k1 parameter (term frequency saturation). Default: 1.2
    /// Higher values = less saturation, more weight to term frequency.
    pub k1: f64,
    /// BM25 b parameter (length normalization). Default: 0.75
    /// 0.0 = no length normalization, 1.0 = full normalization.
    pub b: f64,
}

impl Default for Bm25Config {
    fn default() -> Self {
        Self {
            title_weight: 10.0,
            description_weight: 5.0,
            headings_weight: 2.0,
            body_weight: 1.0,
            concept_type_weight: 0.0,
            k1: 1.2,
            b: 0.75,
        }
    }
}
