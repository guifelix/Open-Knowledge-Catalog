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

The tool is designed for AI agents to use via CLI or MCP server. All operations return structured data with source paths for traceability.

## Core Operations (11 AI-facing MCP tools)

| Operation | CLI Command | Purpose |
|-----------|-------------|---------|
| `browse_directory` | `okc browse [path] [--depth N]` | Inspect one area of the OKF hierarchy |
| `get_document` | `okc get <path> [--include metadata,headings,body,...]` | Retrieve one known concept, with opt-in graph context |
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

### Enriched Document Context

Use one call when an answer needs a known document together with its provenance
and graph neighborhood:

`get_document({ path: "metrics/monthly-revenue.md", include: ["metadata", "custom", "content_hash", "parent_path", "links", "backlinks"] })`

The default remains `body` plus `headings`; optional fields are omitted unless
requested. `custom` returns decoded front-matter fields. `links` uses the normal
outgoing-link shape. Each backlink contains `source_path`, `target_anchor`, and
`exists_in_repository`, so the referring document and target context are
unambiguous. Unknown include values are errors. The complete serialized response
is capped by `max_response_chars`; arrays and body text are shortened only at
valid item or character boundaries and `truncated` is then `true`.

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
1. `query_metadata({ filter: ["type=Metric", "status=published", "tags_contains=finance", "owner=Analytics"], select: ["path", "title", "tags", "owner"] })`
2. Return matching concepts

*No semantic search or LLM interpretation required.*

`query_metadata` uses exact string equality for `type`, `title`, `parse_status`,
and custom front-matter fields. `tags_contains` matches one complete tag and
`path_prefix` restricts results to a repository-relative path prefix. Selectable
fields are the core document fields (`path`, `title`, `type`, `description`,
`file_size`, `modified_at`, `content_hash`, `parse_status`, `parent_path`, and
`id`), `tags`, and custom front-matter field names. Results are ordered by path;
missing requested custom fields are returned as `null`. `total_matches` counts
the complete filtered set, while `truncated` indicates that `limit` omitted one
or more matches. Filter expressions must use `key=value`; malformed expressions,
unsupported `*_contains` operators, and malformed projection names are errors.

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

The MCP server is fully implemented. AI agents connect via Model Context Protocol and use tools directly.
Local MCP clients such as OpenCode start the stdio child process automatically and own its lifetime, so you do not
run a separate daemon for the common local setup.

```json
{
  "name": "search_documents",
  "arguments": { "query": "revenue recognition", "limit": 10 }
}
```

## MCP Tools

OKC exposes 11 AI-facing tools via MCP. See [docs/features.md](features.md) for the complete MCP tools reference.

Every successful tool call includes a typed `structuredContent` object that conforms to the tool's advertised
`outputSchema`. For clients that do not yet consume structured MCP results, OKC also includes a JSON text content block.
The compatibility text preserves the existing response shape; new integrations should read `structuredContent`
directly and avoid parsing JSON from text.

### Transport Options

Local clients launch stdio automatically:

```bash
# stdio (local clients launch the child process)
okc serve --transport stdio

# HTTP (manually hosted for remote/shared access)
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

Add to `~/.config/opencode/opencode.json` for all workspaces, or `./opencode.json` in a project to override just that
workspace. OpenCode starts and stops local MCP servers automatically, and the workspace directory is the default
working directory unless you set `cwd`.

```json
{
  "$schema": "https://opencode.ai/config.json",
  "mcp": {
    "servers": {
      "okc": {
        "type": "local",
        "command": ["okc", "serve", "--transport", "stdio"]
      }
    }
  }
}
```

If `okc` is not in your `PATH`, replace it with the full executable path. If you want to pin the server to a specific
repository from a global config, add `"cwd": "/absolute/path/to/repo"` or use a project-local config with `"cwd": "."`.
If a project-local server shares the same name as a global one, the project-local definition replaces it.

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

### Troubleshooting

If `opencode mcp list` shows `MCP error -32000: Connection closed`, the local server exited before the handshake
completed. Check the following in order:

1. The resolved executable exists and is executable.
2. The binary is rebuilt or reinstalled if you are using a stale artifact.
3. `opencode mcp list` shows the server connecting from the expected workspace.
4. The workspace or explicit `cwd` points at the repository you intended to index.
5. The server is not being launched with a conflicting root or config from a higher-precedence project file.

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
