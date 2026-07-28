# Competitor Assessment: atheneum / Athenaeum / YantrikDB

## Overview

This assessment covers three distinct but related projects that target "agentic memory" and "knowledge persistence for AI agents." They span the gap between session-level context memory and structured knowledge bases, making them highly relevant to OKC's positioning.

### atheneum (oldnordic)

**v0.11.0** (crates.io/atheneum, GitHub: oldnordic/atheneum) — "Episodic memory + evidence graph for AI agents." An embedded SQLite-backed knowledge graph that persists tool calls, decisions, wiki content, code signals, and raw session memory across coding sessions. Part of the grounded-coding ecosystem by Luiz Spies (alongside magellan, llmgrep, mirage-analyzer). Provides CLI (`search`, `navigate`, `memory-list`, `dream`, `sync-logseq`) + MCP server (atheneum-mcp). License: not declared (likely MIT/Apache).

**Key differentiators**: HopGraph for token-budgeted subgraph extraction, dreaming module (6-phase consolidation pipeline), cross-project queries via ATTACHed SQLite databases, Logseq wiki sync. Multiple-assistant architecture (Claude Code + atheneum-py + Hermes write to same graph).

### Athenaeum (Kromatic Innovation)

**PyPI** (pypi.org/project/athenaeum, GitHub: Kromatic-Innovation/athenaeum) — "Production-tested agentic memory for teams deploying multiple AI agents." Append-only intake, a tiered librarian that compiles raw observations into a trustworthy wiki, and a sidecar that makes recall happen passively on every turn. License: Apache 2.0. Author: Tristan Kromer / Kromatic.

**Key differentiators**: Sources as first-class objects (provenance on every claim), librarian as sole writer to wiki (safety by structure), passive passive recall on every turn via hooks (Claude Code user-prompt-recall.sh), configurable observation filter (editable prompt for what gets saved). Team-scale design.

### YantrikDB (yantrikos)

**v0.10.0** (crates.io/yantrikdb, pypi.org/project/yantrikdb, GitHub: yantrikos/yantrikdb) — "A Cognitive Memory Engine for Persistent AI Systems." Embedded Rust engine (like SQLite) with Python bindings. Record/recall/relate/think API. Five indexes: dense vector (HNSW), keyword (delta BM25), knowledge graph (CSR), temporal (B-tree), importance (B-tree). Session tracking, contradiction detection, autonomous consolidation, proactive triggers. MCP server. Author: Pranab Sarkar. License: AGPL-3.0 (engine), MIT (MCP server).

**Key differentiators**: Relevance-conditioned scoring (patented, U.S. App 19/573,392), cognitive state graph with typed nodes/edges, CRDT-based sync, `db.think()` for autonomous cognition, benchmarked 96-99.9% token savings over file-based memory. Published peer-cited pre-print.

---

## Feature Comparison Matrix

