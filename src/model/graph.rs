use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    pub source: String,
    pub target: String,
    pub relation: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraverseResponse {
    pub nodes: Vec<TraverseNode>,
    pub edges: Vec<GraphEdge>,
    pub truncated: bool,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraverseNode {
    pub path: String,
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub concept_type: Option<String>,
    pub depth: usize,
}
