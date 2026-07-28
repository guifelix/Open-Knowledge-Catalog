---
type: Documentation
title: Configuration
description: All configuration options for OKC (TOML, environment variables, CLI)
tags:
  - configuration
  - settings
  - config
owner: Engineering Team
status: published
---

# Configuration

## Command-Line Options

```bash
okc --help
```

Global options:
- `--root <PATH>` - Root directory to scan (can be specified multiple times)
- `--db-path <PATH>` - SQLite database path (default: `okc_index.db`)
- `--config <PATH>` - Configuration file (default: `~/.config/okc/config.toml` or `./okc.toml`)

## Configuration File

OKC reads a TOML config file from `~/.config/okc/config.toml`, `./okc.toml`, or a path specified via `--config`:

```toml
[scanner]
# Repository roots to scan
roots = ["./knowledge"]

# Glob patterns to exclude (in addition to .gitignore)
exclude_patterns = [".git/", "node_modules/", "target/", ".env*"]

# Maximum file size to process (bytes)
max_file_size = 2097152          # 2 MB

# Maximum front-matter size (bytes)
max_front_matter_size = 65536    # 64 KB

# Maximum YAML input size before parsing (rejects pathological inputs)
max_yaml_input_size = 8388608   # 8 MB

# Follow symlinks during scan
follow_symlinks = false

[indexer]
# Maximum results returned by scan operations
max_scan_results = 1000

# Maximum graph traversal depth
max_graph_depth = 5

# Maximum nodes returned by graph traversal
max_graph_nodes = 100

# Maximum characters in tool responses
max_response_chars = 500000

[validation]
# Require index.md files in directories
require_index_files = false
```

## Environment Variables

All config options can be overridden via environment variables with prefix `OKC_`:

```bash
OKC_SCANNER_ROOTS="./knowledge,./docs"
OKC_SCANNER_MAX_FILE_SIZE=5242880
OKC_INDEXER_MAX_GRAPH_DEPTH=10
OKC_DB_PATH="/data/okc_index.db"
```

## Scanner Configuration

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `roots` | `Vec<PathBuf>` | `["."]` | Directories to scan |
| `exclude_patterns` | `Vec<String>` | `[".git/", "node_modules/", "target/", ".env*"]` | Additional glob patterns to exclude |
| `max_file_size` | `u64` | `2_097_152` (2 MB) | Skip files larger than this |
| `max_front_matter_size` | `u64` | `65_536` (64 KB) | Reject front matter larger than this |
| `max_yaml_input_size` | `usize` | `8_388_608` (8 MB) | Reject YAML input larger than this (OOM defense) |
| `follow_symlinks` | `bool` | `false` | Follow symlinks during walk |

## Indexer Configuration

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `max_scan_results` | `usize` | `1000` | Cap on scan output |
| `max_graph_depth` | `usize` | `5` | Max depth for `traverse_graph` |
| `max_graph_nodes` | `usize` | `100` | Max nodes for `traverse_graph` |
| `max_response_chars` | `usize` | `500_000` | Truncate tool responses |

## Validation Configuration

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `require_index_files` | `bool` | `false` | Treat missing `index.md` as validation error |

## Search Configuration (BM25)

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `bm25.title_weight` | `f64` | `10.0` | Weight for title field in BM25 scoring |
| `bm25.description_weight` | `f64` | `5.0` | Weight for description field in BM25 scoring |
| `bm25.headings_weight` | `f64` | `2.0` | Weight for headings field in BM25 scoring |
| `bm25.body_weight` | `f64` | `1.0` | Weight for body field in BM25 scoring |
| `bm25.concept_type_weight` | `f64` | `0.0` | Weight for concept_type field in BM25 scoring |
| `bm25.k1` | `f64` | `1.2` | BM25 term frequency saturation parameter |
| `bm25.b` | `f64` | `0.75` | BM25 document length normalization parameter |

Higher weights increase the importance of that field in relevance ranking. The default weights follow the ADR-002 specification: title > description > headings > body > concept_type.

Example TOML configuration:

```toml
[search]
bm25_title_weight = 10.0
bm25_description_weight = 5.0
bm25_headings_weight = 2.0
bm25_body_weight = 1.0
bm25_concept_type_weight = 0.0
bm25_k1 = 1.2
bm25_b = 0.75
```

Environment variable overrides:

```bash
OKC_SEARCH_BM25_TITLE_WEIGHT=15.0
OKC_SEARCH_BM25_K1=1.5
OKC_SEARCH_BM25_B=0.5
```

## MCP Server Configuration

The MCP server is fully implemented with two transport options:

```toml
[mcp]
# Transport: "stdio" | "http"
transport = "stdio"

# HTTP only
host = "127.0.0.1"
port = 3001

# Request timeout
timeout_seconds = 30
```

Use `okc serve --transport http` to start the HTTP server, or `okc serve` with the default stdio transport.

## Defaults

If no config file is found, the CLI defaults above are used. The config file is optional — all settings can be provided via CLI flags or environment variables.