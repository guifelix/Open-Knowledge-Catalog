---
id: OKC-00037
title: Improve Incremental Scan and Output Determinism for Technical Repos
status: To Do
assignee:
  - '@backend-agent'
created_date: '2026-07-25 19:56'
updated_date: '2026-07-25 23:57'
labels:
  - scanner
  - core
  - reliability
dependencies: []
documentation:
  - docs/incremental-scan.md
priority: high
type: enhancement
ordinal: 38000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Refine change detection, hashing, and output bounding for repositories with large technical documents (heavy tables, code blocks, complex structures). Ensures consistent, efficient updates and bounded responses critical for agent reliability.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Optimized Blake3 usage with sampling for very large files
- [ ] #2 Deterministic truncation strategies preserving key sections
- [ ] #3 Transactional graph/index updates on incremental scans
- [ ] #4 Benchmarks show efficient handling of technical content
- [ ] #5 Property tests for change detection edge cases
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Scanner and index logic updated
- [ ] #2 Safety limits documented
- [ ] #3 No impact on general usability
<!-- DOD:END -->
