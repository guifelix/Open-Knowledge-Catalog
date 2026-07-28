---
id: OKC-00002
title: Add request-scoped SQLite connections for thread safety
status: Done
assignee: []
created_date: '2026-07-23 00:49'
updated_date: '2026-07-26 00:58'
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

**Additional scope from draft-00016 (RefCell audit):**
- Inventory all RefCell usage across the codebase and categorize by shared-state vs thread-local
- Replace RefCell with RwLock/Mutex on shared state paths:
  - Read-heavy paths (search, browse, graph traversal) → RwLock for concurrent reads
  - Write paths (scan, incremental update) → Mutex for exclusive access
  - Consider parking_lot::RwLock for lower overhead
- Audit async boundaries: ensure no .await inside lock guards; use tokio::sync::RwLock if async-aware locking needed
- Design lock hierarchy (index lock → graph lock → config lock) to prevent deadlocks; document in docs/architecture/concurrency-model.md
- Add concurrent stress tests: spawn N search + M browse tasks; verify no panics/data corruption
- Run cargo miri test (if feasible) and cargo clippy -D warnings on concurrency code
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