| Feature | atheneum (oldnordic) | Athenaeum | YantrikDB | OKC | Notes |
|---------|----------------------|-----------|-----------|-----|-------|
| **Storage backend** | SQLite (embedded graph) | File-based wiki + chromadb (optional) | SQLite (embedded engine) | SQLite (single file) | All are SQLite-adjacent |
| **Persistence model** | Episodic memory + evidence graph | Append-only intake → tiered librarian → wiki | Cognitive memory (5 indexes, temporal decay, consolidation) | Structured knowledge base (OKF) | atheneum: logging; Athenaeum: pipeline; YantrikDB: cognitive; OKC: curated |
| **Search** | Lexical (CLI search) | FTS5 + optional chromadb hybrid | Dense HNSW + delta BM25 hybrid | FTS5 BM25 | YantrikDB has strongest search |
| **Knowledge graph** | ✅ SQLiteGraph with typed edges | ❌ No (wiki + backlinks) | ✅ Cognitive State Graph (typed nodes/edges) | ✅ SQLite link graph | atheneum + YantrikDB have native graph |
| **Graph traversal** | ✅ HopGraph (BFS, token-budgeted) | ❌ | ✅ Entity search, relationship depth, link traversal | ✅ BFS traverse (MCP) | atheneum's HopGraph is unique (budgeted subgraph) |
| **MCP server** | ✅ atheneum-mcp (rmcp) | ❌ No MCP (hooks + CLI) | ✅ yantrikdb-mcp | ✅ okc serve (stdio + HTTP) | Athenaeum uses hooks instead |
| **CLI** | ✅ search, navigate, memory-list, dream, sync-logseq, cross-search | ✅ init, run, status | Core engine API; yantrikdb-server for network | ✅ scan, browse, get, search, traverse, validate, stats, watch | atheneum has richest CLI for agent memory |
| **Session tracking** | ✅ First-class (Session entities, tool calls, timestamps) | ✅ Implicit via append log + Claude Code bridge | ✅ Explicit sessions (start/end, history, stale abandonment) | ❌ No session concept | All three have session awareness OKC lacks |
| **Consolidation/GC** | ✅ Dreaming module (6-phase: scan→dedup→stale→contradiction→verbose→consolidated) | ✅ Librarian (tiered compilation: raw→cluster→merge→wiki) | ✅ think() (autonomous: merge, conflict-detect, pattern-mine, decay) | ❌ No automated GC | Three different approaches to the same problem |
| **Provenance** | ✅ Edge-linked (ToolCall→Session→Symbol→File) | ✅ Sources as first-class objects | ⚠️ Implicit via session→record links | ✅ Backlinks + citations (OKF) | Athenaeum has strongest provenance (mandatory) |
| **Multi-assistant** | ✅ Designed for it (Claude Code + Python + Hermes write to same DB) | ✅ Team-scale (multi-agent shared wiki) | ✅ Multi-agent (v5 planned); MCP multi-tenant | ❌ Single-user | atheneum has real multi-assistant deployment |
| **CLI export** | ❌ (concise markdown output) | ❌ | ❌ (Python API) | ❌ (JSON via MCP only) | None excel here |
| **File watching** | ❌ No (CLI commands only) | ❌ No (manual run) | ❌ No (engine API) | ✅ watch (incremental reconcile) | OKC unique advantage |
| **Embeddings** | ❌ No | ✅ Optional chromadb + MiniLM | ✅ Bundled potion-base embedder (no deps) | ❌ No | YantrikDB bundling is unique |
| **License** | Not declared | Apache 2.0 | AGPL-3.0 (+ MIT for MCP) | OKC's (not declared) | Athenaeum is most permissive for teams |

---

## Architecture Comparison

### atheneum: Agent Session Graph

```
atheneum/
├── src/              # Core lib (SQLiteGraph, entities, store)
├── crates/
│   └── atheneum-mcp/ # MCP server (rmcp)
├── atheneum-py/      # Python port (multi-assistant)
└── CLI commands: search, navigate, memory-list, dream, sync-logseq, reindex, cross-search, graph-stats
```

**Entity model** (10 types from live 4,677-entity DB):
- ToolCall, WikiPage, Session, ReasoningLog, Reference, Memory, File, Symbol, TestRun, Knowledge
- Edges: belongs_to_project, observed_in, wikilink, handled_by_tool, accessed, modified, CALLS, IMPORTS

**Dreaming pipeline** (6-phase):
```
SCAN → DEDUPLICATE → STALE → CONTRADICTION → VERBOSE → CONSOLIDATED
```
Uses trigram Jaccard for near-duplicates, produces consolidated `Knowledge` entities.

**Cross-project** via `ATTACH` (up to 10 SQLite DBs, LRU 8). Queries across magellan-indexed projects simultaneously.

### Athenaeum: Tiered Compilation Pipeline

```
raw/ (agent intake, append-only)
  → Librarian (LLM-powered clustering + merging + contradiction check)
    → wiki/ (compiled entities, human-readable)
      → Passive recall hook (FTS5 + optional chromadb)
```

