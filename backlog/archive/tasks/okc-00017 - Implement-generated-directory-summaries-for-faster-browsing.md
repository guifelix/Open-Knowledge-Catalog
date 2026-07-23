---
id: OKC-00017
title: Implement generated directory summaries for faster browsing
status: To Do
assignee: []
created_date: '2026-07-23 00:51'
updated_date: '2026-07-23 19:02'
labels:
  - ux
dependencies: []
priority: low
type: feature
ordinal: 1300
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Auto-generate directory index.md summaries from child documents. Use LLM or extractive summarization. Cache with invalidation on child changes. Improves browse_directory for large directories.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Summary generation on directory scan
- [ ] #2 Invalidation on child document changes
- [ ] #3 Configurable: disabled, extractive, LLM-based
- [ ] #4 Summary visible in browse_directory response
- [ ] #5 okf summary <bundle> shows concept count, tag distribution, last-modified date
- [ ] #6 okf summary --json outputs structured JSON for agent consumption
- [ ] #7 Empty or newly-initialized bundles show meaningful 'no concepts yet' message instead of error
- [ ] #8 Summary computes and displays: total concepts, link count, tags breakdown, orphan pages count
<!-- AC:END -->
