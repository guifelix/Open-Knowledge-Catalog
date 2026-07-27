---
type: Documentation
title: Development Guide
description: Prerequisites, test commands, code structure, workflow, and adding tools/backends
tags:
  - development
  - testing
  - contributing
  - workflow
owner: Engineering Team
status: published
---

# Development Guide

## Prerequisites

- Rust 1.75+ (install via [rustup](https://rustup.rs/))
- SQLite3 development headers (`libsqlite3-dev` on Linux)

## Quick Start

```bash
git clone https://github.com/guifelix/Open-Knowledge-Catalog
cd open-knowledge-catalog
cargo build --release
```

Binary at `target/release/okc`.

## Running Tests

```bash
# Unit + integration tests
cargo test

# Property-based tests (proptest)
cargo test --test property_tests

# Fuzz targets (requires cargo-fuzz)
cargo fuzz run frontmatter_extraction
cargo fuzz run yaml_parsing
cargo fuzz run path_normalization

# Benchmarks (criterion)
cargo bench
```

## Test Fixtures

Test suite uses fixture repositories in `tests/fixtures/`:
- `simple/` — Basic structure, valid docs
- `complex/` — Nested dirs, circular links, custom metadata
- `edge-cases/` — Invalid YAML, oversized front matter, broken links

## Code Structure

```
src/
├── main.rs                 # CLI entry point
├── lib.rs                  # Module declarations
├── config.rs               # Configuration types (figment)
├── scanner/
│   ├── mod.rs
│   ├── walker.rs           # Parallel filesystem walker
│   ├── changes.rs          # Incremental change detection
│   └── watcher.rs          # Filesystem watcher (notify)
├── parser/
│   ├── mod.rs
│   ├── frontmatter.rs      # YAML boundary extraction
│   ├── yaml.rs             # saphyr YAML parsing
│   ├── markdown.rs         # pulldown-cmark event parsing
│   └── links.rs            # Link resolution & validation
├── model/
│   ├── mod.rs
│   ├── document.rs         # Document, front-matter, heading, link, section
│   ├── directory.rs        # Directory tree types
│   └── graph.rs            # Graph edge types
├── index/
│   ├── mod.rs
│   ├── database.rs         # Connection, schema, scan orchestration
│   ├── document_store.rs   # Document CRUD + tags/headings/links/metadata
│   ├── search_index.rs     # FTS5 operations
│   ├── graph_store.rs      # Graph edges + traversal
│   ├── queries.rs          # Metadata filtering, browse, get, section
│   ├── validate.rs         # Repository validation (8 checks)
│   ├── export.rs           # JSON export
│   ├── migrations.rs       # Versioned schema migrations
│   ├── graph.rs            # Graph types
│   └── traits.rs           # DocumentStore, SearchIndex, GraphStore traits
├── service/
│   ├── mod.rs
│   ├── browse.rs           # browse_directory
│   ├── documents.rs        # get_document, get_section
│   ├── search.rs           # search_documents
│   ├── graph.rs            # get_links, get_backlinks, traverse_graph
│   └── validation.rs       # validate_repository
└── transport/
    ├── mod.rs
    ├── cli.rs              # Clap CLI definitions
    └── mcp.rs              # MCP server (rmcp)
```

## Development Workflow

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Make changes with tests
4. Run quality checks:
   ```bash
   cargo fmt --check
   cargo clippy -- -D warnings
   cargo test
   ```
5. Submit a pull request

## Code Style

- Follow Rust standard style (`rustfmt`)
- Add tests for new functionality
- Update documentation for user-facing changes
- Use `thiserror` for domain errors, `anyhow` for application errors
- Use `tracing` for structured logging

## Adding a New AI Tool

1. Add service function in `src/service/` (e.g., `my_tool.rs`)
2. Export from `src/service/mod.rs`
3. Add CLI command in `src/transport/cli.rs`
4. Add MCP tool in `src/transport/mcp.rs` (if applicable)
5. Add integration test in `tests/integration_tests.rs`
6. Update `docs/ai-usage.md`

## Adding a Storage Backend

The storage layer uses traits in `src/index/traits.rs`:

1. Implement `DocumentStore`, `SearchIndex`, `GraphStore` for your backend
2. Add backend module in `src/index/`
3. Update `RepositoryIndex` to be generic over the new backend (or add factory)
4. Ensure all integration tests pass with new backend

## Profiling

```bash
# CPU profiling
cargo bench --bench benchmarks

# Memory profiling
cargo install flamegraph
cargo flamegraph --bench benchmarks
```

## Fuzzing

Fuzz targets in `fuzz/fuzz_targets/`:
- `frontmatter_extraction.rs` — Front matter boundary parsing
- `yaml_parsing.rs` — YAML deserialization
- `path_normalization.rs` — Path resolution and safety

Run:
```bash
cargo install cargo-fuzz
cargo fuzz run frontmatter_extraction
```

## Configuration

Uses `figment` for layered config:
1. Defaults (code)
2. Config file (`okc.toml` or `.okc.toml`)
3. Environment variables (`OKC_*`)
4. CLI flags (highest priority)

See `docs/configuration.md` for all options.

## Release Process

1. Update version in `Cargo.toml`
2. Generate changelog: `git cliff --unreleased --tag v0.x.x --prepend CHANGELOG.md`
3. Review and commit the updated `CHANGELOG.md`
4. Tag release: `git tag -a v0.x.x -m "v0.x.x"`
5. Push tag: `git push origin v0.x.x`
6. CI builds, publishes to crates.io, and creates a GitHub Release

See [docs/release-process.md](release-process.md) for the full checklist.

## Useful Commands

```bash
# Check formatting
cargo fmt --check

# Lint
cargo clippy -- -D warnings

# Test with all features
cargo test --all-features

# Generate docs
cargo doc --no-deps --open

# Check for unused dependencies
cargo machete

# Audit dependencies
cargo audit
```