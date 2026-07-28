---
id: OKC-00095
title: Add headings list to search results with configurable max/depth
status: To Do
assignee:
  - '@backend-agent'
created_date: '2026-07-28 02:32'
updated_date: '2026-07-28 02:35'
labels:
  - backend
  - search
  - feature
  - headings
  - config
dependencies:
  - OKC-00042
references:
  - docs/references/okf-spec.md
documentation:
  - docs/architecture/data-flow.md
priority: high
type: feature
ordinal: 70000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Search results currently return `excerpt` from body text but no structured headings list. Headings provide rich, structured context — showing the document outline at a glance.

**No new MCP tool.** The existing `search` tool gains two optional params (`max_headings`, `heading_depth`) and returns `headings: Vec<String>` per result. An internal helper function queries the already-parsed `headings` table — not a new tool.

Both follow the same fallback chain:
  1. Per-request MCP param (if passed)
  2. TOML `[search]` section (if configured)
  3. Hard default = 1

**Budget interaction:** `max_headings` only counts headings at the allowed depth levels. `heading_depth=2, max_headings=3` → up to 3 headings from h1+h2 combined.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Only the existing `search` MCP tool is affected — no new MCP tool is added
- [ ] #2 Internal helper get_document_headings() returns empty vec when body is empty
- [ ] #3 Internal helper get_document_headings() returns empty vec when heading_depth=0
- [ ] #4 Internal helper get_document_headings() filters by heading depth (depth=2 → h1+h2 only)
- [ ] #5 Internal helper get_document_headings() respects max_headings cap (at most N)
- [ ] #6 Internal helper get_document_headings() with depth=1 returns only h1 headings
- [ ] #7 SearchResult includes headings: Vec<String> field (never null, empty vec when none found)
- [ ] #8 MCP SearchParams adds optional max_headings: Option<usize> and heading_depth: Option<u32>
- [ ] #9 MCP SearchResultOutput includes headings: Vec<String>
- [ ] #10 TOML config accepts [search] section with max_headings and heading_depth
- [ ] #11 Default OkcConfig has search.max_headings = 1 and search.heading_depth = 1
- [ ] #12 Config validation rejects heading_depth = 0 and max_headings = 0
- [ ] #13 Per-request param overrides config default; config overrides hard default
- [ ] #14 Both search paths (FTS5 + SQLite queries) populate the headings field
- [ ] #15 cargo fmt --check, cargo clippy -- -D warnings, cargo test all pass
- [ ] #16 Internal helper get_document_headings() ignores headings inside code blocks
- [ ] #17 max_headings only counts headings within the allowed heading_depth levels (depth=2, max=3 → up to 3 from h1+h2)
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Config unit tests: heading_depth=0 rejected, max_headings=0 rejected, env var override
- [ ] #2 Integration test: search tool returns headings in JSON response
- [ ] #3 cargo fmt --check, cargo clippy -- -D warnings, cargo test clean
- [ ] #4 Unit tests for internal helper: depth filtering, max cap, empty body, no headings, code block exclusion
- [ ] #5 Unit tests for budget interaction: max_headings only counts headings within allowed depth
<!-- DOD:END -->
