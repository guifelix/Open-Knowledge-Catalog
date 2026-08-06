---
id: OKC-00103
title: 'Correct query_metadata projection, filtering, and result counts'
status: Done
assignee:
  - '@codex'
created_date: '2026-08-06 00:47'
updated_date: '2026-08-06 21:03'
labels:
  - mcp
  - backend
  - bug
  - metadata
  - query
dependencies:
  - OKC-00101
references:
  - src/index/queries/metadata.rs
  - src/transport/mcp/mod.rs
documentation:
  - docs/ai-usage.md
priority: high
type: bug
ordinal: 75000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Make query_metadata expose a documented, consistent filter and projection contract across the service and MCP surfaces. Existing type and tag filters are already passed through; the remaining work is to close unsupported projection/filter gaps and compute pagination metadata from the full match set rather than the limited result page.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Documented type, tag, path-prefix, parse-status, and custom-field filters return only matching documents
- [x] #2 Projection supports documented core fields, tags, and requested custom metadata without silently dropping or inventing fields
- [x] #3 Results use deterministic ordering so repeated queries produce stable pages
- [x] #4 total_matches represents the complete filtered match count and truncated is true exactly when the requested limit omits matches
- [x] #5 Invalid filter operators and projection fields return structured validation errors instead of being ignored
- [x] #6 Service and packaged-MCP integration tests assert filtering, projection, combined filters, limits, complete counts, and empty results
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 Unit test covers filter parsing and application
- [x] #2 Integration test verifies MCP tool returns filtered results
- [x] #3 All existing query_metadata tests still pass
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add regression coverage for filtering, projection, deterministic ordering, and complete counts.
2. Refactor metadata queries around validated projections and parameterized EXISTS predicates.
3. Validate MCP filter syntax and document the supported metadata-query contract.
4. Run focused tests, then the full Rust quality gate and finalize the task.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented validated exact-match filtering with path_prefix and tags_contains predicates, arbitrary custom-field filtering/projection, decoded custom JSON values, sorted tag projection, deterministic path ordering, and a separate complete-count query. MCP parsing now rejects malformed and duplicate key=value filters. Updated docs/ai-usage.md and MCP parameter schema descriptions.

Validation: cargo fmt --check; cargo test (269 tests plus 2 doc tests passed); cargo clippy -- -D warnings; git diff --check. Packaged MCP coverage verifies combined filters, projections, count/truncation, empty results, and malformed-filter errors.

Correction to validation count: cargo test passed 267 unit/integration/property tests plus 2 doc tests (269 total).
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Corrected query_metadata across service and MCP: supported filters and projections now behave consistently, pages are stable, counts reflect the full result set, and invalid input is rejected. Verified through unit, service integration, packaged-binary MCP, full Rust test, formatting, lint, and diff checks.
<!-- SECTION:FINAL_SUMMARY:END -->
