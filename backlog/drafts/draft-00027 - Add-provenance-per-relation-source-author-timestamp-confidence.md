---
id: DRAFT-00027
title: 'Add provenance per relation (source, author, timestamp, confidence)'
status: To Do
assignee:
  - '@backend-agent'
created_date: '2026-08-06 16:38'
labels:
  - graph
  - schema
  - feature
  - medium-priority
dependencies: []
references:
  - src/parser/frontmatter.rs
  - src/index/graph_store.rs
  - src/transport/mcp.rs
documentation:
  - docs/ai-usage.md#relationship-reasoning
priority: medium
type: feature
ordinal: 75000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add provenance metadata to typed relations for trust scoring.

**Current:**
- Relations have no provenance
- Cannot assess evidence quality

**Expected:**
- Frontmatter relations include provenance:
```yaml
relations:
  - type: supports
    target: metrics/monthly-revenue.md
    provenance:
      source: human:analyst
      author: jane.doe
      timestamp: 2026-08-06T10:00:00Z
      confidence: 0.9
```
- MCP tool get_relations returns provenance
- Trust scoring: weight relations by confidence
- validate() checks provenance completeness
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Frontmatter relations accept provenance object (source, author, timestamp, confidence)
- [ ] #2 MCP tool get_relations returns provenance for each relation
- [ ] #3 Trust scoring: traverse weights relations by confidence
- [ ] #4 validate() warns on missing provenance for critical relations
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Unit test covers provenance parsing and trust scoring
- [ ] #2 Integration test: traverse with confidence weighting
- [ ] #3 Schema validation for provenance fields
<!-- DOD:END -->
