---
id: OKC-00031
title: Split service/mod.rs into focused sub-modules
status: To Do
assignee: []
created_date: '2026-07-23 23:31'
updated_date: '2026-07-24 01:12'
labels:
  - refactor
dependencies: []
priority: low
ordinal: 23000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The src/service/ directory only contains mod.rs with all service methods in one file (115 lines). The design document (docs/implementation-plan.md section 17) proposes splitting service/ into browse.rs, search.rs, documents.rs, graph.rs, and validation.rs. Refactor to match the documented architecture where it makes sense, keeping it consistent with the rest of the codebase style.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Service methods organized into logical sub-modules
- [ ] #2 No change to public API or behavior
- [ ] #3 All existing tests pass
<!-- AC:END -->
