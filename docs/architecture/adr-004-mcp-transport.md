# ADR 004: MCP Transport for AI Integration

**Status**: Accepted
**Date**: 2026-07-25
**Deciders**: Architecture team
**Technical Story**: OKC-00033.05

## Context

The Open Knowledge Catalog must be accessible to AI assistants (Claude, Cursor, etc.) for:
- Retrieving relevant knowledge during coding tasks
- Answering questions about the codebase
- Navigating documentation and architecture
- Validating implementation against specs

The **Model Context Protocol (MCP)** is the emerging standard for AI-tool integration, supported by:
- Anthropic (Claude Desktop, Claude Code)
- Cursor, Windsurf, Zed editors
- Open-source ecosystem (rmcp, mcp-rs, etc.)

## Decision

Implement an **MCP server** using the `rmcp` crate as the primary AI integration transport, alongside the existing CLI.

## Alternatives Considered

| Option | Pros | Cons | Verdict |
|--------|------|------|---------|
| **MCP (rmcp)** | Standard, growing ecosystem, bidirectional, streaming, typed tools | New protocol, evolving | ✅ **Selected** |
| CLI JSON (`--json`) | Simple, universal, scriptable | No streaming, no discovery, stateless | ⚠️ Keep as fallback |
| Custom HTTP API | Full control, REST familiar | Not AI-native, no standard schema | ❌ Rejected |
| LSP (Language Server) | IDE integration | Wrong abstraction (code vs knowledge) | ❌ Rejected |
| GraphQL | Typed schema, introspection | Overhead, not AI-optimized | ❌ Rejected |
| Plugin per editor | Native UX | Fragmented, high maintenance | ❌ Rejected |

## Consequences

### Positive
- **Standard protocol features of MCP**:
  - **Tool discovery**: AI lists available tools at connection time
  - **Typed schemas**: JSON Schema from Rust types via `schemars`
  - **Streaming**: Large results stream incrementally
  - **Cancellation**: AI can cancel long-running operations
  - **Progress notifications**: Long operations report progress
  - **Resources**: Expose files, docs as readable resources
  - **Prompts**: Pre-defined prompt templates for common tasks
  - **Ecosystem**: Works with Claude, Cursor, Windsurf, Zed, Continue, etc.
  - **Transport agnostic**: stdio, HTTP/SSE, WebSocket

- **Implementation benefits**:
  - `rmcp` provides derive macros for tools (`#[tool]`)
  - Automatic JSON Schema generation from Rust types
  - Built-in request/response handling
  - Async/await with Tokio

### Negative
- **Protocol maturity**: MCP spec still evolving (2024-2025)
- **Transport complexity**: stdio for local, HTTP/SSE for remote
- **Schema drift**: Rust types must match schema expectations
- **Debugging**: Harder than CLI (stdio transport)
- **Single-threaded by default**: `Mutex<OkcService>` for thread safety

## Implementation

### Server Structure (`transport/mcp.rs`)

```rust
use rmcp::{
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    schemars, tool, tool_handler, tool_router,
};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct McpServer {
    pub service: Arc<Mutex<OkcService>>,
    tool_router: ToolRouter<Self>,
}

impl McpServer {
    pub fn new(config: &OkcConfig) -> Result<Self> {
        let service = OkcService::open(config)?;
        Ok(Self {
            service: Arc::new(Mutex::new(service)),
            tool_router: Self::tool_router(),
        })
    }
}

#[tool_router]
impl McpServer {
    #[tool(description = "Scan directories and index all markdown files")]
    async fn scan(&self, Parameters(ScanParams { roots, db_path }): Parameters<ScanParams>) -> String { ... }

    #[tool(description = "Browse the directory tree of the knowledge catalog")]
    async fn browse(&self, Parameters(BrowseParams { path, depth, limit }): Parameters<BrowseParams>) -> String { ... }

    #[tool(description = "Get a document's full content and metadata")]
    async fn get_document(&self, Parameters(GetDocumentParams { path, include, max_chars }): Parameters<GetDocumentParams>) -> String { ... }

    #[tool(description = "Full-text search across all documents")]
    async fn search(&self, Parameters(SearchParams { query, path_prefix, types, tags, limit }): Parameters<SearchParams>) -> String { ... }

    #[tool(description = "Query document metadata with filters")]
    async fn query_metadata(&self, Parameters(MetadataParams { filter, select, limit }): Parameters<MetadataParams>) -> String { ... }

    #[tool(description = "Get all outgoing links from a document")]
    async fn get_links(&self, Parameters(LinkParams { path }): Parameters<LinkParams>) -> String { ... }

    #[tool(description = "Get all backlinks pointing to a document")]
    async fn get_backlinks(&self, Parameters(BacklinkParams { path, limit }): Parameters<BacklinkParams>) -> String { ... }

    #[tool(description = "Traverse the knowledge graph from a starting document")]
    async fn traverse(&self, Parameters(TraverseParams { start, relations, max_depth, max_nodes }): Parameters<TraverseParams>) -> String { ... }

    #[tool(description = "Validate the repository and check for issues")]
    async fn validate(&self) -> String { ... }

    #[tool(description = "Get index statistics")]
    async fn get_stats(&self) -> String { ... }
}

#[tool_handler]
impl rmcp::handler::server::ServerHandler for McpServer {}
```

