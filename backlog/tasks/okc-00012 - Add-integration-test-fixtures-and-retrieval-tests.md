---
id: OKC-00012
title: Add integration test fixtures and retrieval tests
status: Done
assignee: []
created_date: '2026-07-23 00:50'
updated_date: '2026-07-23 05:53'
labels: []
dependencies: []
priority: low
type: feature
ordinal: 12000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create fixture repositories with nested dirs, index.md, valid/invalid docs, circular/broken links, custom metadata. Define representative AI questions and verify tool returns required evidence.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Fixture repos in tests/fixtures/
- [ ] #2 Retrieval tests verify expected concepts returned
- [ ] #3 Tests cover: direct lookup, hierarchical browsing, relationship reasoning, exact metadata query, validation
- [ ] #4 CI runs retrieval tests on every PR
<!-- AC:END -->
