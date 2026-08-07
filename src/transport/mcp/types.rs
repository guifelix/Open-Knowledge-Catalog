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
    /// Optional sections: metadata, headings, body, custom, content_hash,
    /// parent_path, links, and backlinks.
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<LinkInfoOutput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backlinks: Option<Vec<DocumentBacklinkOutput>>,
}

#[derive(Serialize, schemars::JsonSchema)]
pub(crate) struct DocumentBacklinkOutput {
    pub source_path: String,
    pub target_anchor: Option<String>,
    pub exists_in_repository: bool,
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

#[derive(Serialize, schemars::JsonSchema)]
pub(crate) struct SectionResponseOutput {
    pub section: Option<SectionOutput>,
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
    /// Optional title from frontmatter. Falls back to `display_title` when absent.
    pub title: Option<String>,
    /// Human-readable display name. Derived from `title` when available,
    /// otherwise derived from the filename per OKF v0.2 §4.1.
    pub display_title: String,
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
    /// Exact `key=value` filters. Supports type, title, parse_status,
    /// path_prefix, tags_contains, and custom front-matter field names.
    pub filter: Option<Vec<String>>,
    /// Core document fields, tags, or custom front-matter field names to return.
    pub select: Option<Vec<String>>,
    /// Maximum number of matching documents to return (default: 100).
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

#[derive(Serialize, schemars::JsonSchema)]
pub(crate) struct LinksResponseOutput {
    pub links: Vec<LinkInfoOutput>,
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

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used)]

    use std::collections::BTreeMap;

    use serde_json::json;

    use super::{DocumentBacklinkOutput, DocumentDetailOutput, HeadingInfoOutput, LinkInfoOutput};

    #[test]
    fn enriched_document_output_serializes_every_optional_field() {
        let output = DocumentDetailOutput {
            path: "metrics/revenue.md".to_string(),
            title: Some("Revenue".to_string()),
            concept_type: Some("Metric".to_string()),
            description: Some("Revenue metric".to_string()),
            tags: vec!["finance".to_string()],
            file_size: 42,
            modified_at: 1,
            parse_status: "ok".to_string(),
            headings: vec![HeadingInfoOutput {
                level: 1,
                title: "Definition".to_string(),
                anchor: Some("definition".to_string()),
            }],
            body: Some("Body".to_string()),
            truncated: false,
            custom: Some(BTreeMap::from([("owner".to_string(), json!("Finance"))])),
            content_hash: Some("hash".to_string()),
            parent_path: Some("metrics".to_string()),
            links: Some(vec![LinkInfoOutput {
                target_path: Some("datasets/orders.md".to_string()),
                target_anchor: None,
                external_url: None,
                exists_in_repository: true,
            }]),
            backlinks: Some(vec![DocumentBacklinkOutput {
                source_path: "policies/revenue.md".to_string(),
                target_anchor: Some("definition".to_string()),
                exists_in_repository: true,
            }]),
        };

        let value = serde_json::to_value(output).expect("serialize enriched document output");
        for field in [
            "path",
            "title",
            "concept_type",
            "description",
            "tags",
            "file_size",
            "modified_at",
            "parse_status",
            "headings",
            "body",
            "truncated",
            "custom",
            "content_hash",
            "parent_path",
            "links",
            "backlinks",
        ] {
            assert!(value.get(field).is_some(), "missing output field {field}");
        }
    }
}
