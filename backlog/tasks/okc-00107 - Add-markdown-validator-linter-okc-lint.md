---
id: OKC-00107
title: 'Add Markdown style linting for CLI, MCP, and CI'
status: To Do
assignee:
  - '@backend-agent'
created_date: '2026-08-06 15:51'
updated_date: '2026-08-06 19:53'
labels:
  - cli
  - mcp
  - markdown
  - lint
  - ci
dependencies: []
references:
  - src/index/validate.rs
  - src/transport/cli.rs
documentation:
  - docs/ai-usage.md
priority: medium
type: feature
ordinal: 75000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add an optional Markdown style-linting capability distinct from OKF conformance and repository-schema validation. It should reuse a maintained Rust lint engine when suitable, report source diagnostics consistently through CLI and MCP, and establish the shared layered-configuration resolver later consumed by directory-schema validation.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A documented CLI command lints a file or repository and returns stable CI-appropriate exit codes
- [ ] #2 An MCP surface returns structured rule identifiers, severity, message, path, line, and column diagnostics
- [ ] #3 Supported rules are explicitly style-focused and do not duplicate OKF conformance or directory document-schema rules
- [ ] #4 Configuration resolves deterministically from repository defaults through parent and nearest-directory overrides, with documented merge semantics
- [ ] #5 Per-document rule suppression is namespaced to linting, validated, and cannot disable unrelated repository validation
- [ ] #6 Safe auto-fixes support preview and explicit apply modes and never modify files when diagnostics cannot be resolved safely
- [ ] #7 The selected lint engine is justified by maintained library API, license compatibility, diagnostic quality, and testability rather than popularity metrics
- [ ] #8 Tests cover rule diagnostics, configuration inheritance, suppression, exit codes, preview/apply behavior, and unchanged-file safety
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Unit test covers each lint rule via wrapped library
- [ ] #2 Integration test: lint → auto-fix → lint clean
- [ ] #3 MCP tool returns structured violations for agent consumption
- [ ] #4 Config resolution works: global + parent + current directory override
- [ ] #5 Escape hatch works: frontmatter lint.ignore bypasses specific rules
- [ ] #6 Library wrapped cleanly: no fork, upstream updates via cargo update
<!-- DOD:END -->
