# Competitor Assessment: hermes-okf (EliaszDev)

## Overview

**Project:** hermes-okf  
**Repository:** https://github.com/EliaszDev/hermes-okf  
**License:** MIT  
**Language:** Python  
**Architecture:** Agent memory layer for the Hermes agent ecosystem, built on OKF v0.2.9  
**Installation:** `pip install hermes-okf` (Python 3.10+ required)  
**Stars:** 26 · **Commits:** 71 · **Maintainer:** EliaszDev  

**Description:** hermes-okf is the **first open-source memory system built on Google's Open Knowledge Format** for the Hermes agent ecosystem. It provides persistent, structured, version-controlled memory with no database, no lock-in — just Markdown + YAML files. It implements a **two-memory model**: hot memory (active session context) + cold archive (long-term knowledge), with Git history as the versioning backbone. It exposes MCP tools for the Hermes agent to search, list, show, snapshot, and restore memory. This is not a general-purpose OKF catalog — it's a **specialized agent memory system** for a specific agent framework (Hermes).

---

## Feature Comparison with OKC

| Feature | hermes-okf | OKC | Notes |
|---------|------------|-----|-------|
| **Target use case** | Hermes agent memory | General OKF knowledge catalog | Different primary users |
| **OKF version** | v0.2.9 (latest) | v0.2 | hermes-okf tracks latest spec |
| **Memory model** | Hot (session) + Cold (archive) | Single unified catalog | hermes-okf: dual-tier |
| **Versioning** | Git commits + snapshots | Git + `lineage` tool | hermes-okf: Git-native |
| **MCP support** | ✅ Hermes plugin (5 tools) | ✅ 11 tools, stdio + HTTP/SSE | Both MCP-native |
| **MCP transport** | stdio (via Hermes) | stdio + HTTP/SSE | OKC more deployable |
| **Search** | `hermes okf search` (basic) | Hybrid FTS5 + graph + vector | OKC richer |
| **Graph traversal** | ❌ Not exposed | ✅ `traverse` tool | OKC only |
| **Lineage/history** | Git + `snapshot`/`restore` | `lineage` tool (concept-level) | Different granularity |
| **File watching** | ❌ Manual refresh | ✅ `notify` + `observe` | OKC only |
| **Cross-bundle** | Single memory root | Multi-root catalog | OKC more flexible |
| **Authentication** | None (local agent) | None (local-first) | Parity |
| **Deployment** | Python package + Hermes | Single Rust binary | OKC simpler |
| **Code indexing** | ❌ | ❌ (markdown only) | Neither |
| **Vector search** | ❌ | ⚠️ Planned | Neither yet |
| **License** | MIT | MIT | Aligned |

---

## Architecture & Code Quality

### hermes-okf
- **Structure**: Python package (`hermes_okf/`) with modules: `memory/`, `mcp/`, `models/`, `storage/`, `utils/`. ~3,500 LoC Python.
- **Memory model**: Two-tier — `HotMemory` (in-memory, session-scoped) + `ColdMemory` (OKF bundles on disk, Git-tracked). Snapshots move hot→cold.
- **Storage**: Filesystem (Markdown + YAML frontmatter). Git for versioning. No database.
- **MCP integration**: Custom Hermes plugin exposing 5 tools via MCP stdio. Tools: `search`, `list`, `show`, `snapshot`, `restore`.
- **Dependencies**: `pydantic`, `pyyaml`, `gitpython`, `mcp` (Python SDK), `click`, `rich`. ~20 transitive deps.
- **Testing**: `pytest` suite. CI on GitHub Actions.
- **Quality gates**: `ruff`, `mypy`, `pytest-cov`. Pre-commit hooks.
- **Observability**: `rich` console output. Structured logging via `loguru`.
- **Maturity**: 71 commits, 26 stars, active development. Single maintainer but higher traction than most OKF tools.