**Four design principles**:
1. Sources as first-class objects (every claim has provenance)
2. Append-only intake + separate compiler (safety from structure, not trust)
3. Passive recall on every turn (sidecar hook, not agent-remembered)
4. Editable observation filter (auditable prompt)

**Integration**: Claude Code bridge (reads `~/.claude/projects/<scope>/memory/` into raw/), auto-recall via user-prompt-recall.sh hook, context checkpointing before compaction.

### YantrikDB: Five-Index Cognitive Engine

```
┌─────────────────────────────────────────┐
│            YantrikDB Engine              │
├─────────┬─────────┬──────┬──────┬───────┤
│ Dense   │Keyword  │ KG   │Temporal│Import│
│ (HNSW)  │(Delta   │(CSR) │ (B-tree)│(B-tree)│
│         │ BM25)   │      │       │       │
└─────────┴─────────┴──────┴──────┴───────┘
```

**Cognitive operations API** (not SQL):
- `record()`, `record_batch()`, `recall()`, `recall_with_response()`, `forget()`, `correct()`
- `relate()`, `get_edges()`, `search_entities()`, `entity_profile()`, `relationship_depth()`
- `think()` (consolidate, conflict-detect, pattern-mine)
- `session_start()`, `session_end()`, `session_history()`, `active_session()`

**Relevance-conditioned scoring**: Importance and recency gated by relevance multiplicatively. Patented method.

**Ecosystem**: Rust crate + Python bindings + MCP server + network DB (Raft-replicated) + Hermes plugin.

---

## Boundary Analysis: Agentic Memory Systems vs OKC

```
┌─────────────────────────────────────────────────────────────────────┐
│                    SESSION CONTEXT MEMORY                            │
│  (ai-memory, memcrate, mem0, Letta, etc.)                           │
│  Purpose: "Where was I? What did I decide?"                         │
└─────────────────────────────────────────────────────────────────────┘
                              │
                              ▼ Promotion / Extraction
┌─────────────────────────────────────────────────────────────────────┐
│                    AGENTIC MEMORY / KNOWLEDGE GRAPH                  │
│  (atheneum, YantrikDB, Athenaeum)                                   │
│  Purpose: "What did the agent learn? What does it know?"            │
│  Lifetime: Session → Project → Forever                              │
│  Write pattern: Auto-capture + consolidation/dreaming               │
│  Read pattern: Recall (ranked, budgeted, passive)                   │
│  Structure: Entity graph, cognitive indices, wiki pages             │
└─────────────────────────────────────────────────────────────────────┘
                              │
                              ▼ Promotion
┌─────────────────────────────────────────────────────────────────────┐
│                    STRUCTURED KNOWLEDGE BASE                         │
│  (OKC, vaultdb, Obsidian, Notion)                                   │
│  Purpose: "What is the architecture? APIs? Decisions?"              │
│  Lifetime: Project → Forever                                        │
│  Write pattern: Curated, reviewed, versioned                        │
│  Read pattern: Search, traverse, browse, structured query           │
└─────────────────────────────────────────────────────────────────────┘
```

### Where the three assessed projects sit

All three occupy a **middle layer** between raw session memory and curated KB — they auto-capture agent activity, consolidate it into structured knowledge, and make it retrievable. But they differ in architecture:

