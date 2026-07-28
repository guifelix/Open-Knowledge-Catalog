# Competitor Assessment: okf-tool (npm)

## Overview

**okf-tool** (npm: `okf-tool`, v0.2.0, Apache-2.0, https://github.com/hanfang/okf-tool) — A TypeScript library and CLI for parsing, writing, searching, and validating Open Knowledge Format (OKF) knowledge bundles. Published ~1 month ago by hanfang5057. 276.7 kB unpacked. Dependencies: `gray-matter` (frontmatter parsing), `js-yaml` (YAML support). CLI entry point: `okf`.

This is a **pure TypeScript implementation** of OKF bundle operations — focused on the core CRUD operations for OKF bundles. It does not provide an MCP server, persistent indexing, or agent-facing tools. It is a library-first toolkit for developers building OKF-aware applications.

---

## Feature Comparison with OKC

| Feature | okf-tool | OKC | Notes |
|---------|----------|-----|-------|
| **Language** | TypeScript | Rust | Different runtime characteristics |
| **License** | Apache-2.0 | MIT | Both permissive |
| **CLI** | ✅ `okf` command | ✅ `okc` command | Both have CLI |
| **MCP Server** | ❌ None | ✅ 11 tools, stdio + HTTP/SSE | OKC only |
| **Bundle Parsing** | ✅ Full OKF parse | ✅ Via `okf` crate | Parity |
| **Bundle Writing** | ✅ Write/modify | ⚠️ Planned (`ingest`) | okf-tool ahead |
| **Search** | ✅ Basic search | ✅ FTS5/BM25 + graph | OKC richer |
| **Validation** | ✅ Bundle validation | ✅ `validate` tool | Parity |
| **Persistent Index** | ❌ None | ✅ SQLite + FTS5 | OKC only |
| **File Watcher** | ❌ None | ✅ `notify` + `observe` | OKC only |
| **Graph Traversal** | ❌ None | ✅ `traverse` tool | OKC only |
| **Lineage/History** | ❌ None | ✅ `lineage` tool | OKC only |
| **MCP Resources** | ❌ None | ❌ Planned | Neither |
| **MCP Prompts** | ❌ None | ❌ Planned | Neither |

---

## Architecture & Code Quality

### okf-tool
- **Structure**: Single package with CLI + library exports. ~276 kB unpacked.
- **Dependencies**: Minimal — `gray-matter` (frontmatter), `js-yaml` (YAML). Zero native deps.
- **Architecture**: Library-first. Core functions: `parseBundle`, `writeBundle`, `searchBundle`, `validateBundle`. CLI wraps library.
- **Testing**: Not visible in repo metadata.
- **Quality Gates**: No `clippy`/`deny.toml` equivalent. TypeScript strict mode unknown.
- **Documentation**: Basic README with CLI usage. No architecture docs.
- **Maturity**: v0.2.0, ~1 month old, single maintainer (hanfang5057).

### OKC
- **Structure**: Single binary crate (~8k LoC). Modules: `config`, `index`, `model`, `parser`, `scanner`, `service`, `transport`.
- **Dependencies**: `okf` (parser/model/validator), `rusqlite` + `r2d2` (SQLite), `tokio` (MCP), `notify` (file watcher), `clap`, `tracing`, `serde`, `anyhow`.
- **Database**: SQLite (r2d2 pool) with FTS5 virtual table. WAL mode. Schema migrations embedded.
- **Async**: Tokio for MCP server (stdio + HTTP/SSE). Core indexing synchronous.
- **Quality Gates**: `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test` in CI.
- **Observability**: `tracing` structured logs. No metrics export.
- **Maturity**: Pre-1.0, active development.

---

## MCP Capability Inventory

| Tool/Resource | okf-tool | OKC | Notes |
|---------------|----------|-----|-------|
| **Parse bundle** | ✅ Library + CLI | ❌ Internal only | okf-tool: primary use case |
| **Validate bundle** | ✅ Library + CLI | ✅ `validate` tool | okf-tool: spec validation; OKC: index health |
| **Search** | ✅ Basic library | ✅ `search` (BM25, filters) | OKC: richer pipeline |
| **Get document** | ❌ | ✅ `get_document` | OKC only |
| **Graph traverse** | ❌ | ✅ `traverse` (BFS, filters) | OKC only |
| **Lineage/history** | ❌ | ✅ `lineage` | OKC only |
| **Metadata query** | ❌ | ✅ `query_metadata` | OKC only |
| **MCP stdio** | ❌ | ✅ `okc serve --stdio` | OKC only |
| **MCP HTTP/SSE** | ❌ | ✅ `okc serve --http` | OKC only |
| **Resources** | ❌ | ❌ Planned | Neither |
| **Prompts** | ❌ | ❌ Planned | Neither |

---

## Strengths vs OKC

1. **TypeScript ecosystem integration** — Native for Node.js/TypeScript projects. No FFI, no Rust toolchain required. Better IDE support for TS developers.

2. **Library-first design** — Core operations exposed as pure functions. Easy to embed in build tools, generators, linters, CI pipelines. OKC is a standalone binary.

3. **Bundle writing/modification** — `writeBundle` / `modifyBundle` are first-class. OKC's `ingest` is planned but not yet implemented.

4. **Apache-2.0 license** — Permissive, compatible with most corporate policies.

5. **Minimal dependencies** — Only `gray-matter` + `js-yaml`. Tiny attack surface, fast installs.

6. **Recent active development** — v0.2.0 published July 2026. Single maintainer but shipping.

---

## Weaknesses vs OKC

1. **No MCP server** — Cannot be consumed directly by AI agents (Claude, Cursor, VS Code, etc.). OKC's 11 MCP tools over stdio/HTTP/SSE is a major differentiator.

2. **No persistent index** — Every operation re-parses bundles. OKC's SQLite+FTS5 index survives restarts, supports incremental updates via file watcher.

3. **No full-text search engine** — Basic search only. No BM25, no ranking, no faceted filters. OKC has FTS5/BM25 with path prefix, concept type, tag filters.

4. **No graph traversal** — No `traverse`, `get_links`, `get_backlinks`. OKC's `traverse` supports BFS with depth/node limits and relation filters.

5. **No file watcher / live updates** — Must re-run CLI on every change. OKC's `notify`-based watcher + `observe` tool pushes updates to MCP clients in real time.

6. **No lineage/history** — No concept of concept evolution. OKC's `lineage` tool tracks splits, merges, renames.

6. **Single maintainer, low bus factor** — 1 contributor, ~1 month old. OKC has active development.

7. **No MCP resources/prompts** — Cannot expose `okf://bundle/{id}` style resources or prompt templates.

8. **No metadata/faceted query** — Cannot filter by front-matter key/value. OKC's `query_metadata` enables structured faceted search.

---

## OKC Improvement Opportunities

| Area | Gap | Competitor Reference | Action |
|------|-----|---------------------|--------|
| **Bundle writing API** | Planned only | okf-tool: `writeBundle`/`modifyBundle` | Implement `okc ingest` / `okc write` for agent-authored content |
| **Library exposure** | Binary only | okf-tool: library-first | Publish `okc-lib` crate with public `parse`, `validate`, `search` APIs |
| **TypeScript bindings** | None | okf-tool: native TS | Consider `napi-rs` / `wasm-bindgen` for TS/Node interop |
| **Minimal dependency mode** | ~30 deps | okf-tool: 2 deps | Add `minimal` feature flag disabling MCP, SQLite, etc. |
| **Frontmatter flexibility** | Strict via `okf` crate | okf-tool: `gray-matter` (lenient) | Add `--lenient` parse mode for messy real-world bundles |

---

## Threat Level

**Low**

**Rationale:**
- **Different layer**: okf-tool is a *library/CLI for bundle operations*; OKC is a *runtime catalog + MCP server for agents*. They solve adjacent but distinct problems.
- **No MCP overlap**: okf-tool has zero agent-facing protocol. OKC's primary value prop is MCP.
- **No persistence/search overlap**: okf-tool has no index, no FTS, no watcher. OKC's moat is durable, queryable, live catalog.
- **Complementary potential**: OKC could *depend* on okf-tool (or its parser logic) for bundle writing/validation, while providing the MCP/runtime layer okf-tool lacks.
- **Single maintainer risk**: okf-tool's bus factor = 1. OKC has broader contribution base.

**Risk**: If okf-tool adds MCP server + persistent index + FTS, it becomes a direct competitor. Current architecture (library-first, no DB) makes this unlikely without major rewrite.

---

## Verdict

**okf-tool is a complementary library, not a competing product.** It excels at the "bundle operations" layer (parse, write, validate, basic search) for TypeScript developers. OKC excels at the "runtime catalog + agent interface" layer (persistent index, FTS5, graph traversal, MCP server, live updates).

**Strategic implications for OKC:**

1. **Adopt okf-tool's bundle writing patterns** — Implement `okc write` / `okc ingest` for agent-authored content. This closes the "read-only catalog" gap.

2. **Expose library API** — Publish `okc-lib` crate so TypeScript/Rust/Go tools can embed OKC's parsing/validation/search without spawning CLI.

3. **Consider TypeScript bindings** — `napi-rs` or `wasm-bindgen` could make OKC's FTS5/graph traversal available to the TS ecosystem where okf-tool lives.

4. **Monitor for MCP adoption** — If okf-tool adds MCP server + SQLite index, reassess threat level. Current architecture makes this a significant pivot.

**Recommended priority:**
1. Add `okc write` / `okc ingest` (bundle authoring) — Q1
2. Publish `okc-lib` crate with public parse/validate/search — Q1
3. Evaluate `napi-rs` bindings for TypeScript interop — Q2
4. Add `--lenient` parse mode for real-world messy bundles — Q2

okf-tool validates that **TypeScript developers want a library for OKF bundle operations**. OKC should serve that need *in addition to* its agent-facing MCP server — not instead of it.