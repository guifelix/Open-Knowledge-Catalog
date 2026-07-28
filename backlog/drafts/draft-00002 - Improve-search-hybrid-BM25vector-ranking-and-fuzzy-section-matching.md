---
id: DRAFT-00002
title: 'Fuzzy heading matching in get_section'
status: To Do
assignee:
  - '@engineering'
created_date: '2026-07-28 01:23'
updated_date: '2026-07-28 01:23'
labels:
  - enhancement
  - search
  - mcp
  - quality
  - backend
dependencies: []
references:
  - >-
    https://github.com/guifelix/Open-Knowledge-Catalog/blob/main/docs/architecture/adr-002-search-ranking.md
documentation:
  - docs/configuration.md
priority: medium
type: enhancement
ordinal: 68000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The MCP tool interaction revealed a quality gap when retrieving document sections: `get_section` requires exact heading text, causing AI agents to miss valid sections when they approximate the heading name.

**Problem:**
- `get_section("MCP")` returns null because heading is "MCP Server" (exact match only)
- AI agents often approximate the heading name — they don't know the exact text
- Prefix, substring, or fuzzy matching would dramatically improve hit rate

**Scope:**
- Add prefix/substring/case-insensitive heading matching to `get_section` as fallback
- Keep existing exact-match and anchor-slug matching as primary path (higher priority)
- No changes to search, embeddings, or path-based ranking (out of scope)
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Search for "MCP" matches heading "MCP Server" (prefix match)
- [ ] #2 Search for "mcp server" matches with existing case-insensitive exact match
- [ ] #3 Search for "Def" matches heading "Definition" (prefix match)
- [ ] #4 Exact match still takes priority over fuzzy/prefix match
- [ ] #5 Anchor slug matching still works (e.g. "mcp-server" matches "MCP Server")
- [ ] #6 All existing get_section integration/e2e tests still pass
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Prefix/prefix-insensitive matching implemented as fallback in get_section handler
- [ ] #2 Fallback applies when exact match (by title + anchor slug) returns no results
- [ ] #3 Integration test: get_section with partial heading match (e.g., "MCP" → "MCP Server")
- [ ] #4 Integration test: exact match still preferred over partial match
- [ ] #5 All existing tests pass unchanged
<!-- DOD:END -->

## Implementation Plan

### Change 1: Add prefix fallback in `get_section` (`src/index/queries/document.rs`)

After the two existing match passes (case-insensitive exact → anchor slug), add a third fallback pass that uses `starts_with_ignore_ascii_case` to find a heading that starts with the query string.

Logic:
```
1. Case-insensitive exact match (existing, keeps priority)
2. Anchor slug match (existing, keeps priority)
3. NEW: Prefix match — find first heading that starts with query (case-insensitive)
```

### Change 2: Add integration tests (`tests/integration_tests.rs`)

- Test: `get_section("MCP")` on a doc with heading "MCP Server" returns the section
- Test: `get_section("Def")` on a doc with heading "Definition" returns the section
- Test: exact heading "Definition" still returns the correct section (regression)

### Files touched
- `src/index/queries/document.rs` — add prefix fallback logic
- `tests/integration_tests.rs` — add fuzzy match tests
