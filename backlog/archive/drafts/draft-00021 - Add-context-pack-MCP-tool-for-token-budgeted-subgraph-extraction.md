---
id: DRAFT-00021
title: Add context pack MCP tool for token-budgeted subgraph extraction
status: To Do
assignee:
  - '@backend-agent'
created_date: '2026-08-06 16:30'
labels:
  - mcp
  - graph
  - feature
  - high-priority
dependencies:
  - DRAFT-00016
references:
  - src/transport/mcp.rs
  - src/index/graph_store.rs
  - travisjakel okf-mcp context_packs
documentation:
  - docs/ai-usage.md#agent-workflows
priority: high
type: feature
ordinal: 75000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add get_context_pack MCP tool that returns optimized subgraph within token budget.

**Current:**
- traverse_graph returns raw nodes/edges, no token budget
- Agents must make 5+ calls to build context

**Expected:**
- get_context_pack(concept_path, token_budget=2000) → structured context pack
- Includes: primary concept, supporting evidence, contradictions, dependencies, metadata
- Progressive disclosure: metadata → headings → summary → full body
- MMR-style diversification for related concepts
- Returns token_count, truncated, strategy for each section

**Like travisjakel okf-mcp context_packs**
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 get_context_pack(path, token_budget=2000) returns structured context pack
- [ ] #2 Pack includes: primary, supporting, contradicting, dependencies, metadata
- [ ] #3 Token budget strictly enforced with progressive disclosure
- [ ] #4 MMR diversification for related concepts
- [ ] #5 Returns token_count, truncated, strategy for each section
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Unit test covers context packing algorithm
- [ ] #2 Integration test: agent uses context_pack instead of 5+ calls
- [ ] #3 Token budget never exceeded
<!-- DOD:END -->
