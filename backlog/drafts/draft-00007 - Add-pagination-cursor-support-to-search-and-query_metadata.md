---
id: DRAFT-00007
title: Add pagination/cursor support to search and query_metadata
status: To Do
assignee:
  - '@backend-agent'
created_date: '2026-08-06 00:48'
labels:
  - mcp
  - backend
  - enhancement
  - high-priority
dependencies: []
references:
  - src/transport/mcp.rs
  - src/index/search_index.rs
  - src/index/queries.rs
documentation:
  - docs/ai-usage.md#mcp-tools
priority: high
type: enhancement
ordinal: 75000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Search and query_metadata tools return truncated results with no way to paginate.

**Current behavior:**
- search("test", limit=10) → 52 matches, truncated: true
- No offset, cursor, page_token, or next_page parameter
- Cannot retrieve results beyond first page

**Expected:**
- Add cursor-based pagination (opaque token)
- Support offset/limit for simple pagination
- Return next_cursor in response when more results exist
- Consistent across search, query_metadata, browse, traverse
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 search with limit=5 returns next_cursor when more results exist
- [ ] #2 search with cursor=token returns next page of results
- [ ] #3 query_metadata supports cursor pagination
- [ ] #4 browse supports cursor pagination for large directories
- [ ] #5 traverse supports cursor pagination for deep graphs
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Unit test covers cursor encoding/decoding
- [ ] #2 Integration test verifies multi-page retrieval works
- [ ] #3 Cursor is opaque, tamper-proof, expires after 1 hour
<!-- DOD:END -->
