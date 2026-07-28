# Competitor Assessment: okf-toolset (npm)

## Overview

**okf-toolset** is not a single npm package but rather a **collection of three independent tools** in the OKF ecosystem that together form a de facto toolset for working with OKF bundles:

1. **okft** (`poorvaj-ww/okft`) — Python tool for linting and serving OKF bundles with MCP server support (OKF v0.1)
2. **okf-lint** (`thisismydesign/okf-lint`) — TypeScript opinionated linter for OKF knowledge catalogs
3. **okflint** (`mattdav/okflint`) — Python deterministic compliance linter with semantic cohesion scoring (OKF v0.1)

There is **no single `okf-toolset` package on npm**. Users must install and combine these tools individually. This assessment evaluates the collective toolset as a functional unit for OKF bundle management.

---

## Feature Comparison with OKC

| Feature | okft | okf-lint | okflint | OKC | Notes |
|---------|------|----------|---------|-----|-------|
| **Language** | Python | TypeScript | Python | Rust | Polyglot vs unified |
| **License** | Unclear | Unclear | Unclear | MIT | OKC clear |
| **CLI** | ✅ `okft` | ✅ `okf-lint` | ✅ `okflint` | ✅ `okc` | All have CLI |
| **MCP Server** | ✅ `okft[serve]` | ❌ | ❌ (planned) | ✅ 11 tools | okft only |
| **Bundle Parsing** | ✅ OKF v0.1 | ✅ OKF v0.1 | ✅ OKF v0.1 | ✅ OKF v0.2 | OKC newer spec |
| **Validation** | ✅ Lint + hygiene | ✅ Spec + conventions | ✅ Manifest + semantic | ✅ `validate` | All validate |
| **Semantic Cohesion** | ❌ | ❌ | ✅ TF-IDF/cosine/clustering | ❌ | okflint unique |
| **Index Generation** | ❌ | ❌ | ✅ OKF §6 | ❌ | okflint unique |
| **Broken Link Detection** | ✅ | ❌ | ✅ | ✅ | okft + okflint |
| **Reserved File Validation** | ❌ | ❌ | ✅ | ❌ | okflint unique |
| **MCP Tools** | Navigation only | ❌ | ❌ | 11 tools | OKC richest |
| **Persistent Index** | ❌ | ❌ | ❌ | ✅ SQLite+FTS5 | OKC only |
| **File Watcher** | ❌ | ❌ | ❌ | ✅ `notify` | OKC only |
| **Graph Traversal** | ❌ | ❌ | ❌ | ✅ `traverse` | OKC only |
| **Lineage/History** | ❌ | ❌ | ❌ | ✅ `lineage` | OKC only |

---

## Architecture & Code Quality

### okft (`poorvaj-ww/okft`)
- **Language**: Python
- **Architecture**: CLI with `lint` and `serve` (MCP) subcommands. Deterministic graph traversal without embeddings/database.
- **MCP Server**: Exposes bundle navigation tools for AI agents.
- **Maturity**: Medium (27 code snippets, Medium reputation). OKF v0.1 only.

### okf-lint (`thisismydesign/okf-lint`)
- **Language**: TypeScript/JavaScript
- **Architecture**: Opinionated linter with stylish output. Checks spec violations + optional conventions.
- **Maturity**: High (35 code snippets, High reputation). Programmatic + CLI interfaces.

### okflint (`mattdav/okflint`)
- **Language**: Python
- **Architecture**: Manifest-based validation + semantic cohesion scoring (TF-IDF, cosine similarity, clustering). Index generation per OKF §6.
- **MCP**: Planned for chat-first applications.
- **Maturity**: Medium (110 code snippets, Medium reputation, Benchmark Score: 95).

### OKC
- **Language**: Rust
- **Architecture**: Single binary with SQLite+FTS5 index, file watcher, MCP server (stdio + HTTP/SSE).
- **Quality Gates**: `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test`.
- **Maturity**: Pre-1.0, active development.

---

## MCP Capability Inventory

| Tool/Resource | okft | okf-lint | okflint | OKC | Notes |
|---------------|------|----------|---------|-----|-------|
| **MCP Server** | ✅ `okft[serve]` | ❌ | ❌ (planned) | ✅ 11 tools | okft only current |
| **Search** | Navigation only | ❌ | ❌ | ✅ FTS5/BM25 | OKC richest |
| **Graph Traversal** | ❌ | ❌ | ❌ | ✅ `traverse` | OKC only |
| **Validation** | ✅ Lint | ✅ Lint | ✅ Manifest+semantic | ✅ `validate` | All validate |
| **Bundle Writing** | ❌ | ❌ | ❌ | ⚠️ Planned | None write |
| **Resources** | ❌ | ❌ | ❌ | ❌ Planned | None |
| **Prompts** | ❌ | ❌ | ❌ | ❌ Planned | None |

