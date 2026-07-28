---
id: OKC-00095
title: Add headings list to search results with configurable max/depth
status: To Do
assignee:
  - '@backend-agent'
created_date: '2026-07-28 02:30'
updated_date: '2026-07-28 02:32'
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
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 extract_headings() returns empty vec when body is empty
- [ ] #2 extract_headings() returns empty vec when heading_depth=0
- [ ] #3 extract_headings() filters by heading depth (depth=2 returns h1 and h2 only, excludes h3+)
- [ ] #4 extract_headings() respects max_headings cap (at most N headings at allowed depths)
- [ ] #5 extract_headings() ignores headings inside code blocks
- [ ] #6 extract_headings() with depth=1 returns only h1 headings
- [ ] #7 SearchResult includes headings: Vec<String> field (never null, empty vec when none found)
- [ ] #8 MCP SearchParams accepts optional max_headings: Option<usize> and heading_depth: Option<u32>
- [ ] #9 MCP SearchResultOutput includes headings: Vec<String>
- [ ] #10 TOML config accepts [search] section with max_headings and heading_depth
- [ ] #11 Default OkcConfig has search.max_headings = 1 and search.heading_depth = 1
- [ ] #12 Config validation rejects heading_depth = 0 and max_headings = 0
- [ ] #13 Per-request param overrides config default; config overrides hard default
- [ ] #14 Both search paths (FTS5 + SQLite queries) populate the headings field
- [ ] #15 cargo fmt --check, cargo clippy -- -D warnings, cargo test all pass
- [ ] #16 Internal helper `get_document_headings()` returns empty vec when body empty
- [ ] #17 Internal helper `get_document_headings()` returns empty vec when heading_depth=0
- [ ] #18 Internal helper filters by heading depth (depth=2 returns h1 and h2 only)
- [ ] #19 Internal helper respects max_headings cap (at most N headings at allowed depths)
- [ ] #20 Internal helper with depth=1 returns only h1 headings
- [ ] #21 Existing `search` MCP tool is the only affected tool — no new MCP tool is added
- [ ] #22 SearchResult includes headings: Vec<String> field (never null, empty vec when none found)
- [ ] #23 MCP SearchParams accepts optional max_headings: Option<usize> and heading_depth: Option<u32>
- [ ] #24 MCP SearchResultOutput includes headings: Vec<String>
- [ ] #25 TOML config accepts [search] section with max_headings and heading_depth
- [ ] #26 Default OkcConfig has search.max_headings = 1 and search.heading_depth = 1
- [ ] #27 Config validation rejects heading_depth = 0 and max_headings = 0
- [ ] #28 Per-request param overrides config default; config overrides hard default
- [ ] #29 Both search paths (FTS5 + SQLite queries) populate the headings field
- [ ] #30 cargo fmt --check, cargo clippy -- -D warnings, cargo test all pass
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Unit tests for extract_headings(): depth filtering, max cap, empty body, no headings, code block exclusion
- [ ] #2 Config unit tests: heading_depth=0 rejected, max_headings=0 rejected, env var override
- [ ] #3 Integration test: search tool returns headings in JSON response
- [ ] #4 Integration test: per-request max_headings overrides config default
- [ ] #5 cargo fmt --check, cargo clippy -- -D warnings, cargo test clean
- [ ] #6 Unit tests for internal helper: depth filtering, max cap, empty body, no headings
- [ ] #7 Config unit tests: heading_depth=0 rejected, max_headings=0 rejected, env var override
- [ ] #8 Integration test: search tool returns headings in JSON response
- [ ] #9 Integration test: per-request max_headings overrides config default
- [ ] #10 cargo fmt --check, cargo clippy -- -D warnings, cargo test clean
<!-- DOD:END -->
