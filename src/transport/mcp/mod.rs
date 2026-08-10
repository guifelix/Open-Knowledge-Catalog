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
    model::{CallToolResult, ContentBlock, JsonObject},
    schemars, tool, tool_handler, tool_router,
    transport::streamable_http_server::session::local::LocalSessionManager,
    transport::streamable_http_server::tower::{StreamableHttpServerConfig, StreamableHttpService},
    Json,
};
use serde::Serialize;
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;

use crate::config::{OkcConfig, RootConfig};
use crate::service::OkcService;
use crate::transport::mcp::types::RootConfig as McpRootConfig;
use std::path::PathBuf;

use self::types::{
    BacklinkParams, BrowseParams, BrowseResultOutput, DirectoryDocumentOutput,
    DocumentBacklinkOutput, DocumentDetailOutput, GetDocumentParams, GetSectionParams,
    GraphEdgeOutput, HeadingInfoOutput, LinkInfoOutput, LinkParams, LinksResponseOutput,
    MetadataParams, MetadataResponseOutput, ScanParams, ScanResultOutput, SearchParams,
    SearchResponseOutput, SearchResultOutput, SectionOutput, SectionResponseOutput, StatsOutput,
    TraverseNodeOutput, TraverseParams, TraverseResponseOutput, ValidateIssueOutput,
    ValidateOutput, ValidateSummaryOutput,
};

fn structured_with_legacy_text<T: Serialize>(
    structured: &T,
    legacy_text: String,
) -> Result<CallToolResult, String> {
    let value = serde_json::to_value(structured)
        .map_err(|error| format!("Failed to serialize structured MCP response: {error}"))?;
    let mut result = CallToolResult::structured(value);
    result.content = vec![ContentBlock::text(legacy_text)];
    Ok(result)
}

fn object_output_schema<T: schemars::JsonSchema + std::any::Any>() -> Arc<JsonObject> {
    rmcp::handler::server::tool::schema_for_output::<T>()
}

