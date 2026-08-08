---
type: reference
title: Features
description: Overview of Open Knowledge Catalog features including multi-format support, semantic search, and CI/CD integration
tags: [features, overview, reference]
owner: felix
status: draft
---

# Features

OKC provides a comprehensive set of tools for browsing, parsing, searching, and reasoning over OKF repositories.

## CLI Commands

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
| `okc serve` | Start MCP server (stdio for local clients, HTTP for remote/shared use) |
| `okc watch` | File system watching with incremental updates |

## MCP Tools

When running as an MCP server, these tools are exposed to AI agents:

| Tool | Description |
|------|-------------|
| `scan` | Scan/re-scan root directories and index documents |
| `browse_directory` | Inspect one area of the OKF hierarchy |
| `get_document` | Retrieve one concept with opt-in metadata, identity fields, links, and backlinks |
| `get_section` | Extract a specific Markdown section without the full document |
| `search_documents` | Full-text search with optional path/type/tag filters |
| `query_metadata` | Exact structured filtering on front-matter fields |
| `get_links` | Outgoing links from a document |
| `get_backlinks` | Documents referencing a concept |
| `traverse_graph` | Explore related concepts via graph edges |
| `get_stats` | Repository statistics (file counts, link counts, etc.) |
| `validate_repository` | Report structural problems (broken links, malformed YAML, missing index files) |

All successful MCP tool responses advertise an `outputSchema` and return matching `structuredContent`. A JSON text
content block remains available for compatibility with clients that have not adopted structured MCP output.

## Search with Structured Headings

Search results include optional heading lists for each matching document, providing a quick outline view without fetching the full document.

**CLI options:**
```bash
okc search "query" --max-headings 3 --heading-depth 2
```

**MCP tool params:**
```json
{
  "query": "query",
  "max_headings": 3,
  "heading_depth": 2
}
```

**Configurable via TOML or environment variables:**
```toml
[search]
max_headings = 3    # default: 1
heading_depth = 2   # default: 1 (h1 only)
```

```bash
OKC_SEARCH_MAX_HEADINGS=5
OKC_SEARCH_HEADING_DEPTH=3
```

**Behavior:**
- Only headings at or below `heading_depth` are included (1=h1, 2=h1+h2, etc.)
- `max_headings` caps the total across all allowed depths (budget interaction)
- Headings inside fenced code blocks are excluded
- Returns empty list for documents with no body or no matching headings

## MCP Server Transport

Run the MCP server in two modes:

```bash
# stdio (default) — local MCP clients launch the child process automatically
okc serve --transport stdio

# HTTP — for web clients, remote access, or shared hosting
okc serve --transport http --host 0.0.0.0 --port 3001
```

OpenCode and similar local clients own the stdio process lifetime automatically. Use HTTP only when you need a manually
hosted remote server.

## Filesystem Watcher

Keep your index up to date automatically:

```bash
okc watch                    # Watch configured roots
okc watch --root ./knowledge --debounce 300 --reconcile 600
```

Features: debounced event batching, editor temp-file filtering (`.swp`, `~`, `.tmp`), gitignore-aware exclusion, periodic full reconciliation, incremental index updates.

## Incremental Scanning

Content-hash based change detection (Blake3 sampling) enables fast re-scans — unchanged files are skipped entirely.

## Supported OKF Format

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

## Repository Structure

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

## Repository Validation

`okc validate` checks 8 categories of structural problems — broken links, malformed YAML, circular references, duplicate content, missing index files, and more. Supports `--json` for machine-parseable output:

```bash
okc validate --json
```

## Response Size Limits

Configurable limits prevent excessive output:

- `max_response_chars`: 500,000 characters
- `max_scan_results`: 1,000 entries
- `max_graph_depth`: 5
- `max_graph_nodes`: 100

Enriched `get_document` responses include `truncated: true` when the requested
body limit or aggregate `max_response_chars` limit is hit.
