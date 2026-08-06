---
id: DRAFT-00010
title: Add token-based response limits and auto-summarization
status: To Do
assignee:
  - '@backend-agent'
created_date: '2026-08-06 00:49'
labels:
  - mcp
  - backend
  - enhancement
  - high-priority
dependencies: []
references:
  - src/transport/mcp.rs
  - src/config.rs
  - src/service/documents.rs
documentation:
  - docs/ai-usage.md#response-size-limits
priority: high
type: enhancement
ordinal: 75000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Response limits are character-based (500k chars ≈ 125k tokens), not token-based. No summarization for large documents.

**Current:**
- max_response_chars: 500000 (hard char limit)
- Truncates at arbitrary character boundary
- No token counting, no summarization

**Expected:**
- max_response_tokens parameter (default ~4000)
- Auto-summarize body when exceeding token budget
- Truncation strategy: head/tail/middle/summary
- Return token_count in response meta
- Progressive disclosure: metadata → headings → summary → full body
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 max_response_tokens parameter works (default 4000)
- [ ] #2 Large document body auto-summarized to fit token budget
- [ ] #3 Truncation strategy: head (first N tokens) / tail / middle / summary
- [ ] #4 Response meta includes token_count, truncated, strategy_used
- [ ] #5 Progressive disclosure: include=metadata,headings,summary,body
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Unit test covers token counting and summarization
- [ ] #2 Integration test: large doc returns summarized response within token budget
- [ ] #3 Tiktoken or similar for accurate token counting
<!-- DOD:END -->
