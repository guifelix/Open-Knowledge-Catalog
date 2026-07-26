---
id: OKC-00008
title: Handle YAML aliases/anchors and merge keys safely
status: To Do
assignee: []
created_date: '2026-07-23 00:50'
updated_date: '2026-07-25 23:57'
labels:
  - correctness
dependencies: []
priority: high
type: feature
ordinal: 12400
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
saphyr supports YAML aliases/anchors and merge keys (<<). Need to either expand them deterministically or reject with clear error. Current implementation may produce unexpected results.

**Additional scope from draft-00031 (correctness backlog):**
- Link resolution edge cases: handle anchors, case sensitivity, URL encoding, and common edge cases without false broken-link reports
- validate_repository: implement the full intended set of checks (orphan documents, broken internal links, missing index files, duplicate IDs, circular references, parse failures, missing required fields, stale content)
- Regression tests for previously failing link resolution and YAML edge cases
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Aliases/anchors expanded or rejected with clear error
- [ ] #2 Merge keys (<<) expanded or rejected
- [ ] #3 No infinite loops in alias resolution
- [ ] #4 Clear error messages for unsupported YAML features
- [ ] #5 YAML anchors and aliases in frontmatter are correctly resolved before parsing
- [ ] #6 serde_yaml::Value deserialized anchors survive round-trip (Value -> internal -> persisted)
- [ ] #7 Anchor resolution does not change file contents on disk (read-only extraction)
- [ ] #8 Error on circular anchors does not crash — returns structured validation error
<!-- AC:END -->
