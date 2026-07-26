---
id: OKC-00041
title: 'Improve Output Size Control, Diversification, and Query Optimizations'
status: To Do
assignee:
  - '@backend-agent'
created_date: '2026-07-25 19:59'
labels:
  - core
  - performance
  - cli
dependencies: []
documentation:
  - docs/output-optimization.md
priority: medium
type: enhancement
ordinal: 38000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Enforce better context bounding, add result diversification (MMR-style), and optimize queries/indexes for performance on large technical repositories.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Configurable output truncation with smart section preservation
- [ ] #2 Diversification to reduce duplicate results
- [ ] #3 SQLite index optimizations and query tuning
- [ ] #4 Config options for FTS params, depth, limits
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Performance and correctness tests pass
- [ ] #2 Configuration documented
- [ ] #3 No regressions in existing behavior
<!-- DOD:END -->
