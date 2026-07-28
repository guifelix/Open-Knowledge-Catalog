# Competitor Assessment: gnosis + dkp (Enterprise Knowledge Ecosystem)

## Overview

**gnosis** (crates.io: `gnosis`, v0.1.0, https://github.com/Tools-cx-app/gnosis) — **Privileged Linux container runtime and CLI**. Manages lightweight system containers with Linux namespaces, mounts, cgroups, networking, and persistent TOML configuration. Workspace with 3 crates: `gnosis-cli`, `gnosis-config`, `gnosis-runtime`. **Not a knowledge management tool** — this is a container orchestration utility for system administrators.

**dkp / dkp-core / dkp-gen-core** (crates.io: `dkp`, `dkp-core`, `dkp-gen-core`, https://github.com/dkp-standard) — **Domain Knowledge Pack (DKP) Standard**: Open standard for packaging curated domain knowledge into validated, structured bundles so both Human and AI agents stop hallucinating and start knowing. CLI for authoring, inspecting, deploying packs. 8-gate quality certification, supply-chain integrity, MCP-first integration. Pure-Rust PDF/EPUB generation (markdown2pdf, epub-builder + pulldown-cmark). ~3 crates in workspace.

Both are **Rust projects** on crates.io but serve **fundamentally different domains**: gnosis = container runtime; dkp = knowledge packaging standard for AI agents. Only **dkp** overlaps with OKC's knowledge-catalog mission.

---

## Feature Comparison with OKC

| Aspect | gnosis | dkp | OKC | Notes |
|--------|--------|-----|-----|-------|
| **Primary domain** | Linux container runtime | Knowledge packaging for AI agents | Markdown knowledge catalog + MCP server | gnosis: zero overlap; dkp: high overlap |
| **Knowledge model** | N/A (container config) | Structured bundles: manifest, chunks, knowledge graph, eval sets, MCP tools | Markdown files + front-matter + link graph | dkp richer: provenance, rights, audience scoping, localization |
| **Schema/ontology** | TOML config schema | Formal manifest schema (`skill-pack.json` equivalent) | Implicit via front-matter conventions | dkp has explicit versioned schema |
| **Validation** | Config validation at runtime | 8-gate quality certification (lint, schema, links, eval) | `validate` (index issues, broken links) | dkp certification far more rigorous |
| **Packaging format** | N/A | Single self-contained bundle (directory + manifest) | Directory of markdown files (git-native) | dkp: distributable artifact; OKC: living repo |
| **MCP integration** | None | MCP tools bundled in pack (`mcp` feature flag) | MCP server (`okc serve`) | dkp: pack-level MCP; OKC: repo-level MCP |
| **Agent readiness** | None | First-class: packs teach agents domain knowledge | MCP tools for search/traverse | dkp designed for agent consumption |
| **Provenance/traceability** | Container image digest | Supply-chain integrity, rights tracking, audit trail | Git history (implicit) | dkp explicit; OKC via git |
| **Localization** | None | Audience scoping, localization support | None | dkp enterprise feature |
| **Evaluation sets** | None | Built-in eval sets for quality gates | None | dkp unique: test-driven knowledge |
| **Output formats** | Container runtime | PDF, EPUB, HTML (pure Rust) | Markdown, JSON (API) | dkp-gen-core: zero external deps |
| **Distribution** | Cargo install, binaries | Registry (git/local), `dkp install` | Git clone, Cargo install | dkp has registry model |
| **Governance** | Single maintainer | Open standard (dkp-standard org) | Single project | dkp: community standard |

---

## Architecture & Code Quality

### gnosis
- **Structure**: 3-crate workspace (`gnosis-cli`, `gnosis-config`, `gnosis-runtime`), virtual workspace root
- **Lines**: ~2-3k Rust LoC (est. from crate sizes)
- **Architecture**: CLI → config parsing → runtime (namespaces, cgroups, mounts). Privileged operations via capabilities.
- **Database**: None (TOML config files)
- **Async**: Tokio for CLI, runtime uses `nix`/`libc` syscalls
- **Quality gates**: Standard `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test`
- **Observability**: Basic logging via `tracing`
- **Maturity**: v0.1.0, early stage, single maintainer (Tools-cx-app)
- **Security**: Privileged container operations — requires careful capability management

### dkp (dkp-standard/cli + core crates)
- **Structure**: Workspace with `dkp` (CLI), `dkp-core` (domain types, validation, manifest), `dkp-gen-core` (PDF/EPUB generation)
- **Lines**: ~15-20k Rust LoC (est. across 3 crates)
- **Architecture**: Clean separation — `core` = pure domain logic (manifest, validation, graph, eval), `cli` = command dispatch, `gen-core` = rendering pipeline
- **Database**: None (file-based bundles, optional registry index)
- **Async**: Tokio for CLI, sync core
- **Quality gates**: `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test --all-targets`, likely high coverage (eval sets as tests)
- **Observability**: Structured logging, JSON output for CI/CD
- **Maturity**: Active (2026-06-28 releases), 3 repos under dkp-standard org, Apache-2.0/MIT
- **Innovation**: Pure-Rust document generation (no pandoc/LaTeX), eval-driven quality gates, MCP tool bundling

### OKC
- **Structure**: Single binary crate with modules: `config`, `index`, `model`, `parser`, `scanner`, `service`, `transport` (CLI + MCP)
- **Lines**: ~8k Rust LoC (est.)
- **Database**: SQLite (r2d2 + rusqlite), FTS5 for search
- **Async**: Minimal (MCP server uses Tokio), core indexing synchronous
- **Quality gates**: `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test` (basic)
- **Observability**: `tracing` logs only, no metrics export
- **Maturity**: Pre-1.0, active development

---

## MCP Capability Inventory

| Tool/Resource | gnosis | dkp | OKC | Notes |
|---------------|--------|-----|-----|-------|
| **Knowledge ingest** | ❌ | `dkp build` (validates + bundles) | `scan` (index markdown) | dkp: pack creation; OKC: repo indexing |
| **Semantic search** | ❌ | ❌ (pack provides chunks for external RAG) | `search` (FTS5/BM25 only) | Both lack vector search |
| **Graph traversal** | ❌ | Knowledge graph in pack (edges + nodes) | `traverse` (BFS link graph) | dkp: pack-internal; OKC: repo-wide |
| **Item lookup** | ❌ | Via pack manifest + chunk index | `get_document` (by path) | dkp: structured chunks |
| **Context packs** | ❌ | Pack = bounded context for agent | ❌ | dkp unique: pack as context unit |
| **Feedback/quality** | ❌ | 8-gate certification, eval sets | `validate` (structural) | dkp: test-driven knowledge |
| **Job/status** | ❌ | `dkp validate`, `dkp build` (sync) | `get_stats`, `validate` | Both sync |
| **Code graph** | ❌ | ❌ | ❌ | None have code intelligence |
| **File indexing** | ❌ | Pack chunks (markdown + structured) | `scan` (markdown only) | dkp: curated chunks |
| **Resources** | ❌ | Pack manifest as resource | ❌ | dkp exposes pack metadata |
| **Prompts** | ❌ | Pack can include agent prompts | ❌ | dkp: prompt templates in pack |
| **Auth/scopes** | ❌ | Pack-level audience scoping | None | dkp: audience in manifest |
| **Transports** | CLI only | CLI + MCP (feature flag) | stdio, HTTP/SSE | dkp MCP optional |

---

## Strengths vs OKC

### gnosis
1. **None relevant** — wrong domain (container runtime vs knowledge catalog)

### dkp
1. **Knowledge-as-artifact model** — Packs are versioned, signed, distributable units with explicit manifests. OKC's git-repo-as-catalog is more fluid but less portable.
2. **8-gate quality certification** — Schema validation, link checking, eval sets, provenance, rights, audience scoping, localization, MCP readiness. Far exceeds OKC's `validate`.
3. **Eval-driven knowledge** — Built-in evaluation sets mean knowledge packs are *tested* for agent usability, not just structurally valid.
4. **MCP-first pack design** — Packs declare MCP tools they provide; agents install pack → get tools. OKC serves one MCP server per repo.
5. **Supply-chain integrity** — Manifest includes provenance, rights, checksums. Critical for enterprise/regulated deployments.
6. **Audience scoping + localization** — Same pack serves different audiences (dev vs compliance vs support) and languages.
7. **Pure-Rust document generation** — PDF/EPUB/HTML from markdown with zero external deps (markdown2pdf, epub-builder). OKC has no rendering pipeline.
8. **Registry model** — Git-based or local registries with `index.json` for discovery/install. OKC relies on git remotes.
9. **Formal schema** — `skill-pack.json` manifest schema enables tooling, validation, IDE support. OKC uses implicit front-matter conventions.

---

## Weaknesses vs OKC

### gnosis
1. **Wrong problem space** — Not a knowledge tool; no relevance to OKC's mission

### dkp
1. **Authoring friction** — Creating a valid pack requires manifest, chunks, graph, eval sets, MCP config. OKC: write markdown, run `scan`.
2. **No living repository model** — Packs are published artifacts; updating knowledge = new pack version. OKC's markdown files are the source of truth, editable in place.
3. **No repository-wide graph** — Pack graph is internal; no cross-pack traversal (unless registry provides overlay). OKC's `traverse` spans entire repo.
4. **Heavier CLI** — `dkp` does validation, building, rendering, registry ops. OKC's `scan` is fast and incremental.
5. **No incremental indexing** — Pack rebuild is full. OKC's SQLite + FTS5 supports incremental updates.
6. **MCP per-pack, not per-repo** — Agent must install multiple packs for multi-domain knowledge. OKC serves one MCP with all domains.
7. **Early ecosystem** — Few packs exist; registry sparse. OKC works out of box on any markdown repo.
8. **No search API** — Packs provide chunks for external RAG; no built-in query engine. OKC has `search` + `query_metadata`.

---

## OKC Improvement Opportunities

| Area | Gap | dkp Reference | Action |
|------|-----|---------------|--------|
| **Knowledge packaging** | No distributable artifact format | `dkp build` → versioned pack with manifest | Add `okc pack` command: bundle indexed repo into signed, versioned artifact with manifest |
| **Quality gates** | Basic `validate` only | 8-gate certification (schema, links, eval, provenance, rights, audience, locale, MCP) | Extend `validate` with pluggable gates; add `eval` subcommand for test-driven knowledge |
| **Eval sets** | None | Built-in eval sets per pack | Add `okc eval` running golden Q&A pairs against search/traverse; CI integration |
| **Provenance & rights** | Git history only | Manifest fields: `provenance`, `rights`, `license`, `checksums` | Add optional front-matter fields; emit in pack manifest |
| **Audience scoping** | None | `audience` in manifest (dev, compliance, support, etc.) | Add `audience` tag to documents; filter MCP tools by audience |
| **Localization** | None | `locales` in manifest, per-chunk translations | Add `locale` front-matter; `okc pack --locale` |
| **MCP tool bundling** | Repo-level only | Pack declares MCP tools it provides | Allow `okc pack` to embed MCP tool defs; agent installs pack → gets tools |
| **Registry/discovery** | Git remotes only | `dkp install` from registry (git/local), `index.json` | Add `okc registry` commands; publish packs to registry |
| **Document rendering** | None | Pure-Rust PDF/EPUB/HTML (dkp-gen-core) | Add `okc render` (PDF/HTML) using markdown2pdf/epub-builder |
| **Supply-chain integrity** | None | Manifest checksums, signatures, provenance chain | Add `okc pack --sign` (cosign/minisign); verify on install |
| **Cross-pack graph** | Single repo only | Registry could overlay packs | Design `okc repo-set` for multi-repo traverse (like relay-knowledge) |

---

## Threat Level

| Competitor | Threat Level | Rationale |
|------------|--------------|-----------|
| **gnosis** | **None** | Container runtime, not knowledge management. Zero market overlap. |
| **dkp** | **Medium-High** | Directly targets "knowledge for AI agents" with enterprise features OKC lacks: packaging, certification, provenance, MCP bundling, registry model. If dkp-standard gains traction as *the* knowledge pack format, OKC becomes a pack *authoring tool* rather than a standalone catalog. |

---

## Verdict

**gnosis** is a **non-competitor** — misclassified in the task. It solves Linux container orchestration, not knowledge management. Ignore.

**dkp (Domain Knowledge Pack Standard)** is the **real competitive threat** and **strategic reference**. It represents the emerging "knowledge packaging" paradigm: curated, validated, versioned, agent-ready knowledge bundles with explicit provenance, quality gates, and MCP integration.

**OKC's competitive position**: Lightweight, markdown-native, git-centric, zero-config catalog for documentation repositories. Best for: personal knowledge bases, team wikis, project docs, small orgs. **Not** an enterprise knowledge packaging platform.

**To remain relevant as agents adopt MCP**, OKC must evolve from "markdown indexer + MCP server" to **"pack authoring toolkit + catalog server"**:

1. **Adopt pack model** — `okc pack` creates dkp-compatible (or OKC-native) bundles with manifest, eval sets, MCP tools
2. **Add quality gates** — Pluggable validation pipeline (schema, links, eval, provenance) matching dkp's 8 gates
3. **Enable registry workflow** — `okc publish` / `okc install` for pack distribution
4. **Support audience scoping** — Filter knowledge by audience in MCP tools
5. **Integrate eval-driven development** — Golden Q&A pairs as first-class test artifacts

**Recommended priority** (6 steps, 80% gap closure):
1. Add `relation_type` + `confidence` + `evidence_refs` to link model; emit typed relations from markdown syntax (e.g., `[[supports:path]]`)
2. Integrate `sqlite-vec` (feature-flagged) for hybrid BM25+vector search
3. Tree-sitter indexing for Rust/TS/Go/Python → `symbol`, `definition`, `references` MCP tools
4. `context_pack` tool composing search + traverse + metadata with `max_tokens` budget
5. Scope-based tool authorization + `okc://index/status` resource
6. Background scan job + `job_status` tool

These increments preserve OKC's simplicity advantage while closing the agent-facing capability gap with dkp.