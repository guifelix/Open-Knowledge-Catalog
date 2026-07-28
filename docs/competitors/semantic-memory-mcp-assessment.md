# Competitor Assessment: semantic-memory-mcp (RecursiveIntell/semantic-memory-mcp)

## Overview

**Project:** RecursiveIntell/semantic-memory-mcp  
**Repository:** https://github.com/RecursiveIntell/semantic-memory-mcp  
**NPM Package:** @recursiveintell/semantic-memory-mcp (v1.1.0, published 7 days ago)  
**License:** Apache-2.0  
**Language:** Rust (core), TypeScript (npm wrapper)  
**Architecture:** Local-first MCP server wrapping the `semantic-memory` Rust library (v0.5.10)  
**Installation:** `npx -y @recursiveintell/semantic-memory-mcp`, `cargo install semantic-memory-mcp`, or Docker  

**Description:** A local-first Model Context Protocol server providing persistent semantic search with evidence-scored retrieval, contradiction detection, adaptive routing, bitemporal truth, governed authority decisions, and a claim-ledger trust layer. Designed for AI agents to build and query persistent knowledge bases during coding sessions.

---

## Knowledge Model Comparison

| Dimension | semantic-memory-mcp | OKC | Notes |
|-----------|---------------------|-----|-------|
| **Memory/knowledge model** | Hybrid: Facts (authoritative SQLite), Documents/chunks, Conversations, Episodes, Entities, Graph edges (4 types), Claims with evidence bundles | OKF bundles: Concepts with YAML frontmatter + Markdown body, linked via wikilinks/references, typed relationships | SM: rich multi-layer model with provenance, temporal, and graph dimensions. OKC: simpler concept-graph with human-readable Markdown |
| **Evidence scoring** | **Core feature.** Six trust states (supported, partially_supported, unsupported, contradicted, heuristic_only, persisted_unjudged). Proof-debt budget gating via `sm_search_proof_debt`. Claim judgments (supported/unsupported/contested/heuristic_only) with risk-class verification (low→critical) | Not present. OKC relies on source citations in frontmatter and human verification | SM has formal evidence model; OKC has informal attribution |
| **Confidence ratings** | Explicit 0.0–1.0 confidence per fact via `sm_set_provenance` with support count. Bitemporal validity (valid_time / recorded_time). Provenance semirings (Boolean, Tropical, Probability, Confidence) | No native confidence scoring. Frontmatter supports arbitrary fields but no standardized confidence schema | SM: quantitative, multi-semiring provenance. OKC: free-form |
| **Source attribution** | Mandatory. Facts require source spans. Evidence bundles attached to claims. Search receipts capture backend, exactness, candidates, fallback, degradations. Replay mode stores inputs | Via `source` / `references` frontmatter fields. No enforced structure or verification | SM: structured, machine-verifiable. OKC: human-readable, optional |
| **Query by relevance** | Hybrid BM25 + vector (RRF fusion), sparse, late-interaction proxy, Matryoshka coarse/full rerank. `sm_search`, `sm_search_witnessed`, `sm_search_explained`, `sm_search_with_routing` | Semantic search via embeddings (planned), wikilink graph traversal, full-text via OKF CLI | SM: production hybrid pipeline with multiple fusion strategies. OKC: simpler vector + graph |
| **Query by recency** | Explicit temporal weight with configurable half-life. `sm_search_as_of` for bitemporal point-in-time queries. Recency behavior disabled by default in cache | Via `date_modified` / `date_created` frontmatter. No bitemporal semantics | SM: first-class bitemporal model. OKC: single timestamp |
| **Query by confidence** | Confidence semiring in provenance. Trust-index gating in `sm_search_proof_debt`. Authority decisions separate from relevance | Not supported | SM: confidence as query dimension. OKC: no equivalent |

---

## MCP Server Capabilities

### Tools Exposed (48+ tools in `full` profile)

