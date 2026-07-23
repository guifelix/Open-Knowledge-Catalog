---
id: OKC-00023
title: 'Fix naming inconsistency: okc vs okf'
status: To Do
assignee: []
created_date: '2026-07-23 19:03'
labels:
  - consistency
dependencies: []
priority: medium
type: chore
ordinal: 13400
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The CLI binary is named okc (open-knowledge-catalog) but the codebase uses okf (Open Knowledge Format) throughout config, docs, and help text. Pick one and rename consistently.\n\nCurrent state: binary = okf (from Cargo.toml), project = Open-Knowledge-Catalog (okc acronym).\n\nDecision needed: rename binary to okc to match repo name, or rename docs/config to okf to match binary. Either route requires consistent naming.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Single naming convention used throughout: binary name, Cargo.toml, README, help text, config paths, env vars
- [ ] #2 Old name binaries still work as symlinks for one minor version (migration period)
- [ ] #3 All shell completion scripts updated to new binary name
<!-- AC:END -->
