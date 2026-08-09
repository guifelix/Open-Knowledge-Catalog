---
type: Documentation
title: Architecture
description: System architecture, layers, data flow, and design principles
tags:
  - architecture
  - design
  - internals
owner: Engineering Team
status: published
---

# Architecture

## System Layers

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
| MCP | `rmcp` | Model Context Protocol server |
| Async | `tokio` | MCP transport, cancellation |
| Errors | `thiserror` + `anyhow` + `miette` | Domain errors, context, diagnostics |
| Logging | `tracing` + `tracing-subscriber` | Structured logs, JSON in server mode |
| Schema | `schemars` | JSON Schema from Rust types |
| Config | `figment` | Merge defaults + file + env + CLI |
| Paths | `camino` | UTF-8 path types |

## Code Structure

```
src/
├── main.rs                 # CLI entry point
├── lib.rs                  # Module declarations
├── config.rs               # Configuration types (figment)
├── scanner/
│   ├── mod.rs
│   ├── walker.rs           # Parallel filesystem walker
│   ├── changes.rs          # Incremental change detection
│   └── watcher.rs          # Filesystem watcher (notify)
├── parser/
│   ├── mod.rs
│   ├── frontmatter.rs      # YAML boundary extraction
│   ├── yaml.rs             # saphyr YAML parsing
│   ├── markdown.rs         # pulldown-cmark event parsing
│   └── links.rs            # Link resolution & validation
├── model/
│   ├── mod.rs
│   ├── document.rs         # Document, front-matter, heading, link, section
│   ├── directory.rs        # Directory tree types
│   └── graph.rs            # Graph edge types
├── index/
│   ├── mod.rs
│   ├── database.rs         # Connection, schema, scan orchestration
│   ├── document_store.rs   # Document CRUD + tags/headings/links/metadata
│   ├── search_index.rs     # FTS5 operations
│   ├── graph_store.rs      # Graph edges + traversal
│   ├── queries.rs          # Metadata filtering, browse, get, section
│   ├── validate.rs         # Repository validation (8 checks)
│   ├── export.rs           # JSON export
│   ├── migrations.rs       # Versioned schema migrations
│   ├── graph.rs            # Graph types
│   └── traits.rs           # DocumentStore, SearchIndex, GraphStore traits
├── service/
│   ├── mod.rs
│   ├── browse.rs           # browse_directory
│   ├── documents.rs        # get_document, get_section
│   ├── search.rs           # search_documents
│   ├── graph.rs            # get_links, get_backlinks, traverse_graph
│   └── validation.rs       # validate_repository
└── transport/
    ├── mod.rs
    ├── cli.rs              # Clap CLI definitions
    └── mcp.rs              # MCP server (rmcp)
```

## OKF Bundle Model

### Bundle Structure (per OKF v0.2 §3)

A bundle is a directory tree of markdown files. The directory structure is independent of the domain.

```
path/to/bundle/
  index.md                      # Optional. Directory listing for progressive disclosure.
  log.md                        # Optional. Chronological history of updates.
  <concept>.md                  # A concept at the bundle root.
  <subdirectory>/               # Subdirectories organize concepts into groups.
    index.md
    <concept>.md
    <subdirectory>/
      ...
```

**Reserved filenames** (must not be used for concept documents):
- `index.md` — Directory listing (§8 of spec)
- `log.md` — Update history (§9 of spec)

### Concept Documents (per OKF v0.2 §4)

Every concept is a UTF-8 markdown file with:
1. **YAML frontmatter block** — delimited by `---` at start and closing `---`
2. **Markdown body** — free-form content

#### Required Frontmatter (per §4.1)

```yaml
---
type: <Type name>                  # REQUIRED
title: <Optional display name>
description: <Optional one-line summary>
resource: <Optional canonical URI for the underlying asset>
tags: [<tag>, <tag>, ...]          # Optional
# ... trust, lifecycle, provenance, and computation families (§5, §10)
# ... other producer-defined key/value pairs
---
```

- `type` is the only always-required key. A concept carrying just `type` is fully conformant.
- Type values are not registered centrally. Producers SHOULD pick descriptive, self-explanatory values.
- Consumers MUST tolerate unknown types gracefully.

#### Recommended Frontmatter

- `title` — Human-readable display name. If omitted, consumers MAY derive from filename.
- `description` — Single sentence summary. Used by `index.md` generators, search snippets, previews.
- `resource` — URI uniquely identifying the underlying asset. Absent for abstract concepts.
- `tags` — YAML list of short strings for cross-cutting categorization.

#### Trust, Lifecycle, Provenance (per §5)

```yaml
# Provenance: sources the concept derives from
sources:
  - id: ga4-schema
    resource: https://developers.google.com/analytics/bigquery/export-schema
    title: GA4 BigQuery Export schema
    author: team:ga4-docs
    usage_count: 5000
    last_modified: 2026-05-30
usage_window: { from: 2026-06-01, to: 2026-06-30 }

# Trust: how content was produced and verified
generated: { by: reference_agent/gemini-2.5-pro, at: 2026-06-20T22:53:05Z }
verified:
  - { by: human:ahormati, at: 2026-06-25T09:00:00Z }
  - { by: process:finance-nightly, at: 2026-06-26T02:00:00Z }

# Lifecycle
status: stable        # draft | stable | deprecated
stale_after: 2026-09-23   # absolute date; content is stale on/after this day
```

**Trust tiers** (derived from `verified`, per §5.3):
- No `verified` key ⇒ **unverified**
- `verified` by non-`human:` actors only ⇒ **machine-confirmed**
- `verified` by a `human:<id>` actor ⇒ **human-reviewed**

