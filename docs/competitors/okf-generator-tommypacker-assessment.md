# Competitor Assessment: okf-generator (PyPI - tommypacker)

## Overview

**okf-generator** (PyPI: `okf-generator`, v0.2.0, https://github.com/tommypacker/okf-generator) — A **CLI tool for generating OKF v0.2 bundles** from source code repositories. It scans repository files (package manifests, docs, CI workflows, tests, configs, package directories) and writes a reviewable OKF bundle. The repository includes its own generated OKF bundle in `okf/` for inspection. Published by tommypacker. MIT license. Python implementation.

This is a **bundle generator** — not a catalog or server. It produces OKF v0.2 bundles (extended dialect of OKF v0.1) with schema version, concept identity, language, status, and typed relationships. OKC (Open Knowledge Catalog) is a **runtime catalog + MCP server** that indexes and serves bundles to agents. They are complementary: okf-generator *produces* bundles; OKC *indexes and serves* them.

---

## Feature Comparison with OKC

| Feature | okf-generator (tommypacker) | OKC | Notes |
|---------|----------------------------|-----|-------|
| **Primary Purpose** | Bundle generation from source repos | Knowledge catalog + MCP server | Different problem |
| **OKF Version** | v0.2 (extended) | v0.2 | Both v0.2 |
| **Language** | Python | Rust | Different runtime |
| **CLI** | ✅ `okf generate` | ✅ `okc` | Both have CLI |
| **MCP Server** | ❌ None | ✅ 11 tools, stdio + HTTP/SSE | OKC only |
| **Bundle Parsing** | ✅ Scans repo → writes bundle | ✅ Via `okf` crate | Both parse |
| **Bundle Writing** | ✅ Generates bundle files | ⚠️ Planned (`ingest`) | okf-generator ahead |
| **Validation** | ⚠️ Basic (reviewable output) | ✅ `validate` tool | OKC richer |
| **Persistent Index** | ❌ None | ✅ SQLite + FTS5 | OKC only |
| **File Watcher** | ❌ None | ✅ `notify` + `observe` | OKC only |
| **Search** | ❌ None | ✅ FTS5/BM25 + graph | OKC only |
| **Graph Traversal** | ❌ None | ✅ `traverse` (BFS) | OKC only |
| **Lineage/History** | ❌ None | ✅ `lineage` tool | OKC only |
| **MCP Tools** | ❌ None | ✅ 11 tools | OKC only |
| **Dogfooding** | ✅ Own bundle in `okf/` | ❌ Not yet | okf-generator ahead |
| **License** | MIT | MIT | Aligned |

---

## Architecture & Code Quality

### okf-generator (tommypacker)
- **Language**: Python
- **Structure**: Single CLI entry point (`okf generate`) scanning repo → writing OKF v0.2 bundle
- **Dependencies**: Minimal (standard lib + possibly `pyyaml`, `gitpython`)
- **Architecture**: Single-pass scanner → concept extractor → bundle writer
- **Testing**: Not visible in repo metadata
- **Quality Gates**: No visible CI/lint config
- **Documentation**: Good README with installation, usage, dogfooding example
- **Maturity**: Early (v0.2.0), single maintainer (tommypacker), 1 star
- **Dogfooding**: Repository includes its own generated OKF bundle in `okf/` — strong signal

### OKC
- **Structure**: Single binary crate (~8k LoC). Modules: `config`, `index`, `model`, `parser`, `scanner`, `service`, `transport`
- **Dependencies**: `okf` (parser/model/validator), `rusqlite` + `r2d2` (SQLite), `tokio` (MCP), `notify` (file watcher), `clap`, `tracing`, `serde`, `anyhow`
- **Database**: SQLite (r2d2 pool) with FTS5 virtual table. WAL mode. Embedded migrations.
- **Async**: Tokio for MCP server (stdio + HTTP/SSE). Core indexing synchronous.
- **Quality Gates**: `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test` in CI
- **Observability**: `tracing` structured logs. No metrics export.
- **Maturity**: Pre-1.0, active development

---

## MCP Capability Inventory

| Tool/Resource | okf-generator | OKC | Notes |
|---------------|---------------|-----|-------|
| **MCP Server** | ❌ None | ✅ 11 tools, stdio + HTTP/SSE | OKC only |
| **Search** | ❌ | ✅ `search` (BM25, filters) | OKC only |
| **Graph Traverse** | ❌ | ✅ `traverse` (BFS, filters) | OKC only |
| **Lineage** | ❌ | ✅ `lineage` | OKC only |
| **File Watch** | ❌ | ✅ `observe` + fs watcher | OKC only |
| **Bundle Generation** | ✅ `okf generate` | ⚠️ Planned | okf-generator only |
| **Validation** | ⚠️ Reviewable output | ✅ `validate` | OKC richer |
| **Resources** | ❌ | ❌ Planned | Neither |
| **Prompts** | ❌ | ❌ Planned | Neither |

---

## Strengths vs OKC

1. **Bundle generation from source code** — Scans real repos (package manifests, docs, CI, tests, configs) and produces reviewable OKF v0.2 bundles. OKC has no generator yet (`ingest` planned).

2. **Dogfooding** — The repo includes its own generated OKF bundle in `okf/` for inspection. Strong credibility signal.

3. **OKF v0.2 extensions** — Adds schema version, concept identity, language, status, typed relationships. OKC targets v0.2 but generator doesn't exist yet.

4. **Python ecosystem** — Accessible to data science/ML teams already in Python. OKC requires Rust toolchain.

5. **Zero-config scanning** — Point at a repo, get a bundle. OKC requires config + `scan`.

---

## Weaknesses vs OKC

1. **No MCP server** — Cannot be consumed by AI agents directly. OKC's 11 MCP tools over stdio/HTTP/SSE is the primary agent interface.

2. **No persistent index** — Generates bundle once; no incremental updates, no FTS, no query engine. OKC's SQLite+FTS5 survives restarts, supports incremental updates via file watcher.

3. **No search** — No FTS, no BM25, no vector. OKC has hybrid BM25 + vector (planned).

4. **No graph traversal** — No `traverse`, `get_links`, `get_backlinks`. OKC's `traverse` supports BFS with depth/node limits and relation filters.

5. **No lineage/history** — No concept evolution tracking. OKC's `lineage` tool tracks splits/merges/renames.

6. **No file watcher / live updates** — Must re-run CLI on every change. OKC's `notify` + `observe` pushes updates in real-time.

6. **No MCP resources/prompts** — Cannot expose `okf://bundle/{id}` style resources or prompt templates.

7. **Single maintainer, early stage** — 1 star, v0.2.0, single maintainer. OKC has active development.

8. **No validation depth** — Output is "reviewable" but no automated validation against spec. OKC's `validate` checks index health, broken links, schema.

---

## OKC Improvement Opportunities

| Area | Gap | Competitor Reference | Action |
|------|-----|---------------------|--------|
| **Bundle generation** | Planned only | okf-generator: `okf generate` scans repo → writes bundle | Implement `okc generate` / `okc ingest` for repo → bundle |
| **Dogfooding** | Not yet | okf-generator: own bundle in `okf/` | Create OKC's own docs as OKF bundle; `okc scan` on itself |
| **OKF v0.2 extensions** | Targeted | okf-generator: schema version, identity, language, status, typed relations | Ensure OKC's model/schema supports v0.2 fully |
| **Python bindings** | None | okf-generator: native Python | Consider `napi-rs` / `wasm-bindgen` for TS/Python interop |
| **Zero-config scan** | Config required | okf-generator: point at repo | Add `okc scan --auto` for heuristic repo detection |

---

## Threat Level

**Low (direct) / Medium (feature inspiration)**

**Rationale:**
- **Different layer**: okf-generator = bundle *producer*; OKC = catalog *runtime + agent interface*. They compose.
- **No MCP overlap**: okf-generator has zero agent-facing protocol. OKC's moat is MCP.
- **No persistence/search/traversal**: okf-generator is a one-shot generator. OKC's durable index + live updates + graph traversal + lineage is a different product category.
- **Complementary**: OKC could *use* okf-generator (or its logic) for `okc generate` / `okc ingest`.
- **Risk**: If okf-generator adds MCP server + persistent index + search, it becomes a direct competitor. Current architecture (one-shot generator) makes this unlikely without major rewrite.

---

## Verdict

**okf-generator (tommypacker) is a complementary bundle generator, not a catalog competitor.** It solves "how do I create an OKF bundle from my repo?" — OKC solves "how do I index, search, traverse, and serve that bundle to agents via MCP?"

**Strategic implication for OKC:** The generator gap is real and closable. OKC should own the "generate → index → serve" pipeline end-to-end.

**Recommended priority:**
1. **Add `okc generate` / `okc ingest`** — Repo → OKF v0.2 bundle (Q1)
2. **Dogfood OKC's own docs** — Create OKC's docs as OKF bundle; `okc scan` on itself (immediate)
3. **Ensure v0.2 schema support** — Schema version, concept identity, language, status, typed relations (Q1)
4. **Consider Python/TS bindings** — `napi-rs` / `wasm-bindgen` for ecosystem reach (Q2)
5. **Monitor okf-generator for MCP adoption** — If they add MCP server + index, reassess (ongoing)

okf-generator validates that **bundle generation is a real need**. OKC should own the full pipeline.