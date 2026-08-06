---
id: OKC-00098
title: Correct OpenCode MCP onboarding and zero-operator lifecycle documentation
status: Done
assignee:
  - '@engineering'
created_date: '2026-07-29 18:03'
updated_date: '2026-07-29 18:36'
labels:
  - docs
  - mcp
  - opencode
  - onboarding
  - dx
dependencies:
  - DRAFT-00005
references:
  - 'https://opencode.ai/v2/docs/mcp-servers'
documentation:
  - README.md
  - docs/ai-usage.md
  - docs/configuration.md
  - docs/features.md
  - docs/architecture/adr-004-mcp-transport.md
modified_files:
  - README.md
  - docs/ai-usage.md
  - docs/configuration.md
  - docs/features.md
  - docs/architecture/adr-004-mcp-transport.md
priority: high
type: docs
ordinal: 72000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Replace stale and misleading OpenCode setup guidance with the supported local MCP configuration and an explicit lifecycle explanation. The current docs use the obsolete mcpTools shape with separate command and args fields, and generic quick-start sections tell users to start okc serve without explaining that OpenCode automatically spawns the stdio child process.

Document global versus project-local configuration, PATH versus absolute executable paths, arbitrary-workspace root selection, config precedence, verification with opencode mcp list, and targeted troubleshooting for MCP error -32000. Keep manually operated HTTP transport clearly separated as an optional remote/team deployment.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 OpenCode examples use the supported mcp local-server schema with command as an executable-and-arguments array
- [x] #2 Documentation states that OpenCode automatically starts and stops the stdio MCP child process and that users do not manually run a daemon
- [x] #3 Global configuration works from arbitrary projects and documents whether the current workspace or an explicit root is indexed
- [x] #4 Project-local configuration and config precedence are explained without implying that users must duplicate global configuration
- [x] #5 Troubleshooting for MCP error -32000 includes checking the resolved executable, rebuilding or reinstalling stale binaries, running opencode mcp list, and inspecting root/config failures
- [x] #6 HTTP transport is described only as the manually hosted option for remote or shared access
- [x] #7 README quick start, AI usage, configuration, features, and MCP architecture guidance use consistent terminology and command examples
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 Examples are validated against the current OpenCode configuration schema and the packaged OKC binary
- [x] #2 All internal documentation links resolve and no obsolete mcpTools OpenCode examples remain
- [x] #3 cargo test, cargo fmt --check, and cargo clippy -- -D warnings pass
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Audit README and docs for stale OpenCode MCP examples, lifecycle wording, and root/config guidance.\n2. Update the supported local-server example to match the current OpenCode schema and explain that OpenCode auto-starts the stdio child process.\n3. Clarify global versus project-local configuration, arbitrary-workspace root behavior, config precedence, and HTTP as the manual remote option.\n4. Verify internal links and terminology are consistent across README, AI usage, configuration, features, and MCP architecture docs.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Updated README, AI usage, configuration, features, and MCP architecture docs to describe OpenCode local MCP servers with the current command-array schema, automatic stdio lifecycle, workspace root behavior, config precedence, and -32000 troubleshooting. Verified that obsolete mcpTools examples are removed and that the internal documentation links in the touched pages still point at existing docs.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Corrected OpenCode MCP onboarding and lifecycle documentation across README, AI usage, configuration, features, and the MCP transport ADR. The docs now show the supported local-server schema, explain that OpenCode starts and stops the stdio child process automatically, cover global versus project-local config precedence and root selection, and document troubleshooting for MCP error -32000. Verified with cargo fmt --check, cargo test, and cargo clippy -- -D warnings.
<!-- SECTION:FINAL_SUMMARY:END -->
