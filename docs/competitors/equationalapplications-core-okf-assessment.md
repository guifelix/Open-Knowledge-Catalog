# Competitor Assessment: @equationalapplications/core-okf

## Overview

**@equationalapplications/core-okf** (npm: `@equationalapplications/core-okf`, v4.x, MIT, https://github.com/equationalapplications/expo-llm-wiki/tree/main/packages/okf) — Zero-dependency TypeScript library for parsing and producing Open Knowledge Format (OKF) v0.1 bundles. Provides raw primitives for frontmatter serialization, concept documents, index.md/log.md builders, and markdown link extraction, completely decoupled from any database or data model. Part of the equationalapplications/expo-llm-wiki monorepo alongside persistent episodic memory packages (core-llm-wiki, expo-llm-wiki, react-llm-wiki) and tooling (core-llm-tools, prisma-outbox).

The package is the **reference implementation** for the OKF v0.1 specification and the llm-wiki/1 profile, used in production by **Clanker** (AI companion with real-time voice and persistent memory across iOS, Android, and web). It implements conformance fixtures (golden-v1, legacy-profile-0) that validate round-trip fidelity for facts, tasks, events, graph edges, and summary prose. Unlike OKC, it is a **library-only primitive** — no CLI, no MCP server, no storage layer, no search/indexing — designed to be embedded in higher-level applications (core-llm-wiki) that provide SQLite persistence, MCP transport, and agent-facing tools.

---

## Feature Comparison with OKC

| Aspect | @equationalapplications/core-okf | OKC | Notes |
|--------|----------------------------------|-----|-------|
| **Language** | TypeScript | Rust | |
| **Runtime** | Node.js, browser, React Native | Native binary | core-okf: universal JS; OKC: single binary |
| **Dependencies** | Zero (stdlib only) | ~30 crates (rusqlite, tokio, etc.) | core-okf: extreme portability |
| **OKF Support** | v0.1 (reference impl) + llm-wiki/1 profile | OKF v0.1 export/import (planned) | core-okf defines the spec |
| **Storage** | None (caller provides) | SQLite + FTS5 | core-okf: BYO storage |
| **Search/Index** | None | FTS5/BM25 full-text | core-okf: no retrieval |
| **Graph Traversal** | None (link extraction only) | BFS `traverse` tool (depth, node limit) | core-okf: extracts edges; OKC traverses |
| **MCP Server** | ❌ | ✅ (stdio, HTTP, SSE) | core-okf: library only |
| **CLI** | ❌ | ✅ (`okc scan`, `okc serve`, etc.) | core-okf: no binary |
| **File Watcher** | ❌ | ✅ (notify crate) | core-okf: caller responsibility |
| **Vector Search** | ❌ | ❌ (planned) | Both BM25-only currently |
| **Typed Relations** | Markdown links only (`## Related`) | Single `links_to`/`linked_from` | core-okf: richer edge metadata via frontmatter |
| **Fact Lifecycle** | `confidence` (certain/likely/...), `deleted_at` soft-delete | `parse_status` only | core-okf: richer status model |
| **Code Graph** | ❌ | ❌ | Neither does code intelligence |
| **Auth/Scopes** | ❌ | ❌ | Both lack agent auth |
| **Observability** | ❌ | `tracing` logs only | Neither exports metrics |
| **Production Use** | ✅ Clanker (mobile + web) | Pre-1.0, active dev | core-okf: battle-tested in app |
| **Conformance Tests** | ✅ Golden fixtures (golden-v1, legacy) | ❌ | core-okf: spec compliance verified |

---

## Architecture & Code Quality

### @equationalapplications/core-okf
- **Structure**: Single package in monorepo (`packages/okf/`). Source in `src/` with modules: `frontmatter.ts`, `concept.ts`, `index-md.ts`, `log-md.ts`, `markdown-links.ts`, `types.ts`, `entity-index-md.ts`, `related-section.ts`, `path-allowlist.ts`. Entry point `index.ts` re-exports all.
- **Lines**: ~1,500 TypeScript LoC (est. from source files + tests).
- **Build**: `tsup` for ESM/CJS + types. `vitest` for testing.
- **Quality Gates**: `cargo fmt --check` equivalent via `tsc --noEmit`, `eslint` (monorepo), `vitest run`. Conformance tests against golden fixtures ensure spec compliance.
- **Testing**: Unit tests per module (`frontmatter.test.ts`, `concept.test.ts`, `index-md.test.ts`, `log-md.test.ts`, `markdown-links.test.ts`). Integration via `parseOkfBundle`/`formatOkfBundle` in core-llm-wiki.
- **Architecture**: Pure functions, no side effects. Frontmatter uses custom YAML subset serializer (no yaml dep). Markdown link extraction via regex. Path sanitization for Windows reserved names.
- **Maturity**: v4.x (monorepo version), active development (commits June 2026), production-deployed in Clanker.

### OKC
- **Structure**: Single binary crate with modules: `config`, `index`, `model`, `parser`, `scanner`, `service`, `transport` (CLI + MCP).
- **Lines**: ~8k Rust LoC (est.).
- **Database**: SQLite (r2d2 + rusqlite). FTS5 for search. No vector/embedding support.
- **Async**: Minimal (MCP server uses Tokio). Core indexing synchronous.
- **Architecture**: Service layer (`OkcService`) over `RepositoryIndex`. Transport-agnostic tools.
- **Quality Gates**: `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test` (basic).
- **Observability**: `tracing` logs only. No metrics export.
- **Maturity**: Pre-1.0, active development.

---

## MCP Capability Inventory

| Tool/Resource | core-okf | OKC | Notes |
|---------------|----------|-----|-------|
| **Knowledge Ingest** | ❌ (library only) | `scan` (batch index markdown) | core-okf: caller handles persistence |
| **Semantic Search** | ❌ | `search` (FTS5/BM25) | core-okf: no retrieval |
| **Graph Traversal** | ❌ | `traverse` (BFS, depth/node limits) | core-okf: extracts links only |
| **Document Lookup** | ❌ | `get_document` (by path) | core-okf: no storage |
| **Link Resolution** | `extractMarkdownLinks` | `get_links`/`get_backlinks` | core-okf: regex-based extraction |
| **Index/Validate** | ❌ | `get_stats`, `validate` | core-okf: no index |
| **Resources** | ❌ | ❌ | Neither exposes MCP resources |
| **Prompts** | ❌ | ❌ | Neither provides prompt templates |
| **Auth/Scopes** | ❌ | ❌ | Both lack agent authorization |
| **Transports** | N/A | stdio, HTTP, SSE | core-okf: not a server |

---

## Strengths vs OKC

1. **Spec Authority** — core-okf *is* the reference implementation for OKF v0.1 and the llm-wiki/1 profile. Its golden fixtures (golden-v1, legacy-profile-0) define conformance. OKC's OKF support is a consumer, not the standard.

2. **Zero-Dependency Portability** — Runs in Node, browser, React Native, Expo without native bindings. OKC requires Rust toolchain and native SQLite; core-okf works anywhere JS runs (including Clanker's mobile apps via expo-llm-wiki + sql.js).

3. **Richer Fact/Task Model** — Frontmatter includes `confidence` (certain/likely/unconfirmed/inferred), `source_type`, `access_count`, `deleted_at` soft-delete, `okf_type` preservation for custom types. OKC has only `parse_status` and generic front-matter.

4. **Edge Metadata via `## Related`** — Markdown links in concept bodies carry `relation_type` (supports/contradicts/derived_from/etc.) and `target_id` with optional `event_id` comments. OKC has untyped `links_to` only.

5. **Production Hardening** — Battle-tested in Clanker (real-time voice AI companion with persistent memory across platforms). Handles Windows path sanitization, reserved filename avoidance, YAML escaping for control chars.

6. **Profile Versioning** — Explicit `profile: llm-wiki/1` in root index with legacy fallback (profile 0). OKC has no profile concept.

7. **TypeScript First-Class Types** — Full type exports for all frontmatter fields, concept documents, index/log structures. OKC's Rust types don't directly benefit TS/JS consumers.

---

## Weaknesses vs OKC

1. **No Storage or Retrieval** — Caller must implement persistence, indexing, search. OKC provides SQLite+FTS5 out of the box with `scan`/`search`/`traverse`.

2. **No MCP Server** — Cannot be used directly by agents via MCP. Requires wrapper (core-llm-wiki) to expose tools. OKC *is* an MCP server.

3. **No File Watcher / Incremental Indexing** — Caller handles change detection. OKC has `notify`-based watcher with debounced re-index.

4. **No Graph Traversal API** — Only extracts markdown links (`extractMarkdownLinks`). OKC's `traverse` does BFS with depth/node limits and relation filtering.

5. **No CLI** — Cannot run standalone. OKC provides `okc scan`, `okc serve`, `okc validate` for operators.

6. **Single-Entity Scope** — Designed per-entity bundles (entities/{id}/). OKC indexes entire repository roots with multi-root config.

7. **No Observability** — No metrics, health endpoints, or structured logging. OKC has `tracing` (though no Prometheus export yet).

8. **No Auth/Scopes** — Library has no concept of principals or tool authorization. OKC also lacks this, but as an MCP server it's a more critical gap.

---

## OKC Improvement Opportunities

| Area | Gap | Competitor Reference | Action |
|------|-----|---------------------|--------|
| **OKF Conformance** | No golden fixture validation | core-okf: golden-v1/legacy-profile-0 round-trip tests | Add conformance test suite vending core-okf fixtures; verify OKC export/import round-trips |
| **Typed Relations** | Single `links_to` | core-okf: `## Related` with relation_type, event_id | Extend link model: `relation_type`, `confidence`, `evidence_refs` in frontmatter or sidecar |
| **Fact Lifecycle** | `parse_status` only | core-okf: `confidence`, `deleted_at`, `access_count`, `okf_type` | Add `confidence` enum, soft-delete, access tracking to document model |
| **Profile Versioning** | None | core-okf: `profile: llm-wiki/1` + legacy fallback | Add `profile` field to index metadata; implement profile negotiation on import |
| **Path Sanitization** | Basic | core-okf: Windows reserved names, trailing dots, path traversal | Harden export paths per core-okf `path-allowlist.ts` patterns |
| **YAML Safety** | Standard serializer | core-okf: custom subset serializer with key quoting, control-char escape | Audit frontmatter emission for injection safety; adopt subset grammar |
| **Edge Extraction** | Basic link parsing | core-okf: `extractMarkdownLinks` with `## Related` section parsing | Enhance parser to recognize `## Related` semantics; preserve relation metadata |
| **Custom Types** | Fixed document types | core-okf: `okf_type` preserves arbitrary strings | Allow `okf_type` passthrough in frontmatter; don't coerce to known types |
| **Mobile/JS Embedding** | Native binary only | core-okf: runs in React Native, browser via sql.js | Investigate wasm build (wasm-bindgen) or companion JS wrapper for web agents |
| **Conformance CI** | None | core-okf: vitest fixtures in monorepo | Add CI job that pulls core-okf fixtures and validates OKC import/export |

---

## Threat Level

**Low** — Different architectural niche.

**Rationale**: core-okf is a **spec primitive library**, not a knowledge catalog application. It solves "how do I serialize OKF bundles correctly?" — OKC solves "how do I index, search, and serve a markdown knowledge base to agents via MCP?" They are complementary: OKC *should* use core-okf (or port its logic) for OKF import/export conformance. core-okf has no MCP server, no storage, no retrieval, no CLI — it cannot replace OKC for agent-facing use cases. The threat would only materialize if equationalapplications builds an MCP server *on top* of core-llm-wiki (which already exists as a library) and exposes it as a standalone product. Even then, OKC's Rust/SQLite/FTS5 architecture targets different deployment constraints (single binary, no JS runtime).

---

## Verdict

**Strategic Summary**: @equationalapplications/core-okf is the **authoritative OKF v0.1 reference implementation** with production validation in Clanker. It excels at spec compliance, portability, and rich frontmatter semantics — but deliberately stops at serialization. OKC is the **agent-facing MCP server** that *consumes* OKF bundles. The relationship should be **collaborative, not competitive**: OKC should adopt core-okf's conformance fixtures, frontmatter model, and path sanitization to harden its OKF support.

**Priority Actions**:
1. **Adopt conformance fixtures** — Vendor core-okf's `golden-v1` and `legacy-profile-0` bundles; add CI test validating OKC export → import round-trip fidelity.
2. **Enrich frontmatter model** — Add `confidence`, `deleted_at`, `access_count`, `okf_type` fields to OKC's document schema; map to/from OKF on import/export.
3. **Harden serialization** — Port core-okf's YAML subset serializer (key quoting, control-char escaping, Windows path sanitization) to OKC's export path.
4. **Implement `## Related` semantics** — Parse typed relations from markdown links during scan; emit them on OKF export; expose via enhanced `traverse` with `relation_type` filter.
5. **Profile negotiation** — Read/write `profile: llm-wiki/1` in root index; implement legacy fallback for profile 0 bundles.
6. **Consider wasm/JS wrapper** — If web agents need OKC in-browser, a wasm build (or core-okf + sql.js) could bridge the gap core-okf already fills for Clanker.

These six steps close the OKF interoperability gap while preserving OKC's architectural advantages (Rust performance, single binary, MCP-native, FTS5 search).