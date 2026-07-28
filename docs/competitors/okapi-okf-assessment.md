# Competitor Assessment: okapi-okf (npm)

## Overview

**okapi-okf** (npm: `okapi-okf`, v0.2.1, MIT, https://github.com/sebastienfi/okapi-okf-knowledge-studio) — An **interactive knowledge studio** for exploring, understanding, auditing, editing, and querying OKF bundles. Built as a TypeScript monorepo with a CLI (`okapi`), a web UI (React + Vite), and a core parser library. Published July 9, 2026 by Sébastien Fichot. MIT license. Node.js ≥20 required.

This is a **human-facing OKF bundle explorer** — a "studio" for visualizing, editing, and auditing OKF bundles. It provides a web UI with force-directed graph visualization, a CLI for bundle operations, and an "Ask the bundle" AI feature (opt-in, OpenAI/Anthropic). It is **not an MCP server** and does not provide agent-facing tools. It is a **human-centric OKF bundle IDE**.

---

## Feature Comparison with OKC

| Feature | okapi-okf | OKC | Notes |
|---------|-----------|-----|-------|
| **Language** | TypeScript (Node.js ≥20) | Rust | Different runtime |
| **License** | MIT | MIT | Aligned |
| **CLI** | ✅ `okapi` command | ✅ `okc` command | Both have CLI |
| **Web UI** | ✅ React + Vite (graph viz, editor) | ❌ None | okapi unique |
| **MCP Server** | ❌ None | ✅ 11 tools, stdio + HTTP/SSE | OKC only |
| **Bundle Parsing** | ✅ Core parser lib | ✅ Via `okf` crate | Parity |
| **Bundle Writing** | ✅ Web editor + CLI | ⚠️ Planned (`ingest`) | okapi ahead |
| **Graph Visualization** | ✅ Force-directed (web) | ❌ None | okapi unique |
| **Search** | ✅ Basic + AI "Ask" | ✅ FTS5/BM25 + graph | OKC richer |
| **Validation** | ✅ OKF conformance | ✅ `validate` tool | Parity |
| **Persistent Index** | ❌ In-memory | ✅ SQLite + FTS5 | OKC only |
| **File Watcher** | ✅ Watch mode (web) | ✅ `notify` + `observe` | Both live |
| **Graph Traversal** | ⚠️ Visual only | ✅ `traverse` tool | OKC: programmatic |
| **Lineage/History** | ❌ None | ✅ `lineage` tool | OKC only |
| **AI Integration** | ✅ "Ask the bundle" (opt-in) | ❌ None | okapi unique |
| **MCP Server** | ❌ None | ✅ 11 tools | OKC only |
| **MCP Resources** | ❌ None | ❌ Planned | Neither |
| **MCP Prompts** | ❌ None | ❌ Planned | Neither |

---

## Architecture & Code Quality

### okapi-okf
- **Structure**: pnpm monorepo with 3 packages + 1 app:
  - `packages/core` — Pure OKF parser (frontmatter, link extraction, link resolution). ~2-3k LoC TS.
  - `packages/cli` — Hono-based CLI server. Published as `okapi-okf`.
  - `packages/web` — React + Vite frontend (graph viz, editor, AI chat).
  - `okf/` — Okapi's own OKF documentation bundle (dogfooding).
- **Dependencies**: `hono` (CLI server), `react`/`vite` (web), `zod` (validation), `openai`/`@anthropic-ai/sdk` (AI). ~50+ deps total.
- **Architecture**: Clean separation — core (pure parser), CLI (Hono server), Web (React). Core is framework-agnostic.
- **Testing**: Unit + integration tests (`pnpm test`). CI on GitHub Actions.
- **Quality Gates**: TypeScript strict, ESLint, Prettier. `pnpm build` + `pnpm test` in CI.
- **Documentation**: Good README with install options (npx, npm, Homebrew, prebuilt binary). Web UI screenshots.
- **Maturity**: v0.2.1 (July 9, 2026), 0 stars, 0 forks, single maintainer. Early but well-architected.

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

| Tool/Resource | okapi-okf | OKC | Notes |
|---------------|-----------|-----|-------|
| **Parse bundle** | ✅ Core lib + CLI | ❌ Internal | okapi: primary use case |
| **Validate bundle** | ✅ Conformance check | ✅ `validate` tool | Parity |
| **Search** | ✅ Basic + AI "Ask" | ✅ `search` (BM25, filters) | OKC: richer pipeline |
| **Graph visualization** | ✅ Force-directed (web) | ❌ | okapi unique |
| **Bundle editing** | ✅ Web editor + CLI | ⚠️ Planned | okapi ahead |
| **AI "Ask bundle"** | ✅ Opt-in (OpenAI/Anthropic) | ❌ | okapi unique |
| **Watch mode** | ✅ Live reload (web) | ✅ `observe` + fs watcher | Both live |
| **MCP stdio** | ❌ | ✅ `okc serve --stdio` | OKC only |
| **MCP HTTP/SSE** | ❌ | ✅ `okc serve --http` | OKC only |
| **Resources** | ❌ | ❌ Planned | Neither |
| **Prompts** | ❌ | ❌ Planned | Neither |

---

## Strengths vs OKC

1. **Web-based graph visualization** — Force-directed graph of concepts/links is a powerful human-facing feature. OKC has no UI.

2. **Interactive bundle editor** — Web UI for creating/editing concepts, fixing links, auditing bundles. OKC is CLI-only.

3. **AI "Ask the bundle"** — Opt-in natural language Q&A over bundle content (OpenAI/Anthropic, user's API key). OKC has no AI chat.

4. **Multiple install paths** — `npx`, `npm -g`, Homebrew, prebuilt binary (no Node required). OKC: `cargo install` only.

5. **Watch mode with live reload** — Web UI updates in real-time as bundle files change. OKC has `observe` but no UI.

6. **Dogfooding** — The project documents itself as an OKF bundle (`okf/` folder). Strong signal of maturity.

7. **Multiple distribution formats** — npx (zero-install), npm global, Homebrew, prebuilt binary. OKC: `cargo install` only.

---

## Weaknesses vs OKC

1. **No MCP server** — Cannot be consumed by AI agents directly. OKC's 11 MCP tools over stdio/HTTP/SSE is the primary agent interface.

2. **No persistent search index** — In-memory parsing only. OKC's SQLite+FTS5 survives restarts, supports incremental updates.

3. **No programmatic graph traversal** — Visual graph only. No `traverse` tool for agents. OKC's `traverse` supports BFS with depth/node limits.

4. **No lineage/history** — No concept evolution tracking. OKC's `lineage` tool tracks splits/merges/renames.

5. **No metadata/faceted query** — Cannot filter by front-matter key/value. OKC's `query_metadata` enables structured search.

6. **No MCP resources/prompts** — Cannot expose `okapi://bundle/{id}` style resources or prompt templates.

7. **Node.js ≥20 requirement** — Excludes older LTS. OKC's Rust binary runs anywhere.

8. **Single maintainer, 0 stars** — Early stage, low bus factor. OKC has active development.

9. **Heavier dependency footprint** — 50+ deps (React, Vite, Hono, OpenAI SDK, etc.). OKC: ~20 deps.

---

## OKC Improvement Opportunities

| Area | Gap | Competitor Reference | Action |
|------|-----|---------------------|--------|
| **Web UI / Graph viz** | None | okapi: force-directed graph + editor | Add optional `okc serve --web` with embedded React UI for graph viz + editing |
| **AI chat over catalog** | None | okapi: "Ask the bundle" (opt-in) | Add `okc ask` with opt-in LLM integration (user API key) |
| **Bundle editing** | Planned only | okapi: web editor + CLI | Implement `okc edit` / `okc write` for agent/human authoring |
| **Multiple install paths** | `cargo install` only | okapi: npx, Homebrew, prebuilt binary | Publish prebuilt binaries (GitHub Releases), Homebrew tap |
| **Dogfooding** | Not yet | okapi: self-documents as OKF bundle | Create OKC's own docs as OKF bundle; use `okc scan` on itself |
| **Watch mode UX** | `observe` tool only | okapi: web live reload | Add `--web` flag to `okc serve` for live UI updates |

---

## Threat Level

**Low (direct) / Medium (feature inspiration)**

**Rationale:**
- **Different audience**: okapi-okf targets *humans* exploring/editing bundles; OKC targets *agents* consuming catalogs via MCP. Minimal direct competition.
- **No MCP overlap**: okapi has zero agent-facing protocol. OKC's moat is MCP.
- **Complementary potential**: okapi could *author* bundles that OKC *indexes and serves to agents*. OKC could *provide* the MCP layer okapi lacks.
- **Feature inspiration**: okapi's web UI, AI chat, graph viz, and multi-format distribution are features OKC should adopt.

**Risk**: If okapi adds MCP server + persistent index, it becomes a direct competitor. Current architecture (human-first, web UI) makes this a pivot.

---

## Verdict

**okapi-okf is a human-facing OKF bundle IDE, not an agent-facing catalog competitor.** It excels at visualization, editing, auditing, and AI-assisted exploration for *humans*. OKC excels at durable indexing, graph traversal, and MCP serving for *agents*.

**Strategic implications for OKC:**

1. **Adopt okapi's human-facing features** — Web UI with graph viz, bundle editor, AI chat, watch mode. These close the "human operator" gap.

2. **Match distribution breadth** — Prebuilt binaries, Homebrew, npx-style zero-install. OKC's `cargo install` is a barrier for non-Rust users.

3. **Dogfood OKC's own docs** — Create OKC's documentation as an OKF bundle; use `okc scan` on itself. Validates the format and the tool.

4. **Dogfood OKC's own docs** — Create OKC's documentation as an OKF bundle; use `okc scan` on itself. Validates the format and the tool.

**Recommended priority:**
1. Add optional web UI (`okc serve --web`) with graph viz + editing — Q1
2. Add optional AI chat (`okc ask`) with opt-in LLM — Q1
3. Publish prebuilt binaries + Homebrew tap — Q1
4. Create OKC's own docs as OKF bundle — Q1
5. Monitor okapi for MCP adoption — ongoing

okapi validates that **human-facing OKF tools need graph visualization, interactive editing, AI chat, and easy distribution** — all areas where OKC can differentiate.