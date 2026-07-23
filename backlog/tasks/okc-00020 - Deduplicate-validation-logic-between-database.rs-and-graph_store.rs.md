---
id: OKC-00020
title: Deduplicate validation logic between database.rs and graph_store.rs
status: To Do
assignee: []
created_date: '2026-07-23 06:50'
updated_date: '2026-07-23 19:02'
labels:
  - quality
dependencies: []
references:
  - 'src/index/database.rs:1008-1078'
  - 'src/index/graph_store.rs:251-293'
  - 'src/index/graph_store.rs:179'
priority: high
type: enhancement
ordinal: 6400
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Code review found that link validation logic is duplicated across two files:

1. database.rs:1008-1078 - validate() method on RepositoryIndex contains inline link-exists checking
2. graph_store.rs:251-293 - validate_links() method on GraphStore contains near-identical logic

The GraphStore trait defines a validate() method (at graph_store.rs:179) but it is never called through the trait abstraction - only the concrete implementation methods are reached directly.

Refactor to eliminate the duplication: ensure validation logic lives in one place (GraphStore implementation) and the CLI validate command calls through the trait. The trait method should handle all link-validation concerns; DatabaseIndex validate method should delegate to it.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 No duplicated validation logic between database.rs and graph_store.rs
- [ ] #2 CLI validate command goes through GraphStore trait validate method
- [ ] #3 Behavior identical to current state (same issues detected)
- [ ] #4 Existing tests pass without modification
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 grep for validate functions confirms single implementation
<!-- DOD:END -->
