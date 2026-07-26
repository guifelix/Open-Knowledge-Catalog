//! Core document model types for the Open Knowledge Catalog.
//!
//! This module defines all data structures used throughout the system:
//! - File discovery: [`FileRecord`]
//! - Front-matter parsing: [`FrontMatter`], [`ParseStatus`], [`ParseError`]
//! - Markdown structure: [`Heading`], [`Link`], [`Section`], [`ParsedDocument`]
//! - API responses: [`DocumentSummary`], [`DocumentDetail`], [`DocumentMetadata`], [`HeadingInfo`]
//! - Search: [`SearchResult`], [`SearchResponse`]
//! - Links: [`LinkInfo`]
//! - Validation: [`ValidationIssue`], [`ValidationReport`], [`ValidationSummary`], [`CheckResult`], [`CheckStatus`]
//! - Statistics: [`IndexStats`], [`ScanResult`], [`ProcessChangesResult`]
//! - Metadata queries: [`MetadataQueryResponse`]

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

/// Parsed front-matter metadata from a document.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FrontMatter {
    /// Document concept type (e.g., "concept", "reference", "tutorial").
    pub concept_type: Option<String>,
    /// Document title.
    pub title: Option<String>,
    /// Brief description.
    pub description: Option<String>,
    /// User-defined tags.
    pub tags: Vec<String>,
    /// Custom front-matter fields not recognized as standard keys.
    pub custom: BTreeMap<String, serde_json::Value>,
    /// Raw YAML content for debugging/re-processing.
    #[allow(dead_code)]
    pub raw_yaml: String,
}

/// A heading extracted from markdown content.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Heading {
    /// Heading level (1-6).
    pub level: u32,
    /// Heading text.
    pub title: String,
    /// Optional anchor/slug for linking.
    pub anchor: Option<String>,
    /// Byte position in the document.
    pub position: usize,
}

/// A table extracted from markdown content.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Table {
    /// Table headers.
    pub headers: Vec<String>,
    /// Table rows (each row is a vector of cell contents).
    pub rows: Vec<Vec<String>>,
    /// Column alignments (None, Left, Center, Right).
    pub alignments: Vec<TableAlignment>,
    /// Byte position in the document.
    pub position: usize,
}

/// Table column alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum TableAlignment {
    /// No explicit alignment.
    None,
    /// Left-aligned.
    Left,
    /// Center-aligned.
    Center,
    /// Right-aligned.
    Right,
}

/// A fenced code block extracted from markdown content.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CodeBlock {
    /// Programming language (from fence info string).
    pub language: Option<String>,
    /// Optional filename (from fence info string, e.g., `rust:filename.rs`).
    pub filename: Option<String>,
    /// Code content.
    pub content: String,
    /// Byte position in the document.
    pub position: usize,
}

/// A link extracted from markdown content.
#[derive(Debug, Clone)]
pub struct Link {
    /// Original link text as written in markdown.
    pub raw: String,
    /// Resolved target path or URL.
    pub target: String,
    /// Optional anchor fragment.
    pub target_anchor: Option<String>,
    /// Whether this is an external link (http/https/mailto).
    pub is_external: bool,
    /// Whether the target exists in the repository (for internal links).
    pub exists_in_repository: bool,
}

/// Fully parsed document with all extracted structure.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ParsedDocument {
    /// Document path.
    pub path: String,
    /// Parsed front-matter if present.
    pub front_matter: Option<FrontMatter>,
    /// All headings in document order.
    pub headings: Vec<Heading>,
    /// All links in document order.
    pub links: Vec<Link>,
    /// Plain text body content.
    pub body_text: String,
    /// Logical sections (heading + content).
    pub sections: Vec<Section>,
    /// Overall parse status.
    pub parse_status: ParseStatus,
    /// Any parse errors encountered.
    pub parse_errors: Vec<ParseError>,
}

/// A logical section of a document (heading + content).
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Section {
    /// Section heading text.
    pub heading: String,
    /// Heading level.
    pub level: u32,
    /// Section content (markdown).
    pub content: String,
    /// Byte position of section start.
    pub start_position: usize,
    /// Tables found in this section.
    pub tables: Vec<Table>,
    /// Code blocks found in this section.
    pub code_blocks: Vec<CodeBlock>,
}

/// Result of front-matter and markdown parsing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParseStatus {
    /// Parsed successfully with no errors.
    Ok,
    /// Parsed with warnings (e.g., invalid YAML but front-matter extracted).
    Partial,
    /// Failed to parse (e.g., missing closing delimiter).
    Failed,
}

