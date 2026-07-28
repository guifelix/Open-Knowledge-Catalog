# Competitor Assessment: CRAN `okf` (Travis Jakel)

## Overview

**okf** (CRAN: `okf`, v0.7.0, published 2026-07-02, 49 downloads/month, https://github.com/travisjakel/okf, https://cran.r-project.org/package=okf) — Open Knowledge Format (OKF) Ingestion for R. Reads, validates, and loads OKF bundles (directories of markdown files with YAML frontmatter) into a portable DuckDB catalog, builds the concept graph, renders to HTML, and optionally embeds concept bodies for semantic search. Deterministic and agent-free: the same bundle always yields the same catalog, graph, and render, with no LLM calls in the core pipeline. Conformant and permissive per OKF v0.1 specification. Apache License (≥ 2). Author/Maintainer: Travis Jakel <travis.s.jakel@gmail.com>. 5 dependencies (yaml, DBI, duckdb, digest, jsonlite, utils), 3 suggests (httr2, commonmark, testthat ≥ 3.0.0). 28 exported R functions.

**OKC (Open Knowledge Catalog)** is a Rust-based, SQLite+FTS5 knowledge catalog with an MCP server exposing 11 tools (`scan`, `search`, `query_metadata`, `traverse`, `get_links`, `get_backlinks`, `get_document`, `get_stats`, `validate`, `serve`, `health`). It targets AI agents as first-class consumers via stdio/HTTP/SSE transports. OKC implements OKF v0.2 (front-matter v2, concept types, relation types, lineage). It includes a file watcher for incremental indexing, graph traversal with BFS, and a transport-agnostic service layer. Pre-1.0, active development.

Both projects consume OKF bundles (markdown + YAML frontmatter) and produce queryable catalogs. **okf** is an R library for data scientists and analysts working in R/RStudio/Quarto; **OKC** is a Rust CLI + MCP server for AI agents and cross-language tooling. The overlap is the OKF ingestion pipeline and concept graph construction. The divergence is the target consumer (human analyst vs. AI agent), the storage backend (DuckDB vs. SQLite+FTS5), the protocol (R API vs. MCP), and the OKF spec version (v0.1 vs. v0.2).

---

## Feature Comparison with OKC

| Aspect | okf (CRAN v0.7.0) | OKC (pre-1.0) | Notes |
|--------|-------------------|---------------|-------|
| **Language** | R (≥ 4.1.0) | Rust (edition 2021) | okf: R ecosystem; OKC: native binary, FFI-friendly |
| **OKF Spec** | v0.1 conformant | v0.2 (front-matter v2, concept types, relation types, lineage) | okf tracks v0.1; OKC extends |
| **Storage Backend** | DuckDB (portable, columnar, analytical) | SQLite + FTS5 (embedded, full-text, row-oriented) | DuckDB better for OLAP; SQLite+FTS5 better for text search |
| **Ingestion** | `okf_ingest()` → DuckDB catalog | `scan` (MCP/CLI) → SQLite index | okf: batch R function; OKC: incremental + file watcher |
| **Validation** | `okf_validate()`, `okf_doctor()`, `okf_doctor_fix()` | `validate` (MCP tool) | okf richer validation + auto-fix |
| **Concept Graph** | `okf_graph_df()`, `okf_graph_json()`, `okf_graph_mermaid()`, `okf_graph_html()` | `traverse` (BFS, max_depth, max_nodes, relation filter) | okf: full graph export formats; OKC: traversal API only |
| **Search** | `okf_search()` (DuckDB FTS), `okf_rag()` (semantic via embeddings) | `search` (FTS5/BM25), `query_metadata` (structured) | okf has semantic/RAG; OKC BM25 only |
| **Embeddings** | `okf_embed()`, `okf_ollama_embedder()`, `okf_chunk_body()` | None (planned: sqlite-vec/pgvector) | okf leads on semantic search |
| **HTML Render** | `okf_html()`, `okf_concepts()`, `okf_findings()` | None (markdown source only) | okf: human-readable output |
| **Link Analysis** | `okf_links()`, `okf_backlinks()`, `okf_extract_links()`, `okf_extract_wikilinks()`, `okf_resolve_link()` | `get_links`, `get_backlinks`, `traverse` | okf richer link extraction (wikilinks) |
| **Diff/Change** | `okf_diff()` (bundle diff) | Implicit via file mtime/git | okf explicit bundle diff |
| **Context/Window** | `okf_context()` (concept context window) | None | okf: context extraction for RAG |
| **MCP Server** | ❌ None | ✅ 11 tools, stdio/HTTP/SSE | **Major gap**: okf not agent-accessible via MCP |
| **File Watcher** | ❌ None | ✅ Incremental indexing | okf: batch only |
| **Lineage/Provenance** | ❌ Not in v0.1 | ✅ OKF v0.2 lineage fields | okf limited by spec version |
| **Concept Types** | Implicit via front-matter | Explicit `concept_type` (v0.2) | okf: free-form; OKC: typed |
| **Relation Types** | Implicit (markdown links) | Explicit `relation_type` (v0.2) | okf: untyped; OKC: typed |
| **Exports/API** | 28 R functions | 11 MCP tools + CLI | okf: R REPL/script; OKC: agent protocol |
| **Testing** | testthat ≥ 3.0.0 | `cargo test` (unit + integration) | Both tested |
| **License** | Apache ≥ 2 | MIT/Apache-2.0 dual | Compatible |
| **Maturity** | CRAN, v0.7.0, 49 dl/mo | Pre-1.0, active dev | okf more "released" |

---

## Architecture & Code Quality

### okf (R Package)
- **Structure**: Single R package with `R/` (28 exported functions), `inst/` (OKF spec vignette), `tests/testthat/`, `vignettes/`. No compiled code (pure R + DBI/duckdb).
- **Lines**: ~2,500 R LoC (est. from 28 exports + tests + vignettes).
- **Database**: DuckDB via `DBI`/`duckdb` — portable, zero-config, columnar, analytical SQL. Catalog schema: `concepts`, `links`, `chunks`, `embeddings` tables.
- **Async/Concurrency**: None (R is single-threaded). Batch ingestion synchronous.
- **Validation**: `okf_validate()` checks front-matter schema, link integrity, concept IDs. `okf_doctor()` diagnoses bundle health; `okf_doctor_fix()` auto-repairs common issues (missing IDs, broken links).
- **Graph Construction**: Extracts `[[wikilinks]]` and `[markdown](links)` via `okf_extract_wikilinks()` / `okf_extract_links()`. Builds adjacency in DuckDB. Exports as data.frame (`okf_graph_df`), JSON (`okf_graph_json`), Mermaid (`okf_graph_mermaid`), HTML (`okf_graph_html`).
- **Semantic Search**: `okf_embed()` generates embeddings (pluggable backend). `okf_ollama_embedder()` uses local Ollama. `okf_chunk_body()` splits concept bodies for RAG. `okf_rag()` runs hybrid search (BM25 + vector).
- **Context Windows**: `okf_context(concept_id, window = 3)` returns surrounding concepts for LLM context packing.
- **Diff**: `okf_diff(old_bundle, new_bundle)` computes added/removed/changed concepts and links.
- **Quality Gates**: `testthat` (unit + integration), `R CMD check` (CRAN standards), `lintr`/`styler` (implied). No `clippy`/`rustfmt` equivalent.
- **Observability**: None (R `message()`/`warning()` only). No metrics, tracing, or structured logging.
- **Maturity**: CRAN release (v0.7.0) passes `R CMD check --as-cran`. 49 downloads/month indicates niche adoption. 28 exports suggest stable API surface.

### OKC (Rust)
- **Structure**: Single binary crate (`okc`) with modules: `config`, `index`, `model`, `parser`, `scanner`, `service`, `transport` (CLI + MCP). ~8k Rust LoC.
- **Database**: SQLite (r2d2 + rusqlite) + FTS5 virtual table for full-text search. Row-oriented, embedded, ACID.
- **Async/Concurrency**: Tokio for MCP server (stdio/HTTP/SSE). Core indexing synchronous. File watcher (`notify`) for incremental updates.
- **Validation**: `validate` tool checks index integrity, orphan links, front-matter schema (OKF v0.2). No auto-fix.
- **Graph Construction**: Parses markdown links (`[text](path)`) and wikilinks (`[[path]]`). Stores as `links` table (source, target, relation_type). `traverse` tool runs BFS with depth/node limits and relation filter.
- **Search**: FTS5/BM25 only (`search` tool). `query_metadata` for structured front-matter filtering. No vector search.
- **MCP Server**: 11 tools exposed via `rmcp` crate. Transport-agnostic service layer (`OkcService`). Tools return structured JSON.
- **Quality Gates**: `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test`. No formal coverage gate.
- **Observability**: `tracing` structured logs. No OTLP/Prometheus export.
- **Maturity**: Pre-1.0, active development. No CRAN/registry equivalent (crates.io pending).

### Comparison
| Dimension | okf | OKC |
|-----------|-----|-----|
| **Paradigm** | Batch R library | Long-running MCP server + CLI |
| **Concurrency** | Single-threaded R | Multi-threaded Tokio |
| **Incremental** | No (full re-ingest) | Yes (file watcher + mtime) |
| **Schema Evolution** | DuckDB `ALTER TABLE` | SQLite migrations (manual) |
| **Extensibility** | R functions / S3 methods | MCP tools / Rust traits |
| **Deployment** | `install.packages("okf")` | Static binary / `cargo install` |
| **Agent Access** | ❌ (R-only) | ✅ (MCP stdio/HTTP/SSE) |
| **Human Access** | ✅ (R console, Quarto, Shiny) | ✅ (CLI, MCP via agent) |

---

## MCP Capability Inventory

| Capability | okf | OKC | Notes |
|------------|-----|-----|-------|
| **MCP Server** | ❌ None | ✅ `okc serve --stdio/--http/--sse` | okf not accessible to agents |
| **Tools Exposed** | 28 R functions | 11 MCP tools | okf richer API but wrong protocol |
| **Resources** | ❌ | ❌ (planned) | Neither exposes MCP resources yet |
| **Prompts** | ❌ | ❌ (planned) | Neither exposes prompt templates |
| **Auth/Scopes** | ❌ | ❌ (planned) | Both lack agent authorization |
| **Transports** | N/A | stdio, HTTP, SSE | OKC multi-transport |
| **Tool Discovery** | `ls("package:okf")` | `list_tools` (MCP) | Different consumers |

**Gap**: okf has zero MCP support. To make okf agent-accessible, one would need to wrap its R functions in an MCP server (e.g., via `plumber` + `rmcp` or a Rust shim calling R via `extendr`). This is non-trivial and adds latency/dependency complexity.

---

## Strengths vs OKC

1. **Semantic Search & RAG Built-In** — `okf_embed()`, `okf_ollama_embedder()`, `okf_chunk_body()`, `okf_rag()` provide a complete local-first RAG pipeline (BM25 + vector hybrid) with pluggable embedders. OKC has no vector search; adding it requires `sqlite-vec` or `pgvector` integration.

2. **Richer Validation & Auto-Repair** — `okf_validate()` + `okf_doctor()` + `okf_doctor_fix()` form a diagnose/fix loop for bundle health. OKC's `validate` only reports issues; no auto-fix.

3. **Explicit Bundle Diff** — `okf_diff(old, new)` computes concept/link additions, deletions, changes. OKC relies on file mtime/git; no bundle-level diff API.

4. **Context Window Extraction** — `okf_context(concept_id, window)` returns neighboring concepts for LLM context packing. OKC has no equivalent; agents must compose via `traverse` + `get_document`.

5. **Multiple Graph Export Formats** — `okf_graph_df` (data.frame for R analysis), `okf_graph_json` (interchange), `okf_graph_mermaid` (visualization), `okf_graph_html` (standalone report). OKC only returns traversal JSON.

6. **Wikilink & Markdown Link Extraction** — `okf_extract_wikilinks()` and `okf_extract_links()` handle both syntaxes explicitly. OKC's parser handles both but doesn't expose extraction as a separate tool.

7. **HTML Rendering for Humans** — `okf_html()`, `okf_concepts()`, `okf_findings()` produce browsable HTML output. OKC is headless (markdown source only).

8. **R Ecosystem Integration** — Native `data.frame` returns, `ggplot2`-ready graph data, Quarto/R Markdown integration, Shiny app potential. OKC requires FFI or CLI parsing for R users.

9. **CRAN Distribution & Standards** — `install.packages("okf")` works everywhere R runs. CRAN enforces `R CMD check`, no internet in tests, license compliance. OKC requires Rust toolchain + binary install.

10. **Deterministic, Agent-Free Core** — Explicit design principle: no LLM calls in core ingestion/graph/render. Same bundle → same output. OKC also deterministic but doesn't emphasize this as a guarantee.

---

## Weaknesses vs OKC

1. **No MCP Server / Agent Protocol** — okf is an R library, not an MCP server. Agents cannot call `okf_search()` or `okf_traverse()` via standardized protocol. OKC's 11 MCP tools make it immediately usable by Claude, Cursor, Copilot, etc.

2. **No Incremental Indexing / File Watcher** — `okf_ingest()` re-processes the entire bundle. OKC's `notify`-based watcher updates only changed files. For large bundles, okf re-ingest is slow.

3. **Single-Threaded R Runtime** — Ingestion, embedding, search all block the R session. OKC's Tokio runtime handles concurrent MCP requests.

4. **OKF v0.1 Only (No Typed Relations/Lineage)** — okf conforms to v0.1: concepts have `id`, `title`, `type` (free text), links are untyped. OKC implements v0.2: explicit `concept_type` enum, `relation_type` enum, `lineage` (source, derived_from, version). okf cannot represent "supports/contradicts/supersedes" semantics.

5. **No Cross-Language / CLI Access** — okf functions only callable from R. OKC's CLI (`okc scan`, `okc search`, `okc traverse`) works from shell, CI, Make, any language.

6. **No Multi-Transport Server** — okf cannot run as a persistent service. OKC serves stdio (for local agents), HTTP (for remote), SSE (for web).

7. **DuckDB Not Ideal for Text Search** — DuckDB FTS is functional but less mature than SQLite FTS5. OKC's FTS5 + BM25 is battle-tested for full-text.

8. **No Structured Metadata Query Tool** — okf has `okf_concepts()` (returns all concepts as data.frame) but no `query_metadata`-equivalent for filtered front-matter queries. OKC's `query_metadata` supports field projection, operators, pagination.

9. **No Observability / Metrics** — okf emits only R messages/warnings. OKC has `tracing` structured logs (extensible to OTLP).

10. **R Dependency Limits Deployment** — Requires R ≥ 4.1.0 + DuckDB + dependencies. OKC single static binary (musl) runs anywhere Linux/macOS/Windows.

---

## OKC Improvement Opportunities

| Area | Gap | Competitor Reference | Action |
|------|-----|---------------------|--------|
| **Semantic Search / RAG** | BM25 only; no vector search | okf: `okf_embed()`, `okf_ollama_embedder()`, `okf_rag()` (hybrid BM25+vector) | Add `sqlite-vec` extension (feature flag) or `pgvector` backend; implement `embed` + `rag` MCP tools with pluggable embedder trait. |
| **Validation & Auto-Fix** | `validate` reports only; no repair | okf: `okf_doctor()` + `okf_doctor_fix()` | Add `doctor` + `doctor_fix` MCP tools; implement common fixes (missing IDs, broken links, front-matter schema coercion). |
| **Bundle Diff** | No diff API; relies on git | okf: `okf_diff(old, new)` → added/removed/changed concepts & links | Add `diff` MCP tool comparing two bundle paths or index snapshots; emit structured change set. |
| **Context Window Extraction** | Agents must compose via `traverse` + `get_document` | okf: `okf_context(id, window)` returns neighbor concepts | Add `context` MCP tool: given concept ID + radius, return bundled concepts + metadata for LLM context packing. |
| **Graph Export Formats** | Only traversal JSON | okf: data.frame, JSON, Mermaid, HTML | Add `graph_export` tool with `format` param (json, mermaid, html, dot); reuse for diagnostics. |
| **Wikilink Extraction Tool** | Parser handles both but not exposed | okf: `okf_extract_wikilinks()`, `okf_extract_links()` | Expose `extract_links` MCP tool returning both link types with source location. |
| **HTML Rendering** | Headless only | okf: `okf_html()`, `okf_concepts()`, `okf_findings()` | Add `render_html` tool (optional feature) for human-readable bundle reports. |
| **Structured Metadata Query** | `query_metadata` exists but limited | okf: `okf_concepts()` returns all; no filtered query | Enhance `query_metadata`: add operators (contains, regex, range), pagination, sorting, field projection. |
| **Incremental Indexing** | File watcher exists but basic | okf: none (batch only) | Harden watcher: debounce, batch commits, handle renames/deletes, expose `index_status` resource. |
| **Observability** | `tracing` logs only | okf: none | Add OTLP exporter + Prometheus `/metrics` endpoint; expose `okc://index/status` resource. |
| **Typed Relations (v0.2)** | Single `links_to`/`linked_from` | okf: untyped (v0.1) | Already in OKC v0.2 spec; ensure parser emits `relation_type` from `[[type:target]]` syntax. |
| **Lineage/Provenance** | Not implemented | okf: not in v0.1 | Implement `lineage` front-matter fields (source, derived_from, version) per OKF v0.2. |
| **Cross-Language CLI** | `okc` binary works | okf: R-only | Ensure `okc` CLI covers all MCP tools 1:1; add shell completions. |
| **Agent Auth/Scopes** | None | okf: none | Add scope config (`OKC_MCP_ALLOWED_SCOPES`) + per-tool authorization middleware. |

---

## Threat Level

**Medium**

**Rationale**: okf is a **specialized R package** for data scientists working in R/Quarto. It does not compete for the **AI agent / MCP server** niche that OKC targets. However, it demonstrates **feature completeness in areas OKC lacks** (semantic search/RAG, validation+fix, bundle diff, context windows, graph exports, HTML rendering). If okf adds an MCP wrapper (e.g., via `plumber` + `rmcp` or a Rust shim), it could become a viable agent-accessible alternative for R-centric teams. The OKF v0.1 spec limitation (no typed relations/lineage) is a ceiling okf will hit; OKC's v0.2 implementation is a differentiator. The 49 downloads/month indicates niche but real adoption in the R/OKF community. OKC should treat okf as a **feature reference** for the RAG/validation/diff/context capabilities, not a direct market competitor.

---

## Verdict

**Strategic Summary**: okf is the **reference implementation of OKF v0.1 in R** — feature-rich for human analysts (RAG, validation, diff, context, graph exports, HTML render) but **architecturally incompatible with agent workflows** (no MCP, single-threaded R, batch-only). OKC leads on **agent accessibility** (MCP server, multi-transport, incremental indexing, CLI) and **spec evolution** (OKF v0.2 typed relations/lineage). The projects serve different primary users: okf → R data scientists; OKC → AI agents + polyglot tooling.

**Priority Actions for OKC** (close 80% of the capability gap for agent workflows):

1. **Add optional vector search** — `sqlite-vec` feature flag + `embed`/`rag` MCP tools (mirror okf's `okf_embed`/`okf_rag`).
2. **Implement `doctor` + `doctor_fix` tools** — Auto-repair common bundle issues (missing IDs, broken links, schema coercion).
3. **Add `diff` MCP tool** — Compare two bundles or index snapshots; emit structured change set.
4. **Add `context` MCP tool** — Given concept ID + radius, return bundled neighbor concepts for LLM context packing.
5. **Add `graph_export` tool** — Support `json`, `mermaid`, `html`, `dot` formats for diagnostics/visualization.
6. **Enhance `query_metadata`** — Operators, pagination, sorting, field projection (parity with okf's data.frame flexibility).
7. **Expose `extract_links` tool** — Return both markdown and wikilinks with source locations.
8. **Add OTLP/Prometheus observability** — `/metrics` endpoint + `okc://index/status` resource for agent monitoring.

These increments preserve OKC's Rust/agent-first architecture while adopting okf's proven analyst-facing capabilities. The OKF v0.2 typed relation/lineage model remains OKC's strategic moat — okf cannot adopt it without breaking v0.1 conformance.