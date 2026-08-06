---
id: OKC-00100
title: 'Migrate OKC to newer rmcp, tower, tower-http, and toml releases'
status: To Do
assignee: []
created_date: '2026-07-29 18:49'
labels:
  - deps
  - rust
  - mcp
  - http
  - config
  - upgrade
dependencies:
  - OKC-00099
references:
  - Cargo.toml
  - Cargo.lock
modified_files:
  - Cargo.toml
priority: high
type: task
ordinal: 74000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Update the dependency set after the audit identifies a safe migration path for rmcp, tower, tower-http, and toml. These versions affect MCP transport, HTTP plumbing, and configuration parsing, so the implementation needs to keep the existing behavior stable while moving the codebase onto the newer releases.

This task should remain limited to the dependency migration and the code, test, and documentation changes required by that migration.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Cargo.toml and Cargo.lock are updated to the chosen dependency versions from the audit
- [ ] #2 The codebase builds and the existing test suite passes after the upgrade
- [ ] #3 Any required code changes for MCP transport, HTTP server wiring, or config parsing are completed and covered by tests
- [ ] #4 No new regressions are introduced in the packaged binary, e2e MCP behavior, or config loading
<!-- AC:END -->
