# Open Knowledge Catalog - Architecture Overview

## System Context

The Open Knowledge Catalog (OKC) is a markdown knowledge base indexer and query engine. It scans directories of markdown files with YAML front-matter, builds a searchable index with full-text search and graph navigation, and exposes the data via CLI and MCP (Model Context Protocol) server for AI assistant integration.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           EXTERNAL ACTORS                                    │
├─────────────────────────────────────────────────────────────────────────────┤
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐  │
│  │   Human     │    │   AI        │    │   CI/CD     │    │   File      │  │
│  │   User      │    │   Assistant │    │   Pipeline  │    │   System    │  │
│  │   (CLI)     │    │   (MCP)     │    │   (Tests)   │    │   (Markdown)│  │
│  └──────┬──────┘    └──────┬──────┘    └──────┬──────┘    └──────┬──────┘  │
└─────────┼──────────────────┼──────────────────┼──────────────────┼──────────┘
          │                  │                  │                  │
          ▼                  ▼                  ▼                  ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                         TRANSPORT LAYER                                      │
├─────────────────────────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────┐    ┌─────────────────────────────────────┐ │
│  │         CLI (clap)          │    │         MCP Server (rmcp)           │ │
│  │  scan, browse, get, search, │    │  9 tools: scan, browse, get_doc,    │ │
│  │  section, metadata, links,  │    │  get_section, search, query_meta,   │ │
│  │  backlinks, traverse,       │    │  get_links, get_backlinks,          │ │
│  │  validate, stats, watch,    │    │  traverse, validate, get_stats      │ │
│  │  serve                      │    │                                     │ │
│  └──────────────┬──────────────┘    └──────────────┬──────────────────────┘ │
└─────────────────┼──────────────────────────────────┼────────────────────────┘
                  │                                  │
                  ▼                                  ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                         SERVICE LAYER                                        │
├─────────────────────────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────────────────────────────┐ │
│  │                        OkcService (Facade)                              │ │
│  │  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌───────┐ │ │
│  │  │ Browse  │ │Documents│ │ Graph   │ │ Search  │ │Validate │ │ Watch │ │ │
│  │  └────┬────┘ └────┬────┘ └────┬────┘ └────┬────┘ └────┬────┘ └───┬───┘ │ │
│  └───────┼───────────┼───────────┼───────────┼───────────┼───────────┼─────┘ │
└──────────┼───────────┼───────────┼───────────┼───────────┼───────────┼───────┘
           │           │           │           │           │           │
           ▼           ▼           ▼           ▼           ▼           ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                         INDEX LAYER (RepositoryIndex)                        │
├─────────────────────────────────────────────────────────────────────────────┤
│  ┌──────────────────┐  ┌──────────────────┐  ┌──────────────────────────┐  │
│  │ Document Store   │  │ Search Index     │  │ Graph Store              │  │
│  │ (SqliteDocument  │  │ (SqliteSearch    │  │ (SqliteGraphStore)       │  │
│  │  Store)          │  │  Index)          │  │                          │  │
│  │                  │  │                  │  │ - Link edges             │  │
│  │ - Documents      │  │ - FTS5 virtual   │  │ - Traversal              │  │
│  │ - Tags           │  │   table          │  │ - Backlinks              │  │
│  │ - Headings       │  │ - BM25 ranking   │  │                          │  │
│  │ - Links          │  │ - Field weights  │  │                          │  │
│  │ - Metadata       │  │                  │  │                          │  │
│  │ - Scan errors    │  │                  │  │                          │  │
│  └────────┬─────────┘  └────────┬─────────┘  └────────────┬─────────────┘  │
│           │                     │                         │                │
│           └─────────────────────┼─────────────────────────┘                │
│                                 ▼                                          │
│                    ┌────────────────────────┴────────┐                                  │
│                    │   SQLite Database  │                                  │
│                    │   (WAL mode)       │                                  │
│                    │                    │                                  │
│                    │ Tables:            │                                  │
│                    │ - documents        │                                  │
│                    │ - document_tags    │                                  │
│                    │ - headings         │                                  │
│                    │ - links            │                                  │
│                    │ - metadata_fields  │                                  │
│                    │ - scan_errors      │                                  │
│                    │ - file_records     │                                  │
│                    │ - document_search  │  (FTS5 virtual table)            │
│                    └────────────────────┘                                  │
└─────────────────────────────────────────────────────────────────────────────┘
           │
           ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                         SCANNER & PARSER LAYER                               │
├─────────────────────────────────────────────────────────────────────────────┤
│  ┌──────────────────┐  ┌──────────────────┐  ┌──────────────────────────┐  │
│  │ Scanner          │  │ Change Detector  │  │ Parser Pipeline          │  │
│  │ (walker)         │  │ (changes)        │  │                          │  │
│  │                  │  │                  │  │ ┌──────────────────────┐ │  │
│  │ - Parallel walk  │  │ - Added          │  │ │ FrontMatterExtractor │ │  │
│  │ - Glob patterns  │  │ - Modified       │  │ │ (YAML)               │ │  │
│  │ - Exclude rules  │  │ - Deleted        │  │ ├──────────────────────┤ │  │
│  │ - Size limits    │  │ - Unchanged      │  │ │ LinkResolver         │ │  │
│  └────────┬─────────┘  └────────┬─────────┘  │ │ (wiki-links, URLs)   │ │  │
│           │                     │            │ ├──────────────────────┤ │  │
│           └─────────────────────┼────────────┤ │ MarkdownParser       │ │  │
│                                 ▼            │ │ (headings, sections) │ │  │
│                    ┌────────────────────────┐ │ ├──────────────────────┤ │  │
│                    │ File Watcher           │ │ │ YamlParser           │ │  │
│                    │ (watcher)              │ │ │ (serde_yaml)         │ │  │
│                    │ - notify crate         │ │ └──────────────────────┘ │  │
│                    │ - Debouncing           │ └──────────────────────────┘  │
│                    │ - Reconciliation       │                                 │
│                    └────────────────────────┘                                 │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Module Boundaries

