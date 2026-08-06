---
id: OKC-00105
title: Persist root identity and isolate multi-root repositories
status: To Do
assignee:
  - '@backend-agent'
created_date: '2026-08-06 00:52'
updated_date: '2026-08-06 19:53'
labels:
  - backend
  - indexing
  - multi-root
  - data-integrity
  - mcp
dependencies: []
references:
  - src/scanner/walker.rs
  - src/index/migrations.rs
documentation:
  - docs/architecture.md
priority: high
type: feature
ordinal: 75000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Make multi-root catalogs preserve the identity of each configured source root. Today documents are keyed only by root-relative path, so equal paths from different roots can collide and query results cannot be scoped reliably. Introduce stable root identity, collision-safe document identity, migration behavior, root-aware links, and query filtering while preserving single-root compatibility.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Each indexed document is associated with a stable root identifier that is exposed through documented service, CLI, and MCP response fields
- [ ] #2 Documents with the same relative path in different roots coexist without overwrite, ambiguity, or cross-root metadata leakage
- [ ] #3 Existing single-root indexes migrate or rebuild safely with unchanged user-visible paths
- [ ] #4 Search and metadata queries support optional root filtering, and unfiltered queries retain all-root behavior
- [ ] #5 Link resolution and graph traversal define and enforce same-root, explicit cross-root, ambiguous-target, and removed-root behavior
- [ ] #6 Statistics include per-root counts and root removal cleans up only documents owned by that root
- [ ] #7 Integration tests cover collisions, filtering, cross-root links, migration/rebuild, root removal, and single-root compatibility
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Unit test covers root filtering in all query tools
- [ ] #2 Integration test: multi-root scan → filtered queries
- [ ] #3 Backward compatible: no root filter = all roots
<!-- DOD:END -->
