---
id: DRAFT-00023
title: >-
  Define typed relation schema in frontmatter (supports, contradicts,
  supersedes, derived_from)
status: To Do
assignee:
  - '@backend-agent'
created_date: '2026-08-06 16:32'
labels:
  - graph
  - schema
  - feature
  - high-priority
dependencies: []
references:
  - src/parser/frontmatter.rs
  - src/index/validate.rs
  - src/index/graph_store.rs
  - src/transport/mcp.rs
documentation:
  - docs/ai-usage.md#relationship-reasoning
priority: high
type: feature
ordinal: 75000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Define standard typed relation schema for OKF frontmatter.

**Current:**
- Links are untyped markdown links
- OKC-00039 mentions "typed edges" but no schema defined

**Expected:**
- Frontmatter field: relations: [{type: "supports", target: "path"}, {type: "contradicts", target: "path"}]
- Standard relation types: supports, contradicts, supersedes, derived_from, references, depends_on
- Validation: target exists, type is valid enum
- Migration guide for existing links
- MCP tool: get_relations(path) returns typed relations

**Example frontmatter:**
```yaml
relations:
  - type: supports
    target: metrics/monthly-revenue.md
  - type: contradicts
    target: metrics/old-revenue.md
  - type: derived_from
    target: datasets/customer-orders.md
```
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Frontmatter accepts relations array with type and target
- [ ] #2 Relation types validated: supports, contradicts, supersedes, derived_from, references, depends_on
- [ ] #3 Target path validation: must exist in repository
- [ ] #4 MCP tool get_relations(path) returns typed relations
- [ ] #5 validate() checks relation targets exist and types valid
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Unit test covers relation parsing and validation
- [ ] #2 Integration test: scan → validate → typed relations work
- [ ] #3 Migration doc for existing untyped links
<!-- DOD:END -->
