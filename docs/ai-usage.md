# Usage with AI Agents

The tool is designed for AI agents to use via CLI or future MCP server. Example agent workflows:

## Direct Concept Lookup

**User:** "What is monthly recurring revenue?"

**Agent:**
1. `search_documents("monthly recurring revenue")`
2. `get_document(best_match, include=["metadata", "headings"])`
3. `get_section("Definition")`
4. Answer with source path

## Hierarchical Browsing

**User:** "What metrics are available for customer engagement?"

**Agent:**
1. `browse_directory("/")`
2. Identify "metrics" directory
3. `browse_directory("metrics/engagement")`
4. `get_document()` for relevant concepts
5. Summarize

## Relationship Reasoning

**User:** "Which datasets are used to calculate monthly revenue?"

**Agent:**
1. `search_documents("monthly revenue")`
2. Select `metrics/monthly-revenue.md`
3. `get_links("metrics/monthly-revenue.md")`
4. Filter for dataset-type targets
5. `get_document()` on each dataset
6. Answer with linked sources

## Exact Metadata Query

**User:** "List all published finance metrics owned by Analytics."

**Agent:**
1. `query_metadata({ type: "Metric", status: "published", tags_contains: "finance", owner: "Analytics" })`
2. Return matching concepts

No semantic search or LLM interpretation required.

## Repository Validation

**User:** "Are there broken references in this knowledge repository?"

**Agent:**
1. `validate_repository()`
2. Group broken links by source document
3. Explain affected concepts

## CLI JSON Output (for non-MCP agents)

All subcommands support `--json` flag for machine-parseable output:

```bash
okc search "revenue" --json
okc get metrics/monthly-revenue.md --json
okc browse metrics --json
okc metadata --filter type=Metric --json
okc links metrics/monthly-revenue.md --json
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

## MCP Server (Planned)

When MCP transport is implemented, agents will use tools directly:

```json
{
  "name": "search_documents",
  "arguments": { "query": "revenue recognition", "limit": 10 }
}
```

Tools available:
- `browse_directory`
- `get_document`
- `get_section`
- `search_documents`
- `query_metadata`
- `get_links`
- `get_backlinks`
- `traverse_graph`
- `validate_repository`

## AI Usage Principles

The tool contract encourages the model to:

1. **Browse narrowly before reading broadly** — use `browse_directory` to explore
2. **Use metadata filters for exact conditions** — `query_metadata` for type/status/tag
3. **Use text search for lexical discovery** — `search_documents` for keywords
4. **Follow graph links for related concepts** — `get_links`/`traverse_graph`
5. **Retrieve individual sections instead of entire documents** — `get_section`
6. **Include source paths in final answers** — every result has `path`
7. **Stop traversal when evidence is sufficient** — don't over-fetch

The tool enforces limits even when the AI requests excessive output.