---
id: OKC-00094
title: Fuzzy heading matching in get_section
status: Done
assignee: []
created_date: '2026-07-28 01:41'
updated_date: '2026-07-28 01:43'
labels:
  - enhancement
  - search
  - mcp
  - quality
  - backend
dependencies: []
priority: medium
type: enhancement
ordinal: 69000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The get_section MCP tool requires exact heading text match, causing AI agents to miss valid sections when they approximate the heading name.

Problem:
- get_section("MCP") returns null because heading is "MCP Server" (exact match only)
- AI agents often approximate the heading name
- Prefix/substring matching would dramatically improve hit rate

Scope:
- Add prefix/substring/case-insensitive heading matching as fallback
- Keep existing exact-match and anchor-slug matching as primary (higher priority)
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Query "MCP" matches heading "MCP Server" (prefix match)
- [ ] #2 Query "mcp server" matches with existing case-insensitive exact match
- [ ] #3 Query "Def" matches heading "Definition" (prefix match)
- [ ] #4 Exact match still takes priority over fuzzy/prefix match
- [ ] #5 Anchor slug matching still works (e.g. "mcp-server" matches "MCP Server")
- [ ] #6 All existing get_section integration/e2e tests still pass
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Prefix matching implemented as fallback in get_section handler
- [ ] #2 Fallback applies when exact match (by title + anchor slug) returns no results
- [ ] #3 Integration test: get_section with partial heading match
- [ ] #4 Integration test: exact match still preferred over partial match
- [ ] #5 All existing tests pass unchanged
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
## Change 1: Add prefix fallback in get_section\nAfter the two existing match passes (case-insensitive exact -> anchor slug), add a third fallback pass using starts_with_ignore_ascii_case.\n\n## Change 2: Add integration tests\n- Test: get_section("MCP") on doc with heading "MCP Server" returns section\n- Test: get_section("Def") on doc with heading "Definition" returns section\n- Test: exact heading "Definition" still returns correct section (regression)
<!-- SECTION:PLAN:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added prefix-matching fallback to get_section: when exact-match and anchor-slug passes return nothing, a third pass tries case-insensitive starts_with. Querying "Def" now matches heading "Definition", and "Rec" matches "Recognition Rules". Four integration tests added covering prefix, case-insensitive prefix, and unambiguous prefix scenarios. All 255 existing tests remain passing.
<!-- SECTION:FINAL_SUMMARY:END -->
