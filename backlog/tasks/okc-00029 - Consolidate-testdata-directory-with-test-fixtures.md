---
id: OKC-00029
title: Consolidate testdata/ directory with test fixtures
status: To Do
assignee: []
created_date: '2026-07-23 23:30'
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
- [ ] #1 testdata/ no longer duplicates test fixtures
- [ ] #2 All tests and benchmarks continue to pass
- [ ] #3 Clear separation of purpose between testdata/ and tests/fixtures/
<!-- AC:END -->
