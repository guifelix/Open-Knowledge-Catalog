---
id: OKC-00109
title: Add a durable paginated change feed for incremental sync
status: To Do
assignee:
  - '@backend-agent'
created_date: '2026-08-06 16:37'
updated_date: '2026-08-06 19:54'
labels:
  - mcp
  - backend
  - change-feed
  - sync
  - indexing
dependencies:
  - OKC-00005
references:
  - src/scanner/watcher.rs
  - src/index/migrations.rs
  - src/transport/mcp/mod.rs
documentation:
  - docs/ai-usage.md
priority: medium
type: feature
ordinal: 75000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Persist catalog changes so agents can retrieve created, modified, metadata-only, and deleted document events after process restarts. This task delivers the durable get_changes query contract only; realtime push transport is intentionally excluded until the persistent feed and MCP transport requirements are proven.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Successful document index transactions append a durable change event containing event ID, change type, root/document identity, path, content hash when applicable, and modification time
- [ ] #2 Deleted documents produce tombstones that remain queryable for the configured retention period
- [ ] #3 get_changes supports an opaque stable cursor, deterministic ordering, a bounded limit, and a next cursor when more retained events exist
- [ ] #4 Queries can filter by change type, root, path prefix, and document type without skipping or duplicating cursor results
- [ ] #5 Expired or invalid cursors return structured recovery information describing the earliest retained position
- [ ] #6 Index updates and their change events commit atomically, including rollback behavior after parse, validation, or storage failures
- [ ] #7 Retention cleanup is configurable, restart-safe, and does not block normal indexing for an unbounded period
- [ ] #8 Tests cover create, update, metadata-only change, delete, rollback, restart, pagination, filtering, retention expiry, and concurrent readers
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Unit test covers change tracking and cursor encoding
- [ ] #2 Integration test: scan → modify → get_changes returns diff
- [ ] #3 Packaged MCP integration tests verify the durable feed contract without requiring realtime transport
<!-- DOD:END -->
