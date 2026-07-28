# Competitor Assessment: okq (mikevalstar)

## Overview

**Project:** okq  
**Repository:** https://github.com/mikevalstar/okq (trustpublish org)  
**License:** Apache-2.0  
**Language:** Rust  
**Architecture:** CLI tool + library for full-text search over OKF bundles using Tantivy  
**Installation:** `cargo install okq` (Rust toolchain required)  
**Stars:** 0 (private org) · **Commits:** Active (9 versions published) · **Maintainer:** mikevalstar  

**Description:** okq is a **search-first** OKF tool — a CLI and library for fast full-text search and navigation over OKF bundles. It uses **Tantivy** (Rust's Lucene-equivalent) as its search engine, providing BM25 ranking, field-specific search, and faceted filtering. Unlike OKC's SQLite+FTS5 approach, okq builds a dedicated Tantivy index from OKF bundles, optimized for search performance over large corpora. It implements OKF v0.1 via a custom `okf-permissive` parser (less strict than the reference `okf` crate). The project has the highest download count in the OKF crate ecosystem (147 downloads across 9 versions), indicating real adoption.

---

## Feature Comparison with OKC

| Feature | okq | OKC | Notes |
|---------|-----|-----|-------|
| **OKF version** | v0.1 (via `okf-permissive`) | v0.2 | OKC ahead on spec |
| **Search engine** | Tantivy (dedicated index) | SQLite FTS5 (embedded) | Different architectures |
| **Full-text search** | ✅ BM25, field queries, facets | ✅ BM25/FTS5, filters | okq: more tuning knobs |
| **Vector/semantic search** | ❌ Not implemented | ⚠️ Planned | Neither has it yet |
| **CLI interface** | ✅ `okq search`, `okq index`, `okq serve` | ✅ `okc search`, `okc scan` | Both have CLI |
| **Library API** | ✅ Embeddable search library | ❌ Internal only | okq: reusable component |
| **MCP server** | ❌ None | ✅ 11 tools, stdio + HTTP/SSE | OKC only |
| **File watcher / live index** | ❌ Batch re-index only | ✅ `notify` + incremental | OKC only |
| **Cross-bundle search** | ✅ Single index spans bundles | ✅ Unified catalog | Both support |
| **Remote bundle fetch** | ❌ Local only | ❌ Local only | Neither |
| **Single binary deploy** | ✅ Rust static binary | ✅ Rust static binary | Parity |
| **Schema validation** | ⚠️ Permissive parser | ✅ Strict (`okf` crate) | OKC stricter |
| **Graph traversal** | ❌ Search-only | ✅ `traverse` tool | OKC only |
| **Lineage/history** | ❌ None | ✅ `lineage` tool | OKC only |
| **Metadata filtering** | ✅ Faceted (tags, types) | ✅ `query_metadata` | Both support |
| **Performance (large corpus)** | ✅ Tantivy optimized | ✅ FTS5 + file watcher | okq: search-optimized |
| **License** | Apache-2.0 | MIT | Both permissive |

---

## Architecture & Code Quality

### okq
- **Structure**: Single crate with `lib.rs` + `main.rs` + modules. ~4,158 LoC across 25 `.rs` files.
- **Search engine**: **Tantivy** — pure Rust, Lucene-compatible inverted index. Supports BM25, phrase queries, field boosting, faceted search, fast startup.
- **Index storage**: Tantivy index directory (separate from source bundles). Must be rebuilt on bundle changes.
- **Parser**: `okf-permissive` (custom, lenient OKF v0.1 parser) — not the reference `okf` crate.
- **Dependencies**: `clap`, `tantivy`, `schemars`, `serde`, `regex`, `ureq`, `ignore`, `pulldown-cmark`, `okf-permissive`. ~50+ transitive deps (Tantivy is heavy).
- **Async**: `ureq` (blocking HTTP) for remote fetches; Tantivy indexing is sync.
- **Testing**: Unit tests present. CI via GitHub Actions (trustpublish org).
- **Packaging**: Cargo binary + library. Prebuilt binaries not published.
- **Maturity**: 9 versions published (v0.1.0 → v0.5.2), 147 downloads — most mature OKF crate by release cadence.

### OKC
- **Structure**: Single binary crate (~8k LoC). Modules: `config`, `index`, `model`, `parser`, `scanner`, `service`, `transport`.
- **Database**: SQLite (r2d2 + rusqlite) with FTS5 virtual table. Persistent, incremental, file-watched.
- **Parser**: Reference `okf` crate (strict, spec-compliant).
- **Dependencies**: `okf`, `rusqlite`, `r2d2`, `tokio`, `notify`, `clap`, `tracing`, `serde`, `anyhow`. ~30 transitive deps.
- **Async**: Tokio for MCP server (stdio + HTTP/SSE). Core indexing synchronous.
- **Testing**: Unit + integration tests. CI: `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test`.
- **Packaging**: Cargo install. No prebuilt binaries yet.
- **Maturity**: Pre-1.0, active development.

---

## Search Capability Deep Dive

### okq (Tantivy-based)
- **Index build**: `okq index <bundle-path>` — parses bundles via `okf-permissive`, extracts fields (title, body, tags, concept_type, bundle_id), writes Tantivy index.
- **Query interface**: `okq search "query" --field tags --facet concept_type --limit 20`
- **Ranking**: BM25 with configurable field weights. Supports phrase queries (`"exact phrase"`), field queries (`title:auth`), fuzzy (`term~`), boost (`term^2`).
- **Facets**: Fast faceted counts on `concept_type`, `bundle_id`, `tags`.
- **Snippets**: Tantivy highlighter returns matched fragments with `<b>` tags.
- **Cross-bundle**: Single index merges all bundles; `bundle_id` field enables filtering.
- **Refresh**: Full re-index required (`okq index` again). No incremental update API exposed.
- **Remote bundles**: Not supported (local filesystem only).

### OKC (SQLite FTS5-based)
- **Index build**: `okc scan` — walks roots, parses markdown via `okf` crate, upserts into SQLite FTS5 + graph tables.
- **Query interface**: `okc search "query" --path-prefix docs --type concept --tag rust --limit 20`
- **Ranking**: BM25 via FTS5. Configurable column weights. Recency boost via `last_modified` column.
- **Facets**: `query_metadata` tool for structured front-matter filtering (key=value).
- **Snippets**: FTS5 `snippet()` function with custom delimiters.
- **Cross-bundle**: Unified catalog; `root_id` column enables root-scoped queries.
- **Refresh**: File watcher (`notify`) triggers incremental re-index on file change. Sub-second latency.
- **Hybrid search**: Planned vector column (`sqlite-vec` or `pgvector`) for BM25+vector RRF fusion.

### Key Differentiators

| Feature | okq (Tantivy) | OKC (SQLite FTS5) |
|---------|---------------|-------------------|
| **Index persistence** | Separate Tantivy dir | Single SQLite file |
| **Incremental updates** | ❌ Full rebuild | ✅ File watcher |
| **Faceted search** | ✅ Native, fast | ⚠️ Via `query_metadata` |
| **Phrase/fuzzy/boost** | ✅ Full Tantivy syntax | ⚠️ FTS5 subset |
| **Schema flexibility** | Fixed at index creation | Dynamic (ALTER TABLE) |
| **Vector search** | ❌ Not in Tantivy yet | ✅ Planned (`sqlite-vec`) |
| **Graph traversal** | ❌ Search-only | ✅ `traverse` tool |
| **Lineage/history** | ❌ | ✅ `lineage` tool |
| **MCP exposure** | ❌ | ✅ 11 tools |

---

## Strengths vs OKC

1. **Tantivy is a best-in-class search engine** — Purpose-built for full-text search with Lucene-grade features (BM25, phrase, fuzzy, facets, highlighting). OKC's FTS5 is capable but less tunable.

2. **Faceted search as a first-class feature** — `okq search --facet concept_type` returns instant facet counts. OKC requires separate `query_metadata` calls.

3. **Library-first design** — okq exposes its search logic as a public Rust library (`okq::Searcher`, `okq::Indexer`). Can be embedded in other Rust tools. OKC's search is internal to the binary.

4. **Highest ecosystem adoption** — 147 downloads / 9 versions = real users. Indicates the search use case resonates.

5. **Permissive parser for messy bundles** — `okf-permissive` tolerates malformed front-matter that the strict `okf` crate rejects. Practical for real-world data.

6. **Search syntax richness** — Field queries, phrase, fuzzy, boost operators out of the box. OKC's FTS5 syntax is more limited.

7. **Performance at scale** — Tantivy's segment-based index handles millions of documents with fast startup. SQLite FTS5 can slow on very large corpora without careful tuning.

---

## Weaknesses vs OKC

1. **No MCP server** — Cannot be consumed by agents directly. OKC's 11 MCP tools over stdio/HTTP/SSE is the primary agent interface.

2. **No live indexing** — Full re-index on every change. OKC's file watcher + incremental SQLite updates provide sub-second live reload.

3. **No graph traversal** — Search-only. No `traverse`, `get_links`, `get_backlinks`, `lineage`. OKC's graph tools are a major differentiator.

4. **No lineage/history** — Concepts have no version tracking. OKC's `lineage` tool tracks splits, merges, renames.

5. **Permissive parser = spec drift risk** — `okf-permissive` accepts invalid OKF. OKC's strict `okf` crate ensures spec compliance.

6. **Separate index management** — Tantivy index directory must be managed separately from source bundles. OKC's single SQLite file is self-contained.

7. **No document ingestion pipeline** — Raw markdown → concepts requires manual front-matter. OKC's planned `ingest` tool will auto-chunk, embed, extract.

8. **No vector/semantic search** — Tantivy doesn't yet support HNSW/IVF. OKC plans `sqlite-vec` integration.

9. **No cross-bundle reference resolution** — Searches across bundles but doesn't resolve `[[wikilinks]]` between them. OKC's unified catalog does.

10. **Single maintainer, private org** — 0 stars, trustpublish org suggests automated publishing but low community visibility. Bus factor = 1.

11. **Heavier dependency footprint** — Tantivy + deps = ~50 transitive crates. OKC's ~30 deps are leaner.

12. **No HTTP/SSE transport** — CLI only. OKC serves remote agents via HTTP/SSE.

---

## OKC Improvement Opportunities

| Area | Gap | Competitor Reference | Action |
|------|-----|---------------------|--------|
| **Faceted search API** | `query_metadata` separate from `search` | okq: `--facet` returns counts inline | Add `facets` param to `search` tool; return facet counts with results |
| **Search syntax richness** | FTS5 subset only | okq: phrase, fuzzy, boost, field queries | Document FTS5 syntax; consider `tantivy` feature flag for advanced syntax |
| **Library exposure** | Search internal only | okq: public `Searcher`/`Indexer` | Expose `okc::search` module as public API for embedding |
| **Permissive parse mode** | Strict `okf` crate only | okq: `okf-permissive` | Add `--lenient` flag to `scan` using permissive parser for messy data |
| **Index performance at scale** | FTS5 can degrade >100k docs | okq: Tantivy segments scale | Benchmark; add `tantivy` backend as optional feature for large catalogs |
| **Facet-aware ranking** | BM25 only | okq: field weights + facets | Add `boost_field` config to `search`; expose in MCP tool schema |
| **Search receipts/audit** | None | okq: none either | **Both gap** — Add `search_receipt` with query, results, timing, index version |
| **Remote bundle search** | Local only | okq: local only | **Both gap** — Add federation (see copperbox assessment) |

---

## Threat Level

**Medium**

**Rationale:**
- **Direct search overlap** — okq targets the exact same "search OKF bundles" use case as OKC's `search` tool. Its Tantivy backend is technically superior for pure search.
- **Library embeddability** — okq as a library can be integrated into other tools (linters, generators, CI checks) where OKC's binary-only approach cannot.
- **Adoption signal** — 147 downloads / 9 versions is the strongest traction in the OKF crate ecosystem.
- **Mitigating factors**: No MCP (agent interface), no live indexing, no graph traversal, no lineage, permissive parser, single maintainer. OKC's broader feature set (MCP + graph + live + lineage) makes it a more complete catalog.
- **Trajectory risk**: If okq adds MCP server + incremental indexing, it becomes a credible search-focused alternative. The library API makes it a natural component for other OKF tools.

---

## Verdict

**okq is the best pure-search implementation in the OKF ecosystem** — Tantivy-backed, library-first, faceted, syntax-rich, and adopted. It wins on **search quality and embeddability**. OKC wins on **catalog completeness** (MCP, graph, live, lineage, lineage, strict validation).

**Strategic implication for OKC:** The search capability gap is real and closable. OKC doesn't need to match Tantivy feature-for-feature, but it should:
1. **Expose faceted counts in `search`** — closes the biggest UX gap for agent consumers.
2. **Consider a `tantivy` feature flag** — optional backend for large catalogs (>50k docs) where FTS5 struggles.
3. **Publish search as a library** — `okc-search` crate enables embedding in linters, generators, CI.
4. **Add search receipts** — provenance for agent trust (neither has this; first-mover advantage).

**Priority adoption order:**
1. Faceted search in `search` tool (immediate)
2. Public `okc::search` library API (Q1)
3. Optional Tantivy backend behind feature flag (Q2)
4. Search receipts / audit trail (Q2)
5. Monitor okq for MCP adoption — if they add it, accelerate vector search + MCP resources

okq validates that **search is a standalone product category** in the OKF ecosystem. OKC should own the "catalog" category (MCP + graph + live + lineage) while matching okq on search ergonomics.