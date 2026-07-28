---
type: Documentation
title: AI Agent Usage
description: How AI agents use OKC via CLI or MCP - workflows, JSON output, and usage principles
tags:
  - ai
  - agents
  - usage
  - mcp
  - cli
owner: Engineering Team
status: published
---

# Usage with AI Agents

The tool is designed for AI agents to use via CLI or future MCP server. All operations return structured data with source paths for traceability.

## Core Operations (11 AI-facing MCP tools)

| Operation | CLI Command | Purpose |
|-----------|-------------|---------|
| `browse_directory` | `okc browse [path] [--depth N]` | Inspect one area of the OKF hierarchy |
| `get_document` | `okc get <path> [--include metadata,headings,body]` | Retrieve one known concept |
| `get_section` | `okc section <path> "<heading>"` | Extract a specific Markdown section |
| `search_documents` | `okc search "query" [--path-prefix] [--type] [--tags]` | Full-text search with filters |
| `query_metadata` | `okc metadata --filter key=value --select fields` | Exact structured filtering |
| `get_links` | `okc links <path>` | Outgoing links from a document |
| `get_backlinks` | `okc backlinks <path>` | Documents referencing a concept |
| `traverse_graph` | `okc traverse <path> [--max-depth] [--max-nodes]` | Explore related concepts |
| `scan` | `okc scan --root <path>` | Scan/re-scan root directories and index |
| `get_stats` | `okc stats` | Repository statistics |
| `validate_repository` | `okc validate` | Report structural problems |

## Agent Workflows

### Direct Concept Lookup

**User:** "What is monthly recurring revenue?"

**Agent:**
1. `search_documents("monthly recurring revenue")`
2. `get_document(best_match, include=["metadata", "headings"])`
3. `get_section("Definition")`
4. Answer with source path

### Hierarchical Browsing

**User:** "What metrics are available for customer engagement?"

**Agent:**
1. `browse_directory("/")`
2. Identify "metrics" directory
3. `browse_directory("metrics/engagement")`
4. `get_document()` for relevant concepts
5. Summarize

*Follows OKF's progressive-disclosure model (§8 of spec).*

### Relationship Reasoning

**User:** "Which datasets are used to calculate monthly revenue?"

**Agent:**
1. `search_documents("monthly revenue")`
2. Select `metrics/monthly-revenue.md`
3. `get_links("metrics/monthly-revenue.md")`
4. Filter for dataset-type targets
5. `get_document()` on each dataset
6. Answer with linked sources

### Exact Metadata Query

**User:** "List all published finance metrics owned by Analytics."

**Agent:**
1. `query_metadata({ type: "Metric", status: "published", tags_contains: "finance", owner: "Analytics" })`
2. Return matching concepts

*No semantic search or LLM interpretation required.*

### Repository Validation

**User:** "Are there broken references in this knowledge repository?"

**Agent:**
1. `validate_repository()`
2. Group broken links by source document
3. Explain affected concepts

## CLI Output (for non-MCP agents)

Subcommands output structured text by default. The `validate` subcommand also supports
`--json` for machine-parseable output:

```bash
okc validate --json
```

Output format:
```json
{
  "status": "ok",
  "data": { ... },
  "meta": {
    "duration_ms": 12,
    "result_count": 5
  }
}
```

Error format:
```json
{
  "status": "error",
  "error": {
    "code": "NOT_FOUND",
    "message": "Document not found: metrics/unknown.md"
  }
}
```

## MCP Server

The MCP server is fully implemented. AI agents connect via Model Context Protocol and use tools directly:

```json
{
  "name": "search_documents",
  "arguments": { "query": "revenue recognition", "limit": 10 }
}
```

## MCP Tools

OKC exposes 11 AI-facing tools via MCP. See [docs/features.md](features.md) for the complete MCP tools reference.

### Transport Options

Start the server:

```bash
# stdio (for AI agents that launch the binary directly)
okc serve

# HTTP (for remote agent access)
okc serve --transport http --host 0.0.0.0 --port 3001
```

### Agent Configuration

Configure OKC as an MCP tool server in your AI coding environment. Each agent uses a slightly different config format.

#### Claude Desktop

Add an entry to `claude_desktop_config.json` (Claude Desktop → Settings → Developer → Edit Config):

```json
{
  "mcpServers": {
    "okc": {
      "command": "okc",
      "args": ["serve"]
    }
  }
}
```

Replace `"okc"` with the full path if the binary is not in your PATH.

#### Claude Code

Add to `~/.claude/settings.json`:

```json
{
  "mcpServers": {
    "okc": {
      "command": "okc",
      "args": ["serve"]
    }
  }
}
```

Or place a project-local config at `./.claude/settings.json` to bind OKC to a specific repository.

#### OpenCode

Add to `~/.config/opencode/opencode.json` (global) or `./opencode.json` (project-local):

```json
{
  "mcpTools": {
    "okc": {
      "command": "okc",
      "args": ["serve"]
    }
  }
}
```

#### Codex (OpenAI CLI)

Add to `~/.codex/config.json`:

```json
{
  "mcp": {
    "servers": {
      "okc": {
        "command": "okc",
        "args": ["serve"]
      }
    }
  }
}
```

#### Cursor

Place a `.cursor/mcp.json` in your project root:

```json
{
  "mcpServers": {
    "okc": {
      "command": "okc",
      "args": ["serve"]
    }
  }
}
```

#### Windsurf

Place a `.windsurf/mcp_config.json` in your project root:

```json
{
  "mcpServers": {
    "okc": {
      "command": "okc",
      "args": ["serve"]
    }
  }
}
```

#### Cline (VS Code)

Configure in `~/.config/cline/cline_mcp_settings.json`:

```json
{
  "mcpServers": {
    "okc": {
      "command": "okc",
      "args": ["serve"]
    }
  }
}
```

> **Tip:** If `okc` is not in your PATH, replace `"okc"` with the full binary path (e.g., `"/usr/local/bin/okc"` or `"$HOME/.local/bin/okc"`).

## AI Usage Principles

The tool contract encourages the model to:

1. **Browse narrowly before reading broadly** — use `browse_directory` to explore
2. **Use metadata filters for exact conditions** — `query_metadata` for `type=Metric`, not text search
3. **Use text search for lexical discovery** — `search_documents` for keywords
4. **Follow graph links for related concepts** — `get_links`/`traverse_graph`
5. **Retrieve individual sections instead of entire documents** — `get_section`
6. **Include source paths in final answers** — every result has `path`
7. **Stop traversal when evidence is sufficient** — don't over-fetch

The tool enforces limits even when the AI requests excessive output.

## Response Size Limits

Default limits (configurable):
- `max_response_chars`: 500,000 characters
- `max_scan_results`: 1,000 entries
- `max_graph_depth`: 5
- `max_graph_nodes`: 100

Responses include `truncated: true` when limits are hit.