---
id: OKC-00025
title: Remove unused Tokio dependency
status: To Do
assignee: []
created_date: '2026-07-23 19:03'
updated_date: '2026-07-25 23:57'
labels:
  - cleanup
dependencies: []
priority: low
type: chore
ordinal: 14400
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Tokio is listed as a dependency in Cargo.toml but only used by the serve subcommand stub (which prints a placeholder). Until MCP server (OKC-00004) is implemented, Tokio pulls in unnecessary compile time and binary size.\n\nRemove Tokio from [dependencies] and move it to [dev-dependencies] or gate it behind a 'server' feature flag. Re-add when OKC-00004 starts implementing the actual MCP protocol handler.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Tokio removed from [dependencies] -- placed behind a "server" feature flag
- [ ] #2 cargo build --no-default-features succeeds (no Tokio in tree)
- [ ] #3 cargo build succeeds with default features (serve subcommand still works)
- [ ] #4 Binary size reduction measurable via du -sh on target/release/okf
<!-- AC:END -->
