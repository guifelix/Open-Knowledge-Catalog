# Configuration

## Command-Line Options

```bash
okc --help
```

Global options:
- `--root <PATH>` - Root directory to scan (can be specified multiple times)
- `--db-path <PATH>` - SQLite database path (default: `okc_index.db`)
- `--config <PATH>` - Configuration file (not yet implemented)

## Configuration File (Planned)

Future versions will support a TOML config file at `~/.config/okc/config.toml` or via `--config`:

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

## MCP Server Configuration (Planned)

```toml
[mcp]
# Transport: "stdio" | "http" | "sse"
transport = "stdio"

# HTTP/SSE only
host = "127.0.0.1"
port = 3000

# Request timeout
timeout_seconds = 30
```

## Defaults

If no config file is found, the CLI defaults above are used. The config file is optional — all settings can be provided via CLI flags or environment variables.