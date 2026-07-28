# Competitor Assessment: okf-tools (npm)

## Overview

**okf-tools** (npm: `okf-tools`, v0.0.1, MIT, https://github.com/okfbrain/okf-tools) — A TypeScript CLI and library for managing OKF knowledge bundles, part of the "okf-brain" company brain ecosystem. Published 3 weeks ago by okfbrain (castelli@okfbrain.com). 1.1 kB unpacked. Zero dependencies. CLI entry point: `okf-tools`.

This is an **early-stage, company-backed tool** focused on knowledge bundle management within the okf-brain ecosystem. It provides config-driven, zero-assumption tooling for OKF bundles with MCP integration as a stated goal.

---

## Feature Comparison with OKC

| Feature | okf-tools | OKC | Notes |
|---------|-----------|-----|-------|
| **Language** | TypeScript | Rust | Different runtime |
| **License** | MIT | MIT | Aligned |
| **CLI** | ✅ `okf-tools` | ✅ `okc` | Both have CLI |
| **MCP Server** | ⚠️ Keyword only | ✅ 11 tools, stdio + HTTP/SSE | OKC only |
| **Bundle Management** | ✅ Core | ✅ Core | Parity |
| **Persistent Index** | ❌ None | ✅ SQLite + FTS5 | OKC only |
| **File Watcher** | ❌ None | ✅ `notify` + `observe` | OKC only |
| **Search** | ❌ None | ✅ FTS5/BM25 | OKC only |
| **Graph Traversal** | ❌ None | ✅ `traverse` | OKC only |
| **Lineage/History** | ❌ None | ✅ `lineage` | OKC only |
| **MCP Resources** | ❌ None | ❌ Planned | Neither |
| **MCP Prompts** | ❌ None | ❌ Planned | Neither |
| **Company Ecosystem** | ✅ okf-brain | ❌ Independent | okf-tools integrated |

---

## Architecture & Code Quality

### okf-tools
- **Structure**: Single package, CLI + library exports. ~1.1 kB unpacked (very small).
- **Dependencies**: Zero — minimal footprint.
- **Architecture**: Config-driven, no assumptions about repository layout. Designed for okf-brain integration.
- **Testing**: Not visible in npm metadata.
- **Quality Gates**: No visible CI/lint config in package.
- **Documentation**: Minimal npm README. No architecture docs.
- **Maturity**: v0.0.1, 3 weeks old, corporate-backed (okfbrain).

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

| Tool/Resource | okf-tools | OKC | Notes |
|---------------|-----------|-----|-------|
| **MCP Server** | ⚠️ Keyword only | ✅ 11 tools, stdio + HTTP/SSE | OKC only |
| **Search** | ❌ | ✅ `search` (FTS5/BM25) | OKC only |
| **Graph Traverse** | ❌ | ✅ `traverse` (BFS, filters) | OKC only |
| **Lineage** | ❌ | ✅ `lineage` | OKC only |
| **File Watch** | ❌ | ✅ `observe` + fs watcher | OKC only |
| **Bundle Management** | ✅ CLI | ✅ `scan`, `ingest` | Both |
| **Validation** | ❌ | ✅ `validate` | OKC only |

---

## Strengths vs OKC

1. **Zero dependencies** — Minimal footprint, fast install, low supply-chain risk.
2. **Config-driven, no assumptions** — Flexible for any repository layout; okf-brain integration.
3. **Company backing** — okfbrain provides dedicated development resources.
4. **MIT license** — Permissive, aligned with OKC.
4. **TypeScript** — Accessible to large JS/TS developer pool.

---

## Weaknesses vs OKC

1. **v0.0.1 — extremely early** — Pre-alpha, APIs unstable, minimal testing visible.
2. **No persistent index** — Re-parses bundles on every run. OKC's SQLite+FTS5 survives restarts, incremental updates.
3. **No file watcher** — No live reload. OKC's `notify` + `observe` pushes updates in real-time.
4. **No search** — No FTS, no BM25, no vector. OKC has hybrid BM25 + vector (planned).
5. **No graph traversal** — No `traverse`, `get_links`, `get_backlinks`. OKC's `traverse` supports BFS with depth/node limits.
6. **No lineage/history** — No concept evolution tracking. OKC's `lineage` tool.
7. **No MCP server** — Only a keyword. OKC has 11 MCP tools over stdio + HTTP/SSE.
8. **No MCP resources/prompts** — Cannot expose resources or prompt templates.
9. **Corporate ecosystem lock-in** — Designed for okf-brain; may not prioritize general OKF community needs.
10. **Minimal documentation** — npm README only. No architecture guides, no examples.

---

## OKC Improvement Opportunities

| Area | Gap | Competitor Reference | Action |
|------|-----|---------------------|--------|
| **Config-driven flexibility** | Fixed root config | okf-tools: no layout assumptions | Add `okc config init --flexible` for arbitrary layouts |
| **Zero-dep install** | Requires Rust toolchain | okf-tools: `npm i -g okf-tools` | Publish prebuilt binaries + Homebrew tap + npm wrapper |
| **Corporate integration** | Independent | okf-tools: okf-brain | Add `okc enterprise` features (SSO, audit, RBAC) |

---

## Threat Level

**Low**

**Rationale:**
- v0.0.1 with minimal features — not a functional competitor today.
- Corporate focus (okf-brain) limits community adoption.
- No MCP server, no search, no persistent index, no graph traversal.
- OKC's unified binary (lint + index + search + traverse + MCP) is a major integration advantage.
- Only threat is if okfbrain invests heavily and builds out full feature parity — monitor version progression.

---

## Verdict

**okf-tools is an early-stage, company-internal toolkit, not a general-purpose OKF competitor.** Its value is within the okf-brain ecosystem. For the broader OKF community, OKC's unified binary with persistent indexing, live file watching, hybrid search, graph traversal, lineage, and 11 MCP tools is a far more complete solution.

**Strategic implication for OKC:** The config-driven, zero-assumption philosophy is worth adopting for flexibility. Publishing prebuilt binaries (via `cargo dist` or similar) would match okf-tools' install simplicity. Corporate features (SSO, audit, RBAC) could be a future differentiator if okf-brain targets enterprise.

**Recommended priority:**
1. Publish prebuilt binaries + Homebrew tap + npm wrapper — Q1
2. Add flexible config for arbitrary repo layouts — Q1
3. Monitor okf-tools version progression — ongoing