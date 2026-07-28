---
id: DRAFT-00004
title: 'Add meta-query tool: suggest-best-doc for content placement'
status: To Do
assignee:
  - '@engineering'
created_date: '2026-07-28 01:23'
labels:
  - feature
  - mcp
  - dx
  - ai-agent
dependencies: []
references:
  - >-
    https://github.com/guifelix/Open-Knowledge-Catalog/blob/main/docs/ai-usage.md
priority: low
type: feature
ordinal: 68000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
During the MCP analysis session, the AI spent most of its reasoning time manually reading 5+ docs to decide where to add new content (the Agent Configuration section). This is a common pattern: "I have this content, where should it go?"

A lightweight MCP tool like `suggest_best_document` would scan the doc hierarchy and return the best parent document/directory based on content similarity with the query.

Proposed tool:
- Input: content snippet or topic description
- Input (optional): path_prefix to scope search
- Output: ranked list of existing docs with relevance scores

This is cheap (<100ms) since it can use existing BM25/vector indexes -- no new indexing needed.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 suggest_best_document('MCP setup instructions') returns ai-usage.md as top result
- [ ] #2 suggest_best_document('TOML config reference') returns configuration.md as top result
- [ ] #3 Tool returns at most 5 suggestions with relevance scores and paths
- [ ] #4 Tool accepts optional path_prefix to scope suggestions to a directory
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 MCP tool handler registered in tools list
- [ ] #2 docs/features.md and docs/ai-usage.md updated with new tool
- [ ] #3 Integration test verifies suggestion relevance
<!-- DOD:END -->
