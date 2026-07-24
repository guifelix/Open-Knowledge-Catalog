use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileRecord {
    pub path: String,
    pub absolute_path: String,
    pub size: u64,
    pub modified_at: i64,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FrontMatter {
    pub concept_type: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub custom: BTreeMap<String, serde_json::Value>,
    #[allow(dead_code)]
    pub raw_yaml: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Heading {
    pub level: u32,
    pub title: String,
    pub anchor: Option<String>,
    pub position: usize,
}

#[derive(Debug, Clone)]
pub struct Link {
    pub raw: String,
    pub target: String,
    pub target_anchor: Option<String>,
    pub is_external: bool,
    pub exists_in_repository: bool,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ParsedDocument {
    pub path: String,
    pub front_matter: Option<FrontMatter>,
    pub headings: Vec<Heading>,
    pub links: Vec<Link>,
    pub body_text: String,
    pub sections: Vec<Section>,
    pub parse_status: ParseStatus,
    pub parse_errors: Vec<ParseError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParseStatus {
    Ok,
    Partial,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseError {
    pub stage: String,
    pub message: String,
    pub line: Option<usize>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Section {
    pub heading: String,
    pub level: u32,
    pub content: String,
    pub start_position: usize,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentSummary {
    pub path: String,
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub concept_type: Option<String>,
    pub description: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentDetail {
    pub path: String,
    pub metadata: DocumentMetadata,
    pub headings: Vec<HeadingInfo>,
    pub body: Option<String>,
    pub truncated: bool,
    pub errors: Vec<ParseError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetadata {
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub concept_type: Option<String>,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub custom: BTreeMap<String, serde_json::Value>,
    pub file_size: u64,
    pub modified_at: i64,
    pub parse_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadingInfo {
    pub level: u32,
    pub title: String,
    pub anchor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub path: String,
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub concept_type: Option<String>,
    pub score: f64,
    pub matching_section: Option<String>,
    pub excerpt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkInfo {
    pub target_path: Option<String>,
    pub target_anchor: Option<String>,
    pub external_url: Option<String>,
    pub exists_in_repository: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationIssue {
    pub path: String,
    pub severity: String,
    pub category: String,
    pub message: String,
    pub line: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    pub summary: ValidationSummary,
    pub issues: Vec<ValidationIssue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationSummary {
    pub total_issues: usize,
    pub errors: usize,
    pub warnings: usize,
    pub infos: usize,
    pub checks: Vec<CheckResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckResult {
    pub check_name: String,
    pub status: CheckStatus,
    pub issue_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CheckStatus {
    Pass,
    Warn,
    Fail,
}

impl std::fmt::Display for CheckStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CheckStatus::Pass => write!(f, "pass"),
            CheckStatus::Warn => write!(f, "warn"),
            CheckStatus::Fail => write!(f, "fail"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexStats {
    pub document_count: usize,
    pub error_count: usize,
    pub link_count: usize,
    pub heading_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub total_files: usize,
    pub added: usize,
    pub modified: usize,
    pub deleted: usize,
    pub parse_failures: usize,
    pub broken_links: usize,
    pub total_links: usize,
    pub duration_secs: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessChangesResult {
    pub files_added: usize,
    pub files_modified: usize,
    pub files_deleted: usize,
    pub parse_failures: usize,
    pub broken_links: usize,
    pub total_links: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    pub results: Vec<SearchResult>,
    pub total_matches: usize,
    pub truncated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataQueryResponse {
    pub results: Vec<serde_json::Value>,
    pub total_matches: usize,
    pub truncated: bool,
}
