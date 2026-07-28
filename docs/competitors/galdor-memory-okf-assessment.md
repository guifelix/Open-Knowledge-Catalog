# Competitor Assessment: @galdor/memory-okf (npm)

## Overview

**@galdor/memory-okf** (npm: `@galdor/memory-okf`, v0.5.0, Apache-2.0, https://github.com/galdor/memory-okf) — An OKF knowledge backend for the galdor-bun ecosystem. Full OKF v0.1 implementation with BM25 retrieval (code-aware tokenizer), link graph, progressive-disclosure browsing, change logs, citations, validation, bundle writing, and `okf_search` / `okf_browse` agents. Published July 22, 2026 by Yasser Rosas. TypeScript ES Module. Dependencies: `@galdor/core` (0.5.0), `zod` (^4.0.0). Node.js ≥22.5, Bun ≥1.3.

This is a **specialized OKF backend for the galdor agent framework** — not a standalone catalog. It provides knowledge storage/retrieval as a library for galdor agents, with BM25 search, link graph, and agent interfaces (`okf_search`, `okf_browse`). It is **tightly coupled to the galdor ecosystem**.

---

## Feature Comparison with OKC

| Feature | @galdor/memory-okf | OKC | Notes |
|---------|-------------------|-----|-------|
| **Language** | TypeScript (Node.js ≥22.5, Bun ≥1.3) | Rust | Different runtime |
| **License** | Apache-2.0 | MIT | Both permissive |
| **CLI** | ❌ None (library) | ✅ `okc` command | OKC only |
| **MCP Server** | ❌ None | ✅ 11 tools, stdio + HTTP/SSE | OKC only |
| **Bundle Parsing** | ✅ OKF v0.1 | ✅ OKF v0.2 | OKC newer spec |
| **BM25 Search** | ✅ Code-aware tokenizer | ✅ FTS5/BM25 | Both BM25 |
| **Link Graph** | ✅ Progressive disclosure | ✅ `traverse` tool | OKC: programmatic |
| **Bundle Writing** | ✅ Library API | ⚠️ Planned | galdor ahead |
| **Validation** | ✅ OKF v0.1 | ✅ `validate` | Parity |
| **Change Logs** | ✅ Built-in | ❌ None | galdor unique |
| **Citations** | ✅ Built-in | ❌ None | galdor unique |
| **Persistent Index** | ❌ In-memory | ✅ SQLite + FTS5 | OKC only |
| **File Watcher** | ❌ None | ✅ `notify` + `observe` | OKC only |
| **Graph Traversal** | ⚠️ Browse agents | ✅ `traverse` (BFS) | OKC richer |
| **Lineage/History** | ❌ None | ✅ `lineage` | OKC only |
| **MCP Server** | ❌ None | ✅ 11 tools | OKC only |
| **MCP Resources** | ❌ None | ❌ Planned | Neither |
| **MCP Prompts** | ❌ None | ❌ Planned | Neither |
| **Ecosystem** | galdor-bun only | Independent | galdor locked |

---

## Architecture & Code Quality

### @galdor/memory-okf
- **Structure**: Single ES Module package. ~260 kB unpacked. Exports: `okf_search`, `okf_browse`, validation, bundle writing, link graph, BM25 retrieval.
- **Dependencies**: 2 — `@galdor/core` (0.5.0), `zod` (^4.0.0). Minimal.
- **Architecture**: Library-first for galdor agents. Core: BM25 index (code-aware tokenizer), link graph, progressive-disclosure browser, change log, citation manager, validator, bundle writer.
- **Testing**: Not visible in npm metadata.
- **Quality Gates**: TypeScript strict, zod for validation. No visible CI config.
- **Documentation**: npm README only. No architecture docs.
- **Maturity**: v0.5.0, 3 versions (0.4.0, 0.4.1, 0.5.0), single maintainer (yassercr). Pre-1.0.

### OKC
- **Structure**: Single binary crate (~8k LoC). Modules: `config`, `index`, `model`, `parser`, `scanner`, `service`, `transport`.
- **Dependencies**: `okf`, `rusqlite` + `r2d2`, `tokio`, `notify`, `clap`, `tracing`, `serde`, `anyhow`.
- **Database**: SQLite + FTS5 (r2d2 pool). WAL mode. Embedded migrations.
- **Async**: Tokio for MCP server. Core indexing synchronous.
- **Quality Gates**: `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test`.
- **Observability**: `tracing` logs only.
- **Maturity**: Pre-1.0, active development.

---

## MCP Capability Inventory

| Tool/Resource | @galdor/memory-okf | OKC | Notes |
|---------------|-------------------|-----|-------|
| **MCP Server** | ❌ None | ✅ 11 tools | OKC only |
| **Search** | ✅ `okf_search` agent | ✅ `search` (FTS5/BM25) | Both BM25 |
| **Browse/Traverse** | ✅ `okf_browse` agent | ✅ `traverse` (BFS) | OKC: programmatic |
| **Bundle Writing** | ✅ Library API | ⚠️ Planned | galdor ahead |
| **Validation** | ✅ OKF v0.1 | ✅ `validate` | Parity |
| **Change Logs** | ✅ Built-in | ❌ | galdor unique |
| **Citations** | ✅ Built-in | ❌ | galdor unique |
| **Resources** | ❌ | ❌ Planned | Neither |
| **Prompts** | ❌ | ❌ Planned | Neither |

---

## Strengths vs OKC

1. **Code-aware BM25 tokenizer** — Optimized for code/technical content retrieval. OKC's FTS5 is general-purpose.

2. **Progressive-disclosure browsing** — Link graph with hierarchical disclosure for large knowledge bases. OKC's `traverse` is flat BFS.

3. **Built-in change logs & citations** — First-class support for provenance and attribution. OKC has neither.

4. **Bundle writing as library** — `writeBundle` is a first-class export. OKC's `ingest` is planned.

5. **Minimal dependencies** — Only `@galdor/core` + `zod`. Tiny footprint.

6. **galdor ecosystem integration** — Native for galdor agents (`okf_search`, `okf_browse`).

---

## Weaknesses vs OKC

1. **No MCP server** — Cannot be consumed by general AI agents (Claude, Cursor, etc.). OKC's 11 MCP tools over stdio/HTTP/SSE is the primary agent interface.

2. **galdor-locked** — Only usable within galdor-bun ecosystem. OKC is agent-agnostic via MCP.

3. **No persistent index** — In-memory BM25 index rebuilds on restart. OKC's SQLite+FTS5 survives restarts, incremental updates.

4. **No file watcher** — No live reload. OKC's `notify` + `observe` pushes updates in real-time.

5. **No lineage/history** — No concept evolution tracking. OKC's `lineage` tool.

6. **No metadata/faceted query** — Cannot filter by front-matter key/value. OKC's `query_metadata`.

7. **No CLI** — Library only. OKC has full `okc` CLI.

8. **OKF v0.1 only** — Behind OKC's v0.2 support.

9. **Single maintainer, no GitHub visibility** — Repo not linked in npm metadata. Bus factor = 1.

10. **No MCP resources/prompts** — Cannot expose resources or prompt templates.

---

## OKC Improvement Opportunities

| Area | Gap | Competitor Reference | Action |
|------|-----|---------------------|--------|
| **Code-aware BM25** | General FTS5 | galdor: code-aware tokenizer | Add `code` tokenizer option for Rust/TS/Go/Python |
| **Progressive disclosure** | Flat BFS | galdor: hierarchical browse | Add `max_depth` + `expand` to `traverse` |
| **Change logs** | None | galdor: built-in | Add `okc log` for index mutation audit trail |
| **Citations** | None | galdor: built-in | Add `citation` front-matter field + `okc cite` |
| **Bundle writing API** | Planned | galdor: library export | Implement `okc write` / `okc-lib` crate |

---

## Threat Level

**Low**

**Rationale:**
- **Ecosystem lock-in** — Only works with galdor-bun. Not a general OKF tool.
- **No MCP** — Zero agent-facing protocol outside galdor.
- **No persistence/index** — In-memory only. OKC's durable index is a major moat.
- **Single maintainer, no public repo visibility** — High bus factor risk.
- **OKF v0.1** — Behind OKC's v0.2.

**Complementary potential**: galdor agents could *consume* OKC-indexed catalogs via MCP if galdor adds MCP client support.

---

## Verdict

**@galdor/memory-okf is a specialized OKF backend for the galdor agent framework, not a standalone catalog competitor.** Its strengths (code-aware BM25, progressive disclosure, change logs, citations) are valuable patterns for agent-facing knowledge systems, but its galdor-lock-in and lack of MCP/persistence make it irrelevant for the broader OKC market.

**Strategic implication for OKC:** The code-aware tokenizer, progressive-disclosure browsing, change logs, and citations are **high-value patterns** OKC should adopt to improve agent-facing retrieval quality and provenance.

**Recommended priority:**
1. Add code-aware tokenizer option for BM25 — Q1
2. Add progressive-disclosure to `traverse` — Q1
3. Add change log / audit trail for index mutations — Q2
4. Add citation front-matter + `okc cite` — Q2
5. Implement `okc write` / `okc-lib` for bundle authoring — Q1

galdor validates that **agent-facing knowledge backends need code-aware retrieval, hierarchical browsing, and built-in provenance** — all areas where OKC can differentiate.