**Search & Retrieval (11 tools):**
- `sm_search` — Hybrid BM25+vector RRF
- `sm_search_witnessed` — Mandatory receipt, cache bypass, source provenance hydration
- `sm_search_with_routing` — Adaptive query profiling + stage routing + factor graph + community grouping
- `sm_search_proof_debt` — Trust-index gated retrieval with proof-debt budget
- `sm_search_as_of` — Bitemporal search at specific valid_time
- `sm_search_conversations` — Hybrid search over stored messages
- `sm_search_explained` — Per-stage score breakdown (BM25, vector, RRF)
- `sm_route_query` — Profile query, return routing decision
- `sm_get_search_receipt` — Load durable receipt by ID
- `sm_replay_search` — Replay from stored inputs (opt-in)
- `sm_replay_search_receipt` — Replay with caller-supplied query
- `sm_benchmark_trust` — Trust quality distribution benchmark
- `sm_get_routing_policy` — RL routing policy weights

**Facts & Memory (13 tools):**
- `sm_add_fact`, `sm_get_fact`, `sm_get_fact_neighbors`, `sm_update_fact`, `sm_delete_fact`
- `sm_supersede_fact` — Replacement + supersedes edge (preferred over delete)
- `sm_consolidate_facts` — Merge near-duplicates
- `sm_list_facts`, `sm_list_namespaces`, `sm_delete_namespace`
- `sm_set_provenance` — Confidence (0–1) + support count
- `sm_ingest_document` — Auto-chunk, embed, index

**Knowledge Graph (11 tools):**
- `sm_add_graph_edge` — Typed: Semantic, Temporal, Causal, Entity
- `sm_list_graph_edges`, `sm_invalidate_graph_edge` (append-only)
- `sm_graph_path` — BFS shortest path
- `sm_community` — Leiden-inspired + contradiction scanning
- `sm_topology` — Betti numbers, void detection
- `sm_factor_graph` — Belief propagation over 4 edge types
- `sm_decoder_analyze` — Contradiction detection + BP refinement
- `sm_detect_contradictions` — Content-based (no pre-asserted edges)
- `sm_discord_search` — Second-order graph traversal from results
- `sm_subgraph_prune` — Access-frequency pruning (dry-run default)

**Trust & Claims (11 tools):**
- `sm_create_claim` — Typed claim from fact with source-spanned provenance
- `sm_add_evidence` — EvidenceBundle attachment
- `sm_judge_support` — Supported/unsupported/contested/heuristic_only
- `sm_verify_claim` — Risk-class verification (falsification for high+)
- `sm_decide_assertion_authority` — Governed assertion decision receipt
- `sm_decide_action_authority` — Governed action decision receipt
- `sm_query_claim_versions`, `sm_query_relation_versions`, `sm_query_episodes`, `sm_query_entity_aliases`, `sm_query_evidence_refs` — Bitemporal projections
- `sm_compact_claim_ledger` — Hash-chained ledger rotation (dry-run default)

**Lifecycle & Maintenance (8 tools):**
- `sm_stats`, `sm_run_lifecycle`, `sm_reconcile` (ReportOnly/RebuildFts/ReEmbed)
- `sm_vacuum`, `sm_reembed_all`, `sm_embeddings_are_dirty`
- `sm_rebuild_hnsw`, `sm_compact_hnsw`

**Import/Export (3 tools):**
- `sm_import_envelope` — Atomic bulk import with provenance
- `sm_import_status`, `sm_list_imports`

**Utility Parsers (9 tools):**
- JSON repair/extraction, choice/number/list parsing, think-tag stripping, RL feedback recording

### Tool Profiles (Runtime Profile Selection)

| Profile | Tools | Use Case |
|---------|-------|----------|
| `lean` / `standard` | 4 tools (witnessed search, replay, assertion authority, action authority) | Autonomous read-only recall + authority decisions |
| `agent` | 11 read-only tools (search, facts, graph paths, namespaces, stats, receipts) | Daily coding agent — read-only until trusted authority issuer injected |
| `full` | All 48+ tools (mutation, deletion, admin, experimental) | Operator context with explicit approval controls |

