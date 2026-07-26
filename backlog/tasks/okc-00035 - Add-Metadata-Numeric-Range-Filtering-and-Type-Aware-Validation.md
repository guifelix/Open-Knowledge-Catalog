---
id: OKC-00035
title: Add Metadata Numeric Range Filtering and Type-Aware Validation
status: To Do
assignee:
  - '@backend-agent'
created_date: '2026-07-25 19:55'
updated_date: '2026-07-25 23:57'
labels:
  - metadata
  - query
  - validation
dependencies:
  - OKC-00001
documentation:
  - docs/metadata-querying.md
priority: high
type: feature
ordinal: 38000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Enable robust querying on numeric/custom metadata fields (e.g., accuracy thresholds, version numbers, dates) and add lightweight schema validation for common technical patterns. Improves structured querying reliability for technical knowledge bases.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Support range filters (>, <, between) for numeric frontmatter fields in query_metadata
- [ ] #2 Type inference/validation for common fields (numbers, dates, arrays)
- [ ] #3 Safe parameterization to prevent injection (building on OKC-00001)
- [ ] #4 Validation warnings during scan for malformed technical metadata
- [ ] #5 Tests cover numeric filtering and edge cases
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 All queries use proper parameterization
- [ ] #2 Updated examples in README/quickstart
- [ ] #3 Fuzz/property tests extended
<!-- DOD:END -->
