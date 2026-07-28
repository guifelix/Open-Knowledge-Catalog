# Wicked Knowledge — Competitor Assessment

## Overview

Wicked Knowledge was the third engine in the **wicked stack**, a four-engine local-first Rust ecosystem for AI coding agents built by [mikeparcewski](https://github.com/mikeparcewski). It provided an MCP stdio server (`wicked-knowledge-mcp`) for storing, relating, and recalling **curated, citable knowledge** with typed relations — distinct from the code graph (wicked-estate) and experiential memory (wicked-memory). Each engine owned its own isolated SQLite store; they never co-mingled data.

As of **wicked-estate v0.13.0**, Wicked Knowledge was **deprecated and absorbed** into the wicked-estate monorepo as the internal crate `wicked-estate-knowledge`. The standalone repository was archived. Its 7 MCP tools are now served through the unified `wicked-estate-mcp` server alongside the estate (10 tools) and memory (6 tools) tools — 23 total.

A parallel but separate project is **[knowledge-mcp by fulminate-io](https://github.com/fulminate-io/knowledge-mcp)**, an MCP-native knowledge graph with code/cloud/CI indexing, persistent reasoning, and team sync via Fulminate Cloud. This is a different product with a different author and architecture, though it occupies a similar conceptual space.

## Key Features

### Wicked Knowledge (standalone, now absorbed)

- **Curated knowledge nodes** with typed relations — knowledge is intentionally written, not automatically captured
- **Own SQLite store + FTS5** — isolated from code and memory stores, eliminating FTS dilution and writer contention
- **Single-writer design** — structural impossibility of `SQLITE_BUSY` via dedicated store per engine
- **Cross-store differentiators** — `knowledge.relate_code` and `knowledge.recall_about_code` link knowledge to code symbols and recall from a code seed
- **Reused recall pipeline** — knowledge recall reuses the `wicked-memory-core` pipeline (RRF fusion, budget packing) instead of reimplementing
- **7 MCP tools**: `knowledge.ingest`, `knowledge.write`, `knowledge.relate`, `knowledge.recall`, `knowledge.coverage`, `knowledge.relate_code`, `knowledge.recall_about_code`

### wicked-estate (the absorbing project, v0.13+)

- **Code + infrastructure estate graph** — 105 wired languages including legacy enterprise stacks (COBOL, JCL, RACF, IMS, MQ, VB6, RPG, Delphi, etc.)
- **23 MCP tools across 3 domains**: estate (10), memory (6), knowledge (7)
- **Portable storage** — SQLite local-first default, Postgres for shared team graphs with the `--db` flag
- **Hybrid retrieval** — graph + FTS5 core, with optional embedding sidecar (model2vec or fastembed)
- **Cross-domain joins** — single query links code symbols, infrastructure resources, and knowledge
- **Blast radius, lineage, community detection, context packing** — agent-facing retrieval primitives

### fulminate-io/knowledge-mcp (separate project)

- **Ten-graph architecture** — separate graphs for code, cloud, logs, decisions, findings, etc., all cross-linked
- **Persistent reasoning** — hypotheses as first-class graph nodes with weighted evidence charges and DeGroot propagation
- **Hybrid BM25 + vector search** — one query surface across all graphs
- **Structural AST search** — for patterns regex cannot express
- **Workflow integration** — brainstorm → ticket → plan → revise → implement, synced to Linear
- **Team sync** — local-first OSS with optional Fulminate Cloud for shared graphs
- **22 MCP tools** across ten graph families
- **Collectors** — for code (30+ languages via tree-sitter), cloud (AWS/GCP/Azure/K8s), logs (CloudWatch/Loki/Elasticsearch), web pages, and PDFs

## Architecture

### Wicked Knowledge / wicked-estate architecture

```
Engine isolation: Estate (code graph) | Memory (experiential) | Knowledge (curated)
                                      |
                      XedgeStore (cross-engine overlay)
```

Each engine is a separate Rust crate with its own SQLite database file and FTS5 index, connected through an **Overlay/XedgeStore** layer for cross-engine search. The architecture enforces:

- **Disjoint stores** — no store is ever shared between engines. FTS dilution and multi-writer `SQLITE_BUSY` are structurally impossible.
- **Single writer per store** — an in-process Mutex provides the advisory lock; a file lock / writer actor is the multi-process upgrade path.
- **Trait-based storage** — `GraphStore` trait with SQLite default and in-memory reference; Postgres lands behind the same seam.
- **RRF hybrid retrieval** — graph edges + FTS5 full-text with reciprocal-rank fusion for ranking; embeddings are an optional sidecar.
- **PageRank importance** — a dedicated ranker over CALLS/IMPORTS edges for hotspot detection.

The MCP server (`wicked-estate-mcp`) is a stdio JSON-RPC 2.0 server. It routes `tools/list` and `tools/call` via unified dispatch across all 23 tools.

### fulminate-io/knowledge-mcp architecture

```
Collectors → Graph per domain → Cross-linked nodes → MCP server (22 tools)
                            ↓
               Reasoning engine (thoughts/charges/propagation)
                            ↓
               Workflow engine (tickets/plans with Linear sync)
```

A daemon-based architecture: collectors run as drivers feeding domain-specific graphs, search and traverse act as syscalls, and persistent reasoning (thoughts, decisions, plans) forms the state layer. Supports local-only OSS mode or cloud-synced team mode via Fulminate Cloud with OAuth login.

## Comparison with OKF

| Dimension | Wicked Knowledge / Estate | OKF |
|---|---|---|
| **Knowledge representation** | Graph nodes + typed edges in SQLite; FTS5 for text; optional embeddings | Flat Markdown files with YAML frontmatter; knowledge bundles as plain files |
| **Storage approach** | Isolated SQLite stores per engine; Postgres option for teams | Git-versioned Markdown files on disk; no database dependency |
| **MCP integration** | First-class MCP server (23 tools); JSON-RPC 2.0 over stdio | MCP server planned but not primary; focus on file-based agent consumption |
| **Querying** | Graph traversal, FTS5, blast radius, PageRank, embeddings (optional) | File grep/search; OKF CLI for structured queries; relies on filesystem tools |
| **Portability / format** | SQLite binary format; locked to Rust toolchain for tooling | Plain Markdown — any editor, any git host, any language |
| **Agent-readiness** | Deep MCP integration; tools designed specifically for LLM consumption | File-based — agents read/write Markdown; MCP server in development |
| **Collaboration model** | SQLite = single-file, single-writer; Postgres = shared team graph; git not involved | Git-based — PRs, branching, merge conflicts, code review workflows |
| **Language/framework** | Rust (compiled static binary) | Python (CLI), Markdown (knowledge format) |
| **Setup complexity** | `cargo install`; indexing required; per-project database | `pip install`; no indexing; files are immediately usable |
| **Extensibility** | Plugin system for extractors; trait-based architecture | New knowledge types via YAML frontmatter; command-line composability |

## Strengths

1. **Engine isolation** — the decision to give knowledge its own SQLite store (and thus its own FTS index and single writer) is architecturally sound. No store can dilute or block another.

2. **Cross-engine linking** — `knowledge.relate_code` and `knowledge.recall_about_code` are genuinely novel features. Linking curated knowledge directly to code symbols in a queryable graph is something most knowledge tools do not attempt.

3. **Language coverage** — 105 languages with tree-sitter grammars, including obscure enterprise stacks. This is a moat for estate customers in large organizations.

4. **Rust performance** — single static binary, no runtime dependencies, millisecond-level queries. The performance story is excellent.

5. **Postgres option** — the ability to swap from local SQLite to a shared team Postgres backend with the same query interface is a pragmatic enterprise feature.

6. **MCP-first design** — the full stack was designed around MCP from day one, resulting in a coherent tool surface rather than an afterthought.

## Weaknesses

1. **Format lock-in** — knowledge is trapped in SQLite. There is no human-readable text format, no git-versionable artifact, no way to diff, review, or merge knowledge changes in a PR workflow. This is the single biggest architectural divergence from OKF.

2. **Rust dependency** — `cargo install` requires the Rust toolchain. While the binary is static, building it is not trivial for non-Rust developers.

3. **Complexity** — four engines, cross-store overlays, XedgeStore, trait-based storage backends. The architecture is elegant but heavy for teams that just want to manage knowledge as files.

4. **No collaboration model** — SQLite does not merge. The Postgres backend helps for shared access but does not provide branching, code review, or asynchronous collaboration. Git is entirely absent.

5. **Deprecation churn** — Wicked Knowledge was deprecated within months and absorbed into wicked-estate. The standalone repo was archived. This creates uncertainty about long-term stability of the knowledge-specific APIs within the monorepo.

6. **Single maintainer** — mikeparcewski is the sole author. Bus-factor risk is high for a project with this much architectural surface area.

7. **Niche scope** — wicked-estate is primarily a code intelligence tool for LLM agents (blast radius, callers, symbol search). Knowledge is a secondary feature within that scope, not the primary product.

8. **No knowledge-specific versioning** — no concept of knowledge entry history, provenance tracking, or change attribution beyond what the SQLite schema provides for edges.

## Lessons for OKC MCP Design

### Key Takeaways

**What wicked-knowledge got RIGHT:**

- **Engine isolation** (separate SQLite per engine) eliminates FTS dilution and writer contention
- **Trait-based storage** (`GraphStore` trait) enables backend swapping
- **RRF hybrid retrieval** (graph edges + FTS5) is a proven pattern
- **MCP-first design** from day one produces coherent tool surfaces

**What it got WRONG:**

- **Format lock-in to SQLite** (no human-readable, git-versionable format)
- **Complexity from 4-engine architecture**
- **No collaboration model** (SQLite doesn't merge, no git)
- **Single maintainer bus-factor risk**
- **Deprecation churn** (knowledge dep'd in months)

### Actionable Recommendations for OKC

1. **Keep OKF format as the portable, versionable, git-native format** — this is OKC's strongest architectural advantage
2. **Keep MCP as a first-class transport, not an afterthought** (OKC already has this right)
3. **Guard against single-maintainer risk**: good documentation, comprehensive test suite, CI, clear CONTRIBUTING.md
4. **Maintain focused scope** (one tool, one format) — avoid the multi-engine complexity trap
5. **Consider adding trait-based storage backends as a future-proofing pattern** (SQLite is fine now, but the trait seam is valuable)

### What NOT to Do

- Do NOT abandon human-readable formats for a database-only approach
- Do NOT add a second engine/knowledge store without a clear seam
- Do NOT become dependent on a single maintainer's availability
- Do NOT deprecate standalone APIs into a monorepo without migration docs

## Threat Level

**Low to Medium** — Wicked Knowledge / wicked-estate is not a direct competitor to OKF in its current form. It is a **code intelligence graph** that happens to include a curated knowledge storage feature. OKF's format-first, git-native, no-database approach is an opposite architectural philosophy.

The threat is higher in one dimension: wicked-estate has **deeper MCP integration** (23 tools designed for agent consumption) and a more mature agent-facing query interface. If a team prioritizes MCP-native agent tooling over human-readable formats and git-based collaboration, wicked-estate is the better choice.

However, for teams that want:
- Version-controlled knowledge
- Human-readable files
- PR-based collaboration
- No database dependencies
- Multi-language tooling (not just Rust)

OKF offers a fundamentally different and better-suited approach. The two projects serve different primary use cases (code intelligence vs. portable knowledge management).

### fulminate-io/knowledge-mcp threat level

**Medium** — the fulminate project is more directly competitive with OKF's knowledge management ambitions. Its ten-graph architecture, persistent reasoning engine, workflow integration (Linear sync), and team sync capabilities are compelling. However, it is even heavier (daemon-based, cloud dependency) and more complex. Its format is also proprietary and not human-readable. The project has only 1 GitHub star and appears to be in early stages, so execution risk is high.

## Notes

- The wicked stack is impressively engineered. The design notes in the repository show deep thought about failure modes, trait-based storage, and agent-facing API contracts.
- Wicked Knowledge was absorbed into wicked-estate as `wicked-estate-knowledge` — the crate is still published and maintained, just not as a standalone product.
- The architecture is Rust-only. Building `wicked-estate-mcp` requires the full Rust toolchain and takes significant time to compile due to tree-sitter grammar blobs.
- The project rejects vector databases as infrastructure dependencies — BM25 + graph edges + optional local embeddings is the philosophy.
- The "Karpathy pattern" (LLM as research librarian maintaining structured markdown) is acknowledged in wicked-brain (the Node.js bridge adapter) but is not how the core Rust tools work.
- fulminate-io/knowledge-mcp occupies adjacent space but is built by a different author (fulminate-io, not mikeparcewski) and is not part of the wicked stack.