### Resources Exposed
- `memory://knowledge-graph` — Full graph as JSON (same shape as `read_graph`)
- Search receipts, claim ledger snapshots, import envelopes accessible via tool calls

### Resource Templates
None explicitly defined; resources are tool-addressable

---

## Search and Retrieval Capabilities

### Pipeline Architecture
1. **Query embedding/tokenization** — Candle (in-process, nomic-embed-text-v1.5) or Ollama
2. **Lexical lane** — SQLite FTS5/BM25
3. **Vector lane** — usearch 2.25 (default) or HNSW (opt-in)
4. **Sparse lane** — V36 sparse dot-product (disabled by default, `sparse_weight=0`)
5. **Fusion** — Weighted Reciprocal Rank Fusion (RRF)
6. **Policy** — Temporal/provenance filtering, superseded-head exclusion
7. **Receipts** — Durable capture of backend, exactness, candidates, fallback, degradations, result identity

### Key Differentiators
- **Witnessed retrieval** (`sm_search_witnessed`): Cache bypass, mandatory receipt, source provenance hydration
- **Opt-in replay** (V35): `ReplayMode::StoreInputs` retains query/filter for complete replay; privacy default is `NoReplay`
- **Explained results**: `sm_search_explained` returns per-stage scoring breakdown
- **Adaptive routing**: `sm_search_with_routing` profiles query, selects stages, optional factor graph + community grouping
- **Trust enrichment**: Six-state trust index derived from verified claim ledger; `sm_search_proof_debt` gates by proof-debt budget
- **Bitemporal search**: `sm_search_as_of` queries facts valid at a specific point in time
- **Contradiction detection**: `sm_detect_contradictions` (content-based, no pre-asserted edges) + `sm_decoder_analyze` (belief propagation refinement)

### Embedding Model
- Default: `nomic-ai/nomic-embed-text-v1.5` (768-dim) via Candle (pure Rust, CPU-only)
- Alternative: Ollama (any local model)
- Test-only: MockEmbedder (deterministic)
- Purpose-separated: `EmbeddingPurpose::Query` vs `EmbeddingPurpose::Document` with role prefixes

---

## Architecture and Integration Patterns

### Core Architecture
```
semantic-memory-mcp (MCP stdio server, rmcp SDK)
  └── semantic-memory (Rust library v0.5.10)
        ├── SQLite (authoritative storage, FTS5, WAL)
        ├── usearch 2.25 (vector sidecar, default backend)
        ├── Candle embedder (in-process, CPU, no API keys)
        ├── Provenance semirings (Boolean, Tropical, Probability, Confidence)
        ├── Temporal weight (age + supersession + support + contradiction)
        ├── Decoder (syndromes + corrections + belief propagation)
        ├── Subtraction (lawful forgetting + invariant verification)
        ├── Compression governor (importance-driven quantization)
        ├── Routing (query profiling + adaptive stage selection)
        ├── Discord (second-order graph-neighbor retrieval)
        ├── Stored graph edges (durable, typed, append-only with invalidation)
        ├── Factor graph (unified probabilistic reasoning)
        ├── Topology (Betti numbers, void detection)
        ├── Community detection (Leiden-inspired, contradiction-aware)
        └── Integration (cross-feature wiring)
```

### Storage Model
- **Canonical**: SQLite with WAL, pooled readers, serialized writer
- **Derived** (rebuildable from SQLite): FTS5 indexes, vector sidecars (usearch/HNSW), sparse representations, compressed artifacts
- **Receipts**: V35 search receipts with BLAKE3 digests; opt-in replay inputs table
- **Claim Ledger**: Hash-chained JSONL (`claim_ledger.jsonl`) → compacted to verified snapshot + retained tail (V35 compaction)