---

## Strengths vs OKC

1. **Specialized validation depth** — okf-lint (conventions), okflint (semantic cohesion, manifest validation, index generation) go deeper on validation than OKC's `validate`.

2. **Semantic cohesion scoring** — okflint's TF-IDF/cosine/clustering analysis for document coherence is unique. OKC has no equivalent.

3. **Manifest-based validation** — okflint's manifest-driven approach ensures reproducible, deterministic validation. OKC uses implicit front-matter conventions.

4. **Index generation (OKF §6)** — okflint generates formal indexes per spec. OKC has no index export.

5. **Reserved file validation** — okflint checks for reserved filenames. OKC does not.

5. **MCP server (okft)** — okft provides basic MCP navigation tools. OKC has richer MCP but okft proves the pattern.

6. **Language diversity** — Python + TypeScript options for teams can pick. OKC is Rust-only.

---

## Weaknesses vs OKC

1. **No unified package** — Must install/configure 3 separate tools. OKC is single binary.

2. **No persistent index** — All tools re-parse on every run. OKC's SQLite+FTS5 survives restarts, incremental updates.

3. **No file watcher / live updates** — Must re-run manually. OKC's `notify` + `observe` pushes real-time updates.

4. **No graph traversal** — None expose `traverse`/BFS for agents. OKC's `traverse` supports depth/node limits, relation filters.

5. **No lineage/history** — No concept evolution tracking. OKC's `lineage` tool.

6. **No metadata/faceted query** — Cannot filter by front-matter key/value. OKC's `query_metadata`.

6. **Limited MCP** — Only okft has MCP (basic navigation). okf-lint/okflint have none. OKC has 11 tools over stdio+HTTP/SSE.

7. **OKF v0.1 only** — Behind OKC's v0.2 support.

8. **Fragmented maintenance** — 3 separate repos, 3 maintainers, 2 languages. OKC: single codebase.

9. **No bundle writing** — None create/modify bundles. OKC's `ingest` planned.

10. **No MCP resources/prompts** — Cannot expose resources or prompt templates.

---

## OKC Improvement Opportunities

| Area | Gap | Toolset Reference | Action |
|------|-----|-------------------|--------|
| **Semantic cohesion** | None | okflint: TF-IDF/cosine/clustering | Add `okc cohesion` for document coherence scoring |
| **Manifest validation** | Implicit conventions | okflint: manifest-driven | Add `okc validate --manifest` for reproducible checks |
| **Index generation** | None | okflint: OKF §6 index | Add `okc index --export` for formal index export |
| **Reserved file check** | None | okflint: reserved files | Add `okc validate --reserved` |
| **Opinionated conventions** | Basic validate | okf-lint: stylish conventions | Add `--conventions` flag for opinionated checks |
| **MCP server** | ✅ Has | okft: basic nav | OKC already richer; maintain lead |
| **Unified binary** | ✅ Single | 3 separate tools | OKC advantage; maintain |

---

## Threat Level

**Low (as unified toolset)**

**Rationale:**
- **Fragmentation** — 3 separate tools, 2 languages, 3 maintainers. High friction vs OKC's single binary.
- **No persistence/live updates** — Fundamental architectural gap vs OKC's SQLite+FTS5+file watcher.
- **No graph traversal/lineage** — Core agent-facing capabilities missing.
- **MCP only on okft** — 2/3 tools have zero agent interface.
- **OKF v0.1** — Behind OKC's v0.2.

**Complementary potential**: OKC could *integrate* okflint's semantic cohesion and manifest validation as optional validation gates. okft's MCP navigation pattern validates OKC's MCP approach.

---

## Verdict

**The "okf-toolset" is a loose collection of specialized validators, not a unified competitor.** Each tool excels at a specific validation niche (linting, semantic cohesion, manifest compliance, MCP navigation), but together they lack the unified catalog runtime that OKC provides: persistent indexing, live file watching, graph traversal, lineage, faceted query, and a rich MCP server.

**Strategic implication for OKC:** The toolset validates that **validation depth matters** — OKC should adopt okflint's semantic cohesion scoring, manifest-driven validation, index generation, and reserved-file checks as optional validation gates. okft's MCP navigation confirms the agent-facing pattern OKC already leads on.

**Recommended priority:**
1. Add semantic cohesion scoring (`okc cohesion`) — Q1
2. Add manifest-driven validation mode (`okc validate --manifest`) — Q1
3. Add index export (`okc index --export`) — Q2
4. Add reserved file check (`okc validate --reserved`) — Q2
5. Add opinionated conventions flag (`okc validate --conventions`) — Q2

The toolset is a **validation layer**, not a **catalog runtime**. OKC remains the only unified catalog + MCP server in the OKF ecosystem.