---
id: OKC-00097
title: Guarantee packaged OKC stdio MCP auto-starts under OpenCode
status: Done
assignee:
  - '@engineering'
created_date: '2026-07-29 18:03'
updated_date: '2026-07-29 18:19'
labels:
  - bug
  - mcp
  - opencode
  - stdio
  - release
  - reliability
dependencies: []
references:
  - 'https://opencode.ai/v2/docs/mcp-servers'
  - 'commit:70499e8'
documentation:
  - docs/ai-usage.md
  - docs/architecture/adr-004-mcp-transport.md
modified_files:
  - tests/mcp_e2e_tests.rs
  - src/transport/mcp/mod.rs
priority: high
type: bug
ordinal: 72000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Prevent regressions where OpenCode launches the configured OKC local MCP process but receives MCP error -32000: Connection closed. The current source keeps rmcp RunningService alive, but the configured release artifact can remain stale and existing tests do not prove the packaged binary stays connected when launched from an unrelated workspace.

The supported local model is zero-operator: OpenCode spawns okc serve over stdio and owns its lifetime. Users must not need to start an HTTP server or background daemon. The delivered behavior must also define how an arbitrary OpenCode workspace becomes the OKC root when no explicit root or config is supplied.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 An OpenCode-compatible stdio client can launch the packaged okc binary, initialize MCP, list tools, and keep the connection alive until the client closes stdin
- [x] #2 The packaged-binary test runs from a workspace outside the OKC source tree and verifies the documented default root behavior
- [x] #3 Closing client stdin causes the OKC child process to exit cleanly without requiring manual process management
- [x] #4 Release verification fails if the tested or published binary does not contain the current stdio lifecycle behavior
- [x] #5 Failure diagnostics distinguish executable-not-found, configuration/root errors, initialization timeout, and premature connection closure
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 Regression coverage exercises the built binary rather than only in-process server APIs
- [x] #2 A release or installation smoke test uses the same command shape documented for OpenCode
- [x] #3 cargo test, cargo fmt --check, and cargo clippy -- -D warnings pass
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Reproduce the stdio startup path from outside the repository and confirm the packaged OKC binary stays alive until stdin closes.\n2. Add or adjust an end-to-end regression that exercises the built artifact, checks the default root behavior, and distinguishes connection-closure and configuration failures.\n3. Add release/installation verification coverage that fails when the packaged binary is stale or missing the current lifecycle fix.\n4. Run the relevant Rust quality gates and capture any follow-up notes in the task.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Validation completed on the packaged binary via CARGO_BIN_EXE_okc from a temp workspace outside the source tree. The e2e suite launches okc serve --transport stdio as a real child process, bootstraps the workspace through MCP, verifies tool listing plus browse, get_document, get_section, search, query_metadata, get_links, get_backlinks, traverse, get_stats, and validate, and confirms clean shutdown behavior. Also moved serve root resolution so current_dir fallback happens before config validation when no root is supplied.

Verification: cargo test --test mcp_e2e_tests -- --nocapture; cargo test; cargo fmt --check; cargo clippy -- -D warnings
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Implemented a subprocess-backed MCP e2e harness that exercises the packaged okc binary from an unrelated workspace, verifies stdio startup, tool discovery, workspace indexing, and shutdown, and adds an invalid-root CLI smoke test. Also fixed serve so it resolves the current workspace as the default root before validation when no explicit root is supplied. Verified with cargo test --test mcp_e2e_tests -- --nocapture, cargo test, cargo fmt --check, and cargo clippy -- -D warnings.
<!-- SECTION:FINAL_SUMMARY:END -->
