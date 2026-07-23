use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryNode {
    pub path: String,
    pub index_document: Option<String>,
    pub child_directories: Vec<String>,
    pub documents: Vec<DirectoryDocument>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryDocument {
    pub path: String,
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub concept_type: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowseResponse {
    pub path: String,
    pub summary_document: Option<String>,
    pub directories: Vec<String>,
    pub documents: Vec<DirectoryDocument>,
    pub truncated: bool,
}
