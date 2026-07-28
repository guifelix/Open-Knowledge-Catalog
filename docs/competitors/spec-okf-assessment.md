# Competitor Assessment: spec-okf (npm)

## Overview

**spec-okf** (npm: `spec-okf`, v0.1.0, MIT, https://github.com/giossaurus/spec-okf) — A **CLI scaffolding tool** that configures projects with **Spec-Driven Development (SDD)** and an **Open Knowledge Format (OKF)** bundle, sharing the same `.md`/`.yaml` context across multiple AI agents (Claude, Codex, Cursor, Gemini, Copilot, Windsurf). Published a month ago by Giovanni Della Dea. MIT license. TypeScript. 3 weekly downloads. 6 commits.

This is a **project initialization / methodology tool** — not a knowledge catalog. It creates a project structure with a shared OKF knowledge bundle and agent-specific configuration files pointing to it. The methodology (Lean Inception → DDD → TDD → SDD) lives in `methodology/sdd.md`; the knowledge bundle lives in `knowledge/`. Each agent gets a "view" (config file) referencing both.

---

## Feature Comparison with OKC

| Feature | spec-okf | OKC | Notes |
|---------|----------|-----|-------|
| **Primary Purpose** | Project scaffolding + SDD methodology | Knowledge catalog + MCP server | Different problem |
| **OKF Bundle** | ✅ Created at init | ✅ Indexed at scan | Both use OKF |
| **Multi-Agent Support** | ✅ 6 agents (Claude, Codex, Cursor, Gemini, Copilot, Windsurf) | ⚠️ Via MCP (any MCP client) | spec-okf: config files; OKC: protocol |
| **Spec-Driven Development** | ✅ Core methodology | ⚠️ Implicit in workflow | spec-okf explicit |
| **CLI Interface** | ✅ Interactive + non-interactive | ✅ `okc` command | Both CLI |
| **MCP Server** | ❌ None | ✅ 11 tools, stdio + HTTP/SSE | OKC only |
| **Persistent Index** | ❌ None | ✅ SQLite + FTS5 | OKC only |
| **File Watcher** | ❌ None | ✅ `notify` + `observe` | OKC only |
| **Search** | ❌ None | ✅ FTS5/BM25 + graph | OKC only |
| **Graph Traversal** | ❌ None | ✅ `traverse` tool | OKC only |
| **Lineage/History** | ❌ None | ✅ `lineage` tool | OKC only |
| **Bundle Updates** | ✅ Preserves specs/docs/knowledge | ❌ Re-indexes | spec-okf preserves |
| **Agent Config Generation** | ✅ Per-agent config files | ❌ None | spec-okf unique |
| **License** | MIT | MIT | Aligned |

---

## Architecture & Code Quality

### spec-okf
- **Structure**: Single package with CLI entry point. ~3k LoC TS (est.).
- **Dependencies**: 3 deps. Build: `tsup`. Package manager: `pnpm`.
- **Architecture**: CLI with interactive menu (agent selection) + non-interactive flags. Generates project structure: `methodology/`, `docs/`, `specs/`, `knowledge/`, `agents/`. Preserves existing `specs/`, `docs/`, `knowledge/` on update.
- **Testing**: Not visible in repo metadata.
- **Quality Gates**: TypeScript strict, ESLint, Prettier. `pnpm build` + `pnpm test` in CI.
- **Documentation**: Good README with usage examples, flags, agent list.
- **Maturity**: v0.1.0, 1 month old, 6 commits, 3 weekly downloads, single maintainer.

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

| Tool/Resource | spec-okf | OKC | Notes |
|---------------|----------|-----|-------|
| **MCP Server** | ❌ None | ✅ 11 tools, stdio + HTTP/SSE | OKC only |
| **Search** | ❌ None | ✅ `search` (FTS5/BM25) | OKC only |
| **Graph Traverse** | ❌ None | ✅ `traverse` (BFS) | OKC only |
| **Lineage** | ❌ None | ✅ `lineage` | OKC only |
| **File Watch** | ❌ None | ✅ `observe` + fs watcher | OKC only |
| **Bundle Creation** | ✅ At init | ⚠️ Planned (`ingest`) | spec-okf ahead |
| **Bundle Preservation** | ✅ On update | ❌ Re-indexes | spec-okf unique |
| **Agent Config** | ✅ Per-agent files | ❌ None | spec-okf unique |

---

## Strengths vs OKC

1. **Multi-agent config generation** — Creates agent-specific config files (Claude, Codex, Cursor, Gemini, Copilot, Windsurf) all pointing to the same OKF knowledge bundle. OKC has no equivalent.

2. **Spec-Driven Development methodology** — Explicit SDD workflow (Lean Inception → DDD → TDD → SDD) with single source of truth in `methodology/sdd.md`. OKC has no prescribed methodology.

3. **Bundle preservation on update** — `spec-okf init` preserves existing `specs/`, `docs/`, `knowledge/` directories. OKC re-indexes from scratch.

4. **Interactive + non-interactive CLI** — Menu-driven agent selection plus `--agents` flag for automation. OKC is non-interactive only.

5. **OKF bundle as shared context** — The knowledge bundle is the explicit "single source of truth" for all agents. OKC's catalog is implicit.

6. **Lightweight, zero-runtime** — Generates files and exits. No server, no database, no background process. OKC runs a persistent MCP server.

---

## Weaknesses vs OKC

1. **No MCP server** — Cannot be consumed by AI agents at runtime. OKC's 11 MCP tools over stdio/HTTP/SSE is the primary agent interface.

2. **No persistent index** — Generates bundle once; no incremental updates, no FTS, no query engine. OKC's SQLite+FTS5 survives restarts, supports incremental updates.

3. **No file watcher** — No live reload. OKC's `notify` + `observe` pushes updates in real-time.

4. **No search** — No FTS, no BM25, no vector. OKC has hybrid BM25 + vector (planned).

5. **No graph traversal** — No `traverse`, `get_links`, `get_backlinks`. OKC's `traverse` supports BFS with depth/node limits.

6. **No lineage/history** — No concept evolution tracking. OKC's `lineage` tool tracks splits/merges/renames.

7. **No metadata/faceted query** — Cannot filter by front-matter key/value. OKC's `query_metadata`.

7. **Single maintainer, 3 downloads/week** — Very early adoption. OKC has active development.

8. **No MCP resources/prompts** — Cannot expose resources or prompt templates.

9. **OKF v0.1 only** — Behind OKC's v0.2 support.

---

## OKC Improvement Opportunities

| Area | Gap | Competitor Reference | Action |
|------|-----|---------------------|--------|
| **Project scaffolding** | None | spec-okf: `init` with methodology + bundle | Add `okc init` for new projects (bundle + agent configs) |
| **Bundle preservation** | Re-indexes | spec-okf: preserves specs/docs/knowledge | Add `--preserve` to `okc scan` / `okc ingest` |
| **Multi-agent config** | None | spec-okf: per-agent config files | Add `okc config agent --generate` for Claude/Cursor/etc. |
| **SDD methodology** | Implicit | spec-okf: explicit `methodology/sdd.md` | Document OKC's recommended workflow as `METHODOLOGY.md` |
| **Interactive init** | Non-interactive only | spec-okf: menu-driven agent selection | Add `okc init --interactive` |
| **Bundle as context** | Implicit | spec-okf: explicit shared bundle | Emphasize OKF bundle as "single source of truth" in docs |

---

## Threat Level

**Low (direct) / Medium (conceptual)**

**Rationale:**
- **Different problem**: spec-okf = project initialization + methodology; OKC = runtime catalog + MCP server. They can coexist.
- **No MCP**: spec-okf has zero agent-facing protocol. OKC's moat is MCP.
- **No persistence/index**: spec-okf generates once; OKC indexes continuously.
- **Conceptual overlap**: Both use OKF bundle as shared context. spec-okf makes this explicit at project start; OKC makes it implicit at runtime.
- **Adoption signal**: If spec-okf gains traction as "the way to start an OKF project," OKC should be the recommended runtime for the bundle it creates.

**Risk**: If spec-okf adds MCP server + persistent index, it becomes a direct competitor. Current architecture (scaffolding tool) makes this unlikely.

---

## Verdict

**spec-okf is a project scaffolding tool with an OKF bundle, not a knowledge catalog competitor.** It solves "how do I start a project with shared context for multiple AI agents?" OKC solves "how do I query, traverse, and serve a knowledge catalog to agents at runtime?"

**Strategic implication for OKC:** The scaffolding + methodology pattern is valuable. OKC should add `okc init` to create a project with an OKF bundle, agent configs, and a documented workflow — making OKC the natural runtime for spec-okf-initialized projects.

**Recommended priority:**
1. Add `okc init` with bundle + agent configs — Q1
2. Add `--preserve` to `okc scan` / `okc ingest` — Q1
3. Document recommended SDD-style workflow as `METHODOLOGY.md` — Q1
4. Add `okc config agent --generate` for per-agent MCP config — Q2
5. Monitor spec-okf for MCP adoption — ongoing

spec-okf validates that **OKF bundles as shared multi-agent context** is a compelling pattern. OKC should own the runtime layer for that pattern.