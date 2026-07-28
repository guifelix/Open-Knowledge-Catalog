# Open Knowledge Catalog (OKC)

[![Crates.io](https://img.shields.io/crates/v/okc.svg)](https://crates.io/crates/okc)
[![Crates.io Downloads](https://img.shields.io/crates/d/okc.svg)](https://crates.io/crates/okc)
[![CI](https://github.com/guifelix/Open-Knowledge-Catalog/actions/workflows/ci.yml/badge.svg)](https://github.com/guifelix/Open-Knowledge-Catalog/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A local-first tool that allows AI agents to safely browse, parse, search, and reason over an [Open Knowledge Format (OKF)](https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md) repository — via CLI, MCP server, or filesystem watcher.

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

OKC provides a comprehensive set of tools for browsing, parsing, searching, and reasoning over OKF repositories. See [docs/features.md](docs/features.md) for the full feature reference.

### Quick Reference

| Command | Purpose |
|---------|---------|
| `okc scan` | Index a knowledge repository |
| `okc browse` | Browse the directory hierarchy |
| `okc get` | Retrieve a document with metadata, headings, and/or body |
| `okc section` | Extract a specific Markdown section |
| `okc search` | Full-text search with BM25 ranking and filters |
| `okc metadata` | Structured metadata queries with filtering and projection |
| `okc links` | Outgoing links from a document |
| `okc backlinks` | Documents referencing a concept |
| `okc traverse` | Explore related concepts via graph edges |
| `okc validate` | 8-category repository validation |
| `okc stats` | Repository statistics |
| `okc serve` | Start MCP server (stdio or HTTP/SSE) |
| `okc watch` | File system watching with incremental updates |

## Installation

### From crates.io (recommended)

```bash
cargo install okc
```

### From GitHub Releases

Download the pre-built binary for your platform from the
[latest release](https://github.com/guifelix/Open-Knowledge-Catalog/releases):

| Platform | Download |
|----------|----------|
| Linux (x86_64, glibc) | `okc-x86_64-unknown-linux-gnu` |
| Linux (x86_64, musl — static) | `okc-x86_64-unknown-linux-musl` |
| macOS (Intel) | `okc-x86_64-apple-darwin` |
| macOS (Apple Silicon) | `okc-aarch64-apple-darwin` |
| Windows (x86_64) | `okc-x86_64-pc-windows-msvc.exe` |

### Build from source

```bash
git clone https://github.com/guifelix/Open-Knowledge-Catalog
cd open-knowledge-catalog
cargo build --release
# Binary at target/release/okc
```

## Quick Start

```bash
# Create a knowledge repository
mkdir -p my-knowledge/{metrics,datasets}

# Scan and index it
okc scan --root my-knowledge

# Browse the hierarchy
okc browse

# Search
okc search "revenue recognition"

# Retrieve a document
okc get metrics/monthly-revenue.md --include metadata,headings,body

# Extract a section
okc section metrics/monthly-revenue.md "Definition"

# Structured query
okc metadata --filter type=Metric --filter tags_contains=finance

# Link navigation
okc links metrics/monthly-revenue.md
okc backlinks metrics/monthly-revenue.md

# Graph traversal
okc traverse metrics/monthly-revenue.md --max-depth 3

# Validate
okc validate

# Statistics
okc stats

# Start MCP server
okc serve

# Watch for changes
okc watch
```

## Configuration

OKC reads a TOML config file from `~/.config/okc/config.toml`, `./okc.toml`, or a path specified via `--config`:

```toml
[scanner]
roots = ["./knowledge"]
exclude_patterns = [".git/", "node_modules/"]
max_file_size = 2097152           # 2 MB
max_front_matter_size = 65536     # 64 KB
max_yaml_input_size = 8388608    # 8 MB
follow_symlinks = false

[indexer]
max_scan_results = 1000
max_graph_depth = 5
max_graph_nodes = 100
max_response_chars = 500000

[validation]
require_index_files = false
```

Global CLI flags: `--root`, `--config`, `--db-path`.

See [docs/configuration.md](docs/configuration.md) for full details.

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
| Filesystem traversal | `ignore` |
| Filesystem watching | `notify` |
| Front-matter parsing | `memchr` + custom |
| YAML | `saphyr` |
| TOML config | `toml` + `figment` |
| Markdown | `pulldown-cmark` |
| Storage | `rusqlite` (SQLite + FTS5 with BM25) |
| Connection pooling | `r2d2` + `r2d2_sqlite` |
| Content hashing | `blake3` |
| URL encoding | `percent-encoding` |
| CLI | `clap` |
| MCP | `rmcp` |
| Async runtime | `tokio` + `tokio-util` |
| HTTP server | `axum` + `tower` / `tower-http` |
| Serialization | `serde` + `serde_json` |
| Errors | `thiserror` + `anyhow` + `miette` |
| Logging | `tracing` + `tracing-subscriber` |
| Schema | `schemars` |
| Paths | `camino` + `dirs` |

## Development

```bash
# Run tests
cargo test

# Check formatting
cargo fmt --check

# Lint
cargo clippy -- -D warnings

# Run benchmarks
cargo bench --features benchmarks

# Run fuzz targets (requires nightly)
cargo +nightly fuzz run frontmatter

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
| 5: Continuous Updates (watch, incremental scan) | ✅ Done |
| 6: Advanced Retrieval | 🔮 Future |

See [docs/roadmap.md](docs/roadmap.md) for details.

## License

MIT License — see [LICENSE](LICENSE) for details.

## Contributing

Contributions welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.
