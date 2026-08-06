---
id: OKC-00099
title: 'Audit dependency upgrades for rmcp, tower, tower-http, and toml'
status: To Do
assignee: []
created_date: '2026-07-29 18:48'
updated_date: '2026-07-29 18:50'
labels:
  - deps
  - audit
  - rust
dependencies: []
references:
  - Cargo.toml
  - Cargo.lock
modified_files:
  - Cargo.toml
priority: medium
type: spike
ordinal: 73000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Review the current dependency pins and determine which upgrades are safe to take now versus which require a planned migration. Focus on rmcp, tower, tower-http, and toml, since they affect MCP transport, HTTP plumbing, and config parsing.

The audit should produce a concrete compatibility matrix and migration order so the follow-up implementation task can be scoped without guesswork.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Document the latest available compatible release for each target dependency and note whether it is a patch, minor, or major jump
- [ ] #2 Call out the API or behavior changes that matter to OKC for each major upgrade candidate
- [ ] #3 Recommend a safe upgrade order and identify any dependencies that should move together
- [ ] #4 Record whether the current test suite is sufficient to validate each upgrade path or whether new coverage is needed
<!-- AC:END -->
