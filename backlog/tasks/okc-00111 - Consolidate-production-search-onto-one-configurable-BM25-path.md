---
id: OKC-00111
title: Consolidate production search onto one configurable BM25 path
status: Done
assignee:
  - '@codex'
created_date: '2026-08-06 23:05'
updated_date: '2026-08-06 23:12'
labels:
  - search
  - bm25
  - backend
  - quality
dependencies:
  - OKC-00110
references:
  - src/index/queries/search.rs
  - src/index/search_index.rs
  - src/service/search.rs
documentation:
  - docs/search-baseline-v1.md
priority: high
type: enhancement
ordinal: 78000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Eliminate the divergent full-text query implementations identified by search baseline v1. CLI, service, MCP, and the SearchIndex abstraction must execute one canonical SQLite FTS5/BM25 implementation so configuration, filtering, counts, ranking, and future retrieval experiments cannot drift across paths. This task preserves search semantics; typo fallback and graph expansion remain separate follow-up capabilities.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 CLI, service, MCP, and SearchIndex calls execute one canonical search implementation
- [x] #2 Configured BM25 field weights are applied by the production search path
- [x] #3 Path-prefix, type, and tag filters preserve documented behavior, including combined filters
- [x] #4 total_matches counts the complete filtered set, truncated reflects the requested limit, and equal-score ordering is deterministic
- [x] #5 The duplicate query implementation is removed or reduced to a thin delegation layer that cannot diverge
- [x] #6 Search baseline v1 has no Recall@5, Recall@10, MRR@10, intentional-zero-hit, or exact-query regression
- [x] #7 Service and packaged-MCP tests cover ranking, combined filters, limits, counts, and empty results
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 Production-path audit documentation names only the canonical implementation
- [x] #2 Search evaluation and full Rust quality gates pass
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add regression tests for configurable field-weight ranking, combined filters, deterministic limited pages, complete counts, empty results, packaged MCP behavior, and baseline metric floors.
2. Make SqliteSearchIndex::search the canonical parameterized FTS5/BM25 query with full filtered counts and stable tie ordering.
3. Change RepositoryIndex::search into filter translation plus SearchIndex delegation and remove the duplicate query module.
4. Update the ADR/baseline audit to name the single path, then run focused and full Rust quality gates before finalization.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Consolidated runtime search into SqliteSearchIndex::search; RepositoryIndex now only translates public arguments to SearchFilters. Canonical query applies all configured FTS5 column weights (including zero path weight), uses EXISTS tag filters, normalizes slash-delimited prefixes, counts the complete filtered set separately, and orders score ties by path. Removed src/index/queries/search.rs. Focused service, packaged MCP, configured-ranking, and baseline regression tests pass.

Final validation: cargo fmt --check, cargo test (280 non-doc tests plus 2 doc tests), cargo clippy -- -D warnings, and git diff --check all passed. Regression coverage proves configurable field-weight ranking, combined filters, complete counts/truncation, deterministic tie ordering, empty results, packaged MCP behavior, and baseline metric floors.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Consolidated CLI/service/MCP search onto SqliteSearchIndex::search, removed the divergent query module, and made configured BM25 weights, combined filters, complete counts, truncation, and deterministic ordering authoritative in one implementation. Updated the search audit/ADR and verified the change with 282 Rust tests, formatting, strict Clippy, and diff checks.
<!-- SECTION:FINAL_SUMMARY:END -->
