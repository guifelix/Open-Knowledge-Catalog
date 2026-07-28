---
id: OKC-00093
title: 'Fix: get_document MCP tool returns null body for all documents'
status: Done
assignee:
  - '@engineering'
created_date: '2026-07-28 01:22'
updated_date: '2026-07-28 01:29'
labels:
  - bug
  - priority-p0
  - mcp
  - tools
  - backend
dependencies: []
references:
  - >-
    https://github.com/guifelix/Open-Knowledge-Catalog/blob/main/src/mcp/handlers.rs
  - >-
    https://github.com/guifelix/Open-Knowledge-Catalog/blob/main/src/storage/models.rs
documentation:
  - docs/features.md
priority: high
type: bug
ordinal: 68000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The `get_document` MCP tool always returns `body: null` (and `headings: []`) regardless of the document, content size, or `max_chars` parameter. This makes the tools primary purpose -- retrieving document content -- non-functional.

Observed behavior:
- Every call to `get_document` returns `{"headings":[], "body":null, "truncated":false}`
- Occurs for all document types (docs, ADRs, feature pages)
- Varying `max_chars` parameter has no effect
- Varying `include` parameter was not tested systematically

Impact:
- AI agents cannot rely on the MCP tool for content retrieval
- Forces fallback to direct filesystem read, defeating MCP layer purpose
- Adds ~3s latency and ~8 extra round-trips per retrieval attempt

Likely root cause (unconfirmed):
- MCP serialization of the Document struct omits body field
- Or `get_document` handler strips body before sending response
- Or the SQLite query retrieves body but response serialization drops it
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 get_document returns non-null body for a small document (< 1KB)
- [ ] #2 get_document returns non-null body for a large document (> 100KB)
- [ ] #3 get_document respects max_chars parameter and truncates body when exceeded
- [ ] #4 get_document body includes the full document content (headings + body text)
- [ ] #5 Headings array in response is correctly populated
- [ ] #6 Existing tests pass after fix
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Root cause identified and fixed in serialization layer
- [ ] #2 Manual test with  returns full content via MCP
- [ ] #3 No regression in  or  tools
- [ ] #4 Regression test added to CI: basic MCP get_document round-trip
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Identify root cause: MCP handler defaults include to empty Vec via unwrap_or_default() —> query layer skips body\n2. Fix: change default to ['body', 'headings'] for the canonical AI-agent call pattern (path-only)\n3. Verify: cargo test (251 pass), cargo fmt --check, cargo clippy -- -D warnings
<!-- SECTION:PLAN:END -->

## Comments

<!-- COMMENTS:BEGIN -->
created: 2026-07-28 01:29
---
Fix verified. All quality gates pass.
---
<!-- COMMENTS:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Fixed the P0 bug where the get_document MCP tool returned null body for all documents. Root cause was at src/transport/mcp/mod.rs:193 — include.unwrap_or_default() produced an empty Vec<String>, causing the query layer's include.contains('body') guard to always skip body and headings. The fix defaults to ['body', 'headings'] so the common AI-agent call pattern (path-only, no include) returns full document content. All 251 tests pass, formatting and clippy clean.
<!-- SECTION:FINAL_SUMMARY:END -->
