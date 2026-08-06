---
id: DRAFT-00013
title: Fix HTTP/SSE transport session management for MCP
status: To Do
assignee:
  - '@backend-agent'
created_date: '2026-08-06 00:50'
labels:
  - mcp
  - backend
  - bug
  - medium-priority
dependencies: []
references:
  - src/transport/mcp.rs
  - rmcp crate HTTP transport docs
documentation:
  - docs/ai-usage.md#transport-options
priority: medium
type: bug
ordinal: 75000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
HTTP/SSE transport unusable for real clients - requires single connection for initialize + tools/call.

**Current:**
- SSE requires single persistent connection
- curl cannot maintain session across requests
- No documented way to use HTTP transport properly
- Each request creates new session, loses initialization

**Expected:**
- Session management with session_id cookie/header
- Initialize returns session_id
- Subsequent requests include session_id
- Proper SSE event stream per session
- Documented client usage patterns
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 HTTP initialize returns session_id in response
- [ ] #2 Subsequent tools/call requests use session_id header/cookie
- [ ] #3 SSE stream delivers responses for correct session
- [ ] #4 Multiple concurrent sessions supported
- [ ] #5 Session timeout configurable (default 30 min)
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Integration test: HTTP initialize → tools/call → tools/call works
- [ ] #2 Documented curl/python client examples
- [ ] #3 No regression in stdio transport
<!-- DOD:END -->
