---
id: OKC-00021
title: Refactor database.rs god-file into focused modules
status: To Do
assignee: []
created_date: '2026-07-23 19:03'
labels:
  - tech-debt
dependencies: []
priority: high
type: enhancement
ordinal: 7400
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
src/index/database.rs is 1265 lines handling scanning, indexing, queries, validation, and export. Split into focused modules under src/index/.\n\nCurrent responsibilities (all in one file):\n- SQLite schema creation and migrations\n- Document insertion (concept, document, link tables)\n- Query building via string interpolation (query_metadata)\n- Search with FTS5 query assembly\n- Link graph traversal (traverse_subgraph)\n- Repository validation (validate_repository)\n- Export to JSON/Markdown\n\nTarget structure:\n- src/index/database.rs — schema + connection management (keep ~200 lines)\n- src/index/queries.rs — query builders and search\n- src/index/validate.rs — validation logic\n- src/index/export.rs — export/format operations\n- src/index/graph.rs — graph traversal
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 database.rs reduced from ~1265 to <250 lines
- [ ] #2 New module files have clear single responsibilities with no circular dependencies
- [ ] #3 Each new module has its own comment explaining its public API surface
- [ ] #4 All existing tests pass without modification after the split
- [ ] #5 git diff --stat shows parallel code movement — not logic changes — in the refactor commit
<!-- AC:END -->
