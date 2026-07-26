---
id: DRAFT-00027
title: Produce public performance numbers + criterion benchmarks for core ops
status: To Do
assignee:
  - '@backend-agent'
created_date: '2026-07-25 20:03'
labels:
  - performance
  - docs
dependencies: []
documentation:
  - docs/performance.md
priority: medium
type: task
ordinal: 38000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Claims about parsed once, incremental, context-efficient need numbers. Publish latency / throughput for scan, search, get_document, and graph traversal on a realistic corpus size so users can decide whether the tool is fast enough.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Criterion benches for scan, search, get, traverse on a fixture of meaningful size
- [ ] #2 README or docs contain a short Performance section with measured numbers
- [ ] #3 Numbers are reproducible (fixture + command documented)
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Benches run in CI or at least on demand without failure
- [ ] #2 Docs updated with methodology
<!-- DOD:END -->