| Dimension | atheneum (oldnordic) | Athenaeum | YantrikDB | OKC |
|-----------|---------------------|-----------|-----------|-----|
| **Primary writer** | AI agent (auto) + human (wiki sync) | AI agent (append-only) + Librarian (compile) | AI agent (record) + engine (think/consolidate) | Human (curated) or AI (with review) |
| **Primary reader** | AI agent (search + navigate) | AI agent (passive recall) | AI agent (recall + think) | Human + AI agent (MCP query) |
| **Mutation rate** | High (every tool call) | Medium (per session compile) | High (every record) | Low (deliberate edits) |
| **Schema** | Rich entity types + edges | Wiki pages + sources | Cognitive state graph | OKF (type, citations, links, tags) |
| **Search** | Lexical + graph traversal | FTS5 (+ optional vector) | HNSW + BM25 hybrid | BM25 FTS5 |
| **Automation** | Dreaming consolidation | Librarian compilation | think() + proactive triggers | Manual (validate only) |
| **Multi-agent** | Yes (3 assistants live) | Yes (team-scale) | Planned (v5) | No |
| **Session awareness** | First-class sessions | Implicit via append log | Explicit sessions | No |
| **Human-readable** | Via Logseq sync + concise output | Yes (wiki/ is markdown) | Via MCP + Python API | Yes (OKF markdown) |

---

## Strengths vs OKC

### atheneum (oldnordic)

1. **HopGraph traversal**: Token-budgeted subgraph extraction with BFS — navigate returns a trimmed, context-window-sized graph, not an unbounded traversal. Unique.
2. **Dreaming consolidation**: 6-phase pipeline that runs autonomously post-session. Produces consolidated `Knowledge` entities from raw memories. OKC has no automated GC.
3. **Cross-project querying**: `ATTACH`-based queries across multiple SQLite databases simultaneously (8-project LRU). OKC is single-database.
4. **Multi-assistant architecture**: Three consumers (Claude Code, Python, Hermes) write to the same graph. Proven in production.
5. **Rich entity model**: 10 typed entities with meaningful edges (ToolCall→Session, Symbol→File, ReasoningLog→Session). Richer than OKC's flat document model.
6. **Session accountability**: Every tool call timestamped and linked to session. OKC has no session concept.
7. **Logseq wiki sync**: Bidirectional sync with personal wiki. Bridges agent memory with human thinking notes.

### Athenaeum (Kromatic)

1. **Passive recall**: Sidecar hook fires on every turn, injects wiki breadcrumbs into context. The agent never has to remember to look. OKC requires explicit MCP query.
2. **Tiered compilation**: Raw intake → librarian → wiki. Safety from structure (agents append-only, compiler is the only writer). OKC has no write pipeline.
3. **Provenance everywhere**: Sources as first-class objects. Every wiki entity carries its origin. OKC has backlinks but no mandatory provenance.
4. **Configurable observation filter**: Editable prompt governing what the agent saves. Auditable, transparent. OKC has no write filter.
5. **Team-scale by design**: Multiple agents, concurrent writes, shared wiki. OKC is single-user.
6. **Claude Code bridge**: Reads `~/.claude/projects/<scope>/memory/` into the intake pipeline. Direct integration with existing Claude Code memory.

### YantrikDB

1. **Relevance-conditioned scoring**: Patented multiplicative gating of importance/recency by relevance. Surfaces the right memories at the right time.
2. **Five-index unification**: HNSW + BM25 + KG + temporal + importance in one engine — strongest integrated search among the four.
3. **Autonomous cognition**: `db.think()` consolidates, detects conflicts, mines patterns proactively. Not just passive storage — the engine does work between conversations.
4. **Proactive triggers**: Decaying memories, unresolved conflicts, emerging patterns trigger the agent without being polled.
5. **Cognitive state graph**: Typed nodes (beliefs, goals, intents, preferences) with typed edges (supports, contradicts, causes, predicts). Richer semantic than OKC's link graph.
6. **Bundled embeddings**: `potion-base` embedder ships with the engine (no pip install sentence-transformers, no ONNX). Zero-dependency semantic search.
7. **Published benchmarks**: 96-99.9% token savings over file-based memory at 100-5000 memories. Reproducible.
8. **Ecosystem breadth**: Rust engine + Python bindings + MCP server + network DB (Raft) + Hermes plugin + REPL (yql) + Cursor directory listing.
9. **Patent + pre-print**: Academic credibility that the others lack.