**Actor convention** (per §7):
- `<producer>/<version>` for agents/tools: `reference_agent/gemini-2.5-pro`
- `human:<id>` for people: `human:ahormati`
- `process:<id>` for automated processes: `process:finance-nightly`

#### Cross-Linking (per §6)

Concepts link to other concepts using standard markdown links:

- **Absolute (bundle-relative):** begins with `/`, interpreted relative to bundle root (recommended)
  ```markdown
  See the [customers table](/tables/customers.md) for the join key.
  ```
- **Relative:** standard markdown relative path
  ```markdown
  See the [neighboring concept](./other.md).
  ```

Links assert a relationship; the specific kind (parent/child, references, joins-with, depends-on) is conveyed by surrounding prose. Consumers treat all links as directed edges of an untyped relationship.

Consumers MUST tolerate broken links — a link whose target does not exist is not malformed; it may represent not-yet-written knowledge.

### Directory Index (`index.md`, per §8)

Optional file at any directory level providing a listing for progressive disclosure. Consumers can synthesize this from frontmatter at consumption time.

### Update Log (`log.md`, per §9)

Optional chronological history of updates to the bundle.

## Storage & Indexing

### SQLite Schema

Key tables:
- `documents` — core document metadata + content hash + parse status
- `document_tags` — many-to-many tags
- `headings` — heading level, title, anchor, position
- `links` — source doc, target path, anchor, external URL, existence
- `metadata_fields` — custom front-matter fields as key/value
- `scan_errors` — parse failures per file
- `file_records` — incremental scan state (path, mtime, size, hash)

### Full-Text Search (FTS5)

Virtual table `document_search` with fields:
- `path`, `title`, `description`, `headings`, `body`

BM25 ranking with field weights:
1. `title` (highest)
2. `description`
3. `headings`
4. `body` (lowest)

### Incremental Indexing

1. Discover current files (parallel walk)
2. Compare with stored `file_records` (path, mtime, size)
3. Skip unchanged files
4. Hash content only when mtime/size changed
5. Parse new/modified files
6. Delete records for removed files
7. Rebuild affected links and search entries

Content hash includes `parser_version` and `index_schema_version` so parser/schema upgrades trigger re-indexing.

## AI Tool Interface

### Core Operations (11 MCP tools)

| Operation | Purpose |
|-----------|---------|
| `browse_directory` | Inspect one area of the OKF hierarchy |
| `get_document` | Retrieve one known concept with metadata, headings, and/or body |
| `get_section` | Extract a specific Markdown section without the full document |
| `search_documents` | Full-text search with optional path/type/tag filters |
| `query_metadata` | Exact structured filtering on front-matter fields |
| `get_links` | Outgoing links from a document |
| `get_backlinks` | Documents referencing a concept |
| `traverse_graph` | Explore related concepts via graph edges |
| `validate_repository` | Report structural problems |

### Transport Options

- **MCP Server** (`okc serve`) — for AI agents via Model Context Protocol (stdio or HTTP transport)
- **CLI** — `okc <command>` for direct shell consumption
- **Native Rust** — library API for embedded use

## Security & Resource Limits

Required protections:
- Fixed allowed repository roots (no `..` escape)
- Configurable symlink policy
- Maximum file size
- Maximum front-matter size
- Maximum scan results
- Maximum graph depth/nodes
- Maximum response characters
- Binary file rejection
- Excluded secret directories (`.git/`, `node_modules/`, `target/`, `.env*`, `secrets/`, `credentials/`)
- Read-only operation by default

Exclusion policy is configurable for repositories that intentionally document similarly-named concepts.

## Multi-Root Repositories

Starting with OKC-00105, the index supports multiple repository roots within a single database. Each root is assigned a stable integer ID and a user-provided or auto-generated string `root_id`. Documents are uniquely identified by the composite key `(root_id, path)`, allowing documents with the same relative path in different roots to coexist without collision.

### Root Configuration
Roots are configured via `OkcConfig.roots` as a list of `RootConfig` objects:
```rust
pub struct RootConfig {
    pub id: Option<String>,  // Stable root identifier (auto-generated from path hash if omitted)
    pub path: PathBuf,       // Absolute path to the root directory
}
```

### Key Behaviors
- **Collision-safe identity**: Documents with the same relative path in different roots are stored separately
- **Root-aware queries**: All query operations (search, browse, get_document, metadata, graph) accept an optional `root_id` filter
- **Cross-root link resolution**: Links are resolved within the same root by default. Cross-root links require explicit `target_root_id` and are represented with a `links_to_cross_root` relation type
- **Per-root statistics**: `IndexStats.roots` provides per-root breakdown of document/error/link/heading counts
- **Migration safety**: Existing single-root indexes upgrade automatically (assigned `root_id = 1`)

### Transport
- **CLI**: `--root` flag to select root
- **MCP**: `root_id` parameter on `search`, `query_metadata`, `browse`, `get_document` tools
- **CLI scan**: Accepts both simple paths and root config objects with explicit IDs

1. **Deterministic software, probabilistic AI** — Tool does retrieval; AI does reasoning
2. **Progressive disclosure** — Browse hierarchy → search → get document → get section
3. **Source traceability** — Every result includes repository path and location
4. **Safety by default** — Path confinement, size limits, read-only, no shell access
5. **Incremental everything** — Scan, index, graph updates all incremental
6. **Trait-based storage** — Swap SQLite → PostgreSQL, FTS5 → Tantivy without changing service layer
7. **Structured errors** — AI agents get actionable error codes, not stack traces