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
//!
//! Transport options:
//! - stdio (for Claude Code, local CLI)
//! - HTTP/SSE (for web clients, remote access)

pub(crate) mod types;

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex, PoisonError};
use std::time::Duration;

use axum::Router;
use rmcp::{
    handler::server::{router::tool::ToolRouter, wrapper::Parameters, ServerHandler},
    schemars, tool, tool_handler, tool_router,
    transport::streamable_http_server::session::local::LocalSessionManager,
    transport::streamable_http_server::tower::{StreamableHttpServerConfig, StreamableHttpService},
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;

use crate::config::OkcConfig;
use crate::service::OkcService;

use self::types::{
    BacklinkParams, BrowseParams, BrowseResultOutput, DirectoryDocumentOutput,
    DocumentDetailOutput, GetDocumentParams, GetSectionParams, GraphEdgeOutput, HeadingInfoOutput,
    LinkInfoOutput, LinkParams, MetadataParams, MetadataResponseOutput, ScanParams,
    ScanResultOutput, SearchParams, SearchResponseOutput, SearchResultOutput, SectionOutput,
    StatsOutput, TraverseNodeOutput, TraverseParams, TraverseResponseOutput, ValidateIssueOutput,
    ValidateOutput, ValidateSummaryOutput,
};

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

    /// Run the MCP server with stdio transport (for Claude Code, etc.)
    pub async fn serve_stdio(self) -> Result<(), anyhow::Error> {
        let (stdin, stdout) = rmcp::transport::io::stdio();
        rmcp::service::serve_server(self, (stdin, stdout)).await?;
        Ok(())
    }

    /// Run the MCP server with HTTP/SSE transport.
    ///
    /// This starts an HTTP server that implements the MCP Streamable HTTP transport,
    /// allowing web clients and remote AI assistants to connect.
    pub async fn serve_http(self, addr: SocketAddr) -> Result<(), anyhow::Error> {
        let config = StreamableHttpServerConfig::default()
            .with_stateful_mode(true)
            .with_allowed_hosts(vec!["localhost".to_string(), "127.0.0.1".to_string()])
            .with_allowed_origins(vec![
                "http://localhost".to_string(),
                "http://127.0.0.1".to_string(),
            ]);

        let session_manager = Arc::new(LocalSessionManager::default());
        let service_factory = move || Ok::<_, std::io::Error>(self.clone());

        let http_service = StreamableHttpService::new(service_factory, session_manager, config);

        let router = Router::new().nest_service("/mcp", http_service);

        let listener = TcpListener::bind(addr).await?;
        tracing::info!("MCP HTTP server listening on http://{}/mcp", addr);

        let ct = CancellationToken::new();
        axum::serve(listener, router)
            .with_graceful_shutdown(async move {
                ct.cancelled().await;
            })
            .await?;

        Ok(())
    }
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
                        display_title: r.display_title,
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
        let filters: HashMap<String, serde_json::Value> = filter
            .unwrap_or_default()
            .into_iter()
            .filter_map(|f| {
                let mut parts = f.splitn(2, '=');
                match (parts.next(), parts.next()) {
                    (Some(k), Some(v)) => {
                        Some((k.to_string(), serde_json::Value::String(v.to_string())))
                    }
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
impl ServerHandler for McpServer {}
