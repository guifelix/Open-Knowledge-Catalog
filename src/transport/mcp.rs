//! Model Context Protocol (MCP) server implementation.
//!
//! Provides an MCP server that exposes OKC functionality as tools for
//! AI assistant integration. Uses `rmcp` for protocol handling.
//!
//! Available tools:
//! - `scan` - Index repository
//! - `browse` - Directory browsing
//! - `get_document` - Document retrieval
//! - `get_section` - Section extraction
//! - `search` - Full-text search
//! - `query_metadata` - Structured metadata queries
//! - `get_links` / `get_backlinks` - Link navigation
//! - `traverse` - Graph traversal
//! - `validate` - Index validation
//! - `get_stats` - Index statistics

use std::collections::HashMap;
use std::sync::{Arc, Mutex, PoisonError};

use rmcp::{
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    schemars, tool, tool_handler, tool_router,
};
use serde::{Deserialize, Serialize};

use crate::config::OkcConfig;
use crate::service::OkcService;

/// MCP server wrapping the OKC service.
///
/// Holds a thread-safe reference to the service and the tool router
/// for MCP protocol handling.
#[derive(Clone)]
pub struct McpServer {
    /// Thread-safe reference to the OKC service.
    pub service: Arc<Mutex<OkcService>>,
    tool_router: ToolRouter<Self>,
}

impl McpServer {
    /// Create a new MCP server with the given configuration.
    pub fn new(config: &OkcConfig) -> Result<Self, anyhow::Error> {
        let service = OkcService::open(config)?;
        Ok(Self {
            service: Arc::new(Mutex::new(service)),
            tool_router: Self::tool_router(),
        })
    }
}

#[derive(Deserialize, schemars::JsonSchema)]
struct ScanParams {
    roots: Vec<String>,
    db_path: Option<String>,
}

#[derive(Serialize, schemars::JsonSchema)]
struct ScanResultOutput {
    total_files: usize,
    added: usize,
    modified: usize,
    deleted: usize,
    parse_failures: usize,
    broken_links: usize,
    total_links: usize,
    duration_secs: f64,
}

#[derive(Deserialize, schemars::JsonSchema)]
struct BrowseParams {
    path: Option<String>,
    depth: Option<usize>,
    limit: Option<usize>,
}

#[derive(Serialize, schemars::JsonSchema)]
struct BrowseResultOutput {
    path: String,
    summary_document: Option<String>,
    directories: Vec<String>,
    documents: Vec<DirectoryDocumentOutput>,
    truncated: bool,
}

#[derive(Serialize, schemars::JsonSchema)]
struct DirectoryDocumentOutput {
    path: String,
    title: Option<String>,
    concept_type: Option<String>,
    description: Option<String>,
}

#[derive(Deserialize, schemars::JsonSchema)]
struct GetDocumentParams {
    path: String,
    include: Option<Vec<String>>,
    max_chars: Option<usize>,
}

#[derive(Serialize, schemars::JsonSchema)]
struct DocumentDetailOutput {
    path: String,
    title: Option<String>,
    concept_type: Option<String>,
    description: Option<String>,
    tags: Vec<String>,
    file_size: u64,
    modified_at: i64,
    parse_status: String,
    headings: Vec<HeadingInfoOutput>,
    body: Option<String>,
    truncated: bool,
}

#[derive(Serialize, schemars::JsonSchema)]
struct HeadingInfoOutput {
    level: u32,
    title: String,
    anchor: Option<String>,
}

#[derive(Deserialize, schemars::JsonSchema)]
struct GetSectionParams {
    path: String,
    heading: String,
    max_chars: Option<usize>,
}

#[derive(Serialize, schemars::JsonSchema)]
struct SectionOutput {
    heading: String,
    content: String,
}

#[derive(Deserialize, schemars::JsonSchema)]
struct SearchParams {
    query: String,
    path_prefix: Option<String>,
    types: Option<Vec<String>>,
    tags: Option<Vec<String>>,
    limit: Option<usize>,
}

