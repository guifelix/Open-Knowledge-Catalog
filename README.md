# Open Knowledge Catalog (OKC)

A local-first tool that allows AI agents to safely browse, parse, search, and reason over an Open Knowledge Format (OKF) repository.

## Overview

The Open Knowledge Catalog transforms a filesystem-based collection of Markdown documents with YAML front matter into a structured, searchable knowledge base that AI agents can query through a small set of deterministic operations.

### Why This Tool?

OKF gives knowledge a portable, human-readable representation, but it doesn't provide fast retrieval, structured querying, validation, or an AI tool interface. Without a dedicated tool, an AI agent would need to:

- Recursively inspect the filesystem
- Open many files individually
- Repeatedly parse front matter
- Search raw Markdown text
- Resolve relative links
- Infer directory structure
- Manage its own context limits

This tool moves those responsibilities into deterministic software, giving the AI a controlled view:

```
OKF files → scanner & parser → structured index → bounded AI tool calls → relevant source context → AI answer
```

### Benefits

- **Performance**: Repository parsed once, updated incrementally; unchanged files skipped
- **Accuracy**: YAML metadata queried as structured data, not plain text search
- **Context Efficiency**: AI receives only relevant metadata, headings, excerpts, or sections
- **Navigability**: Directory hierarchy supports progressive disclosure; document graph supports link-following
- **Safety**: Restricts accessible directories, file types, sizes, traversal depth, output size
- **Source Traceability**: Every result includes repository path and source location

## Features

### Core Operations (9 AI-facing tools)

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
| `validate_repository` | Report structural problems (broken links, malformed YAML, missing index files) |

### Supported OKF Format

Each document is a Markdown file with YAML front matter:

```markdown
---
type: Metric
title: Monthly Revenue
description: Recognized recurring revenue for the month
tags:
  - finance
  - executive
owner: Finance Analytics
status: published
---

# Definition

Monthly Revenue represents...

# Calculation

Revenue is recognized when...
```

### Repository Structure

```
/
├── metrics/
│   ├── index.md
│   ├── monthly-revenue.md
│   └── customer-count.md
└── datasets/
    ├── index.md
    └── customer-orders.md
```

- `index.md` files provide directory summaries (optional, configurable)
- Relative links between documents are resolved and validated
- Custom front-matter fields are preserved as generic metadata

## Installation

### Prerequisites

