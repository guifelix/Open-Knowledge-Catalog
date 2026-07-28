# Competitor Assessment: okf-generator (UmairBaig8)

## Overview

**okf-generator** (PyPI: `okf-generator`, v0.1.51, 1 GitHub star, https://github.com/UmairBaig8/okf-generator) — Python CLI tool that scans source codebases using tree-sitter AST parsers across 17 languages and 22 manifest formats, producing OKF v0.2 knowledge bundles (structured markdown mirroring the source tree). Includes optional LLM enrichment (4 tiers, multi-provider routing), incremental updates via SHA256 manifest, bundle diff with dependency impact analysis, interactive D3.js visualization, MCP server (11 tools), FastAPI dashboard, and built-in fine-tuning dataset generator (`okf pairs`) exporting 5 pair types (codegen, QA, doc, summarize, crosslink) as JSONL. Integrates with 7 AI agents (Claude Code, Cursor, Copilot, Windsurf, Cline, OpenCode, MCP) via `okf install`. Also offers Sitespeak.ai web-based OKF generator for websites (free, crawl-based).

**OKC** (Open Knowledge Catalog) is a Rust-based knowledge catalog for markdown documents with SQLite+FTS5 backend, file watcher, MCP server (11 tools), graph traversal, and lineage tracking. OKC targets *document collections* (markdown files with front-matter) while okf-generator targets *source codebases* (AST-extracted concepts with cross-references). Both produce OKF v0.2 bundles but serve different primary use cases: OKC = document catalog for RAG/knowledge management; okf-generator = code intelligence for AI coding agents.

---

## Feature Comparison with OKC

| Aspect | okf-generator | OKC | Notes |
|--------|---------------|-----|-------|
| **Primary target** | Source code (17 langs, AST) | Markdown documents (front-matter) | Different domains |
| **OKF version** | Generates v0.2 bundles | Consumes/produces v0.2 | Both v0.2 compatible |
| **Language support** | ✅ 17 (tree-sitter + stdlib AST) | ❌ Markdown only | okf-generator far broader |
| **Manifest parsing** | ✅ 22 formats (Cargo.toml, package.json, etc.) | ❌ None | okf-generator unique |
| **Cross-references** | ✅ Imports→deps, calls→caller/callee (typed edges) | ⚠️ Markdown links only (untyped) | okf-generator has semantic graph |
| **Incremental updates** | ✅ SHA256 manifest + edge-diff | ⚠️ Full re-scan only | okf-generator more efficient |
| **Bundle diff/impact** | ✅ `okf diff --impact` | ❌ None | okf-generator unique |
| **LLM enrichment** | ✅ 4 tiers, multi-provider routing | ❌ None | okf-generator optional layer |
| **LSP enrichment** | ✅ pyright, gopls, rust-analyzer, TS | ❌ None | okf-generator compiler-accurate |
| **MCP server** | ✅ 11 tools (`lookup`, `get_concept`, `find_callers`, `find_callees`, `list_by_file`, `list_dependencies`, `bundle_info`, `list_by_type`, `search_by_tag`, `get_related`, `get_manifest_source`) | ✅ 11 tools (`search`, `query_metadata`, `traverse`, `get_links`, `get_backlinks`, `get_document`, `get_stats`, `validate`, `scan`, `serve`, `health`) | Both have MCP; different tool shapes |
| **MCP tools: code graph** | ✅ `find_callers`, `find_callees`, `list_dependencies` | ❌ None | okf-generator wins for code |
| **MCP tools: doc graph** | ❌ None | ✅ `traverse`, `get_links`, `get_backlinks` | OKC wins for doc navigation |
| **Visualization** | ✅ D3.js force-directed HTML (offline) | ❌ None | okf-generator unique |
| **Dashboard** | ✅ FastAPI live browser | ❌ None | okf-generator unique |
| **Fine-tuning export** | ✅ `okf pairs` → JSONL (5 pair types) | ❌ None | okf-generator unique |
| **Agent integrations** | ✅ 7 agents via `okf install` | ❌ None | okf-generator broader |
| **GitHub Action** | ✅ Pre-built workflow with impact comments | ❌ None | okf-generator CI-ready |
| **File watcher** | ❌ Manual `okf update` | ✅ `okc watch` (notify) | OKC has live indexing |
| **Search** | ✅ Fuzzy/camelCase, type/tag/file filters, JSON output | ✅ FTS5/BM25, path/type/tag filters | Both strong; different models |
| **Vector/semantic search** | ❌ None (deterministic only) | ❌ None (BM25 only) | Both lack embeddings |
| **Offline/deterministic** | ✅ Core extraction 100% offline | ✅ Fully offline | Both offline-first |
| **License** | MIT | MIT | Same |
| **Language** | Python | Rust | Different stacks |
| **Maturity** | v0.1.51, active releases, 1 star | Pre-1.0, active dev | okf-generator more releases |

---

## Architecture & Code Quality

### okf-generator
- **Structure**: Single Python package (`okf_generator/`) with modules: `cli`, `parsers` (17 language parsers + plugin system), `linker` (cross-reference resolution), `bundle` (OKF v0.2 writer), `enrichment` (4 tiers, multi-provider), `mcp` (FastMCP server), `dashboard` (FastAPI), `visualize` (D3.js), `pairs` (training data), `install` (agent configs), `diff`, `update`, `config`.
- **Lines**: ~15k Python LoC (est. from repo structure).
- **Parsers**: tree-sitter for 16 languages; Python stdlib `ast` for Python. Plugin architecture for external parsers.
- **Linker**: Two-pass — extract concepts → resolve imports/calls across languages. Edge types: `imports`, `calls`, `called_by`, `depends_on`.
- **Incremental**: SHA256 manifest (mtime + content hash) → edge-diff cascade detection → dirty concept rewrite only.
- **Enrichment**: Resumable, checkpointed. 4 modes: `base` (descriptions), `deep` (examples + side effects), `security` (risk audit), `full` (all + semantic links). Multi-provider routing via config (local llama.cpp, Ollama, Claude, OpenAI-compatible).
- **MCP**: FastMCP-based stdio/HTTP server. 11 tools exposing lookup, graph traversal, manifest queries.
- **Quality gates**: `ruff`, `mypy`, `pytest` (CI). Pre-commit hooks. Docker image published (`ghcr.io/umairbaig8/okf-generator/okf-generator`).
- **Testing**: Unit + integration tests. GitHub Actions for multi-version Python matrix.
- **Maturity**: v0.1.51 (51 releases), active development, comprehensive docs site, but low GitHub stars (1).

### OKC
- **Structure**: Single Rust binary crate with modules: `config`, `index` (SQLite+FTS5), `model`, `parser` (markdown front-matter), `scanner` (walkdir + notify), `service` (OkcService), `transport` (CLI + MCP stdio/HTTP).
- **Lines**: ~8k Rust LoC.
- **Database**: SQLite (r2d2 + rusqlite). FTS5 for full-text. No vector/embedding support.
- **Async**: Minimal (MCP uses Tokio). Core indexing synchronous.
- **Architecture**: Service layer over `RepositoryIndex`. Transport-agnostic tools.
- **Code indexing**: Markdown only. No tree-sitter, no code graph.
- **Quality gates**: `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test`.
- **Observability**: `tracing` logs only. No metrics export.
- **Maturity**: Pre-1.0, active development.

---

## MCP Capability Inventory

| Tool | okf-generator | OKC | Overlap |
|------|---------------|-----|---------|
| **Concept lookup** | `lookup` (exact symbol, fuzzy, filters) | `search` (FTS5), `query_metadata` | Different: code symbol vs doc full-text |
| **Concept detail** | `get_concept` (full markdown) | `get_document` (by path) | Similar |
| **Graph traversal** | `find_callers`, `find_callees`, `list_dependencies`, `get_related` | `traverse` (BFS, link graph), `get_links`, `get_backlinks` | Different: call graph vs link graph |
| **Listing/filtering** | `list_by_file`, `list_by_type`, `search_by_tag` | `query_metadata` (structured filter) | Similar capability |
| **Bundle metadata** | `bundle_info`, `get_manifest_source` | `get_stats` | Similar |
| **Code-specific** | `find_callers`, `find_callees`, `list_dependencies` | — | okf-generator only |
| **Doc-specific** | — | `traverse`, `get_links`, `get_backlinks` | OKC only |
| **Auth/scopes** | None | None | Both lack auth |
| **Transports** | stdio, HTTP (FastMCP) | stdio, HTTP/SSE | Both support stdio+HTTP |

---

## Strengths vs OKC

1. **Code intelligence depth** — Tree-sitter AST extraction across 17 languages with typed cross-references (imports, calls, caller/callee) creates a true code knowledge graph. OKC only indexes markdown links.

2. **Manifest ecosystem awareness** — 22 manifest formats (Cargo.toml, package.json, go.mod, requirements.txt, Dockerfile, etc.) auto-indexed and linked to code concepts. OKC has no equivalent.

3. **Incremental update efficiency** — SHA256 manifest + edge-diff cascade detection means 1-file edit touches ~8 concepts in a 68-concept bundle. OKC re-scans entire corpus on every `scan`.

4. **Bundle diff with impact analysis** — `okf diff --impact` shows exactly which dependency changes affect which modules. Critical for CI/CD and architectural review. OKC has no diff capability.

5. **Multi-tier LLM enrichment with provider routing** — 4 enrichment modes (base/deep/security/full), resumable, multi-provider config (local llama.cpp for cheap work, Claude for security). OKC has no enrichment.

6. **LSP-backed call graph** — Optional `okf enrich --lsp` uses pyright/gopls/rust-analyzer/TS-server for compiler-accurate caller/callee resolution at zero token cost. OKC has no code analysis.

7. **Interactive visualization** — Self-contained D3.js force-directed graph HTML (search, filter, dark/light, no server). OKC has no visualization.

8. **Live dashboard** — FastAPI bundle browser with real-time exploration. OKC has no UI.

9. **Fine-tuning dataset generator** — `okf pairs` exports 5 pair types (codegen, QA, doc, summarize, crosslink) as JSONL for training private SLMs. Unique capability; OKC has none.

10. **Broad agent integration** — `okf install` writes config/rules for 7 agents (Claude Code, Cursor, Copilot, Windsurf, Cline, OpenCode, MCP). OKC has no agent installers.

11. **GitHub Action with PR comments** — Pre-built workflow auto-generates bundle on push/PR, diffs with impact, posts PR comment. OKC has no CI integration.

12. **Plugin architecture** — External parser plugins via `okf plugin install`. Extensible to new languages/formats. OKC has no plugin system.

---

## Weaknesses vs OKC

1. **No live file watching** — Requires manual `okf update` or CI trigger. OKC's `okc watch` uses `notify` for instant incremental indexing on file save.

2. **No document-centric workflow** — Optimized for code concepts, not prose documents. OKC's front-matter + markdown model suits knowledge bases, wikis, docs.

3. **Python runtime dependency** — Requires Python 3.10+, tree-sitter wheels, optional LSP servers. OKC is single static Rust binary (easier distribution).

4. **No vector/semantic search** — Purely deterministic lookup (exact/fuzzy/tag). OKC also lacks embeddings, but both share this gap.

5. **MCP tools lack document graph traversal** — No equivalent of OKC's `traverse` (BFS over markdown links with depth/node limits). okf-generator's graph is call/import-based.

6. **Single-threaded enrichment bottleneck** — LLM enrichment runs with limited parallelism (`OKF_MAX_WORKERS=2` default). OKC has no enrichment so N/A.

7. **Lower GitHub adoption** — 1 star vs OKC's active development. Community/ecosystem smaller.

8. **No built-in validation/linting** — OKC has `validate` tool checking index consistency, broken links, front-matter schema. okf-generator lacks bundle validation.

9. **Configuration via env vars + config file** — Mix of `.okfconfig`, env vars (`OKF_ENRICH`, `OKF_API_KEY`, etc.), CLI flags. OKC uses single `okc.toml` + CLI.

10. **No lineage/temporal tracking** — Bundle diff shows changes but no persistent version graph. OKC tracks file mtime/git implicitly but no explicit lineage either.

---

## OKC Improvement Opportunities

| Area | Gap | Competitor Reference | Action |
|------|-----|---------------------|--------|
| **Code indexing** | Markdown only; no AST, no symbols, no cross-refs | okf-generator: 17 langs, tree-sitter, typed edges (imports/calls) | Integrate `tree-sitter` crate for Rust/TS/Go/Python MVP; add `symbol`, `caller`, `callee` concept types |
| **Manifest parsing** | None | okf-generator: 22 formats (Cargo.toml, package.json, go.mod, Dockerfile, etc.) | Add manifest parser module; extract deps as `Dependency` concepts with `depends_on` edges |
| **Incremental updates** | Full re-scan only | okf-generator: SHA256 manifest + edge-diff → dirty-only rewrite | Add `FileManifest` (path, mtime, hash) table; `scan --incremental` mode |
| **Bundle diff/impact** | None | okf-generator: `okf diff --impact` shows dep→module impact | Add `diff` command comparing two index DBs; compute transitive impact via call graph |
| **LLM enrichment** | None | okf-generator: 4 tiers, multi-provider, resumable, checkpointed | Add optional `enrich` pipeline (feature-gated); support local (llama.cpp) + remote providers |
| **LSP enrichment** | None | okf-generator: pyright/gopls/rust-analyzer/TS for compiler-accurate call graph | Integrate `lsp-types` + language server clients for Rust/TS/Go/Python |
| **Visualization** | None | okf-generator: D3.js force-directed HTML (offline) | Add `visualize` command emitting interactive HTML graph |
| **Dashboard** | None | okf-generator: FastAPI live browser | Add optional `okc dashboard` (Axum + HTMX or similar) |
| **Fine-tuning export** | None | okf-generator: `okf pairs` → JSONL (5 pair types) | Add `pairs` command generating training data from index |
| **Agent installers** | None | okf-generator: `okf install` for 7 agents | Add `okc install <agent>` generating MCP config, Cursor rules, etc. |
| **GitHub Action** | None | okf-generator: pre-built workflow with PR impact comments | Publish `.github/workflows/okc-bundle.yml` template |
| **Plugin architecture** | None | okf-generator: `okf plugin install` for external parsers | Define `ParserPlugin` trait; dynamic loading via `libloading` |
| **Validation/linting** | Basic `validate` (index issues) | okf-generator: none | Enhance `validate`: broken links, front-matter schema, orphan concepts, circular deps |
| **MCP code graph tools** | Only doc link graph | okf-generator: `find_callers`, `find_callees`, `list_dependencies` | Add code-graph MCP tools when code indexing implemented |

---

## Threat Level: **Medium**

**Rationale**: okf-generator targets a *different primary use case* (code intelligence for AI coding agents) vs OKC (document catalog for knowledge management/RAG). However, the overlap is significant and growing:
- Both produce OKF v0.2 bundles → same interchange format
- Both expose MCP servers → agents can consume both
- okf-generator's `okf install mcp` + `okf install opencode` directly competes for "AI agent knowledge layer" mindshare
- If OKC adds code indexing (planned), okf-generator becomes a direct alternative for codebases
- okf-generator's 51 releases, comprehensive feature set, and agent integrations show sustained investment
- Low GitHub stars (1) suggests limited community adoption *so far*, but PyPI downloads and active releases indicate real usage

**Monitor**: OKC should track okf-generator's code graph MCP tools — if agents standardize on `find_callers`/`find_callees` interfaces, OKC will need compatible tools to remain interoperable.

---

## Verdict

**Strategic summary**: okf-generator is a **specialized code intelligence engine** that happens to output OKF bundles. OKC is a **general-purpose document catalog** that happens to support OKF. They are complementary today but on a collision course if OKC adds code indexing.

**Priority actions for OKC**:
1. **MVP code indexing** — Integrate tree-sitter for Rust/TypeScript/Go/Python; emit `Function`, `Class`, `Module` concepts with `calls`/`imports` edges. This unlocks parity on the code graph MCP tools.
2. **Incremental scanning** — Adopt SHA256 manifest + dirty-only rewrite (okf-generator's approach is proven efficient).
3. **Bundle diff** — Add `okc diff` with impact analysis; critical for CI/CD adoption.
4. **Agent installers** — Implement `okc install <agent>` for Claude Code, OpenCode, Cursor, Copilot. Low effort, high adoption leverage.
5. **Visualization** — Add `okc visualize` emitting D3.js HTML; strong demo value for users and agents.
6. **MCP tool parity** — When code indexing lands, expose `find_callers`, `find_callees`, `list_dependencies` to match okf-generator's agent-facing API.

**Non-priorities**: LLM enrichment, LSP integration, fine-tuning export, manifest parsing — these are okf-generator's differentiators for *code-specific* workflows. OKC should focus on being the best *document* catalog with *optional* code awareness, not replicate okf-generator's full code intelligence stack.