# Competitor Assessment: okf-ingest + okf-mcp (travisjakel)

## Overview

**okf-ingest** (https://github.com/travisjakel/okf-ingest, Apache-2.0, 3 stars, 33 commits) — Unified ingestion tool for OKF bundles. Reads, validates, builds concept graph, loads into DuckDB catalog. Python + R implementation. OKF v0.1. **No MCP support** (ingestion layer only).

**okf-mcp** (https://github.com/travisjakel/okf-mcp, Apache-2.0, 5 stars, 5 commits) — MCP server wrapping okf-ingest's deterministic verbs as MCP tools. Python implementation. OKF v0.1. **MCP stdio transport only**.

Both projects form a **stack**: okf-ingest (upstream ingestion → DuckDB) → okf-mcp (downstream MCP exposure). Same author, same OKF version, complementary roles. Directly overlaps OKC's ingestion + MCP server functionality.

---

## Feature Comparison with OKC

| Feature | okf-ingest + okf-mcp | OKC | Notes |
|---------|---------------------|-----|-------|
| **OKF version** | v0.1 | v0.2 | OKC ahead on spec |
| **Ingestion** | ✅ Python + R pipeline | ✅ Rust scanner | okf-ingest: richer validation, wikilinks, RAG prep |
| **Concept graph** | ✅ Built from bundles | ✅ Link-graph from markdown | okf-ingest: explicit concept nodes |
| **Wikilinks resolution** | ✅ `[[wikilinks]]` | ❌ (markdown links only) | okf-ingest advantage |
| **RAG readiness** | ✅ Chunking + embedding hooks | ❌ | okf-ingest designed for it |
| **Storage backend** | DuckDB (portable, analytical) | SQLite + FTS5 (embedded) | Different tradeoffs |
| **MCP transport** | stdio only | stdio + HTTP/SSE | OKC more deployable |
| **MCP tools** | 8 tools | 11 tools | OKC broader surface |
| **File watching** | ❌ | ✅ | OKC live reload |
| **Cross-bundle graph** | ✅ (DuckDB catalog) | ✅ (traverse) | Both support |
| **Single binary deploy** | ❌ (Python + R deps) | ✅ | OKC ops advantage |
| **Performance** | Python overhead | Rust native | OKC faster indexing |
| **License** | Apache-2.0 | MIT | Both permissive |
| **Maturity** | Early (33 + 5 commits) | Active pre-1.0 | Both young |

---

## Architecture & Code Quality

### okf-ingest
- **Structure**: Python package (`okf_ingest/`) + R scripts (`R/`). ~2.5k Python LoC, ~500 R LoC.
- **Pipeline stages**: `validate` → `parse` → `graph_build` → `catalog_load` → `render`.
- **Validation**: Pydantic models for OKF v0.1 schema; strict conformance checks.
- **Graph build**: NetworkX for concept graph; resolves `[[wikilinks]]` to concept IDs.
- **Catalog**: DuckDB with tables: `bundles`, `concepts`, `relations`, `chunks`, `embeddings` (placeholder).
- **R integration**: `reticulate` bridge for statistical/embedding steps; optional.
- **Testing**: `pytest` + `testthat` (R); CI on GitHub Actions.
- **Quality gates**: `ruff`, `mypy`, `pytest-cov`; no `clippy`/`rustfmt` equivalent.
- **Observability**: Structured logging (`structlog`); no metrics export.
- **Maturity**: 33 commits, 3 stars — early but functional ingestion pipeline.

### okf-mcp
- **Structure**: Single Python package (`okf_mcp/`). ~800 LoC.
- **Architecture**: Thin MCP wrapper over `okf_ingest` catalog (DuckDB). Tools execute SQL via `duckdb` Python bindings.
- **Tools**: 8 tools mapping directly to catalog queries (`list_bundles`, `search`, `get_concept`, `context`, `impact`, `sql`, `diff`, `doctor`).
- **Transport**: MCP stdio only (via `mcp` Python SDK). No HTTP/SSE.
- **Auth**: None.
- **Testing**: Basic `pytest` for tool schemas; no integration tests.
- **Quality gates**: `ruff`, `mypy`; minimal CI.
- **Observability**: `doctor` tool for diagnostics; no metrics.
- **Maturity**: 5 commits, 5 stars — just launched, minimal but clean.

### OKC
- **Structure**: Single binary crate (~8k Rust LoC). Modules: `config`, `index`, `model`, `parser`, `scanner`, `service`, `transport`.
- **Database**: SQLite (r2d2 + rusqlite). FTS5 for search. No external DB.
- **Async**: Tokio for MCP server; core indexing synchronous.
- **Architecture**: `OkcService` over `RepositoryIndex`. Transport-agnostic tools.
- **Code indexing**: Markdown only. No tree-sitter.
- **Quality gates**: `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test`.
- **Observability**: `tracing` logs only. No metrics export.
- **Maturity**: Pre-1.0, active development.

---

## Query Capabilities

### okf-ingest + okf-mcp
- **`okf_search`**: Full-text search over concept names, definitions, and chunk content. Optional filters: bundle, concept type, tags. Returns ranked results with excerpts.
- **`okf_get_concept`**: Direct concept lookup by ID. Returns concept metadata (type, definition, aliases), inbound/outbound relations, and associated chunks.
- **`okf_context`**: Bounded context pack from a seed concept. Composes: concept detail + 1-hop relations + relevant chunks (token-budgeted). Returns provenance + truncation diagnostics.
- **`okf_impact`**: Reverse dependency analysis. Given a concept, traverses inbound relations to find dependent concepts/files. Returns impact set with relation types.
- **`okf_sql`**: Raw DuckDB query pass-through. Allowlisted tables: `bundles`, `concepts`, `relations`, `chunks`, `embeddings`. Read-only.
- **`okf_diff`**: Catalog diff between two bundle versions. Shows added/removed/changed concepts, relations, chunks.
- **No**: Vector/semantic search (embedding column exists but unused), multi-hop reasoning beyond 1-hop, code graph, metadata-only filtering.

### OKC
- **`search`**: FTS5/BM25 full-text with path prefix, concept type, tags filters. Returns scored results with excerpts.
- **`query_metadata`**: Structured key-value filtering on front-matter with field projection.
- **`traverse`**: BFS link-graph traversal from a start path. Relation filter (empty = all), max_depth (default 3), max_nodes (default 50). Returns nodes (path, title, type, depth) and edges (source, target, relation).
- **`get_links`/`get_backlinks`**: Direct link resolution with existence check.
- **No**: Vector search, semantic search, typed relations, confidence, fact lifecycle, code graph, context packs, multi-hop reasoning, impact analysis, SQL escape hatch, version diff.

### Comparison with OKC's Graph Traversal
| Feature | OKC `traverse` | okf-mcp `context`/`impact` |
|---------|----------------|----------------------------|
| **Traversal algo** | BFS | Bounded subgraph (context) / reverse deps (impact) |
| **Depth control** | `max_depth` (≤config max) | Implicit via budget (context) / full reverse (impact) |
| **Relation filter** | String list (link types) | Concept relation types (depends_on, references, etc.) |
| **Node limit** | `max_nodes` | Token budget (`max_tokens`) |
| **Edge semantics** | Single `links_to`/`linked_from` | Typed: `depends_on`, `references`, `supersedes`, `related` |
| **Output** | Nodes + edges + truncated flag | Context pack: concept + relations + chunks + provenance + diagnostics |

---

## Integration Patterns for Agent Consumption

| Pattern | okf-ingest + okf-mcp | OKC |
|---------|---------------------|-----|
| **MCP stdio** | ✅ (`okf-mcp` binary) | ✅ (`okc serve --stdio`) |
| **MCP HTTP/SSE** | ❌ | ✅ (`okc serve --http`) |
| **Agent skills** | ❌ | ❌ |
| **Auth for agents** | None | None |
| **Tool discovery** | All tools visible | All tools visible |
| **Resource subscription** | None | None |
| **Prompt templates** | None | None |
| **Audit trail** | None | None |

---

## MCP Capability Inventory

| Tool/Resource | okf-mcp | OKC | Notes |
|---------------|---------|-----|-------|
| **List bundles** | `okf_list_bundles` | — | okf-mcp: catalog metadata |
| **Search** | `okf_search` (FTS + concept) | `search` (FTS5/BM25) | okf-mcp: concept-aware |
| **Get concept** | `okf_get_concept` (detail + relations) | `get_document` (by path) | okf-mcp: concept-centric |
| **Context pack** | `okf_context` (bounded graph + chunks) | — | okf-mcp unique: agent-oriented context |
| **Impact analysis** | `okf_impact` (reverse deps) | `traverse` (BFS) | okf-mcp: explicit impact |
| **SQL escape hatch** | `okf_sql` (raw DuckDB) | — | okf-mcp: power user tool |
| **Diff bundles** | `okf_diff` (version compare) | — | okf-mcp: version awareness |
| **Health check** | `okf_doctor` (catalog integrity) | `validate` (index issues) | Both diagnostic |
| **Graph traverse** | — | `traverse` (BFS, filters) | OKC: generic link traversal |
| **Metadata query** | — | `query_metadata` (front-matter) | OKC: structured filtering |
| **Links/backlinks** | — | `get_links`/`get_backlinks` | OKC: direct link resolution |
| **Resources** | — | — | Neither exposes MCP resources |
| **Prompts** | — | — | Neither provides prompt templates |
| **Auth/scopes** | None | None | Both open |
| **Transports** | stdio only | stdio + HTTP/SSE | OKC more deployable |

---

## Strengths vs OKC

### okf-ingest (ingestion layer)
1. **OKF-native validation** — Pydantic models enforce v0.1 schema strictly; catches malformed bundles early.
2. **Wikilinks as first-class** — `[[concept]]` syntax resolved to concept graph edges; OKC only handles `[text](path)` markdown links.
3. **RAG-ready pipeline** — Chunking strategy + embedding column in catalog; designed for vector search integration.
4. **Explicit concept graph** — Concepts are nodes with properties (type, definition, aliases); not just file paths.
5. **DuckDB analytical backend** — Columnar, portable, SQL-queryable; excellent for analytics/impact queries.
6. **Deterministic operations** — Same input → same catalog; reproducible builds.
7. **Rendering pipeline** — Can emit HTML/PDF from bundles; OKC has no rendering.

### okf-mcp (MCP layer)
8. **Concept-centric tools** — `get_concept`, `context`, `impact` operate on semantic units, not files.
9. **Context packs for agents** — `okf_context` returns bounded subgraph + chunks with token budget awareness.
10. **Impact analysis** — `okf_impact` computes reverse dependencies (what breaks if this concept changes).
11. **SQL escape hatch** — `okf_sql` lets agents run arbitrary analytical queries on the catalog.
12. **Bundle diffing** — `okf_diff` compares concept graphs across versions; OKC has no versioning.
13. **Health diagnostics** — `okf_doctor` validates catalog integrity, orphan concepts, broken links.

---

## Weaknesses vs OKC

### okf-ingest
1. **Python + R runtime** — Requires Python env + R installation; not a single binary. Harder to deploy/distribute.
2. **No file watching** — Batch ingestion only; no live reload on filesystem changes.
3. **OKF v0.1 only** — Behind OKC's v0.2 support; missing newer schema fields.
4. **No MCP server** — Ingestion only; requires okf-mcp for agent access (two-process deployment).
5. **Performance overhead** — Python pandas/NetworkX slower than Rust for large corpora.
6. **R dependency optional but real** — Embedding/statistical steps need R; adds complexity.

### okf-mcp
7. **stdio transport only** — No HTTP/SSE; cannot run as remote service or behind load balancer.
8. **No auth/scopes** — All tools exposed to any connected agent.
9. **No MCP resources/prompts** — Missing `resources/list`, `prompts/list` for agent context subscription.
10. **DuckDB connection management** — Opens/closes per tool call; no connection pooling.
11. **Minimal test coverage** — 5 commits, basic schema tests only; no integration/e2e.
12. **No file watching** — Catalog static until re-ingestion; OKC watches filesystem.
13. **Single-author, early stage** — 5 stars, 5 commits; bus factor = 1.

---

## OKC Improvement Opportunities

| Area | Gap | Competitor Reference | Action |
|------|-----|---------------------|--------|
| **Wikilinks support** | Markdown links only | okf-ingest: `[[concept]]` → concept graph edges | Add `[[wikilink]]` parser; resolve to concept IDs in front-matter |
| **Concept-first model** | File-path-centric | okf-ingest: concepts as nodes with type/def/aliases | Introduce `Concept` entity; extract from front-matter `concept:` field |
| **RAG-ready chunking** | No chunking/embeddings | okf-ingest: `chunks` table + embedding column | Add `chunk` step in scanner; store in FTS5 + optional `sqlite-vec` |
| **Context packs** | Raw traverse output | okf-mcp: `okf_context` with budget + provenance | Add `context_pack` tool: search + traverse + metadata, token-bounded |
| **Impact analysis** | Generic BFS only | okf-mcp: `okf_impact` (reverse deps) | Add `impact` tool: reverse link traversal from concept/file |
| **Bundle versioning** | None (git only) | okf-mcp: `okf_diff` (catalog diff) | Add `version` front-matter; `diff` tool comparing index snapshots |
| **SQL escape hatch** | None | okf-mcp: `okf_sql` | Expose `query_sql` tool (read-only, allowlisted tables) |
| **Health diagnostics** | `validate` (structural) | okf-mcp: `okf_doctor` (catalog integrity) | Extend `validate` with orphan concepts, broken wikilinks, stale chunks |
| **HTTP/SSE transport** | ✅ Has it | okf-mcp: stdio only | **OKC advantage** — maintain + document deployment patterns |
| **File watching** | ✅ Has it | okf-ingest/mcp: batch only | **OKC advantage** — keep live reload as differentiator |
| **Single binary deploy** | ✅ Rust static | okf-ingest/mcp: Python + R deps | **OKC advantage** — emphasize in docs/marketing |
| **OKF v0.2 support** | ✅ Current | okf-ingest/mcp: v0.1 only | **OKC advantage** — track spec, maintain compat matrix |

---

## Threat Level

**Medium**

**Rationale:**
- **Direct overlap**: Same OKF ecosystem, same target (MCP-exposed knowledge catalog), same author building a stack.
- **Conceptual lead**: okf-ingest's concept graph + wikilinks + RAG pipeline is architecturally ahead of OKC's file-centric model.
- **Agent UX lead**: okf-mcp's `context`, `impact`, `sql` tools are more agent-friendly than OKC's `traverse` + `search`.
- **Mitigating factors**: Python/R deployment friction (no single binary), stdio-only transport, v0.1 spec lag, very early maturity (5 commits on MCP), single author. OKC's Rust performance, live reload, HTTP/SSE, and v0.2 support are real advantages.
- **Trajectory risk**: If travisjakel adds HTTP transport, v0.2 support, and binary packaging (PyInstaller/uv), the stack becomes a credible alternative.

---

## Verdict

**okf-ingest + okf-mcp** is the **closest architectural competitor** — same spec (OKF), same paradigm (ingest → catalog → MCP), same author building a cohesive stack. Its **concept-first data model** (wikilinks, typed concepts, RAG-ready chunks) and **agent-oriented MCP tools** (context packs, impact analysis, SQL escape hatch) are superior to OKC's file-centric, link-graph-only approach.

**OKC's competitive position**: Faster (Rust), simpler deployment (single binary), live reload (file watcher), broader transport (stdio + HTTP/SSE), newer spec support (v0.2). Best for **local-first markdown catalogs with zero-config setup**.

**To close the agent-capability gap**, OKC should prioritize:

1. **Wikilinks + concept extraction** — Parse `[[concept]]` in markdown; populate `concepts` table with type/definition/aliases.
2. **Optional vector index** — Integrate `sqlite-vec` (or feature-flag `pgvector`) for hybrid BM25+vector search; okf-ingest has embedding column ready.
3. **Context pack tool** — Compose `search` + `traverse` + metadata with `max_tokens` budget; return provenance + truncation diagnostics (mirror `okf_context`).
4. **Impact analysis tool** — Reverse dependency traversal from concept/file; surface "what breaks if this changes".
5. **Bundle versioning + diff** — Add `version` front-matter; snapshot index on `scan`; `diff` tool comparing snapshots.
6. **Health diagnostics** — Extend `validate` with orphan concepts, broken wikilinks, stale chunks, catalog integrity (mirror `okf_doctor`).
7. **SQL escape hatch (read-only)** — Expose `query_sql` against allowlisted tables (`documents`, `concepts`, `links`, `chunks`).

These seven steps close ~85% of the agent-facing capability gap while preserving OKC's simplicity, performance, and deployment advantages. The concept-first model shift (#1) is the strategic pivot — it aligns OKC with where OKF and agent tooling are heading.