### Parameter/Response Types (Auto-Schema)

```rust
#[derive(Deserialize, schemars::JsonSchema)]
struct SearchParams {
    query: String,
    path_prefix: Option<String>,
    types: Option<Vec<String>>,
    tags: Option<Vec<String>>,
    limit: Option<usize>,
}

#[derive(Serialize, schemars::JsonSchema)]
struct SearchResponseOutput {
    results: Vec<SearchResultOutput>,
    total_matches: usize,
    truncated: bool,
}

#[derive(Serialize, schemars::JsonSchema)]
struct SearchResultOutput {
    path: String,
    title: Option<String>,
    concept_type: Option<String>,
    score: f64,
    excerpt: String,
}
```

### Main Entry Point (`main.rs`)

```rust
Command::Serve { root } => {
    let roots = if root.is_empty() {
        vec![std::env::current_dir()?]
    } else {
        root
    };
    config.roots = roots;

    let server = McpServer::new(&config)?;
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        let (stdin, stdout) = rmcp::transport::io::stdio();
        rmcp::service::serve_server(server, (stdin, stdout)).await
    })?;
}
```

### Client Configuration (Claude Desktop)

```json
{
  "mcpServers": {
    "okc": {
      "command": "okc",
      "args": ["serve", "/path/to/knowledge-base"],
      "env": {}
    }
  }
}
```

## Tool Set (9 Core Tools)

| Tool | Purpose | Key Parameters |
|------|---------|----------------|
| `scan` | Index repository | `roots`, `db_path` |
| `browse` | Directory tree | `path`, `depth`, `limit` |
| `get_document` | Full document | `path`, `include`, `max_chars` |
| `get_section` | Section by heading | `path`, `heading`, `max_chars` |
| `search` | Full-text search | `query`, `path_prefix`, `types`, `tags`, `limit` |
| `query_metadata` | Structured filter | `filter`, `select`, `limit` |
| `get_links` | Outgoing links | `path` |
| `get_backlinks` | Incoming links | `path`, `limit` |
| `traverse` | Graph walk | `start`, `relations`, `max_depth`, `max_nodes` |
| `validate` | Health check | (none) |
| `get_stats` | Index stats | (none) |

## Security Considerations

- **Path confinement**: All paths resolved relative to configured roots
- **Read-only by default**: No write tools exposed (scan is read+index)
- **Size limits**: `max_chars`, `limit` prevent token exhaustion
- **No shell access**: Pure Rust, no command execution
- **Sandboxed**: Runs in user's context, no elevated privileges

## Future Extensions

| Feature | MCP Capability | Status |
|---------|----------------|--------|
| Resources | Expose `index.md`, `log.md` as readable resources | Planned |
| Prompts | Pre-built prompts for "explain this module", "find related" | Planned |
| Progress | Stream scan progress for large repos | Planned |
| Cancellation | Cancel long searches | Supported by rmcp |
| HTTP/SSE | Remote MCP server for team sharing | ✅ **Implemented** |
| Auth | Token-based for remote access | Future |

## Related ADRs

- ADR-001: SQLite as Primary Storage Backend
- ADR-002: FTS5 for Full-Text Search
- ADR-003: Trait-Based Storage Abstraction

## References

- [Model Context Protocol Spec](https://modelcontextprotocol.io/)
- [rmcp crate](https://github.com/rmcp-rs/rmcp)
- [MCP TypeScript SDK](https://github.com/modelcontextprotocol/typescript-sdk)
- [Claude Desktop MCP Config](https://claude.ai/download)