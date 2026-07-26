---
id: DRAFT-00025
title: Add Markdown / Obsidian vault importer adapter
status: To Do
assignee:
  - '@backend-agent'
created_date: '2026-07-25 20:03'
labels:
  - adoption
  - interoperability
dependencies: []
documentation:
  - docs/markdown-importer.md
priority: medium
type: feature
ordinal: 38000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Most potential users do not already have pure OKF repos. An importer (or treat as OKF with best-effort frontmatter) for plain Markdown folders and Obsidian vaults dramatically expands the addressable market and removes the chicken-and-egg problem.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Can point okc at a plain Markdown tree or Obsidian vault and produce a usable index
- [ ] #2 Missing frontmatter is handled gracefully (synthetic type/title where possible)
- [ ] #3 Wiki-links and common Obsidian conventions are resolved where feasible
- [ ] #4 Documented limitations vs full OKF
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Example vault or fixture in tests
- [ ] #2 Getting-started docs mention the importer path
- [ ] #3 No crashes on typical real-world vault noise
<!-- DOD:END -->
