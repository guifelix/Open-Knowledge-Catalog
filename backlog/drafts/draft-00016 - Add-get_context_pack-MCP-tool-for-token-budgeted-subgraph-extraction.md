---
id: DRAFT-00016
title: Add get_context_pack MCP tool for token-budgeted subgraph extraction
status: To Do
assignee:
  - '@backend-agent'
created_date: '2026-08-06 00:52'
labels:
  - mcp
  - backend
  - feature
  - medium-priority
dependencies:
  - DRAFT-00021
references:
  - src/transport/mcp.rs
  - src/index/graph_store.rs
  - travisjakel okf-mcp context_packs
documentation:
  - docs/ai-usage.md#agent-workflows
priority: medium
type: feature
ordinal: 75000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
No way to get optimized context for a concept within token budget.

**Missing:**
- get_context_pack(concept_path, token_budget=2000) → optimized subgraph
- Progressive disclosure: metadata → headings → summary → full body
- MMR-style diversification for related concepts
- Includes: concept, supporting evidence, contradictions, dependencies

**Expected (like travisjakel context_packs):**
- Single call returns agent-ready context pack
- Respects token budget strictly
- Prioritizes: direct links, same-type concepts, high-confidence relations
- Returns: primary_concept, supporting, contradicting, dependencies, metadata
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