- Rust 1.75+ (install via [rustup](https://rustup.rs/))
- SQLite3 development headers (usually `libsqlite3-dev` on Linux)

### Build from Source

```bash
git clone https://github.com/your-org/open-knowledge-catalog
cd open-knowledge-catalog
cargo build --release
```

The binary will be at `target/release/okc`.

### Install

```bash
cargo install --path .
```

Or copy the binary to your PATH:

```bash
cp target/release/okc ~/.local/bin/
```

## Quick Start

### 1. Create an OKF Repository

```bash
mkdir -p my-knowledge/{metrics,datasets}
```

Create `my-knowledge/metrics/monthly-revenue.md`:

```markdown
---
type: Metric
title: Monthly Revenue
description: Recognized recurring revenue for the month
tags: [finance, executive]
owner: Finance Analytics
status: published
---

# Definition

Monthly Revenue represents the total recognized revenue for a calendar month.

# Recognition Rules

Revenue is recognized when:
1. Service is delivered
2. Payment is reasonably assured
3. Amount is measurable
```

Create `my-knowledge/datasets/customer-orders.md`:

```markdown
---
type: Dataset
title: Customer Orders
description: Raw order data from the e-commerce platform
tags: [sales, raw-data]
owner: Data Engineering
status: published
---

# Schema

| Column | Type | Description |
|--------|------|-------------|
| order_id | string | Unique order identifier |
| customer_id | string | Customer identifier |
| order_date | date | Date of order |
| amount | decimal | Order total in USD |
```

### 2. Scan the Repository

```bash
okc scan --root my-knowledge
```

Output:
```
Scan complete:
  Total files: 2
  Added: 2
  Modified: 0
  Deleted: 0
  Parse failures: 0
  Broken links: 0
  Total links: 0
  Duration: 0.01s
```

This creates `okc_index.db` in the current directory (configurable via `--db-path`).

### 3. Query the Knowledge Base

```bash
# Browse the hierarchy
okc browse

# Browse a specific directory
okc browse metrics --depth 1

# Search for concepts
okc search "revenue recognition"

# Get a document
okc get metrics/monthly-revenue.md --include metadata,headings,body

# Extract a specific section
okc section metrics/monthly-revenue.md "Recognition Rules"

# Exact metadata query
okc metadata --filter type=Metric --filter tags_contains=finance --select path,title,owner

# View links
okc links metrics/monthly-revenue.md
okc backlinks metrics/monthly-revenue.md

# Traverse the graph
okc traverse metrics/monthly-revenue.md --max-depth 2

# Validate
okc validate

# Statistics
okc stats
```

## Configuration

### Command-Line Options

```bash
okc --help
```

Global options:
- `--root <PATH>` - Root directory to scan (can be specified multiple times)
- `--db-path <PATH>` - SQLite database path (default: `okc_index.db`)
- `--config <PATH>` - Configuration file (not yet implemented)

### Configuration File (Planned)

Future versions will support a TOML config file:

```toml
[scanner]
roots = ["./knowledge"]
exclude_patterns = [".git/", "node_modules/", "target/", ".env*"]
max_file_size = 2097152          # 2 MB
max_front_matter_size = 65536    # 64 KB
follow_symlinks = false

[indexer]
max_scan_results = 1000
max_graph_depth = 5
max_graph_nodes = 100
max_response_chars = 500000

[validation]
require_index_files = false
```

## Architecture

The system has five main layers:

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

### Technology Stack

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
| MCP | `rmcp` | Model Context Protocol server (planned) |

## Development

### Running Tests

```bash
# Unit tests
cargo test
```

### Test Fixtures

The test suite uses fixture repositories with:
- Nested directories
- Valid and invalid documents
- Circular and broken links
- Duplicate titles
- Custom metadata
- Modified/deleted files for incremental scan testing

### Code Structure

```
src/
├── main.rs                 # CLI entry point
├── lib.rs                  # Module declarations
├── config.rs               # Configuration types
├── scanner/
│   ├── mod.rs
│   ├── walker.rs           # Parallel filesystem walker
│   └── changes.rs          # Incremental change detection
├── parser/
│   ├── mod.rs
│   ├── frontmatter.rs      # YAML front-matter boundary extraction
│   ├── yaml.rs             # saphyr.rs           # YAML parsing with serde
│   ├── markdown.rs         # Heading/link/section extraction
│   └── links.rs            # Link resolution & existence checking
├── model/
│   ├── mod.rs
│   ├── document.rs         # Document, front-matter, heading, link, section
│   ├── directory.rs        # Directory tree types
│   └── graph.rs            # Graph edge types
├── index/
│   ├── mod.rs
│   ├── database.rs         # SQLite operations, all AI tool implementations
│   └── migrations.rs       # Schema initialization
├── service/
│   └── mod.rs              # High-level service facade
└── transport/
    ├── mod.rs
    ├── cli.rs              # CLI command definitions
    └── mcp.rs              # MCP server (planned)
```

## Usage with AI Agents

The tool is designed for AI agents to use via the CLI or future MCP server. Example agent workflows:

### Direct Concept Lookup
```
User: "What is monthly recurring revenue?"
Agent:
1. search_documents("monthly recurring revenue")
2. get_document(best_match, include=["metadata", "headings"])
3. get_section("Definition")
4. Answer with source path
```

### Hierarchical Browsing
```
User: "What metrics are available for customer engagement?"
Agent:
1. browse_directory("/")
2. Identify "metrics" directory
3. browse_directory("metrics/engagement")
4. get_document() for relevant concepts
5. Summarize
```

### Relationship Reasoning
```
User: "Which datasets are used to calculate monthly revenue?"
Agent:
1. search_documents("monthly revenue")
2. get_links("metrics/monthly-revenue.md")
3. Filter for dataset-type targets
4. get_document() on each dataset
5. Answer with linked sources
```

### Exact Metadata Query
```
User: "List all published finance metrics owned by Analytics."
Agent:
1. query_metadata({
     type: "Metric",
     status: "published",
     tags_contains: "finance",
     owner: "Analytics"
   })
2. Return matching concepts
```

### Repository Validation
```
User: "Are there broken references in this knowledge repository?"
Agent:
1. validate_repository()
2. Group broken links by source document
3. Explain affected concepts
```

## Roadmap

### Phase 1 (Current) - Minimal Repository Reader
- ✅ Filesystem traversal with ignore support
- ✅ Front-matter extraction & YAML parsing
- ✅ Normalized document records
- ✅ Basic CLI output

### Phase 2 - Markdown Structure
- ✅ Heading extraction
- ✅ Internal link extraction & resolution
- ✅ Broken link detection
- ✅ Directory tree construction

### Phase 3 - Persistent Index
- ✅ SQLite schema with FTS5
- ✅ Incremental file updates
- ✅ Metadata indexes
- ✅ Deleted file handling

### Phase 4 - AI-Facing Operations
- ✅ browse_directory
- ✅ get_document / get_section
- ✅ search_documents
- ✅ query_metadata
- ✅ get_links / get_backlinks
- ✅ traverse_graph
- ✅ validate_repository
- 🔲 MCP server transport

### Phase 5 - Continuous Updates (Planned)
- Filesystem watcher (`notify`)
- Debounced updates
- Partial graph rebuilding
- Index health reporting

### Phase 6 - Advanced Retrieval (Future)
- Fuzzy filename matching
- Trigram search
- Semantic embeddings
- Reranking
- Generated directory summaries
- PageIndex-style hierarchical reasoning
- Relationship extraction from custom metadata

## References

### OKF Specification
- [Open Knowledge Format](https://github.com/open-knowledge-format/spec) - Human-readable, git-versionable knowledge representation

### Key Libraries
- [ignore](https://github.com/BurntSushi/ripgrep/tree/master/crates/ignore) - Filesystem traversal with `.gitignore` support
- [saphyr](https://github.com/saphyr-rs/saphyr) - YAML 1.2 parser with Serde integration
- [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) - Streaming CommonMark parser
- [rusqlite](https://github.com/rusqlite/rusqlite) - SQLite wrapper with FTS5 support
- [blake3](https://github.com/BLAKE3-team/BLAKE3) - Fast cryptographic hashing
- [clap](https://github.com/clap-rs/clap) - Command-line argument parsing
- [rmcp](https://github.com/modelcontextprotocol/rust-sdk) - Model Context Protocol SDK

### Prior Art
- [Ripgrep](https://github.com/BurntSushi/ripgrep) - Fast search with ignore support
- [Tantivy](https://github.com/quickwit-oss/tantivy) - Full-text search engine
- [sqlite-fts5](https://www.sqlite.org/fts5.html) - SQLite full-text search
- [Model Context Protocol](https://modelcontextprotocol.io/) - Standard for AI tool interfaces

## License

MIT License - see [LICENSE](LICENSE) for details.

## Contributing

Contributions welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Workflow

1. Fork the repository
2. Create a feature branch
3. Make changes with tests
4. Run `cargo test` and `cargo clippy`
5. Submit a pull request

### Code Style

- Follow Rust standard style (`rustfmt`)
- Add tests for new functionality
- Update documentation for user-facing changes

## Support

- [Issues](https://github.com/your-org/open-knowledge-catalog/issues) - Bug reports and feature requests
- [Discussions](https://github.com/your-org/open-knowledge-catalog/discussions) - Questions and community