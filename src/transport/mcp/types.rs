//! MCP tool parameter and response types.
//!
//! Defines all input parameters and output structures for the MCP tools.
//! Every param/output pair matches a corresponding tool handler in `tools.rs`.

use rmcp::schemars;
use serde::{Deserialize, Serialize};

// ── Scan ────────────────────────────────────────────────────────────────────

#[derive(Deserialize, schemars::JsonSchema)]
pub(crate) struct ScanParams {
    pub roots: Vec<String>,
    pub db_path: Option<String>,
}

#[derive(Serialize, schemars::JsonSchema)]
pub(crate) struct ScanResultOutput {
    pub total_files: usize,
    pub added: usize,
    pub modified: usize,
    pub deleted: usize,
    pub parse_failures: usize,
    pub broken_links: usize,
    pub total_links: usize,
    pub duration_secs: f64,
}

// ── Browse ──────────────────────────────────────────────────────────────────

#[derive(Deserialize, schemars::JsonSchema)]
pub(crate) struct BrowseParams {
    pub path: Option<String>,
    pub depth: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Serialize, schemars::JsonSchema)]
pub(crate) struct BrowseResultOutput {
    pub path: String,
    pub summary_document: Option<String>,
    pub directories: Vec<String>,
    pub documents: Vec<DirectoryDocumentOutput>,
    pub truncated: bool,
}

#[derive(Serialize, schemars::JsonSchema)]
pub(crate) struct DirectoryDocumentOutput {
    pub path: String,
    pub title: Option<String>,
    pub concept_type: Option<String>,
    pub description: Option<String>,
}

// ── GetDocument ──────────────────────────────────────────────────────────────

#[derive(Deserialize, schemars::JsonSchema)]
pub(crate) struct GetDocumentParams {
    pub path: String,
    pub include: Option<Vec<String>>,
    pub max_chars: Option<usize>,
}

#[derive(Serialize, schemars::JsonSchema)]
pub(crate) struct DocumentDetailOutput {
    pub path: String,
    pub title: Option<String>,
    pub concept_type: Option<String>,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub file_size: u64,
    pub modified_at: i64,
    pub parse_status: String,
    pub headings: Vec<HeadingInfoOutput>,
    pub body: Option<String>,
    pub truncated: bool,
}

#[derive(Serialize, schemars::JsonSchema)]
pub(crate) struct HeadingInfoOutput {
    pub level: u32,
    pub title: String,
    pub anchor: Option<String>,
}

// ── GetSection ───────────────────────────────────────────────────────────────

#[derive(Deserialize, schemars::JsonSchema)]
pub(crate) struct GetSectionParams {
    pub path: String,
    pub heading: String,
    pub max_chars: Option<usize>,
}

#[derive(Serialize, schemars::JsonSchema)]
pub(crate) struct SectionOutput {
    pub heading: String,
    pub content: String,
}

// ── Search ───────────────────────────────────────────────────────────────────

#[derive(Deserialize, schemars::JsonSchema)]
pub(crate) struct SearchParams {
    pub query: String,
    pub path_prefix: Option<String>,
    pub types: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub limit: Option<usize>,
}

#[derive(Serialize, schemars::JsonSchema)]
pub(crate) struct SearchResultOutput {
    pub path: String,
    pub title: Option<String>,
    pub concept_type: Option<String>,
    pub score: f64,
    pub excerpt: String,
}

#[derive(Serialize, schemars::JsonSchema)]
pub(crate) struct SearchResponseOutput {
    pub results: Vec<SearchResultOutput>,
    pub total_matches: usize,
    pub truncated: bool,
}

// ── Metadata ─────────────────────────────────────────────────────────────────

#[derive(Deserialize, schemars::JsonSchema)]
pub(crate) struct MetadataParams {
    pub filter: Option<Vec<String>>,
    pub select: Option<Vec<String>>,
    pub limit: Option<usize>,
}

#[derive(Serialize, schemars::JsonSchema)]
pub(crate) struct MetadataResponseOutput {
    pub results: Vec<serde_json::Value>,
    pub total_matches: usize,
    pub truncated: bool,
}

// ── Links ────────────────────────────────────────────────────────────────────

#[derive(Deserialize, schemars::JsonSchema)]
pub(crate) struct LinkParams {
    pub path: String,
}

#[derive(Serialize, schemars::JsonSchema)]
pub(crate) struct LinkInfoOutput {
    pub target_path: Option<String>,
    pub target_anchor: Option<String>,
    pub external_url: Option<String>,
    pub exists_in_repository: bool,
}

#[derive(Deserialize, schemars::JsonSchema)]
pub(crate) struct BacklinkParams {
    pub path: String,
    pub limit: Option<usize>,
}

// ── Traverse ─────────────────────────────────────────────────────────────────

#[derive(Deserialize, schemars::JsonSchema)]
pub(crate) struct TraverseParams {
    pub start: String,
    pub relations: Option<Vec<String>>,
    pub max_depth: Option<usize>,
    pub max_nodes: Option<usize>,
}

#[derive(Serialize, schemars::JsonSchema)]
pub(crate) struct TraverseNodeOutput {
    pub path: String,
    pub title: Option<String>,
    pub concept_type: Option<String>,
    pub depth: usize,
}

#[derive(Serialize, schemars::JsonSchema)]
pub(crate) struct GraphEdgeOutput {
    pub source: String,
    pub target: String,
    pub relation: String,
}

#[derive(Serialize, schemars::JsonSchema)]
pub(crate) struct TraverseResponseOutput {
    pub nodes: Vec<TraverseNodeOutput>,
    pub edges: Vec<GraphEdgeOutput>,
    pub truncated: bool,
}

// ── Stats / Validate ─────────────────────────────────────────────────────────

#[derive(Serialize, schemars::JsonSchema)]
pub(crate) struct StatsOutput {
    pub document_count: usize,
    pub error_count: usize,
    pub link_count: usize,
    pub heading_count: usize,
}

#[derive(Serialize, schemars::JsonSchema)]
pub(crate) struct ValidateOutput {
    pub summary: ValidateSummaryOutput,
    pub issues: Vec<ValidateIssueOutput>,
}

#[derive(Serialize, schemars::JsonSchema)]
pub(crate) struct ValidateSummaryOutput {
    pub total_issues: usize,
    pub errors: usize,
    pub warnings: usize,
    pub infos: usize,
}

#[derive(Serialize, schemars::JsonSchema)]
pub(crate) struct ValidateIssueOutput {
    pub path: String,
    pub severity: String,
    pub category: String,
    pub message: String,
    pub line: Option<usize>,
}
