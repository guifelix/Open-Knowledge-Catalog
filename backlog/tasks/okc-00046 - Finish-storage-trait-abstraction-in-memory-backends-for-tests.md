---
id: OKC-00046
title: Finish storage trait abstraction + in-memory backends for tests
status: To Do
assignee:
  - '@backend-agent'
created_date: '2026-07-25 20:05'
updated_date: '2026-07-25 23:57'
labels:
  - architecture
  - testability
dependencies: []
documentation:
  - docs/storage-traits.md
priority: medium
type: task
ordinal: 38000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The ADRs already call for trait-based storage. Completing the traits and providing in-memory implementations improves test speed, enables future backend swaps, and removes the god file pressure around SQLite details.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 DocumentStore, SearchIndex, GraphStore traits are the only types the service layer depends on
- [ ] #2 In-memory implementations exist and are used by a meaningful subset of tests
- [ ] #3 SQLite implementations remain the default production path
- [ ] #4 No behavioral regression on existing tests
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Architecture docs match the code
- [ ] #2 Test suite can run largely without disk I/O for unit tests
- [ ] #3 Related backlog items closed
<!-- DOD:END -->
