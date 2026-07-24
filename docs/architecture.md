# Architecture

## Overview

The system has five main layers, each with a single responsibility:

```
┌─────────────────────────────────────────────────────────────┐
│                    AI Tool Interface                        │
│  browse │ get │ section │ search │ filter │ links │ graph  │
│  backlinks │ traverse │ validate                           │
└─────────────────────────┬───────────────────────────────────┘
                          ▼
┌─────────────────────────────────────────────────────────────┐
│                   Index & Storage Layer                     │
│  SQLite: metadata indexes, FTS5 full-text search, graph    │
└─────────────────────────┬───────────────────────────────────┘
                          ▼
┌─────────────────────────────────────────────────────────────┐
│                     Repository Model                        │
│  Directory tree (hierarchy) + Document graph (relationships)│
└─────────────────────────┬───────────────────────────────────┘
                          ▼
┌─────────────────────────────────────────────────────────────┐
│                      Parsing Layer                          │
│  Front-matter extraction │ YAML parsing │ Markdown parsing │
│  Heading extraction │ Link resolution │ Section boundaries  │
└─────────────────────────┬───────────────────────────────────┘
                          ▼
┌─────────────────────────────────────────────────────────────┐
│                      Filesystem Layer                       │
│  Parallel walk (ignore crate) │ ignore-file support │      │
│  Symlink policy │ Size/mod-time tracking │ Change detection │
└─────────────────────────────────────────────────────────────┘
```

## Layer Details

### 1. Filesystem Layer (`src/scanner/`)

**Responsibilities:**
- Recursively walk approved repository roots
- Discover Markdown files (`.md`, `.markdown`)
- Respect `.gitignore` and custom exclude patterns
- Skip hidden/excluded directories
- Normalize paths to repository-relative form
- Enforce symlink policy (follow/skip/reject)
- Collect file size and modification time
- Detect added, changed, and deleted files

**Key Components:**
- `walker.rs` — Parallel filesystem walker using `ignore` crate
- `changes.rs` — Incremental change detection via mtime/size/content hash

**Output:** `FileRecord { path, absolute_path, size, modified_at }`

### 2. Parsing Layer (`src/parser/`)

**Responsibilities:**
- Extract YAML front matter from Markdown files
- Parse YAML into structured data (preserving custom fields)
- Parse Markdown for headings, links, sections, searchable text
- Resolve relative links against source document path
- Check link target existence

**Key Components:**
- `frontmatter.rs` — Fast boundary detection using `memchr`
- `yaml.rs` — YAML parsing via `saphyr` (Serde-compatible, panic-free)
- `markdown.rs` — Streaming event parser via `pulldown-cmark`
- `links.rs` — Link resolution, normalization, existence checking

**Output:** `ParsedDocument { front_matter, headings, links, sections, searchable_text }`

### 3. Repository Model (`src/model/`)

**Directory Tree (`directory.rs`):**
- Represents filesystem containment hierarchy
- Each node: path, optional `index.md`, child directories, documents
- Used for browsing, progressive disclosure, subtree search limiting

**Document Graph (`graph.rs`):**
- Represents relationships between concepts
- Edge types: `contains`, `parent`, `links_to`, `linked_from`
- Future: `depends_on`, `owned_by`, `implements`, `uses`, `related_to`
- Coexists with directory tree (hierarchy remains first-class API)

### 4. Index & Storage Layer (`src/index/`)

**SQLite Schema:**
- `documents` — Core document metadata + content hash + parse status
- `document_tags` — Many-to-many tags
- `headings` — Heading level, title, anchor, position
- `links` — Source doc, target path, anchor, external URL, existence
- `metadata_fields` — Custom front-matter fields as key/value
- `scan_errors` — Parse failures per file
- `document_search` (FTS5) — Full-text search: path, title, description, headings, body

**Key Components:**
- `database.rs` — Connection management, schema, scan orchestration
- `document_store.rs` — Document CRUD, tags, headings, links, metadata
- `search_index.rs` — FTS5 operations, BM25 ranking
- `graph_store.rs` — Graph edges, traversal (recursive CTEs)
- `queries.rs` — Metadata filtering, browse, get, section retrieval
- `validate.rs` — Repository validation (8 checks)
- `export.rs` — JSON export for CLI/benchmarks
- `migrations.rs` — Versioned schema migrations
- `traits.rs` — `DocumentStore`, `SearchIndex`, `GraphStore` traits for backend swapping

### 5. AI Tool Interface (`src/service/` + `src/transport/`)

**Service Layer (`service/`):**
- `browse.rs` — `browse_directory`
- `documents.rs` — `get_document`, `get_section`
- `search.rs` — `search_documents`
- `graph.rs` — `get_links`, `get_backlinks`, `traverse_graph`
- `validation.rs` — `validate_repository`

**Transport Layer (`transport/`):**
- `cli.rs` — Clap-based CLI with all 9 operations
- `mcp.rs` — MCP server (rmcp, child-process transport)

## Technology Stack

| Layer | Library | Purpose |
|-------|---------|---------|
| Filesystem | `ignore` | Parallel walk with `.gitignore` support |
| Front-matter | `memchr` + custom | Fast boundary detection, size limits |
| YAML | `saphyr` | Serde-compatible, panic-free, source spans |
| Markdown | `pulldown-cmark` | Streaming event parser, no heavy AST |
| Storage | `rusqlite` (SQLite) | Metadata, FTS5, graph edges, transactions |
| Hashing | `blake3` | Content fingerprints for incremental scans |
| CLI | `clap` | Command-line interface |
| Serialization | `serde` + `serde_json` | Structured I/O |
| MCP | `rmcp` | Model Context Protocol server |
| Async | `tokio` | MCP transport, cancellation |
| Errors | `thiserror` + `anyhow` + `miette` | Domain errors, context, diagnostics |
| Logging | `tracing` + `tracing-subscriber` | Structured logs, JSON in server mode |
| Config | `figment` | Merge defaults + file + env + CLI |

## Data Flow

```
OKF repository
    ↓
parallel filesystem walker
    ↓
front-matter & Markdown parsers
    ↓
normalized documents
    ├── directory hierarchy
    ├── metadata
    ├── headings & sections
    └── document-link graph
    ↓
SQLite
    ├── metadata indexes
    ├── FTS5 text index
    └── graph edges
    ↓
bounded AI tools
    ├── browse
    ├── search
    ├── filter
    ├── read section
    ├── follow links
    └── validate
    ↓
AI-generated answer with source paths
```

## Design Principles

1. **Deterministic software, probabilistic AI** — Tool does retrieval; AI does reasoning
2. **Progressive disclosure** — Browse hierarchy → search → get document → get section
3. **Source traceability** — Every result includes repository path and location
4. **Safety by default** — Path confinement, size limits, read-only, no shell access
5. **Incremental everything** — Scan, index, graph updates all incremental
6. **Trait-based storage** — Swap SQLite → PostgreSQL, FTS5 → Tantivy without changing service layer
7. **Structured errors** — AI agents get actionable error codes, not stack traces