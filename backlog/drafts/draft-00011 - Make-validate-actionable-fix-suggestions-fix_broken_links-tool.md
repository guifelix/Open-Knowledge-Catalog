---
id: DRAFT-00011
title: 'Make validate actionable: fix suggestions + fix_broken_links tool'
status: To Do
assignee:
  - '@backend-agent'
created_date: '2026-08-06 00:50'
labels:
  - mcp
  - backend
  - feature
  - high-priority
dependencies: []
references:
  - src/transport/mcp.rs
  - src/index/validate.rs
  - src/parser/links.rs
documentation:
  - docs/ai-usage.md#repository-validation
priority: high
type: feature
ordinal: 75000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
validate() returns 55 issues but no fix suggestions or auto-repair.

**Current:**
- 52 broken link warnings with no line numbers, context, or suggested fixes
- 3 YAML errors with no repair guidance
- No fix_broken_links tool

**Expected:**
- Each issue includes: line_number, column, source_context, suggested_fix
- fix_broken_links tool: dry_run + apply modes
- Auto-repair common patterns: wikilinks → markdown links, path traversal → relative paths
- Batch fix with confirmation
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 validate returns line_number, column, source_context for each issue
- [ ] #2 validate returns suggested_fix for broken links (wikilink → markdown, path fix)
- [ ] #3 fix_broken_links dry_run shows proposed changes
- [ ] #4 fix_broken_links apply repairs issues, updates index
- [ ] #5 Batch fix with per-issue confirmation option
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Unit test covers fix suggestion generation
- [ ] #2 Integration test: validate → fix_broken_links → validate clean
- [ ] #3 Common patterns auto-repaired: [[wikilink]], email links, path traversal
<!-- DOD:END -->