/// A parse error with context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseError {
    /// Parsing stage where error occurred (e.g., "frontmatter", "yaml", "markdown").
    pub stage: String,
    /// Human-readable error message.
    pub message: String,
    /// Optional line number in source.
    pub line: Option<usize>,
}

/// A resource limit violation error with structured details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitError {
    /// The name of the limit that was exceeded (e.g., "max_file_size", "max_front_matter_size").
    pub limit_name: String,
    /// The configured limit value.
    pub limit_value: String,
    /// The actual value that exceeded the limit.
    pub actual_value: Option<String>,
    /// Human-readable error message.
    pub message: String,
    /// Error code for programmatic handling.
    pub code: String,
}

impl LimitError {
    /// Create a new limit error.
    pub fn new(limit_name: &str, limit_value: &str, message: &str) -> Self {
        Self {
            limit_name: limit_name.to_string(),
            limit_value: limit_value.to_string(),
            actual_value: None,
            message: message.to_string(),
            code: format!("LIMIT_EXCEEDED_{}", limit_name.to_uppercase()),
        }
    }

    /// Set the actual value that exceeded the limit.
    pub fn with_actual(mut self, actual: &str) -> Self {
        self.actual_value = Some(actual.to_string());
        self
    }
}

impl std::fmt::Display for LimitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} (limit: {}, code: {})",
            self.message, self.limit_value, self.code
        )
    }
}

impl std::error::Error for LimitError {}

/// Lightweight document summary for listings.
#[allow(dead_code)]
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
    pub headings: Vec<HeadingInfo>,
    /// Optional body content (may be truncated).
    pub body: Option<String>,
    /// Whether body was truncated.
    pub truncated: bool,
    /// Any parse errors.
    pub errors: Vec<ParseError>,
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

/// Heading information for API responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadingInfo {
    /// Heading level (1-6).
    pub level: u32,
    /// Heading text.
    pub title: String,
    /// Optional anchor.
    pub anchor: Option<String>,
}

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

/// Link information for API responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkInfo {
    /// Target path (for internal links).
    pub target_path: Option<String>,
    /// Target anchor fragment.
    pub target_anchor: Option<String>,
    /// External URL (for external links).
    pub external_url: Option<String>,
    /// Whether target exists in repository.
    pub exists_in_repository: bool,
}

/// A validation issue found during index validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationIssue {
    /// Document path where issue was found.
    pub path: String,
    /// Severity: "error", "warning", or "info".
    pub severity: String,
    /// Issue category.
    pub category: String,
    /// Human-readable description.
    pub message: String,
    /// Optional line number.
    pub line: Option<usize>,
}

/// Complete validation report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    /// Summary statistics.
    pub summary: ValidationSummary,
    /// All issues found.
    pub issues: Vec<ValidationIssue>,
}

/// Validation summary statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationSummary {
    /// Total number of issues.
    pub total_issues: usize,
    /// Number of errors.
    pub errors: usize,
    /// Number of warnings.
    pub warnings: usize,
    /// Number of infos.
    pub infos: usize,
    /// Per-check results.
    pub checks: Vec<CheckResult>,
}

/// Result of a single validation check.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckResult {
    /// Check name.
    pub check_name: String,
    /// Check status.
    pub status: CheckStatus,
    /// Number of issues found by this check.
    pub issue_count: usize,
}

/// Status of a validation check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckStatus {
    /// Check passed (no issues).
    Pass,
    /// Check found warnings.
    Warn,
    /// Check found errors.
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

/// Index statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexStats {
    /// Total documents indexed.
    pub document_count: usize,
    /// Documents with parse errors.
    pub error_count: usize,
    /// Total links in index.
    pub link_count: usize,
    /// Total headings in index.
    pub heading_count: usize,
}

/// Result of a full repository scan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    /// Total files discovered.
    pub total_files: usize,
    /// Files added in this scan.
    pub added: usize,
    /// Files modified in this scan.
    pub modified: usize,
    /// Files deleted since last scan.
    pub deleted: usize,
    /// Files with parse failures.
    pub parse_failures: usize,
    /// Broken internal links found.
    pub broken_links: usize,
    /// Total links processed.
    pub total_links: usize,
    /// Scan duration in seconds.
    pub duration_secs: f64,
}

/// Result of incremental change processing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessChangesResult {
    /// Files added.
    pub files_added: usize,
    /// Files modified.
    pub files_modified: usize,
    /// Files deleted.
    pub files_deleted: usize,
    /// Parse failures encountered.
    pub parse_failures: usize,
    /// Broken links found.
    pub broken_links: usize,
    /// Total links processed.
    pub total_links: usize,
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