#[derive(Serialize, schemars::JsonSchema)]
struct SearchResultOutput {
    path: String,
    title: Option<String>,
    concept_type: Option<String>,
    score: f64,
    excerpt: String,
}

#[derive(Serialize, schemars::JsonSchema)]
struct SearchResponseOutput {
    results: Vec<SearchResultOutput>,
    total_matches: usize,
    truncated: bool,
}

#[derive(Deserialize, schemars::JsonSchema)]
struct MetadataParams {
    filter: Option<Vec<String>>,
    select: Option<Vec<String>>,
    limit: Option<usize>,
}

#[derive(Serialize, schemars::JsonSchema)]
struct MetadataResponseOutput {
    results: Vec<serde_json::Value>,
    total_matches: usize,
    truncated: bool,
}

#[derive(Deserialize, schemars::JsonSchema)]
struct LinkParams {
    path: String,
}

#[derive(Serialize, schemars::JsonSchema)]
struct LinkInfoOutput {
    target_path: Option<String>,
    target_anchor: Option<String>,
    external_url: Option<String>,
    exists_in_repository: bool,
}

#[derive(Deserialize, schemars::JsonSchema)]
struct BacklinkParams {
    path: String,
    limit: Option<usize>,
}

#[derive(Deserialize, schemars::JsonSchema)]
struct TraverseParams {
    start: String,
    relations: Option<Vec<String>>,
    max_depth: Option<usize>,
    max_nodes: Option<usize>,
}

#[derive(Serialize, schemars::JsonSchema)]
struct TraverseNodeOutput {
    path: String,
    title: Option<String>,
    concept_type: Option<String>,
    depth: usize,
}

#[derive(Serialize, schemars::JsonSchema)]
struct GraphEdgeOutput {
    source: String,
    target: String,
    relation: String,
}

#[derive(Serialize, schemars::JsonSchema)]
struct TraverseResponseOutput {
    nodes: Vec<TraverseNodeOutput>,
    edges: Vec<GraphEdgeOutput>,
    truncated: bool,
}

#[derive(Serialize, schemars::JsonSchema)]
struct StatsOutput {
    document_count: usize,
    error_count: usize,
    link_count: usize,
    heading_count: usize,
}

#[derive(Serialize, schemars::JsonSchema)]
struct ValidateOutput {
    summary: ValidateSummaryOutput,
    issues: Vec<ValidateIssueOutput>,
}

#[derive(Serialize, schemars::JsonSchema)]
struct ValidateSummaryOutput {
    total_issues: usize,
    errors: usize,
    warnings: usize,
    infos: usize,
}

#[derive(Serialize, schemars::JsonSchema)]
struct ValidateIssueOutput {
    path: String,
    severity: String,
    category: String,
    message: String,
    line: Option<usize>,
}

