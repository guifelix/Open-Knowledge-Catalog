//! Graph traversal data structures.
//!
//! Types for representing and traversing the document link graph.

use serde::{Deserialize, Serialize};

/// A directed edge in the document graph.
///
/// Represents a link relationship between two documents.
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    /// Source document path.
    pub source: String,
    /// Target document path.
    pub target: String,
    /// Relationship type (e.g., "links_to", "references").
    pub relation: String,
}

/// Response from a graph traversal operation.
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraverseResponse {
    /// Visited nodes in traversal order.
    pub nodes: Vec<TraverseNode>,
    /// Edges traversed.
    pub edges: Vec<GraphEdge>,
    /// Whether results were truncated due to limits.
    pub truncated: bool,
}

/// A node visited during graph traversal.
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraverseNode {
    /// Document path.
    pub path: String,
    /// Optional title from front-matter.
    pub title: Option<String>,
    /// Optional concept type from front-matter.
    #[serde(rename = "type")]
    pub concept_type: Option<String>,
    /// Depth from traversal start (0 = start node).
    pub depth: usize,
}
