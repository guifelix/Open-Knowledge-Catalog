---
id: OKC-00002
title: Add request-scoped SQLite connections for thread safety
status: To Do
assignee: []
created_date: '2026-07-23 00:49'
updated_date: '2026-07-23 19:02'
labels:
  - blocks-mcp
dependencies: []
priority: high
type: feature
ordinal: 200
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Current Rusqlite Connection uses RefCell and is not Sync. For MCP server, need connection pool or per-request connections with sync channel.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 RepositoryIndex implements Sync
- [ ] #2 Connection pool or request-scoped connections implemented
- [ ] #3 MCP server can handle concurrent requests
- [ ] #4 All shared DocumentStore/SearchIndex/GraphStore trait methods accept &self or &mut self only — no &self methods unsafely mutate interior state
- [ ] #5 Clippy warning-free on Send + Sync bounds in all public APIs
- [ ] #6 Fuzz harness with 8 concurrent readers + 1 writer runs without panic for 60 seconds
<!-- AC:END -->