#[tool_router]
impl McpServer {
    #[tool(
        description = "Scan directories and index all markdown files into the knowledge catalog"
    )]
    async fn scan(
        &self,
        Parameters(ScanParams { roots, db_path }): Parameters<ScanParams>,
    ) -> String {
        let config = OkcConfig {
            roots: roots.into_iter().map(std::path::PathBuf::from).collect(),
            db_path: db_path.map_or_else(
                || std::path::PathBuf::from("okc_index.db"),
                std::path::PathBuf::from,
            ),
            ..Default::default()
        };

        match OkcService::open(&config).and_then(|mut svc| svc.scan()) {
            Ok(r) => serde_json::to_string(&ScanResultOutput {
                total_files: r.total_files,
                added: r.added,
                modified: r.modified,
                deleted: r.deleted,
                parse_failures: r.parse_failures,
                broken_links: r.broken_links,
                total_links: r.total_links,
                duration_secs: r.duration_secs,
            })
            .unwrap_or_default(),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Browse the directory tree of the knowledge catalog")]
    async fn browse(
        &self,
        Parameters(BrowseParams { path, depth, limit }): Parameters<BrowseParams>,
    ) -> String {
        let depth = depth.unwrap_or(1);
        let limit = limit.unwrap_or(100);
        let path = path.unwrap_or_default();

        let svc = self.service.lock().unwrap_or_else(|e| e.into_inner());
        match svc.browse(&path, depth, limit) {
            Ok(r) => serde_json::to_string(&BrowseResultOutput {
                path: r.path,
                summary_document: r.summary_document,
                directories: r.directories,
                documents: r
                    .documents
                    .into_iter()
                    .map(|d| DirectoryDocumentOutput {
                        path: d.path,
                        title: d.title,
                        concept_type: d.concept_type,
                        description: d.description,
                    })
                    .collect(),
                truncated: r.truncated,
            })
            .unwrap_or_default(),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Get a document's full content and metadata from the knowledge catalog")]
    async fn get_document(
        &self,
        Parameters(GetDocumentParams {
            path,
            include,
            max_chars,
        }): Parameters<GetDocumentParams>,
    ) -> String {
        let include = include.unwrap_or_default();
        let max_chars = max_chars.unwrap_or(12000);

        let svc = self.service.lock().unwrap_or_else(|e| e.into_inner());
        match svc.get_document(&path, &include, max_chars) {
            Ok(r) => serde_json::to_string(&DocumentDetailOutput {
                path: r.path,
                title: r.metadata.title,
                concept_type: r.metadata.concept_type,
                description: r.metadata.description,
                tags: r.metadata.tags,
                file_size: r.metadata.file_size,
                modified_at: r.metadata.modified_at,
                parse_status: r.metadata.parse_status,
                headings: r
                    .headings
                    .into_iter()
                    .map(|h| HeadingInfoOutput {
                        level: h.level,
                        title: h.title,
                        anchor: h.anchor,
                    })
                    .collect(),
                body: r.body,
                truncated: r.truncated,
            })
            .unwrap_or_default(),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Get a specific section of a document by heading title or anchor")]
    async fn get_section(
        &self,
        Parameters(GetSectionParams {
            path,
            heading,
            max_chars,
        }): Parameters<GetSectionParams>,
    ) -> String {
        let max_chars = max_chars.unwrap_or(5000);

        let svc = self.service.lock().unwrap_or_else(|e| e.into_inner());
        match svc.get_section(&path, &heading, max_chars) {
            Ok(Some((heading, content))) => {
                serde_json::to_string(&SectionOutput { heading, content }).unwrap_or_default()
            }
            _ => "null".to_string(),
        }
    }

    #[tool(description = "Full-text search across all documents in the knowledge catalog")]
    async fn search(
        &self,
        Parameters(SearchParams {
            query,
            path_prefix,
            types,
            tags,
            limit,
        }): Parameters<SearchParams>,
    ) -> String {
        let limit = limit.unwrap_or(20);

        let svc = self.service.lock().unwrap_or_else(|e| e.into_inner());
        match svc.search(
            &query,
            path_prefix.as_deref(),
            types.as_deref(),
            tags.as_deref(),
            limit,
        ) {
            Ok(r) => serde_json::to_string(&SearchResponseOutput {
                results: r
                    .results
                    .into_iter()
                    .map(|r| SearchResultOutput {
                        path: r.path,
                        title: r.title,
                        concept_type: r.concept_type,
                        score: r.score,
                        excerpt: r.excerpt,
                    })
                    .collect(),
                total_matches: r.total_matches,
                truncated: r.truncated,
            })
            .unwrap_or_default(),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Query document metadata with filters and field selection")]
    async fn query_metadata(
        &self,
        Parameters(MetadataParams {
            filter,
            select,
            limit,
        }): Parameters<MetadataParams>,
    ) -> String {
        let select = select.unwrap_or_default();
        let limit = limit.unwrap_or(100);
        let filters: HashMap<String, String> = filter
            .unwrap_or_default()
            .into_iter()
            .filter_map(|f| {
                let mut parts = f.splitn(2, '=');
                match (parts.next(), parts.next()) {
                    (Some(k), Some(v)) => Some((k.to_string(), v.to_string())),
                    _ => None,
                }
            })
            .collect();

        let svc = self.service.lock().unwrap_or_else(|e| e.into_inner());
        match svc.query_metadata(&filters, &select, limit) {
            Ok(r) => serde_json::to_string(&MetadataResponseOutput {
                results: r.results,
                total_matches: r.total_matches,
                truncated: r.truncated,
            })
            .unwrap_or_default(),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Get all outgoing links from a document")]
    async fn get_links(&self, Parameters(LinkParams { path }): Parameters<LinkParams>) -> String {
        let svc = self.service.lock().unwrap_or_else(|e| e.into_inner());
        match svc.get_links(&path) {
            Ok(links) => serde_json::to_string(
                &links
                    .into_iter()
                    .map(|l| LinkInfoOutput {
                        target_path: l.target_path,
                        target_anchor: l.target_anchor,
                        external_url: l.external_url,
                        exists_in_repository: l.exists_in_repository,
                    })
                    .collect::<Vec<_>>(),
            )
            .unwrap_or_default(),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Get all backlinks pointing to a document")]
    async fn get_backlinks(
        &self,
        Parameters(BacklinkParams { path, limit }): Parameters<BacklinkParams>,
    ) -> String {
        let limit = limit.unwrap_or(50);
        let svc = self.service.lock().unwrap_or_else(|e| e.into_inner());
        match svc.get_backlinks(&path, limit) {
            Ok(links) => serde_json::to_string(
                &links
                    .into_iter()
                    .map(|l| LinkInfoOutput {
                        target_path: l.target_path,
                        target_anchor: l.target_anchor,
                        external_url: l.external_url,
                        exists_in_repository: l.exists_in_repository,
                    })
                    .collect::<Vec<_>>(),
            )
            .unwrap_or_default(),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Traverse the knowledge graph starting from a document, following links")]
    async fn traverse(
        &self,
        Parameters(TraverseParams {
            start,
            relations,
            max_depth,
            max_nodes,
        }): Parameters<TraverseParams>,
    ) -> String {
        let relations = relations.unwrap_or_default();
        let max_depth = max_depth.unwrap_or(3);
        let max_nodes = max_nodes.unwrap_or(50);

        let svc = self.service.lock().unwrap_or_else(|e| e.into_inner());
        match svc.traverse(&start, &relations, max_depth, max_nodes) {
            Ok(r) => serde_json::to_string(&TraverseResponseOutput {
                nodes: r
                    .nodes
                    .into_iter()
                    .map(|n| TraverseNodeOutput {
                        path: n.path,
                        title: n.title,
                        concept_type: n.concept_type,
                        depth: n.depth,
                    })
                    .collect(),
                edges: r
                    .edges
                    .into_iter()
                    .map(|e| GraphEdgeOutput {
                        source: e.source,
                        target: e.target,
                        relation: e.relation,
                    })
                    .collect(),
                truncated: r.truncated,
            })
            .unwrap_or_default(),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(
        description = "Get index statistics: document count, error count, link count, heading count"
    )]
    async fn get_stats(&self) -> String {
        let svc = self.service.lock().unwrap_or_else(|e| e.into_inner());
        match svc.get_stats() {
            Ok(s) => serde_json::to_string(&StatsOutput {
                document_count: s.document_count,
                error_count: s.error_count,
                link_count: s.link_count,
                heading_count: s.heading_count,
            })
            .unwrap_or_default(),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(
        description = "Validate the repository and check for issues across all indexed documents"
    )]
    async fn validate(&self) -> String {
        let svc = self.service.lock().unwrap_or_else(|e| e.into_inner());
        match svc.validate_report() {
            Ok(r) => serde_json::to_string(&ValidateOutput {
                summary: ValidateSummaryOutput {
                    total_issues: r.summary.total_issues,
                    errors: r.summary.errors,
                    warnings: r.summary.warnings,
                    infos: r.summary.infos,
                },
                issues: r
                    .issues
                    .into_iter()
                    .map(|i| ValidateIssueOutput {
                        path: i.path,
                        severity: i.severity,
                        category: i.category,
                        message: i.message,
                        line: i.line,
                    })
                    .collect(),
            })
            .unwrap_or_default(),
            Err(e) => format!("Error: {}", e),
        }
    }
}

#[tool_handler]
impl rmcp::handler::server::ServerHandler for McpServer {}