---

## Weaknesses vs OKC

| Dimension | atheneum | Athenaeum | YantrikDB | OKC advantage |
|-----------|----------|-----------|-----------|---------------|
| **Structured KB schema** | ❌ No OKF-style formatting | ⚠️ Wiki pages but no schema enforcement | ❌ No document schema | OKF with type, citations, tags, reserved files |
| **File watching** | ❌ | ❌ | ❌ | `watch` with debounce + periodic reconcile |
| **Validation tooling** | ❌ | ❌ | ❌ | `validate` for broken links, FM issues, heading hierarchy |
| **Document browsing** | ❌ No | ❌ No | ❌ No | `browse` directory tree + `get_document` + `get_section` |
| **HTTP MCP transport** | ❌ stdio only | ❌ No MCP | ❌ stdio MCP | stdio + HTTP/SSE |
| **Section extraction** | ❌ No | ❌ No | ❌ No | `get_section` by heading title |
| **FTS5 BM25 weighting** | ⚠️ Lexical only | ⚠️ FTS5 default | ✅ but bundled | Configurable BM25 weights (Bm25Config) |
| **Export** | ❌ | ❌ | ❌ | JSON via MCP (still weak but exists) |
| **Mutability safety** | ❌ Direct writes | ⚠️ Append-only + compiler | ❌ Direct API writes | Read-only MCP (no agent writes) |
| **Simplicity of setup** | ⚠️ Needs magellan ecosystem | ✅ pip install | ✅ pip / cargo install | `cargo install okc` + `okc scan` |
| **OKF-aligned format** | ❌ SQLite opaque | ⚠️ Wiki is markdown | ❌ SQLite opaque | OKF: plain Markdown + YAML FM, git-versionable |

---

## OKC Improvement Opportunities

### P0 — Critical Gaps

1. **Add session awareness** (inspired by all three)
   - `okc_session_start` / `okc_session_end` MCP tools — session log writes to KB
   - `okc_recall_session` — returns last N session summaries
   - Entity model: `Session` with timestamp, duration, tool_count, file_touches
   - atheneum proves this is valuable: 221 sessions tracked in live database

2. **Add consolidation / GC pipeline** (inspired by atheneum dreaming + YantrikDB think + Athenaeum librarian)
   - `okc consolidate` — scans documents for stale, duplicate, contradictory content
   - `okc consolidate --dry-run` — preview without committing
   - 6-phase pipeline modeled on atheneum: SCAN → DEDUP → STALE → CONTRADICTION → VERBOSE → CONSOLIDATED
   - Report: "Consolidated 5 KB entries, archived 12 stale, flagged 2 contradictions"