### Transport & Auth
- **Primary**: MCP over stdio
- **Optional HTTP sidecar**: `--http-port` binds loopback-only; `--http-only` disables stdio
- **Auth**: Bearer token required for all non-health HTTP endpoints; loopback Host/Origin validation
- **Profile filtering**: Applied below transport — lean/standard/agent expose only `/health` on HTTP

### Agent Integrations (First-Class)
- **Hermes**: Plugin validates inputs, wraps `hermes mcp add/list/test/configure` CLI
- **Claude Code**: Manifest, plugin-scoped launcher, semantic-memory skill, commands
- **Codex**: Agent Skill layout + stdio MCP install/config examples
- **Install/test matrix**: Side-by-side setup and smoke checks in `integrations/README.md`

### Cargo Feature Strategy
- `default = ["full"]` → `full` = alias for `search` + local `cfg(full)` wiring (TurboQuant, etc.)
- `search` = composed production router: usearch, Candle, provenance, temporal, multiscale, discord, decoder, subtraction, compression, routing, admin, late-interaction, TurboQuant, RL routing, integration
- Narrow features (`brute-force`, `hnsw`, `candle-embedder`, `claim-integration`, `llm-parser`, `orchestration`) for development builds
- **Key principle**: Feature flag proves compilation, not runtime activation. Use receipts/explained results for runtime claims.

---

## Feature Comparison with OKC MCP

| Capability | semantic-memory-mcp | OKC MCP | Gap / Advantage |
|------------|---------------------|---------|-----------------|
| **Evidence-scored retrieval** | ✅ Six trust states, proof-debt budget, claim judgments | ❌ | SM: formal evidence model |
| **Confidence/provenance semirings** | ✅ Boolean, Tropical, Probability, Confidence | ❌ | SM: quantitative, composable |
| **Source attribution (structured)** | ✅ Source spans, evidence bundles, receipts | ⚠️ Frontmatter only | SM: machine-verifiable |
| **Bitemporal queries** | ✅ `valid_time` / `recorded_time`, `sm_search_as_of` | ❌ | SM: point-in-time truth |
| **Contradiction detection** | ✅ Content-based + decoder (belief propagation) | ❌ | SM: automated conflict surfacing |
| **Adaptive query routing** | ✅ Profile → stage selection → factor graph | ❌ | SM: self-optimizing pipeline |
| **Governed authority decisions** | ✅ `sm_decide_assertion_authority`, `sm_decide_action_authority` | ❌ | SM: recall ≠ permission to act |
| **Claim ledger with compaction** | ✅ Hash-chained, digest-verified snapshots | ❌ | SM: tamper-evident trust layer |
| **Graph memory (4 edge types)** | ✅ Semantic, Temporal, Causal, Entity + community/topology | ⚠️ Wikilinks only | SM: richer graph semantics |
| **Tool profiles (security)** | ✅ lean/standard/agent/full runtime selection | ❌ | SM: principle of least privilege |
| **Witnessed retrieval + replay** | ✅ Cache bypass, receipts, opt-in replay | ❌ | SM: audit-grade retrieval |
| **Local-first, no API keys** | ✅ Candle embedder (CPU), Ollama optional | ✅ | Parity |
| **Human-readable knowledge format** | ❌ SQLite binary + sidecars | ✅ OKF Markdown + YAML | OKC: human-auditable, git-friendly |
| **Concept-centric modeling** | ❌ Fact/chunk/episode oriented | ✅ Concepts with typed relationships | OKC: domain-aligned abstraction |
| **Git-backed versioning** | ❌ | ✅ (via OKF bundles) | OKC: native diff/history |
| **Multi-agent skill ecosystem** | ✅ Hermes, Claude Code, Codex integrations | ⚠️ TBD | SM: broader agent coverage |
| **Document ingestion + chunking** | ✅ `sm_ingest_document` auto-chunk/embed/index | ❌ | SM: turnkey document pipeline |

---

## OKC Improvement Opportunities from semantic-memory-mcp Patterns

