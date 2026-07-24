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

## Quick Start

```bash
git clone https://github.com/guifelix/open-knowledge-catalog
cd open-knowledge-catalog
cargo build --release

# Create a knowledge repository
mkdir -p my-knowledge/{metrics,datasets}

# Scan it
okc scan --root my-knowledge

# Query it
okc browse
okc search "revenue recognition"
okc get metrics/monthly-revenue.md --include metadata,headings,body
okc metadata --filter type=Metric --filter tags_contains=finance
okc validate
```

## Documentation

| Topic | Document |
|-------|----------|
| Installation & Quick Start | [docs/getting-started.md](docs/getting-started.md) |
| Architecture & Internals | [docs/architecture.md](docs/architecture.md) |
| Configuration | [docs/configuration.md](docs/configuration.md) |
| AI Agent Usage | [docs/ai-usage.md](docs/ai-usage.md) |
| Development Guide | [docs/development.md](docs/development.md) |
| Roadmap | [docs/roadmap.md](docs/roadmap.md) |
| References & License | [docs/references.md](docs/references.md) |

## Technology Stack

| Layer | Library |
|-------|---------|
| Filesystem | `ignore` |
| Front-matter | `memchr` + custom |
| YAML | `saphyr` |
| Markdown | `pulldown-cmark` |
| Storage | `rusqlite` (SQLite + FTS5) |
| Hashing | `blake3` |
| CLI | `clap` |
| MCP | `rmcp` |
| Async | `tokio` |
| Errors | `thiserror` + `anyhow` + `miette` |
| Logging | `tracing` |
| Schema | `schemars` |
| Config | `figment` |
| Paths | `camino` |

## Development

```bash
# Run tests
cargo test

# Check formatting
cargo fmt --check

# Lint
cargo clippy -- -D warnings

# Generate docs
cargo doc --no-deps --open
```

## Roadmap Status

| Phase | Status |
|-------|--------|
| 1: Minimal Repository Reader | ✅ Done |
| 2: Markdown Structure | ✅ Done |
| 3: Persistent Index | ✅ Done |
| 4: AI-Facing Operations | ✅ Done |
| 5: Continuous Updates | 🔲 Planned |
| 6: Advanced Retrieval | 🔮 Future |

See [docs/roadmap.md](docs/roadmap.md) for details.

## License

MIT License — see [LICENSE](LICENSE) for details.

## Contributing

Contributions welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.