3. **Add passive recall hook** (inspired by Athenaeum)
   - `okc recall` MCP tool injected on every turn (like Athenaeum's user-prompt-recall.sh)
   - Returns top-3 relevant KB entries with summaries for context injection
   - Configurable via `okc.toml` (max tokens, recall frequency)
   - Bridges the gap between "agent must query KB" and "KB enriches every turn"

### P1 — Competitive Parity

4. **Token-budgeted graph traversal** (inspired by atheneum HopGraph)
   - Add `--max-tokens` flag to `traverse` MCP tool
   - Returns a trimmed subgraph that fits in context window
   - Prioritize edges: high-weight relations before low-weight ones
   - atheneum's `navigate --concise --max-tokens 500` is the model

5. **Cross-project queries** (inspired by atheneum)
   - Support multiple OKC databases via `ATTACH`
   - `okc search --db project1.db --db project2.db "query"`
   - LRU cache of attached databases (max 8 like atheneum's 8)

6. **Provenance as first-class field** (inspired by Athenaeum)
   - Add `source` field to OKF frontmatter (URL, session_id, tool_call_id)
   - Validation: warn on missing source for critical document types
   - Search filter: `source:session:*` to find agent-derived content

### P2 — Nice to Have

7. **Bundled embeddings for hybrid search** (inspired by YantrikDB)
   - Ship a small static embedder (like potion-base) with the `okc` binary
   - Enable HNSW + FTS5 hybrid search without external dependencies
   - Default lightweight embedder, opt-in for higher quality

8. **Proactive trigger system** (inspired by YantrikDB)
   - Stale document detection: "3 documents haven't been read in 90 days"
   - Broken backlink detection: "2 documents link to missing pages"
   - Contradiction detection: "Two documents disagree on API endpoint"
   - MCP tool: `okc_get_triggers` returns actionable maintenance suggestions

9. **Companion memory export for MCP clients** (inspired by Athenaeum)
   - `okc inject-context` MCP tool → returns token-budgeted KB summary for system prompt
   - Like YantrikDB's recall() but from OKF-structured content
   - Model: build context injection payload from browse + linked docs

10. **Write-path for agent observations** (inspired by Athenaeum append-only)
    - `okc observe` CLI/MCP tool — agent appends raw observation to intake queue
    - Separate `okc compile` — LLM-powered consolidation into proper OKF documents
    - Safety by structure: agents append, humans/hooks compile
    - Bridges OKC from pure KB to agent-active knowledge system

---

## Verdict

**atheneum (oldnordic)** is the best real-world example of **agent session memory as a knowledge graph**. Its living 4,677-entity / 15,015-edge database across 221 sessions proves the concept works. HopGraph and the dreaming module are genuinely novel contributions. Weakness: tightly coupled to the magellan ecosystem, no team-scale design, opaque SQLite format.

**Athenaeum (Kromatic)** is the most **production-tested team-scale agentic memory system**. The append-only → librarian → wiki pipeline is a principled architecture for multi-agent safety. Passive recall via hooks is the right UX pattern. Weakness: no MCP server, no graph traversal, Python-only, manual librarian runs.

**YantrikDB (yantrikos)** is the most **technically ambitious** — patented relevance scoring, 5-index unification, cognitive state graph, autonomous cognition, CRDT sync, proactive triggers. The AGPL license and patent filing create adoption friction. Weakness: over-engineered for simple use cases, no structured KB format, single-file opacity.

**OKC occupies a distinct niche:** **structured knowledge base with OKF schema, graph traversal, and MCP serving**, but lacks the session awareness, consolidation, passive recall, and write-path that all three of these projects demonstrate in different ways.

### Strategic Recommendations for OKC

1. **Add session awareness** (P0) — this is the single biggest gap. atheneum proves it works. OKC needs `Session` entities and session MCP tools.
2. **Add consolidation/GC pipeline** (P0) — all three competitors automate knowledge maintenance. OKC's `validate` is useful but passive.
3. **Add passive recall hook** (P0) — Athenaeum's per-turn recall is the right UX. OKC should offer a similar hook for agent context enrichment.
4. **Add token-budgeted traversal** (P1) — HopGraph demonstrates that agents need controlled graph extraction, not unbounded traversals.
5. **Add provenance field to OKF** (P1) — Athenaeum's mandatory provenance is the right pattern for production knowledge systems.

**Bottom line**: The agentic memory space is splitting into three layers — session context memory (memcrate, ai-memory), agentic memory/knowledge graph (atheneum, YantrikDB, Athenaeum), and structured knowledge bases (OKC, vaultdb). OKC should stay in the KB layer but **bridge to the layer below** with session awareness and write-path capabilities, and **bridge to agents** with passive recall and context injection.

---

## Assessment Completeness

- [x] #1 Overview of three related projects
- [x] #2 Feature comparison matrix
- [x] #3 Architecture comparison
- [x] #4 Boundary analysis: agentic memory vs OKC
- [x] #5 Strengths vs OKC (per project)
- [x] #6 Weaknesses vs OKC
- [x] #7 OKC improvement opportunities (P0/P1/P2)
- [x] #8 Verdict and strategic recommendations
