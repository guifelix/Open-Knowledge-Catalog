---
id: OKC-00007
title: 'Fix link resolution edge cases (anchors, case sensitivity, URL encoding)'
status: To Do
assignee: []
created_date: '2026-07-23 00:49'
updated_date: '2026-07-25 23:57'
labels:
  - correctness
dependencies: []
priority: high
type: bug
ordinal: 11400
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Current LinkResolver doesn't handle anchor fragments properly, case-sensitive filesystem differences, or percent-encoded paths. Need proper anchor extraction, case-insensitive matching on macOS/Windows, URL decoding.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Anchor fragments extracted and stored separately
- [ ] #2 Case-insensitive path matching on case-insensitive filesystems
- [ ] #3 Percent-encoded paths decoded before resolution
- [ ] #4 Round-trip: resolve -> check_exists -> get_document works
- [ ] #5 Obsidian WikiLinks [[target]] resolve to the correct bundle concept
- [ ] #6 Relative Markdown links (./file.md#section) resolve against source file directory
- [ ] #7 Broken links produce a non-fatal warning and are omitted from the graph — they do not crash the scan
- [ ] #8 Self-referencing links (to the same file or fragment) are stored but never returned as backlinks
- [ ] #9 Cycle detection: A->B->C->A does not cause infinite traversal in graph queries
<!-- AC:END -->
