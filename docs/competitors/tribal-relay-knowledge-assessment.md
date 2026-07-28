# Competitor Assessment: tribal + relay-knowledge

## Overview

**tribal** (crates.io: `tribal`, v0.0.1, 23 downloads, https://github.com/tribal-memory/tribal) — Knowledge graph MCP server for AI-native code intelligence. Stores engineering knowledge as typed items (Fact, Heuristic, Procedure, DecisionRecord) with typed relationships (Supports, Contradicts, Supersedes, DerivedFrom). Built on Postgres + pgvector. Runs as MCP server (stdio/HTTP/SSE) with OAuth/bearer auth. Emphasizes "semantic compression for project knowledge" — capturing reasoning behind decisions, not code artifacts.

**relay-knowledge** (crates.io: `relay-knowledge`, v1.1.12, 392 downloads, https://github.com/coolplayagent/relay-knowledge) — Local-first knowledge substrate for graph-backed retrieval. Hybrid GraphRAG with BM25, local semantic signatures, local hashed-vector ANN, optional external backends. SQLite storage (single or partitioned). Code repository indexing via tree-sitter (20+ languages). MCP Streamable HTTP + ACP adapter. Web diagnostics UI. ~217k LoC (Rust + TS/JS + Python).

Both are **Rust MCP servers** exposing knowledge graph capabilities to agents — directly overlapping with OKC's MCP server functionality.

---

## Graph Data Model Comparison

| Aspect | tribal | relay-knowledge | OKC | Notes |
|--------|--------|-----------------|-----|-------|
| **Core entity** | `KnowledgeItem` (Fact, Heuristic, Procedure, DecisionRecord) | `Evidence`, `Entity`, `Relation`, `Claim`, `Event` | `Document` (markdown file) + front-matter metadata | tribal: knowledge *about* work; relay: evidence + structured facts; OKC: documents as nodes |
| **Entity identity** | `KnowledgeItemId` (prefixed `ki_`) | `KnowledgeEntity` with labels + `source_scope` | File path (string) + `id` (integer) | tribal: UUID-like; relay: label-based deduplication; OKC: path-based |
| **Relation types** | 4 fixed: Supports, Contradicts, Supersedes, DerivedFrom | Open-ended typed relations (`relation_type: String`) with evidence refs | Single `links_to` / `linked_from` (markdown links) | tribal: closed vocabulary; relay: open vocabulary; OKC: implicit via links |
| **Properties on relations** | `justification` (optional string), `principal_id`, `created_at` | `confidence`, `status` (accepted/proposed), `version_range`, `evidence_ids[]` | None (just relation string) | relay richest; tribal minimal; OKC none |
| **Graph versioning** | `GraphVersion` per mutation batch; `RelationBatchId` groups | `GraphVersion` (u64) incremented per commit; `CommitReceipt` | None (implicit via file mtime/git) | tribal & relay have explicit versioning |
| **Confidence model** | 3-tier: Verified / Inferred / Uncertain (enum) | `ConfidenceScore` (basis points 0-10000) + tier | None (parse_status only) | tribal simpler; relay more granular |
| **Fact lifecycle** | Immutable items; contradictions via new relations | `FactStatus` (accepted/proposed); proposals + audit | File-level (modified/added/deleted) | tribal/relay: append-only; OKC: mutable files |
| **Scope/tenancy** | `ProjectId` (git remote), `PrincipalId` | `SourceScope` (repository alias, doc source, file index) | Single repository (roots config) | tribal: project-scoped; relay: multi-scope; OKC: single repo |
| **Embeddings** | pgvector (Postgres); per-profile geometry | Local deterministic (token sigs, hashed vectors) + optional external | None (BM25/FTS5 only) | tribal: cloud/local via pgvector; relay: local-first; OKC: none |
| **Code graph** | Not a focus (ingests text, not code) | Full tree-sitter indexing: symbols, refs, calls, imports, chunks, SBOM | None | relay strongest for code intelligence |

---

## MCP Server Capability Inventory

| Tool/Resource | tribal | relay-knowledge | OKC | Notes |
|---------------|--------|-----------------|-----|-------|
| **Knowledge ingest** | `tribal_ingest` (async job, returns job_id) | `ingest` (evidence + relations + claims + events, sync commit) | `scan` (index markdown files) | tribal: async pipeline; relay: sync commit; OKC: batch scan |
| **Semantic search** | `tribal_discover` (vector + filters) | `query` (hybrid: BM25 + semantic + vector + graph fallback) | `search` (FTS5/BM25 only) | relay most sophisticated; tribal vector-only; OKC BM25-only |
| **Graph traversal** | `tribal_explore` (inbound/outbound/both, depth≤3, typed relations) | `query` (hybrid), `repo query` (code graph), `graph inspect` | `traverse` (BFS, max_depth, max_nodes, relation filter) | tribal: typed relation filter; relay: multi-modal; OKC: link-graph only |
| **Item lookup** | `tribal_get_item` (by ID) | Implicit via query results | `get_document` (by path) | tribal: direct ID lookup |
| **Context packs** | — | `repo context` (bounded hybrid/definition/symbol + refs/callers/imports) | — | relay unique: agent-oriented context packs with budgets |
| **Feedback/quality** | `tribal_feedback` (session-level positive/negative) | `worker run-once`, `proposal accept` (human-in-loop) | `validate` (index issues) | tribal: retrieval quality; relay: fact proposals; OKC: structural |
| **Job/status** | `tribal_job_status` (ingest pipeline stages) | `worker status`, `repo status`, `index refresh` | `get_stats`, `validate` | tribal: async pipeline visibility |
| **Reindex/embeddings** | `tribal_reindex`, `tribal_reindex_cancel`, `tribal_reindex_prune` | `index refresh` (bm25/semantic/vector) | — | tribal: embedding geometry migration |
| **Code graph** | — | `repo query` (symbol, def, refs, callers, callees, imports, SBOM), `repo impact`, `repo software`, `repo-set` | — | relay: comprehensive code intelligence |
| **File indexing** | — | `files index`/`query` (SQLite FTS5, local roots) | `scan` (markdown only) | relay: generic file index |
| **Resources** | `tribal://session_context` | Service status, health, index status, Prometheus metrics | — | Both expose resources; OKC does not |
| **Prompts** | — | Retrieval & code-impact planning templates | — | relay: prompt templates for agents |
| **Auth/scopes** | OAuth/bearer; scopes: `tribal:write`, `tribal.knowledge:read`, `tribal.jobs:read`, `tribal.embedding:execute` | Scope policy via `RELAY_KNOWLEDGE_MCP_ALLOWED_SCOPES` | None | tribal more mature auth model |
| **Transports** | stdio, HTTP, SSE | Streamable HTTP (MCP), local ACP | stdio, HTTP/SSE | All three support stdio + HTTP |

---

## Query Capabilities

### tribal
- **`tribal_discover`**: Semantic vector search (pgvector) with optional filters (project, kind, tags, time range, include_superseded). Returns ranked items with optional `standing` (support/contradiction/observation counts).
- **`tribal_explore`**: Graph traversal from a known item ID. Directions: inbound (what supports/contradicts/supersedes this), outbound (what this supports/derived_from), both. Depth 1-3. Relation type filter.
- **`tribal_get_item`**: Direct ID lookup.
- **No**: Multi-hop reasoning, hybrid lexical+vector fusion, code-aware queries, context budgets, truncation diagnostics.

### relay-knowledge
- **Hybrid retrieval** (`query`): Reciprocal Rank Fusion over BM25, local semantic signatures, local hashed-vector ANN, optional external semantic/vector backends, graph evidence fallback, schema-guided paths, temporal events, community summaries, code graph documents. Deterministic local rerank before truncation.
- **Context packs** (`repo context`): Composed bounded queries (hybrid/definition/symbol + refs/callers/callees/imports/snippets) with budget, freshness, truncation diagnostics.
- **Code graph queries** (`repo query`): Kinds — `hybrid`, `symbol`, `definition`, `references`, `callers`, `callees`, `imports`, `sbom`. Resolution metadata (`target_hint`, `resolution_state`, confidence basis points/tier).
- **Graph inspection** (`graph inspect`): Node/edge counts, version, index cursor freshness.
- **Repository-set queries** (`repo-set query`): Cross-repo overlay via workspace detection (pnpm, Go workspaces, Cargo).
- **Impact analysis** (`repo impact`): Base vs head diff impact paths.

### OKC
- **`search`**: FTS5/BM25 full-text with path prefix, concept type, tags filters. Returns scored results with excerpts.
- **`query_metadata`**: Structured key-value filtering on front-matter with field projection.
- **`traverse`**: BFS link-graph traversal from a start path. Relation filter (empty = all), max_depth (default 3), max_nodes (default 50). Returns nodes (path, title, type, depth) and edges (source, target, relation).
- **`get_links`/`get_backlinks`**: Direct link resolution with existence check.
- **No**: Vector search, semantic search, typed relations, confidence, fact lifecycle, code graph, context packs, multi-hop reasoning.

### Comparison with OKC's Graph Traversal
| Feature | OKC `traverse` | tribal `explore` | relay `query`/`repo query` |
|---------|----------------|------------------|----------------------------|
| **Traversal algo** | BFS | Directional (in/out/both) | Hybrid RRF + graph fallback |
| **Depth control** | `max_depth` (≤config max) | `depth` (capped at 3) | Implicit via fusion/truncation |
| **Relation filter** | String list (link types) | Typed enum (Supports, Contradicts, etc.) | Kind-specific (symbol, callers, etc.) |
| **Node limit** | `max_nodes` | Implicit via depth cap | Budget-based (`max_context_bytes`) |
| **Edge semantics** | Single `links_to`/`linked_from` | 4 semantic types + justification | Rich: resolution_state, confidence, evidence |
| **Output** | Nodes + edges + truncated flag | Related items with standing | Context pack with provenance, diagnostics |

---

## Architecture & Code Quality

### tribal
- **Structure**: 16 crates in workspace (`tribal-*`), clean separation: domain, db, mcp, auth, config, inference, worker, agent-runtime, server, ui, telemetry.
- **Lines**: ~12k Rust LoC (per crates.io `linecounts`).
- **Database**: Postgres + pgvector + sqlx (compile-time checked queries). Migrations in `.sqlx/`.
- **Async**: Tokio. Worker pipeline: extraction → triage → relation (async jobs with status tracking).
- **Auth**: OAuth 2.1 + bearer tokens; scope-based authorization per tool.
- **Testing**: Unit + integration (`tribal-e2e` crate), golden snapshot tests for tool schemas.
- **Quality gates**: `deny.toml`, `clippy.toml`, `rustfmt.toml`, `cargo fmt --check`, `cargo clippy -D warnings`.
- **Observability**: OpenTelemetry (OTLP), structured tracing, `tracing` crate throughout.
- **Maturity**: v0.0.1, single version, 23 downloads — **very early**.

### relay-knowledge
- **Structure**: Single crate (`relay_knowledge`) with 18 modules under `src/relay_knowledge/`: `api`, `application`, `code`, `domain`, `env`, `evaluation`, `indexing`, `interfaces`, `model_provider`, `net`, `observability`, `paths`, `project`, `retrieval`, `storage`, `watcher`.
- **Lines**: ~217k LoC (Rust 210k, TS 4.5k, CSS 1.2k, Python 590, Shell 345).
- **Database**: SQLite (single or partitioned per-repo shards). WAL mode. Blocking pool for writes.
- **Async**: Tokio. Bounded worker pools for indexing, embedding, OCR, vision, extraction.
- **Architecture**: Clean layering — `domain` (pure), `storage` (traits + SQLite impl), `application` (use cases), `api` (CLI/MCP/HTTP/Web), `interfaces` (traits for external deps).
- **Code indexing**: tree-sitter grammars for 25+ languages. Incremental with checkpoints, leases, dead-letter recovery.
- **Quality gates**: `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test --all-targets --all-features`, `cargo llvm-cov --fail-under-lines 90`. Self-iteration harness for retrieval optimization.
- **Observability**: OTLP HTTP/protobuf traces + metrics. Prometheus `/mcp/metrics`. Web diagnostics UI.
- **Maturity**: v1.1.12, 23 versions since May 2026, 392 downloads — **actively developed, production-oriented**.

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

## Integration Patterns for Agent Consumption

| Pattern | tribal | relay-knowledge | OKC |
|---------|--------|-----------------|-----|
| **MCP stdio** | ✅ (bootstrap emits config) | ✅ (CLI binary) | ✅ (`okc serve --stdio`) |
| **MCP HTTP/SSE** | ✅ (Docker Compose, `--transport http`) | ✅ (`service run --web --mcp streamable-http`) | ✅ (`okc serve --http`) |
| **Agent skills** | ✅ (dedicated skills repo: `tribal-memory/skills`) | ✅ (ClawHub skill per release) | ❌ |
| **Auth for agents** | OAuth 2.1 dynamic registration; stdio = local principal | Scope allow-list + session binding | None |
| **Tool discovery** | `list_tools` filtered by scopes | `list_tools` filtered by `ALLOWED_SCOPES` | All tools visible |
| **Resource subscription** | Session context resource (subscribable) | Status, health, index status, metrics | None |
| **Prompt templates** | — | Retrieval & code-impact planning prompts | None |
| **Audit trail** | `tribal_feedback` (session quality) | Agent audit JSONL (`logs/agent-audit.jsonl`) | None |

---

## Strengths vs OKC

### tribal
1. **Semantic knowledge model** — Typed items (Fact/Heuristic/Procedure/DecisionRecord) with confidence and typed relations capture *engineering reasoning*, not just documents.
2. **Append-only fact lifecycle** — Contradictions/supersession via relations, not mutation. Enables temporal queries and audit.
3. **Async ingest pipeline** — Extraction → triage → relation stages with job status visibility. Handles LLM latency gracefully.
4. **Embedding geometry migration** — `reindex` tools swap embedding models atomically while serving reads.
5. **Agent-oriented auth** — OAuth 2.1 dynamic client registration; per-tool scopes.
6. **Skills ecosystem** — Reusable agent skills for install/verify/use/troubleshoot.
7. **Session context resource** — Subscribable `tribal://session_context` for multi-turn continuity.

### relay-knowledge
1. **Hybrid GraphRAG retrieval** — BM25 + local semantic + local vector + external backends + graph fallback + schema paths + temporal + community + code graph, fused via RRF with deterministic rerank.
2. **Code graph intelligence** — Tree-sitter indexing (25+ langs), symbols/refs/calls/imports/SBOM, impact analysis, cross-repo workspace overlays, feature-flag graphs.
3. **Context packs for agents** — Bounded composition with budgets, freshness, truncation diagnostics, provenance traces.
4. **Local-first, no external deps** — Deterministic semantic signatures, hashed vectors work offline. External backends optional.
5. **Partitioned SQLite sharding** — Per-repo shards for code index; control plane in main DB.
6. **Worker/Proposal/Audit system** — Durable background jobs, human-in-loop proposals, agent audit logging.
7. **Web diagnostics + MCP resources** — Health, index freshness, graph canvas, Prometheus metrics, operation composers.
8. **Self-iteration harness** — Automated retrieval optimization with progressive memory.
9. **ACP adapter** — Agent-Client Protocol support alongside MCP.

---

## Weaknesses vs OKC

### tribal
1. **No code intelligence** — Ingests text only; no symbol/reference/call graph.
2. **Postgres required** — Heavier operational dependency (pgvector extension).
3. **Early stage** — v0.0.1, minimal adoption, limited documentation beyond README.
4. **No local file indexing** — Requires git repo + bootstrap; no ad-hoc directory scan.
5. **No markdown/document retrieval** — Optimized for extracted knowledge, not document search.
6. **Single-vector geometry** — One active embedding profile at a time (reindex to swap).

### relay-knowledge
1. **Complexity** — 217k LoC, many moving parts (worker pools, partitioned storage, multiple protocols). Steep learning curve.
2. **SQLite write contention** — Single writer lane; partitioned shards mitigate but add operational complexity.
3. **No markdown/document-centric workflow** — Knowledge map (`.knowledge/knowledge-map.yaml`) is config, not primary content.
4. **Embedding model management** — Local deterministic only unless external backend configured; no pgvector-style similarity out of box.
5. **Resource heavy** — Web UI, OTLP, multiple worker kinds require more RAM/CPU.
6. **Overkill for simple use cases** — Designed for GraphRAG + code intelligence, not lightweight doc catalog.

---

## OKC Improvement Opportunities

| Area | Gap | Competitor Reference | Action |
|------|-----|---------------------|--------|
| **Semantic search** | BM25 only | tribal: pgvector; relay: local sigs + hashed vectors + external | Add optional vector index (sqlite-vec, or pgvector via feature flag). |
| **Typed relations** | Single `links_to` | tribal: 4 kinds; relay: open typed + evidence | Extend link model with `relation_type`, `confidence`, `evidence_refs`. |
| **Fact lifecycle** | Mutable files | tribal: immutable + relations; relay: accepted/proposed + audit | Add `fact_status` front-matter field; `validate` proposes contradictions. |
| **Code graph** | None | relay: full tree-sitter + symbols/refs/calls/imports/SBOM/impact | Integrate tree-sitter (via `tree-sitter` crate) for Rust/TS/Go/Python as MVP. |
| **Context packs** | Raw traverse output | relay: `repo context` with budget/truncation/provenance | Add `context_pack` tool composing search + traverse + metadata with token budget. |
| **Async ingest** | Sync `scan` | tribal: job pipeline; relay: bounded worker pool + leases | Make `scan` return job_id; add `job_status` tool; background indexing. |
| **Agent auth/scopes** | None | tribal: OAuth + scopes; relay: `ALLOWED_SCOPES` | Add scope config + per-tool authorization in MCP handler. |
| **Resources/prompts** | None | tribal: session resource; relay: status/health/metrics + prompt templates | Expose `okc://index/status`, `okc://graph/summary` resources; add retrieval prompt. |
| **Observability** | Logs only | Both: OTLP + Prometheus | Add `opentelemetry` + `prometheus` exporters; `/metrics` endpoint. |
| **Multi-repo** | Single roots | relay: `repo-set` overlay + workspace detection | Add `repo_set` config + cross-repo traverse. |
| **File indexing** | Markdown only | relay: generic FTS5 file index (authorized roots) | Extend scanner to index non-markdown (configurable extensions). |

---

## Verdict

**tribal** is the closest *conceptual* competitor — both target "AI-native code intelligence" via MCP, both use typed knowledge graphs with relations. But tribal is **pre-alpha** (v0.0.1, 23 downloads) and requires Postgres. Its strength is the **knowledge model** (Fact/Heuristic/Procedure/DecisionRecord + typed relations) and **agent-oriented UX** (skills, auth, session context). OKC should adopt its **typed relation vocabulary** and **session context resource** pattern.

**relay-knowledge** is the **technical heavyweight** — production-grade GraphRAG + code intelligence in a single binary. It exceeds OKC in retrieval sophistication, code graph depth, agent-facing context packs, and operational maturity (OTLP, partitioned SQLite, self-iteration). Its weakness is **complexity** and **lack of document-centric workflow**. OKC should adopt its **hybrid retrieval architecture** (RRF + deterministic rerank), **context pack pattern** (budgets + provenance), and **code graph integration strategy** (tree-sitter + incremental checkpoints).

**OKC's competitive position**: Lightweight, markdown-native, zero-config (SQLite + FTS5). Best for **documentation catalogs, personal knowledge bases, small-team wikis**. Not a GraphRAG platform or code intelligence engine. To remain relevant as agents adopt MCP, OKC must add: (1) optional vector search, (2) typed relations + fact lifecycle, (3) code graph MVP, (4) context packs with budgets, (5) agent auth/scopes, (6) MCP resources/prompts. These are achievable increments without relay's architectural weight.

**Recommended priority**:
1. Add `relation_type` + `confidence` to link model; emit typed relations from markdown (e.g., `[[supports:path]]`).
2. Integrate `sqlite-vec` (or feature-flag `pgvector`) for hybrid BM25+vector search.
3. Tree-sitter indexing for Rust/TS/Go/Python → `symbol`, `definition`, `references` MCP tools.
4. `context_pack` tool composing search + traverse + metadata with `max_tokens` budget.
5. Scope-based tool authorization + `okc://index/status` resource.
6. Background scan job + `job_status` tool.

These six steps close 80% of the agent-facing capability gap while preserving OKC's simplicity advantage.