---
id: DRAFT-00012
title: Add change feed / get_changes MCP tool for incremental sync
status: To Do
assignee:
  - '@backend-agent'
created_date: '2026-08-06 00:50'
labels:
  - mcp
  - backend
  - feature
  - medium-priority
dependencies:
  - DRAFT-00026
references:
  - src/transport/mcp.rs
  - src/scanner/watcher.rs
  - src/index/document_store.rs
documentation:
  - docs/ai-usage.md#mcp-tools
priority: medium
type: feature
ordinal: 75000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
No way for agents to subscribe to changes or get incremental updates.

**Missing:**
- get_changes(since_timestamp) → returns created/modified/deleted docs
- Change feed subscription (SSE/webhook)
- Watch events not exposed via MCP

**Expected:**
- get_changes(since: timestamp, limit: 100) → {changes: [...], next_cursor}
- Change types: created, modified, deleted, metadata_only
- Include content_hash for deduplication
- SSE endpoint for real-time subscriptions
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 get_changes(since=timestamp) returns changes since that time
- [ ] #2 Changes include: type, path, content_hash, modified_at
- [ ] #3 next_cursor for pagination of change feed
- [ ] #4 SSE endpoint at /events for real-time subscriptions
- [ ] #5 Filter by change_type (created/modified/deleted)
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Unit test covers change tracking and cursor encoding
- [ ] #2 Integration test: scan → modify → get_changes returns diff
- [ ] #3 SSE connection stays alive, delivers events
<!-- DOD:END -->
