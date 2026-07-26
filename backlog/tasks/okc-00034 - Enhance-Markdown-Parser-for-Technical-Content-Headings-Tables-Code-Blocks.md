---
id: OKC-00034
title: 'Enhance Markdown Parser for Technical Content (Headings, Tables, Code Blocks)'
status: To Do
assignee: []
created_date: '2026-07-25 19:20'
updated_date: '2026-07-25 23:57'
labels:
  - parser
  - core
dependencies: []
priority: high
type: enhancement
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Strengthen the pulldown-cmark based parser to better handle technical Markdown common in engineering knowledge bases (precise heading hierarchy, table extraction, fenced code block metadata) without altering the general-purpose OKF format. Improves section extraction and indexing accuracy.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Robust nested heading extraction with hierarchy preservation
- [ ] #2 Reliable table detection and basic metadata extraction (headers, row counts)
- [ ] #3 Fenced code block metadata extraction (language, filename, line numbers)
- [ ] #4 Update `get_section` and indexing to leverage improved structure
- [ ] #5 Property-based tests cover technical Markdown edge cases
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Tests pass including integration fixtures
- [ ] #2 Parser documentation updated
- [ ] #3 No regression in general Markdown handling
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Extend event processing in markdown.rs for tables and heading depth
2. Add fenced code block metadata extraction (language, filename, line numbers)
3. Integrate into Document model and database indexing
4. Expand test fixtures
<!-- SECTION:PLAN:END -->