### 1. **Evidence Scoring & Confidence Model**
Adopt a structured evidence model for OKC concepts:
- Add `confidence: 0.0–1.0` and `evidence: [{source, span, weight}]` to frontmatter schema
- Define trust states (supported/contested/heuristic) as concept metadata
- Enable query-time confidence filtering (`concept_query --min-confidence 0.7`)

### 2. **Bitemporal Semantics**
Introduce `valid_time` / `recorded_time` distinction:
- `valid_time`: when the knowledge was true in the domain
- `recorded_time`: when it was captured in OKC
- Enables `as_of` queries for historical analysis

### 3. **Witnessed Retrieval / Audit Trail**
Add optional receipt emission for OKC searches:
- Capture: query, backend, candidates, fusion weights, result IDs, timestamp
- Store as sidecar JSONL (`~/.okc/receipts/`)
- Enable replay for debugging/validation

### 4. **Contradiction Detection**
Implement lightweight contradiction scanner for OKC:
- On `concept_add` / `concept_update`, compute embedding similarity against existing concepts
- Flag pairs with high cosine similarity (>0.85) but opposing assertions
- Surface as `concept_contradictions` report for human review

### 5. **Governed Authority Decisions**
Separate "recall" from "permission to assert/act":
- Add `concept_authority` tool: given concept ID, purpose, scope → returns decision receipt
- Use in CI/CD gates: "only assert concepts with authority receipt"

### 6. **Adaptive Query Routing**
Profile query types (fact lookup, exploration, contradiction check) and route to appropriate retrieval strategy:
- Fact lookup → exact vector + FTS
- Exploration → graph traversal + community detection
- Contradiction check → decoder analysis

### 7. **Tool Profiles for Security**
Implement runtime tool-profile selection for OKC MCP:
- `lean`: read-only search + authority decisions
- `agent`: read-only + graph traversal
- `full`: mutation, deletion, admin
- Enforce via MCP tool annotations + client approval policy

### 8. **Document Ingestion Pipeline**
Add `concept_ingest` tool:
- Accept Markdown/PDF/text → auto-chunk → embed → extract entities → create/link concepts
- Preserve source attribution at chunk level

### 9. **Claim Ledger / Trust Layer**
Optional tamper-evident ledger for high-stakes knowledge:
- Hash-chained concept assertions with evidence bundles
- Periodic compaction to verified snapshots
- Integration with `concept_verify` for risk-class verification

### 10. **Graph Enrichment**
Extend OKC relationships with typed edges:
- `semantic` (related meaning), `temporal` (before/after), `causal` (causes/enables), `entity` (mentions)
- Enable `concept_graph_path` (BFS), `concept_community` (clustering), `concept_topology` (void detection)

---

## Verdict

**semantic-memory-mcp** is the most sophisticated local-first semantic memory MCP server available. Its architecture reflects deep research-grade engineering: bitemporal truth, semiring provenance, witnessed retrieval with durable receipts, contradiction detection via belief propagation, adaptive routing, and a governed authority layer that cleanly separates recall from permission to act. The tool-profile system (lean/standard/agent/full) is a model for secure MCP deployment.

**Key takeaway for OKC:** OKC's strength is its human-readable, git-friendly OKF format and concept-centric modeling. semantic-memory-mcp's strength is its rigorous evidence model, temporal semantics, and security-first tool exposure. The optimal path is **hybrid**: keep OKF as the canonical knowledge representation, but adopt semantic-memory-mcp's evidence scoring, bitemporal fields, witnessed retrieval receipts, contradiction detection, and tool-profile security model as optional enhancements to the OKC MCP server.

**Priority adoption order:**
1. Tool profiles (lean/agent/full) — immediate security win
2. Confidence + evidence fields in OKF frontmatter — low schema cost, high value
3. Witnessed retrieval receipts — debuggability
4. Bitemporal `valid_time` / `recorded_time` — enables historical queries
5. Contradiction detection on write — data quality
6. Governed authority decisions — for CI/CD integration
7. Adaptive routing — when query volume justifies it