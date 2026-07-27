# Changelog

All notable changes to OKC (Open Knowledge Catalog) are documented here.

The format follows [Keep a Changelog](https://keepachangelog.com/),
and this project uses [Semantic Versioning](https://semver.org/).

## [0.1.0] — 2026-07-27

Initial public release of OKC — a local-first tool for AI agents to
browse, parse, search, and reason over Open Knowledge Format (OKF) repositories.

### Added

- **Scanner**: Parallel filesystem walker with gitignore support, front-matter
  boundary detection, YAML parsing, and Markdown structure extraction (headings,
  links, sections).
- **Incremental scanning**: Content-hash based change detection (Blake3 sampling)
  for fast re-scans of unchanged documents.
- **Filesystem watcher**: `okc watch` — notify-based debounced watcher with
  editor temp-file filtering, gitignore support, and periodic full reconciliation.
- **Persistent index**: SQLite-backed storage with FTS5 full-text search, BM25
  relevance ranking with configurable field weights, and link graph for
  bidirectional navigation.
- **CLI commands**:
  - `okc scan` — Index a knowledge repository
  - `okc browse` — Browse directory hierarchy
  - `okc get` — Retrieve documents with metadata, headings, and body
  - `okc section` — Extract specific Markdown sections
  - `okc search` — Full-text search with BM25 ranking
  - `okc metadata` — Structured metadata queries with filtering and projection
  - `okc links` / `okc backlinks` — Link navigation
  - `okc traverse` — Graph traversal with depth limits
  - `okc validate` — 8-category repository validation (broken links, malformed
    YAML, circular references, duplicate content, missing index files)
  - `okc stats` — Index statistics
  - `okc serve` — MCP server (stdio or HTTP/SSE transport)
  - `okc watch` — File system watching with incremental updates
- **MCP server**: 11 AI-facing tools exposed via stdio or HTTP/SSE transport
  using the Model Context Protocol.
- **TOML configuration**: Layered config (defaults → file → env vars → CLI flags),
  config file auto-discovery at `./okc.toml` or `~/.config/okc/config.toml`.
- **Input validation**: Size limits, path confinement, and structured error types
  for all user-facing operations.
- **JSON output**: `--json` flag for non-MCP agent consumption.
- **Property-based tests**: proptest suite for front-matter parsing, YAML
  deserialization, link resolution, and path normalization.
- **Fuzz targets**: 3 cargo-fuzz targets for front-matter extraction, YAML
  parsing, and path normalization.
- **Criterion benchmarks**: 8 benchmark groups across all core operations,
  parameterized by corpus size.

### Changed

- Renamed project from `okf` to `okc` to avoid confusion with the upstream
  specification.
- Refactored monolithic `database.rs` (1265 lines) into focused storage modules.
- Split `service/mod.rs` (120+ lines) into domain-focused modules.
- Extracted trait interfaces (`DocumentStore`, `SearchIndex`, `GraphStore`)
  for storage abstraction.
- Replaced `RefCell`/`Mutex` with `r2d2` connection pool for thread-safe
  concurrent access.
- Upgraded `rmcp` from 1.4.0 to 2.2.0 LTS.

### Fixed

- SQL injection in `query_metadata` metadata query endpoint.
- Path traversal vulnerability in link resolver.
- Broken links in validation reports.
- FTS5 schema mismatch after crate rename.
- Numerous clippy warnings across the codebase.

### Security

- Zero vulnerabilities verified with `cargo-audit` after all upgrades.
- Path confinement enforced on all user-provided paths.
- Size limits on file reads, front-matter, and response bodies.
- SQL injection prevention via column whitelisting.

### Removed

- Wildcard imports replaced with explicit imports throughout.
- Unreferenced `testdata/` directory.
- `unwrap()` calls from production code (mutex poisoning handled gracefully).
- Orphaned module roots after refactors.
