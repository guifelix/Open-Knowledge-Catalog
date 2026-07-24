---
id: OKC-00029
title: Consolidate testdata/ directory with test fixtures
status: Done
assignee:
  - '@felix'
created_date: '2026-07-23 23:30'
updated_date: '2026-07-24 01:19'
labels: []
dependencies: []
priority: medium
ordinal: 21000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
testdata/datasets/customer-orders.md and testdata/metrics/monthly-revenue.md duplicate content already present in tests/fixtures/simple/. This creates confusion about which test data is authoritative. Consolidate by either removing testdata/ and using tests/fixtures/ exclusively, or clearly delineating testdata/ as a separate purpose (e.g., benchmark data).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 testdata/ no longer duplicates test fixtures
- [x] #2 All tests and benchmarks continue to pass
- [x] #3 Clear separation of purpose between testdata/ and tests/fixtures/
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Verify testdata/ is unreferenced dead code (already confirmed: 0 Rust files reference it). 2. Remove testdata/ entirely. 3. Run cargo test to confirm no regressions. 4. Run cargo fmt --all -- --check.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Removed unreferenced testdata/ directory. No Rust code or benches referenced testdata/. Verified: cargo test (42/42 pass), cargo fmt --all -- --check passes.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Removed testdata/ directory (completely unreferenced dead code duplicating topics in tests/fixtures/simple/). Verified with cargo test (42 tests pass) and cargo fmt --check (clean).
<!-- SECTION:FINAL_SUMMARY:END -->
