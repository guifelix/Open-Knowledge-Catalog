---
id: OKC-00100
title: 'Migrate OKC to newer rmcp, tower, tower-http, and toml releases'
status: In Progress
assignee: []
created_date: '2026-07-29 18:49'
updated_date: '2026-08-07 17:57'
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

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. toml 0.8->1.x first (independent, low risk): bump toml dep, run cargo test incl. config round-trip; adjust if to_string_pretty shifts. 2. rmcp 2.2->3.x TOGETHER with tower/tower-http: fix .with_stateful_mode->.with_legacy_session_mode in src/transport/mcp/mod.rs, drop unused direct tower/tower-http deps, rewrite rmcp client in tests/mcp_e2e_tests.rs for rmcp3 lifecycle, ADD HTTP-transport e2e test. 3. cargo test, fmt, clippy clean.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Step 1 (toml 0.8->1.1.4) done: build green, 20/20 integration tests pass (config round-trip + to_string_pretty confirmed stable).
<!-- SECTION:NOTES:END -->

## Comments

<!-- COMMENTS:BEGIN -->
created: 2026-08-07 17:52
---
Starting OKC-00100 dependency migration per OKC-00099 audit matrix.
---

created: 2026-08-07 17:57
---
toml migrated; moving to rmcp 3 migration.
---
<!-- COMMENTS:END -->
