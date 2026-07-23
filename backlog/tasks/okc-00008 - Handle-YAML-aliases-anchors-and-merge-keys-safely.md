---
id: OKC-00008
title: Handle YAML aliases/anchors and merge keys safely
status: To Do
assignee: []
created_date: '2026-07-23 00:50'
updated_date: '2026-07-23 19:02'
labels:
  - correctness
dependencies: []
priority: medium
type: feature
ordinal: 12400
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
saphyr supports YAML aliases/anchors and merge keys (<<). Need to either expand them deterministically or reject with clear error. Current implementation may produce unexpected results.
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
