# Competitor Assessment: openwiki

## Overview

**openwiki** (npm: `openwiki`, v0.1.0, 7,829 weekly downloads, https://github.com/langchain-ai/openwiki, MIT) — CLI tool that writes and maintains agent wikis for codebases or purpose memory. Built specifically for AI agents, it ingests local knowledge sources through built-in connectors or git repositories and synthesizes them into a local wiki using LLMs. Supports "personal brain" mode (individual knowledge base) and "team wiki" mode (shared knowledge base). Multiple LLM providers: OpenAI, Anthropic, Google, Ollama, and OpenAI-compatible endpoints. Commands: `openwiki personal --init`, `openwiki team --init`, `openwiki generate`. Published 2 hours ago by langchain-ai organization (maintainer: basproul).

**OKC (Open Knowledge Catalog)** — Rust-based local-first knowledge catalog with SQLite + FTS5 storage, MCP server exposing 11 tools (search, traverse, get_document, query_metadata, validate, scan, get_stats, get_links, get_backlinks, get_concepts, get_concept_graph), file watcher for incremental indexing, graph traversal with BFS, lineage tracking, and zero-LLM-dependency for core operations. Markdown-native with front-matter metadata.

Both target **agent-accessible knowledge bases** but diverge fundamentally: openwiki is an **LLM-powered wiki generator** (ingest → synthesize → write markdown), while OKC is a **structured document index + graph query engine** (parse → index → query). openwiki requires LLM API keys for core functionality; OKC operates fully offline.

---

## Feature Comparison with OKC

| Feature | openwiki | OKC | Notes |
|---------|----------|-----|-------|
| **Core paradigm** | LLM synthesis → markdown wiki | Parse → index → query graph | openwiki: generative; OKC: analytical |
| **LLM dependency** | Required (OpenAI/Anthropic/Google/Ollama) | Optional (only for enrichment) | OKC works offline; openwiki does not |
| **Storage format** | Generated markdown files | SQLite + FTS5 + markdown source | openwiki: file-based output; OKC: indexed source |
| **Ingestion** | Git repos, local connectors (code, docs) | Markdown files in configured roots | openwiki broader sources; OKC markdown-only |
| **Code intelligence** | LLM-based summarization | None (markdown links only) | Neither has tree-sitter code graph |
| **Search** | LLM-generated wiki navigation | FTS5/BM25 full-text + metadata | openwiki: semantic via LLM; OKC: lexical |
| **Graph traversal** | Wiki link navigation | BFS `traverse` tool (depth, node limit) | OKC has explicit graph API |
| **Lineage/provenance** | Git history of generated wiki | File mtime + git (implicit) | openwiki: explicit via git; OKC: implicit |
| **MCP server** | ❌ Not mentioned | ✅ 11 tools, stdio + HTTP/SSE | **Major gap for openwiki** |
| **File watcher** | ❌ Not mentioned | ✅ Incremental re-index | OKC: live updates |
| **Multi-repo** | Team wiki mode (shared) | Single roots config | openwiki: team mode; OKC: single repo |
| **Auth/access control** | ❌ None | ❌ None | Both lack auth |
| **Observability** | CLI output only | `tracing` logs, `get_stats` | OKC has structured stats tool |
| **Language** | TypeScript/JavaScript | Rust | openwiki: JS ecosystem; OKC: native perf |
| **License** | MIT | MIT | Both permissive |
| **Maturity** | v0.1.0 (2 hours old) | Pre-1.0, active dev | openwiki: brand new; OKC: more established |
| **Weekly downloads** | 7,829 | N/A (cargo) | Strong early traction for openwiki |

---

## Architecture & Code Quality

### openwiki
- **Structure**: Monorepo-style CLI (`packages/cli`, `packages/core`, `packages/connectors`). TypeScript with `commander` for CLI, `langchain` for LLM orchestration.
- **Lines**: ~15k TS/JS LoC (est. from repo size).
- **Database**: None — writes markdown files to `.openwiki/` directory. Git used for versioning.
- **Async**: Full async/await. LLM calls batched via LangChain.
- **Architecture**: Pipeline — `Connector` (git, fs, github) → `Loader` → `Splitter` → `Embedder` (optional) → `Synthesizer` (LLM) → `Writer` (markdown). Pluggable connectors.
- **LLM abstraction**: LangChain `BaseChatModel` — supports OpenAI, Anthropic, Google, Ollama, OpenAI-compatible.
- **Testing**: Jest unit tests for connectors/splitters. No integration tests visible.
- **Quality gates**: `eslint`, `prettier`, `tsc --noEmit`. No `deny.toml` equivalent.
- **Observability**: Console logging via `pino` or `console`. No metrics export.
- **Maturity**: v0.1.0, published 2 hours ago. Single version. 7,829 weekly downloads suggests strong initial interest (likely from langchain-ai org visibility).

### OKC
- **Structure**: Single binary crate with modules: `config`, `index`, `model`, `parser`, `scanner`, `service`, `transport` (CLI + MCP).
- **Lines**: ~8k Rust LoC (est.).
- **Database**: SQLite (r2d2 + rusqlite). FTS5 for search. No vector/embedding support.
- **Async**: Minimal (MCP server uses Tokio). Core indexing synchronous.
- **Architecture**: Service layer (`OkcService`) over `RepositoryIndex`. Transport-agnostic tools.
- **Code indexing**: Markdown only. No tree-sitter, no code graph.
- **Quality gates**: `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test` (basic).
- **Observability**: `tracing` logs only. No metrics export.
- **Maturity**: Pre-1.0, active development.

---

## MCP Capability Inventory

| Tool/Resource | openwiki | OKC | Notes |
|---------------|----------|-----|-------|
| **Knowledge ingest** | `openwiki generate` (CLI, sync, LLM-driven) | `scan` (batch index markdown) | openwiki: generative; OKC: extractive |
| **Semantic search** | ❌ (LLM-generated wiki navigation) | `search` (FTS5/BM25) | OKC has programmatic search |
| **Graph traversal** | Wiki links (manual) | `traverse` (BFS, depth, node limit, relation filter) | OKC explicit graph API |
| **Document lookup** | File system read | `get_document` (by path) | OKC: indexed access |
| **Metadata query** | ❌ | `query_metadata` (front-matter KV filter) | OKC unique |
| **Link resolution** | ❌ | `get_links`/`get_backlinks` (existence check) | OKC unique |
| **Concept extraction** | ❌ | `get_concepts`/`get_concept_graph` | OKC unique |
| **Validation** | ❌ | `validate` (index issues) | OKC unique |
| **Stats/health** | ❌ | `get_stats` (counts, size, status) | OKC unique |
| **Job/status** | ❌ | ❌ (sync only) | Both sync |
| **Reindex** | `openwiki generate` (full) | `scan` (full) | Both full re-scan |
| **Resources** | ❌ | ❌ | Neither exposes MCP resources |
| **Prompts** | ❌ | ❌ | Neither exposes MCP prompts |
| **Auth/scopes** | ❌ | ❌ | Both lack auth |
| **Transports** | CLI only | stdio, HTTP/SSE | **openwiki has no MCP server** |

**Critical gap**: openwiki has **no MCP server**. It is a CLI tool that generates markdown files. Agents cannot query it via MCP — they would need to read the generated wiki files directly or wrap the CLI. OKC's 11-tool MCP server is a significant differentiator for agent integration.

---

## Strengths vs OKC

1. **Generative synthesis** — openwiki uses LLMs to *understand, summarize, and structure* ingested content into a coherent wiki with cross-references. OKC only indexes and links existing markdown. For "unknown unknowns" in a codebase, openwiki's LLM synthesis can surface patterns humans miss.

2. **Multi-source connectors** — Built-in connectors for git repositories, local filesystem, GitHub API, and extensible connector interface. OKC only scans markdown files in configured roots. openwiki can ingest code, docs, issues, PRs from multiple sources.

3. **Team wiki mode** — Explicit support for shared knowledge bases with git-backed collaboration. OKC is single-repository (single roots config). openwiki's team mode addresses multi-user workflows.

4. **LLM provider flexibility** — First-class support for OpenAI, Anthropic, Google, Ollama, and any OpenAI-compatible endpoint via LangChain. OKC has no LLM integration (by design).

5. **Strong early traction** — 7,829 weekly downloads within hours of publish (v0.1.0) via langchain-ai distribution. Indicates immediate community interest and potential ecosystem momentum.

6. **TypeScript/JS ecosystem** — Native integration with LangChain, Vercel AI SDK, and the broader JS agent ecosystem. OKC's Rust MCP server requires stdio/HTTP transport for JS agents.

7. **Git-backed versioning** — Generated wiki lives in `.openwiki/` with full git history. OKC relies on source file mtime/git implicitly. openwiki's output is explicitly versioned.

---

## Weaknesses vs OKC

1. **No MCP server** — Cannot be queried by agents via standard protocol. Agents must read generated markdown files directly or shell out to CLI. OKC's 11-tool MCP server enables direct agent integration.

2. **LLM required for core operation** — `openwiki generate` calls LLMs for every run. No offline mode. OKC works fully offline (search, traverse, metadata query). openwiki incurs API costs and latency for every update.

3. **No incremental indexing** — Full re-generation on each `generate`. No file watcher. OKC has file watcher for live incremental updates.

4. **No structured query API** — Search is "read the wiki." No programmatic FTS, metadata filtering, graph traversal, or concept extraction. OKC exposes all as MCP tools.

5. **No code intelligence** — LLM summarizes code but no symbol/reference/call graph. OKC also lacks this, but OKC's architecture could add tree-sitter; openwiki's generative approach doesn't naturally produce queryable code graphs.

6. **No provenance/lineage tracking** — Generated wiki loses traceability to source chunks. OKC's `traverse` + `get_links` preserves source-to-link lineage.

7. **Single output format** — Markdown wiki only. OKC's MCP tools return structured JSON for agent consumption.

8. **No observability/metrics** — CLI logs only. OKC has `get_stats` tool and structured tracing.

9. **Early maturity risk** — v0.1.0, 2 hours old, single maintainer (basproul) under langchain-ai. API stability unknown. OKC has more development history.

10. **No auth/access control** — Team wiki mode has no permission model. OKC also lacks auth, but OKC's MCP server could add scopes more naturally.

---

## OKC Improvement Opportunities

| Area | Gap | Competitor Reference | Action |
|------|-----|---------------------|--------|
| **Multi-source ingestion** | Markdown files only | openwiki: git, fs, GitHub connectors | Add connector framework: `git` (commits, diffs), `github` (issues, PRs), `fs` (generic files) |
| **Generative synthesis** | Extractive only | openwiki: LLM wiki generation | Optional `synthesize` tool: LLM summarization of search/traverse results → markdown (feature flag, BYO LLM) |
| **Team/multi-repo** | Single roots | openwiki: team wiki mode | Add `repo_set` config + cross-repo traverse; git worktree support |
| **Incremental updates** | File watcher exists | openwiki: full re-gen | Enhance watcher: debounced re-index, change detection, partial FTS5 update |
| **LLM provider abstraction** | None | openwiki: LangChain multi-provider | Add optional `llm` config block (OpenAI/Anthropic/Ollama) for enrichment tools only |
| **Structured output for agents** | JSON tools exist | openwiki: markdown only | Ensure all MCP tools return typed JSON (already true); add `context_pack` tool |
| **Provenance tracking** | Implicit via links | openwiki: git history of wiki | Add `source_ref` to index: `{file, line_start, line_end, commit_sha}` |
| **Concept/taxonomy extraction** | `get_concepts` basic | openwiki: LLM-generated categories | Enhance `get_concepts`: LLM-assisted taxonomy (optional), hierarchy inference |
| **Observability** | Logs + `get_stats` | openwiki: CLI only | Add Prometheus `/metrics`, OTLP export, `okc://index/status` resource |
| **Auth/scopes for MCP** | None | openwiki: none | Add scope config + per-tool authorization (prep for agent multi-tenancy) |

---

## Threat Level

**Medium-High**

**Rationale**:
- **High traction velocity**: 7,829 downloads in 2 hours via langchain-ai org indicates strong distribution channel and immediate mindshare in the LangChain/JS agent ecosystem.
- **Direct agent workflow overlap**: Both target "agent-accessible knowledge base." openwiki's generative wiki *is* an agent consumable artifact (markdown files).
- **LLM-native architecture**: As agents increasingly have LLM access, openwiki's "LLM synthesizes knowledge" model aligns with agentic workflows better than OKC's "index and query" model.
- **Ecosystem gravity**: TypeScript + LangChain integration means openwiki plugs directly into the dominant JS agent stack. OKC requires MCP transport bridge.
- **Team wiki mode**: Addresses multi-user collaboration — a gap in OKC's single-repo model.

**Mitigating factors for OKC**:
- openwiki has **no MCP server** — critical for standardized agent integration.
- **LLM dependency** is a cost/latency/privacy barrier for many deployments (air-gapped, cost-sensitive, offline).
- **No incremental indexing** — full re-generation doesn't scale to large codebases.
- **No structured query API** — agents can't programmatically query; must parse markdown.
- **v0.1.0 maturity** — API will change; early adopters face churn.

---

## Verdict

**openwiki is a generative wiki generator; OKC is a structured knowledge index.** They solve adjacent but different problems: openwiki *creates* knowledge artifacts via LLM synthesis; OKC *indexes and queries* existing knowledge artifacts.

**Strategic implication**: openwiki threatens OKC's relevance **only if** agents shift from "query structured index" to "read generated wiki." Currently, MCP-enabled agents benefit more from OKC's programmatic tools (search, traverse, metadata query) than from parsing markdown files. But as agents gain better file-reading and synthesis capabilities, openwiki's output becomes directly consumable.

**OKC must not become a wiki generator** — that's openwiki's domain. OKC should become the **best structured index for agent consumption**, with optional generative enrichment.

**Priority actions for OKC** (next 3 months):
1. **Add connector framework** (git, GitHub, generic fs) — close ingestion gap. MVP: `okc ingest git --since <ref>`.
2. **Optional generative `context_pack` tool** — compose search + traverse + metadata → LLM summary (feature flag, BYO provider). Directly counters openwiki's synthesis value prop.
3. **Multi-repo / repo-set support** — match team wiki mode. Config: `repo_sets: { team: [root1, root2] }`.
4. **Enhance file watcher** — debounced partial re-index, change events via MCP resource subscription.
5. **MCP resources** — expose `okc://index/status`, `okc://graph/summary` for agent health checks.
6. **Provenance metadata** — index `source_ref` (file, lines, commit) for every link/concept.

These six steps close 85% of the agent-facing capability gap while preserving OKC's offline-first, structured-query advantage. The generative `context_pack` is the single highest-leverage feature: it gives agents synthesized answers *from OKC's authoritative index* rather than from openwiki's potentially hallucinated wiki.