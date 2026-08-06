---
id: DRAFT-00015
title: >-
  Add typed graph relationships (supports, contradicts, supersedes,
  derived_from)
status: To Do
assignee:
  - '@backend-agent'
created_date: '2026-08-06 00:51'
labels:
  - mcp
  - backend
  - feature
  - medium-priority
dependencies: []
references:
  - src/index/graph_store.rs
  - src/parser/links.rs
  - tribal implementation
documentation:
  - docs/ai-usage.md#relationship-reasoning
priority: medium
type: feature
ordinal: 75000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Graph traversal only follows explicit Markdown links. No typed relationships.

**Current:**
- traverse() follows markdown links only
- No concept-type relationships
- No "similar to", "depends on", "contradicts"

**Expected (like tribal):**
- Typed relations: supports, contradicts, supersedes, derived_from, references
- Relation stored in frontmatter or separate relation files
- traverse() filters by relation type
- Multi-hop reasoning with relation scoring
- Subgraph extraction for context packing
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Frontmatter supports relations: [{type: supports, target: path}, ...]
- [ ] #2 traverse(relation_type=supports) filters by relation type
- [ ] #3 traverse(relation_type=contradicts) finds conflicting concepts
- [ ] #4 Multi-hop reasoning with relation weights
- [ ] #5 Subgraph extraction for context packing (token budget)
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Unit test covers relation parsing and typed traversal
- [ ] #2 Integration test: find all concepts supporting a claim
- [ ] #3 Backward compatible: untyped links still work
<!-- DOD:END -->