### OKC
- **Structure**: Single binary crate (~8k Rust LoC). Modules: `config`, `index`, `model`, `parser`, `scanner`, `service`, `transport`.
- **Database**: SQLite + FTS5 (r2d2 pool). Persistent, incremental, file-watched.
- **Parser**: Reference `okf` crate (strict, spec-compliant).
- **Dependencies**: `okf`, `rusqlite`, `r2d2`, `tokio`, `notify`, `clap`, `tracing`, `serde`, `anyhow`. ~30 transitive deps.
- **Async**: Tokio for MCP server (stdio + HTTP/SSE). Core indexing synchronous.
- **Testing**: Unit + integration tests. CI: `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test`.
- **Quality gates**: Automated in CI. No coverage threshold yet.
- **Observability**: `tracing` structured logs only.
- **Maturity**: Pre-1.0, active development.

---

## MCP Capability Inventory

| Tool/Resource | hermes-okf (via Hermes) | OKC MCP | Notes |
|---------------|------------------------|---------|-------|
| **Search memory** | `hermes okf search <query>` | `search` (hybrid BM25+vector+graph) | OKC: richer pipeline |
| **List concepts** | `hermes okf list [--type]` | `graph` + `status` | OKC: graph overview |
| **Show concept** | `hermes okf show <id>` | `read` / `describe` | Parity |
| **Snapshot memory** | `hermes okf snapshot` | ❌ | hermes-okf unique |
| **Restore memory** | `hermes okf restore <id>` | ❌ | hermes-okf unique |
| **Graph traverse** | ❌ | `traverse` (BFS, filters) | OKC only |
| **Lineage/history** | Git + snapshots | `lineage` (concept evolution) | Different approach |
| **File watch / live** | ❌ | `observe` + fs watcher | OKC only |
| **Metadata query** | ❌ | `query_metadata` (KV filter) | OKC only |
| **Links/backlinks** | ❌ | `get_links` / `get_backlinks` | OKC only |
| **MCP stdio** | ✅ (via Hermes) | ✅ `okc serve --stdio` | Parity |
| **MCP HTTP/SSE** | ❌ | ✅ `okc serve --http` | OKC only |
| **MCP Resources** | ❌ | ❌ (planned) | Neither |
| **MCP Prompts** | ❌ | ❌ (planned) | Neither |

---

## Strengths vs OKC

1. **Two-tier memory model (hot + cold)** — Explicitly models the cognitive distinction between working memory (session context) and long-term knowledge. Snapshots provide clean session boundaries. OKC has a single unified catalog.

2. **Git-native versioning** — Every memory change is a Git commit. Snapshots are tagged commits. `restore` checks out a`restore`` does `git checkout`. Leverages Git's mature tooling (diff, blame, bisect) for free. OKC's `lineage` tool is concept-level but requires custom implementation.

3. **Hermes agent integration** — Purpose-built for the Hermes agent framework. The MCP tools are designed around Hermes's cognitive loop (perceive → reason → act → remember). OKC is agent-agnostic.

4. **Tracks latest OKF spec (v0.2.9)** — Actively follows the evolving OKF specification. OKC targets v0.2 stable.

5. **Snapshot/restore as first-class operations** — `snapshot` captures the entire hot memory state; `restore` replays it. Useful for agent checkpointing, debugging, and session resumption. OKC has no equivalent.

6. **Higher community traction** — 26 stars, 71 commits vs most OKF tools at 0-5 stars. Indicates real usage within the Hermes ecosystem.

---

## Weaknesses vs OKC

1. **Hermes-locked** — Only usable within the Hermes agent framework. Cannot be used by Claude, Cursor, VS Code, or other MCP clients directly. OKC's stdio + HTTP/SSE works with any MCP client.

2. **No graph traversal** — Memory is searchable but not traversable as a graph. No `traverse`, `get_links`, `get_backlinks`. OKC's graph tools enable multi-hop reasoning.

3. **No file watching / live updates** — Memory updates require explicit `snapshot` or manual refresh. OKC's `notify`-based watcher provides instant index freshness.

4. **No metadata/faceted query** — Cannot filter by front-matter key/value. OKC's `query_metadata` enables structured faceted search.

5. **No lineage at concept level** — Git history is file-level. Concept splits/merges/renames are not tracked semantically. OKC's `lineage` tool tracks concept evolution.

6. **No vector/semantic search** — Keyword search only. OKC plans hybrid BM25+vector.

7. **Python deployment friction** — Requires Python environment, `pip install`, Hermes framework. OKC's single Rust binary is zero-dependency.

8. **Single memory root** — One hot + one cold store per agent. OKC's `roots` config supports multiple independent catalogs.

9. **No MCP resources/prompts** — Cannot expose `hermes://memory/status` style resources or retrieval prompt templates.