fn parse_metadata_filters(
    filter: Option<Vec<String>>,
) -> Result<HashMap<String, serde_json::Value>, String> {
    let mut filters = HashMap::new();
    for expression in filter.unwrap_or_default() {
        let (key, value) = expression
            .split_once('=')
            .ok_or_else(|| format!("Invalid filter '{expression}': expected key=value"))?;
        if key.is_empty() {
            return Err(format!(
                "Invalid filter '{expression}': filter key must not be empty"
            ));
        }
        if filters
            .insert(
                key.to_string(),
                serde_json::Value::String(value.to_string()),
            )
            .is_some()
        {
            return Err(format!(
                "Invalid filter '{expression}': duplicate key '{key}'"
            ));
        }
    }
    Ok(filters)
}

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
        let service = rmcp::service::serve_server(self, (stdin, stdout)).await?;
        // Keep RunningService alive until stdin closes — the service loop runs
        // as a background task; dropping RunningService prematurely cancels it
        // via the CancellationToken DropGuard, causing immediate exit.
        service.waiting().await?;
        Ok(())
    }

    /// Run the MCP server with HTTP/SSE transport.
    ///
    /// This starts an HTTP server that implements the MCP Streamable HTTP transport,
    /// allowing web clients and remote AI assistants to connect.
    pub async fn serve_http(self, addr: SocketAddr) -> Result<(), anyhow::Error> {
        let config = StreamableHttpServerConfig::default()
            .with_legacy_session_mode(true)
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
        let bound = listener.local_addr()?;
        tracing::info!("MCP HTTP server listening on http://{}/mcp", bound);

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
        Parameters(ScanParams {
            roots,
            root_configs,
            db_path,
        }): Parameters<ScanParams>,
    ) -> Result<Json<ScanResultOutput>, String> {
        // Convert MCP RootConfig to config RootConfig
        let mcp_root_configs: Vec<crate::config::RootConfig> = root_configs
            .into_iter()
            .map(|rc| crate::config::RootConfig {
                id: rc.id,
                path: rc.path,
            })
            .collect();

        // Merge explicit root_configs with simple roots
        let mut root_configs: Vec<crate::config::RootConfig> = mcp_root_configs;
        for path in roots {
            // Check if this path is already covered by root_configs
            if !root_configs.iter().any(|rc| rc.path == path) {
                root_configs.push(crate::config::RootConfig {
                    id: None,
                    path: PathBuf::from(path),
                });
            }
        }

        let config = OkcConfig {
            roots: root_configs,
            db_path: db_path.map_or_else(
                || std::path::PathBuf::from("okc_index.db"),
                std::path::PathBuf::from,
            ),
            ..Default::default()
        };

        match OkcService::open(&config).and_then(|mut svc| svc.scan()) {
            Ok(r) => Ok(Json(ScanResultOutput {
                total_files: r.total_files,
                added: r.added,
                modified: r.modified,
                deleted: r.deleted,
                parse_failures: r.parse_failures,
                broken_links: r.broken_links,
                total_links: r.total_links,
                duration_secs: r.duration_secs,
            })),
            Err(e) => Err(format!("Error: {}", e)),
        }
    }

    #[tool(description = "Browse the directory tree of the knowledge catalog")]
    async fn browse(
        &self,
        Parameters(BrowseParams { path, depth, limit }): Parameters<BrowseParams>,
    ) -> Result<Json<BrowseResultOutput>, String> {
        let depth = depth.unwrap_or(1);
        let limit = limit.unwrap_or(100);
        let path = path.unwrap_or_default();

        let svc = self.service.lock().unwrap_or_else(|e| e.into_inner());
        match svc.browse(&path, depth, limit) {
            Ok(r) => Ok(Json(BrowseResultOutput {
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
            })),
            Err(e) => Err(format!("Error: {}", e)),
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
    ) -> Result<Json<DocumentDetailOutput>, String> {
        let include = include.unwrap_or_else(|| vec!["body".to_string(), "headings".to_string()]);
        let include_custom = include.iter().any(|value| value == "custom");
        let max_chars = max_chars.unwrap_or(12000);

        let svc = self.service.lock().unwrap_or_else(|e| e.into_inner());
        match svc.get_document(&path, &include, max_chars) {
            Ok(r) => {
                let custom = include_custom.then(|| r.metadata.custom.clone());
                Ok(Json(DocumentDetailOutput {
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
                    custom,
                    content_hash: r.content_hash,
                    parent_path: r.parent_path,
                    links: r.links.map(|links| {
                        links
                            .into_iter()
                            .map(|link| LinkInfoOutput {
                                target_path: link.target_path,
                                target_anchor: link.target_anchor,
                                external_url: link.external_url,
                                exists_in_repository: link.exists_in_repository,
                                target_root_id: link.target_root_id,
                            })
                            .collect()
                    }),
                    backlinks: r.backlinks.map(|links| {
                        links
                            .into_iter()
                            .map(|link| DocumentBacklinkOutput {
                                source_path: link.source_path,
                                target_anchor: link.target_anchor,
                                exists_in_repository: link.exists_in_repository,
                            })
                            .collect()
                    }),
                }))
            }
            Err(e) => Err(format!("Error: {}", e)),
        }
    }

    #[tool(
        description = "Get a specific section of a document by heading title or anchor",
        output_schema = object_output_schema::<SectionResponseOutput>()
    )]
    async fn get_section(
        &self,
        Parameters(GetSectionParams {
            path,
            heading,
            max_chars,
        }): Parameters<GetSectionParams>,
    ) -> Result<CallToolResult, String> {
        let max_chars = max_chars.unwrap_or(5000);

        let svc = self.service.lock().unwrap_or_else(|e| e.into_inner());
        match svc.get_section(&path, &heading, max_chars) {
            Ok(Some(section)) => {
                let section = SectionOutput {
                    heading: section.0,
                    content: section.1,
                };
                let legacy_text = serde_json::to_string(&Some(&section))
                    .map_err(|error| format!("Failed to serialize section response: {error}"))?;
                structured_with_legacy_text(
                    &SectionResponseOutput {
                        section: Some(section),
                    },
                    legacy_text,
                )
            }
            Ok(None) => {
                // Distinguish a genuinely-missing document from an existing
                // document whose requested section was not found. Only a
                // missing document should surface a NOT_FOUND error with
                // recovery hints; a missing heading keeps the base success
                // shape (`section: null`) so hints are never misleading.
                match svc.document_exists(&path) {
                    Ok(true) => structured_with_legacy_text(
                        &SectionResponseOutput { section: None },
                        "null".to_string(),
                    ),
                    Ok(false) => {
                        let hints = svc
                            .index
                            .load_paths()
                            .map(|paths| {
                                crate::index::queries::suggest::suggest_paths(
                                    &path,
                                    &paths,
                                    crate::index::queries::suggest::MAX_SUGGESTIONS,
                                )
                            })
                            .unwrap_or_default();
                        Err(format!(
                            "Error: {}",
                            crate::error::OkfError::not_found_with_hints(
                                "document",
                                Some(std::path::PathBuf::from(path)),
                                hints,
                            )
                        ))
                    }
                    Err(e) => Err(format!("Error: {}", e)),
                }
            }
            Err(e) => Err(format!("Error: {}", e)),
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
            max_headings,
            heading_depth,
            root_id,
        }): Parameters<SearchParams>,
    ) -> Result<Json<SearchResponseOutput>, String> {
        let limit = limit.unwrap_or(20);

        let svc = self.service.lock().unwrap_or_else(|e| e.into_inner());
        match svc.search(
            &query,
            path_prefix.as_deref(),
            types.as_deref(),
            tags.as_deref(),
            limit,
            max_headings,
            heading_depth,
            root_id,
        ) {
            Ok(r) => Ok(Json(SearchResponseOutput {
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
                        headings: r.headings,
                    })
                    .collect(),
                total_matches: r.total_matches,
                truncated: r.truncated,
            })),
            Err(e) => Err(format!("Error: {}", e)),
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
    ) -> Result<Json<MetadataResponseOutput>, String> {
        let select = select.unwrap_or_default();
        let limit = limit.unwrap_or(100);
        let filters = parse_metadata_filters(filter)?;

        let svc = self.service.lock().unwrap_or_else(|e| e.into_inner());
        match svc.query_metadata(&filters, &select, limit) {
            Ok(r) => Ok(Json(MetadataResponseOutput {
                results: r.results,
                total_matches: r.total_matches,
                truncated: r.truncated,
            })),
            Err(e) => Err(format!("Error: {}", e)),
        }
    }

    #[tool(
        description = "Get all outgoing links from a document",
        output_schema = object_output_schema::<LinksResponseOutput>()
    )]
    async fn get_links(
        &self,
        Parameters(LinkParams { path }): Parameters<LinkParams>,
    ) -> Result<CallToolResult, String> {
        let svc = self.service.lock().unwrap_or_else(|e| e.into_inner());
        match svc.get_links(&path) {
            Ok(links) => {
                let links = links
                    .into_iter()
                    .map(|l| LinkInfoOutput {
                        target_path: l.target_path,
                        target_anchor: l.target_anchor,
                        external_url: l.external_url,
                        exists_in_repository: l.exists_in_repository,
                        target_root_id: l.target_root_id,
                    })
                    .collect::<Vec<_>>();
                let legacy_text = serde_json::to_string(&links)
                    .map_err(|error| format!("Failed to serialize links response: {error}"))?;
                structured_with_legacy_text(&LinksResponseOutput { links }, legacy_text)
            }
            Err(e) => Err(format!("Error: {}", e)),
        }
    }

    #[tool(
        description = "Get all backlinks pointing to a document",
        output_schema = object_output_schema::<LinksResponseOutput>()
    )]
    async fn get_backlinks(
        &self,
        Parameters(BacklinkParams { path, limit }): Parameters<BacklinkParams>,
    ) -> Result<CallToolResult, String> {
        let limit = limit.unwrap_or(50);
        let svc = self.service.lock().unwrap_or_else(|e| e.into_inner());
        match svc.get_backlinks(&path, limit) {
            Ok(links) => {
                let links = links
                    .into_iter()
                    .map(|l| LinkInfoOutput {
                        target_path: l.target_path,
                        target_anchor: l.target_anchor,
                        external_url: l.external_url,
                        exists_in_repository: l.exists_in_repository,
                        target_root_id: l.target_root_id,
                    })
                    .collect::<Vec<_>>();
                let legacy_text = serde_json::to_string(&links)
                    .map_err(|error| format!("Failed to serialize backlinks response: {error}"))?;
                structured_with_legacy_text(&LinksResponseOutput { links }, legacy_text)
            }
            Err(e) => Err(format!("Error: {}", e)),
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
    ) -> Result<Json<TraverseResponseOutput>, String> {
        let relations = relations.unwrap_or_default();
        let max_depth = max_depth.unwrap_or(3);
        let max_nodes = max_nodes.unwrap_or(50);

        let svc = self.service.lock().unwrap_or_else(|e| e.into_inner());
        match svc.traverse(&start, &relations, max_depth, max_nodes) {
            Ok(r) => Ok(Json(TraverseResponseOutput {
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
            })),
            Err(e) => Err(format!("Error: {}", e)),
        }
    }

    #[tool(
        description = "Get index statistics: document count, error count, link count, heading count"
    )]
    async fn get_stats(&self) -> Result<Json<StatsOutput>, String> {
        let svc = self.service.lock().unwrap_or_else(|e| e.into_inner());
        match svc.get_stats() {
            Ok(s) => Ok(Json(StatsOutput {
                document_count: s.document_count,
                error_count: s.error_count,
                link_count: s.link_count,
                heading_count: s.heading_count,
            })),
            Err(e) => Err(format!("Error: {}", e)),
        }
    }

    #[tool(
        description = "Validate the repository and check for issues across all indexed documents"
    )]
    async fn validate(&self) -> Result<Json<ValidateOutput>, String> {
        let svc = self.service.lock().unwrap_or_else(|e| e.into_inner());
        match svc.validate_report() {
            Ok(r) => Ok(Json(ValidateOutput {
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
            })),
            Err(e) => Err(format!("Error: {}", e)),
        }
    }
}

#[tool_handler]
impl ServerHandler for McpServer {}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used)]

    use crate::config::OkcConfig;

    use super::{parse_metadata_filters, McpServer};

    #[test]
    fn mcp_server_rejects_invalid_configuration() {
        let error = McpServer::new(&OkcConfig::default())
            .err()
            .expect("MCP construction should reject missing roots");
        assert!(error.to_string().contains("At least one root directory"));
    }

    #[test]
    fn metadata_filter_parser_accepts_values_containing_equals() {
        let filters = parse_metadata_filters(Some(vec!["location=s3://bucket?a=b".to_string()]))
            .expect("valid metadata filter");
        assert_eq!(filters["location"], "s3://bucket?a=b");
    }

    #[test]
    fn metadata_filter_parser_rejects_malformed_and_duplicate_filters() {
        assert!(parse_metadata_filters(Some(vec!["type".to_string()])).is_err());
        assert!(parse_metadata_filters(Some(vec!["=Metric".to_string()])).is_err());
        assert!(parse_metadata_filters(Some(vec![
            "type=Metric".to_string(),
            "type=Dataset".to_string(),
        ]))
        .is_err());
    }
}