| Module | Responsibility | Key Types |
|--------|---------------|-----------|
| `config` | Configuration types and defaults | `OkcConfig` |
| `model` | Core data structures | `Document`, `Link`, `GraphEdge`, `SearchResult` |
| `parser` | Markdown/YAML/front-matter/link parsing | `DocumentParser`, `LinkResolver` |
| `scanner` | Filesystem discovery, change detection, watching | `Scanner`, `ChangeDetector`, `FileWatcher` |
| `index` | Storage layer: SQLite, FTS5, Graph, Traits | `RepositoryIndex`, `SqliteDocumentStore`, `SqliteSearchIndex`, `SqliteGraphStore` |
| `service` | High-level API for CLI/MCP | `OkcService` (Browse, Documents, Graph, Search, Validate, Watch) |
| `transport` | CLI commands, MCP server | `Cli`, `McpServer` |

## Data Flow

### Scan Pipeline (Full Index)

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│  Discover   │───▶│  Detect     │───▶│  Parse      │───▶│  Store      │
│  Files      │    │  Changes    │    │  Documents  │    │  Results    │
└─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘
      │                  │                  │                  │
      ▼                  ▼                  ▼                  ▼
  Scanner::         ChangeDetector    DocumentParser    RepositoryIndex
  discover()        ::detect()        ::process_changes()  ::process_changes()
  - walk roots      - compare with    - extract FM      - upsert docs
  - filter .md      file_records      - resolve links   - index FTS5
  - collect meta    - classify        - parse MD        - store graph
```

### Incremental Watch Flow

```
File System Event
       │
       ▼
┌──────────────────┐
│  FileWatcher     │  (notify crate, debounced)
│  - collect events│
│  - batch by      │
│    debounce_ms   │
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│  Reconcile       │  (periodic full scan)
│  - full walk     │
│  - detect drift  │
└────────┬─────────┘
         │
         ▼
    [Same as Scan Pipeline]
```

### Query Flow (Search Example)

```
CLI/MCP Request
       │
       ▼
┌──────────────────┐
│  OkcService      │
│  .search()       │
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│  RepositoryIndex │
│  .search()       │
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│  SqliteSearch    │
│  Index           │
│  - FTS5 query    │
│  - BM25 rank     │
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│  DocumentStore   │
│  - fetch details │
│  (title, path,   │
│   excerpt)       │
└────────┬─────────┘
         │
         ▼
    Response
```

## Key Abstractions

### Storage Traits (`index/traits.rs`)

```rust
pub trait DocumentStore {
    fn upsert_document(&self, doc: &DocumentRecord) -> Result<()>;
    fn get_document(&self, path: &str) -> Result<Option<DocumentRecord>>;
    fn delete_document(&self, path: &str) -> Result<()>;
    // ... tags, headings, links, metadata, errors
}

pub trait SearchIndex {
    fn index_document(&self, doc: &SearchableDocument) -> Result<()>;
    fn remove_document(&self, path: &str) -> Result<()>;
    fn search(&self, query: &SearchQuery) -> Result<SearchResponse>;
}

pub trait GraphStore {
    fn store_links(&self, source: &str, links: &[Link]) -> Result<()>;
    fn remove_links(&self, source: &str) -> Result<()>;
    fn get_links(&self, path: &str) -> Result<Vec<GraphEdge>>;
    fn get_backlinks(&self, path: &str, limit: usize) -> Result<Vec<GraphEdge>>;
    fn traverse(&self, params: &TraverseParams) -> Result<TraverseResponse>;
}
```

This allows swapping backends (e.g., SQLite → PostgreSQL, FTS5 → Tantivy) without changing the service layer.

### Parser Pipeline (`index/parser.rs`)

```rust
pub struct ParsedDocument {
    pub path: String,
    pub parent_path: String,
    pub title: Option<String>,
    pub concept_type: Option<String>,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub markdown_body: String,
    pub body_text: String,           // Plain text for search
    pub headings: Vec<HeadingInfo>,
    pub links: Vec<LinkInfo>,
    pub custom_fields: HashMap<String, serde_json::Value>,
    pub parse_status: ParseStatus,
    pub parse_errors: Vec<ParseError>,
    pub size: u64,
    pub modified_at: i64,
    pub content_hash: String,
}
```

## Concurrency Model

- **SQLite WAL mode**: Multiple readers, single writer
- **Service layer**: `Arc<Mutex<OkcService>>` for MCP (single-threaded async)
- **Scanner**: Rayon parallel walker for file discovery
- **Parser**: Sequential per-file (CPU-bound, low contention)

## Configuration

All configuration via `OkcConfig`:
- Root directories to scan
- Exclude patterns (glob)
- Size limits (file, front-matter)
- Graph traversal limits
- Database path
- Watcher debounce/reconcile intervals

## Security Boundaries

1. **Path confinement**: All operations restricted to configured roots
2. **Symlink policy**: Configurable (default: deny)
3. **Size limits**: Max file size, max front-matter size
4. **Excluded directories**: `.git/`, `node_modules/`, `target/`, `.env*`, `secrets/`, `credentials/`
5. **Read-only by default**: No write operations via MCP/CLI except index management
6. **No shell access**: Pure Rust implementation

## Deployment

- Single binary (`okc`) with embedded SQLite
- Database file: `okc_index.db` (configurable)
- MCP server: stdio transport (JSON-RPC over stdin/stdout)
- No external dependencies at runtime