10. **Single maintainer** — Bus factor = 1 despite higher star count.

---

## OKC Improvement Opportunities

| Area | Gap | Competitor Reference | Action |
|------|-----|---------------------|--------|
| **Session memory model** | Single unified catalog | hermes-okf: hot (session) + cold (archive) tiers | Add `session` concept: ephemeral in-memory layer with explicit `snapshot` → persistent catalog |
| **Snapshot/restore** | None | hermes-okf: `snapshot` (tagged commit) + `restore` (checkout) | Add `okc snapshot` (tag index state) + `okc restore <tag>` |
| **Git-native versioning** | Custom `lineage` tool | hermes-okf: Git commits + tags for everything | Evaluate using Git tags for index snapshots; `lineage` can read Git history |
| **Agent checkpointing** | Not supported | hermes-okf: snapshot = agent checkpoint | Add `checkpoint` MCP tool for agent session save/resume |
| **OKF spec tracking** | v0.2 stable | hermes-okf: v0.2.9 latest | Add CI job testing against OKF spec repo; track version in `Cargo.toml` |
| **Dual-memory architecture** | Single index | hermes-okf: hot (RAM) + cold (disk) | For large catalogs: add in-memory hot tier (recent writes) with async flush to SQLite |
| **Agent-specific tool profiles** | One tool set for all | hermes-okf: tools designed for Hermes loop | Add `tool_profile` config (lean/agent/full) to limit MCP surface per use case |

---

## Threat Level

**Low (direct) / Medium (architectural influence)**

**Rationale:**
- **Different target user**: hermes-okf serves Hermes agent developers; OKC serves general MCP clients (Claude, Cursor, etc.). Minimal direct competition.
- **Complementary potential**: hermes-okf could *consume* OKC-indexed catalogs as its cold memory backend. OKC could add Hermes-compatible tool profile.
- **Architectural lessons**: The hot/cold memory model and snapshot/restore pattern are genuinely valuable for agent memory. OKC should adopt these concepts.
- **Spec leadership**: hermes-okf tracking v0.2.9 means it may drive OKF evolution. OKC should track spec changes closely.

**Risk**: If Hermes becomes a dominant agent framework, hermes-okf becomes the de facto OKF memory layer for that ecosystem. OKC should ensure compatibility.

---

## Verdict

**hermes-okf is a specialized agent memory system, not a general OKF catalog competitor.** It solves a different problem (Hermes agent session memory) with a different architecture (hot/cold tiers, Git snapshots, Hermes MCP plugin).

**However, it pioneers two patterns OKC should adopt:**

1. **Hot/cold memory tiering** — Ephemeral session layer + persistent archive with explicit snapshots. This maps directly to how agents actually use memory (working context vs. long-term knowledge).

2. **Snapshot/restore as first-class operations** — Agent checkpointing is a real need. `okc snapshot` / `okc restore` would enable session resumption, debugging, and reproducible agent runs.

**Strategic implication for OKC:** The "agent memory" and "knowledge catalog" categories are converging. OKC's MCP-native architecture positions it to serve *both* — but it needs the memory-specific primitives (session tier, snapshots, checkpoints) that hermes-okf demonstrates.

**Priority adoption order:**
1. **Session memory tier + snapshot/restore** — Core memory primitive (Q1)
2. **Git-tag-based index snapshots** — Leverage Git for free versioning (Q1)
3. **Agent checkpoint MCP tool** — `checkpoint` / `resume` for session continuity (Q2)
4. **Track OKF spec evolution** — CI against spec repo; version in `Cargo.toml` (ongoing)
5. **Hermes compatibility profile** — Tool subset matching hermes-okf for migration path (Q2)

hermes-okf validates that **OKF + MCP + Git** is a viable agent memory stack. OKC should be the *general-purpose* implementation of that stack.