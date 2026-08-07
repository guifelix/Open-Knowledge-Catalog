---
id: OKC-00113
title: Add bounded lexical typo fallback to search
status: Done
assignee:
  - '@codex'
created_date: '2026-08-07 00:06'
updated_date: '2026-08-07 00:12'
labels:
  - search
  - retrieval
  - fts5
  - quality
dependencies:
  - OKC-00111
references:
  - docs/search-baseline-v1.md
  - src/index/search_index.rs
documentation:
  - docs/search-baseline-v1.md
priority: high
type: enhancement
ordinal: 80000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Improve the measured typo/fuzzy retrieval failure without adding embeddings or changing successful exact-query ranking. When the canonical FTS5 search returns no matches, search may perform one bounded lexical correction attempt using the indexed vocabulary, while preserving all filters, limits, complete counts, deterministic ordering, and shared CLI/service/MCP behavior.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 The baseline misspelled monthly-revenue query retrieves its judged relevant document in the top five
- [x] #2 Fallback runs only after the original canonical query returns no matches and performs at most one corrected FTS query
- [x] #3 Exact-query result ordering and all existing search filter, count, truncation, and deterministic-order behavior remain unchanged
- [x] #4 Corrections are bounded by explicit token length and edit-distance rules so unrelated zero-hit queries remain empty
- [x] #5 CLI, service, MCP, and SearchIndex calls share the same fallback behavior through the canonical implementation
- [x] #6 Search evaluation records the improved metrics and enforces the existing proposal-gate non-regression budget
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 Focused typo, negative, filter, and packaged-MCP regression tests pass
- [x] #2 Search baseline documentation records the post-change metrics and algorithm bounds
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add red production-path evaluation, service, and packaged-MCP tests for the judged typo query, unrelated zero-hit queries, filters, limits, and exact-query ordering.\n2. Add an FTS5 vocabulary-backed correction candidate stage inside SqliteSearchIndex that activates only after zero primary matches and allows one bounded retry.\n3. Use deterministic token selection with explicit minimum lengths and edit-distance thresholds; preserve the original response when correction is unsafe or ineffective.\n4. Record post-change relevance/latency metrics and bounds in the baseline, run full quality gates, finalize, and commit atomically.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented zero-hit-only fuzzy fallback in canonical SqliteSearchIndex. It caches a mutation-invalidated FTS5 vocabulary snapshot capped at 50,000 terms; eligible queries have 1-8 ASCII alphabetic tokens of at least five characters, same-initial candidates, and edit distance <=1 for length five or <=2 for longer tokens. Primary zero-hit queries skip the result SELECT; fallback performs at most one corrected query with identical filters/ranking/counts. Measured Recall@5/10=0.7778, MRR@10=0.7222, zero-required-evidence=0.2222, intentional-zero-hit=1.0, p50=285us, p95=483us.

Final validation passed: cargo fmt --check; cargo test (289 non-doc tests plus 2 doc tests); cargo clippy -- -D warnings; git diff --check. Full evaluation enforces Recall@5/10 >=0.7777, MRR@10 >=0.7221, zero-required-evidence <=0.2223, exact recall 1.0, intentional empty 1.0, typo Recall@5 1.0, and p95 <=540us.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added a bounded, zero-hit-only lexical typo fallback to the canonical FTS5 path with cached mutation-safe vocabulary candidates and one corrected retry. The judged typo now succeeds without changing exact or unrelated-zero behavior; Recall@5 improved to 0.7778 and p95 remains within the predeclared budget. Verified across service, filters, packaged MCP, 291 tests, formatting, strict Clippy, and diff checks.
<!-- SECTION:FINAL_SUMMARY:END -->
