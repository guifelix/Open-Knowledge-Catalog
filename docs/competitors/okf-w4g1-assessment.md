# Competitor Assessment: okf (W4G1)

## Overview

**okf** (crates.io: `okf`, v0.2.1, 112 downloads, https://github.com/W4G1/okf) — The **reference implementation** of the Open Knowledge Format (OKF) specification. A pure-Rust library + CLI binary for parsing, validating, indexing, and querying OKF bundles. Zero external dependencies. Module structure: `cli/`, `model/`, `validate/`, `provenance/`, `index/`. Apache-2.0 license. 7 stars, 3 forks, 25 commits, single maintainer (W4G1).

okf is the **canonical spec implementation** — it defines what "valid OKF" means. OKC (Open Knowledge Catalog) implements the same v0.2 spec but adds a SQLite+FTS5 index, file watcher, and MCP server for agent consumption. Where okf is a **library for bundle operations**, OKC is a **runtime for knowledge catalogs**. They are complementary: okf validates/produces bundles; OKC indexes and serves them to agents via MCP.

---

## Feature Comparison with OKC

| Capability | okf (W4G1) | OKC | Notes |
|------------|------------|-----|-------|
| **OKF Parser** | ✅ Full spec parser (file/string) | ✅ Via `okf` crate dependency | okf is the reference parser |
| **Document Model** | ✅ Complete `Document`, `Bundle`, `Concept`, `Relation` | ✅ Re-exports okf model | Identical model |
| **Validator** | ✅ Spec-compliant validation | ✅ Uses okf validator | okf = source of truth |
| **Provenance/Trust** | ✅ Attestation chain, trust verification | ❌ Not implemented | okf unique strength |
| **Link Graph** | ✅ Cross-bundle reference resolution | ✅ `traverse` tool (BFS) | okf: resolution; OKC: traversal |
| **Index/Log** | ✅ Content index + audit log | ✅ SQLite FTS5 + file watcher | okf: in-memory; OKC: persistent |
| **CLI Binary** | ✅ `okf` (parse, validate, index, graph) | ✅ `okc` (scan, search, serve) | Different use cases |
| **MCP Server** | ❌ None | ✅ 11 tools, stdio + HTTP/SSE | OKC only |
| **File Watcher** | ❌ None | ✅ Live re-index on change | OKC only |
| **Full-text Search** | ❌ None (index only) | ✅ BM25/FTS5 | OKC only |
| **Vector Search** | ❌ None | ❌ Planned | Neither |
| **Code Graph** | ❌ None | ❌ Planned | Neither |
| **Cross-bundle Query** | ✅ Link graph resolution | ✅ `traverse` across roots | Both, different approach |
| **Persistence** | ❌ In-memory only | ✅ SQLite database | OKC only |
| **Agent Auth/Scopes** | ❌ N/A (library) | ❌ Planned | Neither |
| **MCP Resources** | ❌ None | ❌ Planned | Neither |
| **Observability** | ❌ None | ❌ Logs only | Neither |

---

## Architecture & Code Quality

### okf (W4G1)
- **Structure**: Single crate with 5 modules (`cli`, `model`, `validate`, `provenance`, `index`) + binary entry point. ~6,414 LoC across 22 `.rs` files.
- **Dependencies**: **Zero external dependencies** — pure Rust standard library only. Best-in-class supply chain security.
- **Database**: None. In-memory `Index` and `Log` structures. No persistence layer.
- **Async**: Minimal. CLI is synchronous. Library functions are sync.
- **Architecture**: Clean separation — `model` (data structures), `validate` (spec rules), `provenance` (attestation chain), `index` (content indexing), `cli` (commands).
- **Testing**: **No visible test suite** in repository. No `tests/` directory, no `#[cfg(test)]` modules evident. CI runs `cargo test` but likely passes trivially.
- **Quality Gates**: GitHub Actions (build + test). No `clippy.toml`, `deny.toml`, `rustfmt.toml` visible. No coverage enforcement.
- **Documentation**: docs.rs available. README with usage examples. No architecture docs.
- **Maturity**: v0.2.1 (Jul 27, 2026), 3 versions, 112 downloads, 25 commits — **early stage, pre-1.0**.

### OKC
- **Structure**: Single binary crate with modules: `config`, `index`, `model`, `parser`, `scanner`, `service`, `transport` (CLI + MCP). ~8k LoC est.
- **Dependencies**: `okf` (parser/model/validator), `rusqlite` + `r2d2` (SQLite), `tokio` (MCP), `notify` (file watcher), `clap`, `tracing`, `serde`, `anyhow`.
- **Database**: SQLite (r2d2 pool) with FTS5 virtual table for full-text search. Persistent, file-watched.
- **Async**: Tokio for MCP server (stdio + HTTP/SSE). Core indexing synchronous.
- **Architecture**: Service layer (`OkcService`) over `RepositoryIndex`. Transport-agnostic MCP tools.
- **Testing**: Basic `cargo test` suite. Unit + integration tests for scanner, index, search, traverse.
- **Quality Gates**: `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test` in CI.
- **Observability**: `tracing` structured logs. No metrics export.
- **Maturity**: Pre-1.0, active development, CI passing.

---

## MCP Capability Inventory

| Tool/Resource | okf (W4G1) | OKC | Notes |
|---------------|------------|-----|-------|
| **Parse bundle** | ✅ `okf parse <file>` (library + CLI) | ❌ Internal only | okf: primary use case |
| **Validate bundle** | ✅ `okf validate <file>` | ✅ `validate` tool (index issues) | okf: spec validation; OKC: index health |
| **Index content** | ✅ `okf index <file>` (in-memory) | ✅ `scan` (persistent SQLite) | okf: ephemeral; OKC: durable |
| **Link graph** | ✅ `okf graph <file>` (resolve refs) | ✅ `traverse` (BFS, filters) | okf: resolution; OKC: exploration |
| **Provenance verify** | ✅ `okf trust <file>` (attestation chain) | ❌ None | okf unique |
| **Search (FTS)** | ❌ None | ✅ `search` (BM25, filters) | OKC only |
| **Metadata query** | ❌ None | ✅ `query_metadata` (KV filter) | OKC only |
| **Document fetch** | ❌ None | ✅ `get_document` (by path) | OKC only |
| **Links/backlinks** | ❌ None | ✅ `get_links` / `get_backlinks` | OKC only |
| **Stats/health** | ❌ None | ✅ `get_stats`, `validate` | OKC only |
| **MCP stdio** | ❌ N/A | ✅ `okc serve --stdio` | OKC only |
| **MCP HTTP/SSE** | ❌ N/A | ✅ `okc serve --http` | OKC only |
| **Resources** | ❌ None | ❌ Planned | Neither |
| **Prompts** | ❌ None | ❌ Planned | Neither |
| **Auth/Scopes** | ❌ N/A | ❌ Planned | Neither |

---

## Strengths vs OKC

1. **Zero-dependency supply chain** — Pure Rust, no external crates. Eliminates supply-chain risk, compiles instantly, trivial to audit. OKC pulls ~30 transitive deps.

2. **Reference validator** — okf *is* the spec. Its validator defines conformance. OKC delegates to okf for parsing/validation, ensuring compatibility by construction.

3. **Provenance & trust model** — First-class attestation chain (`Attestation`, `TrustRoot`, `VerificationResult`) with cryptographic verification. OKC has no equivalent; critical for supply-chain/knowledge integrity use cases.

4. **Cross-bundle link resolution** — `index.resolve_reference()` handles inter-bundle references with proper scoping. OKC's `traverse` operates on indexed links but doesn't model bundle-level resolution semantics.

5. **Minimal, embeddable** — As a library, okf integrates into any Rust project (build tools, generators, linters). OKC is a standalone server binary.

6. **Spec completeness** — Implements the full OKF v0.2 model: `Document`, `Bundle`, `Concept`, `Relation`, `Attestation`, `Index`, `Log`. OKC uses subset (documents + links + front-matter).

7. **CLI for bundle operations** — `okf parse|validate|index|graph|trust` covers the full bundle lifecycle. OKC's CLI focuses on catalog operations (scan, search, serve).

---

## Weaknesses vs OKC

1. **No persistence** — In-memory `Index` and `Log` vanish on process exit. OKC's SQLite+FTS5 survives restarts, supports incremental updates via file watcher.

2. **No MCP server** — Cannot be consumed by agents directly. OKC exposes 11 MCP tools over stdio/HTTP/SSE for agent integration.

3. **No full-text search** — `Index` provides concept lookup but no BM25/ranked search. OKC's `search` tool returns scored excerpts with filters.

4. **No file watcher / live indexing** — Must re-parse bundles manually. OKC watches roots and re-indexes changed files automatically.

5. **No metadata query** — Cannot filter by front-matter key/value. OKC's `query_metadata` enables structured faceted search.

6. **No document fetch by path** — Library returns parsed model; no path-based retrieval. OKC's `get_document` serves raw markdown + metadata.

7. **No graph traversal API** — `graph` command resolves references but no BFS/DFS with depth/node limits. OKC's `traverse` supports `max_depth`, `max_nodes`, relation filter.

8. **No test suite visible** — Risk of regressions. OKC has basic test coverage enforced in CI.

9. **Single maintainer, low bus factor** — 7 stars, 25 commits, no visible contributors. OKC has active development.

10. **No observability** — No logs, metrics, health endpoints. OKC has structured `tracing` logs (though no metrics export yet).

---

## OKC Improvement Opportunities

| Area | Gap | Competitor Reference | Action |
|------|-----|---------------------|--------|
| **Provenance/Trust** | None | okf: `Attestation`, `TrustRoot`, `VerificationResult`, `trust` command | Adopt okf's provenance model; add `attestation` front-matter; implement `verify` tool |
| **Bundle-level resolution** | File-level links only | okf: `Index::resolve_reference()` across bundles | Extend link model with `bundle_id`; add cross-bundle `traverse` |
| **Spec validation** | Delegates to okf (good) | okf: reference validator | Continue using okf crate; track spec versions in `Cargo.toml` |
| **Zero-dep core** | ~30 transitive deps | okf: zero deps | Consider `okf` as optional feature; keep core minimal |
| **CLI bundle ops** | Catalog-focused only | okf: `parse`, `validate`, `index`, `graph`, `trust` | Add `okc bundle` subcommands wrapping okf CLI for bundle authoring |
| **Link semantics** | Single `links_to` | okf: `Relation` with typed `relation_type` | Adopt okf's `Relation` model; support typed links in markdown (`[[type:target]]`) |
| **Audit logging** | None | okf: `Log` (append-only operations) | Add `audit_log` table; emit events for index mutations |
| **Concept model** | Front-matter only | okf: `Concept` with `id`, `type`, `properties`, `relations` | Map front-matter to `Concept`; enable concept-centric queries |
| **Index persistence** | SQLite FTS5 only | okf: in-memory `Index` (concept → locations) | Keep SQLite; consider exporting okf-compatible index format |
| **CI quality gates** | fmt/clippy/test | okf: basic build+test only | Add `deny.toml`, coverage threshold, clippy pedantic |

---

## Threat Level

**Low** — okf is a **complementary library**, not a competing product.

- **Different layer**: okf = spec implementation (library); OKC = runtime catalog (server).
- **Shared dependency**: OKC *uses* okf for parsing/validation. They are aligned by design.
- **No MCP overlap**: okf has no agent interface; OKC's primary value prop is MCP.
- **No persistence/search overlap**: okf is ephemeral/in-memory; OKC is durable/queryable.
- **Ecosystem role**: okf enables OKF adoption (generators, validators, linters). OKC benefits from a healthy okf.

**Risk**: If okf adds persistence + MCP server, it could become a direct competitor. Current architecture (zero-dep library) makes this unlikely — adding SQLite/tokio would violate its core design principle.

---

## Verdict

**okf (W4G1) is the upstream spec implementation that OKC depends on — not a competitor.** The relationship is **symbiotic**: okf defines and validates the OKF format; OKC indexes and serves OKF-compatible content to agents via MCP.

**Strategic implications for OKC:**

1. **Track okf versions tightly** — Pin `okf` in `Cargo.toml`; test against each release. OKC's compatibility *is* okf's conformance.

2. **Adopt okf's provenance model** — The `Attestation`/`TrustRoot`/`VerificationResult` types are the missing piece for supply-chain trust in OKC catalogs. Add optional `attestation` front-matter and a `verify` MCP tool.

3. **Align link model with okf's `Relation`** — okf supports typed relations (`relation_type: String`) with metadata. OKC's single `links_to` is a subset. Extend markdown link syntax (`[[supports:path]]`) and map to okf `Relation`.

4. **Expose okf CLI via OKC** — Add `okc bundle parse|validate|graph|trust` subcommands that delegate to okf. Makes OKC a full OKF toolchain, not just a catalog.

5. **Contribute upstream** — OKC's file watcher, incremental indexing, and FTS5 integration are valuable patterns. Consider contributing a `persistent-index` feature to okf (opt-in, behind feature flag) or a companion crate.

6. **Monitor for scope creep** — If okf adds `async`, `sqlx`, or `tokio` dependencies, it loses its zero-dep advantage. Advocate for keeping okf pure; build persistence in OKC or a separate `okf-persist` crate.

**Recommended priority:**
1. Integrate okf provenance types → OKC front-matter + `verify` tool
2. Extend link syntax to support okf `Relation` types
3. Add `okc bundle` subcommands wrapping okf CLI
4. Track okf spec evolution (v0.3+) in OKC roadmap

These steps deepen the symbiotic relationship and close OKC's trust/verification gap without architectural disruption.