---
id: OKC-00004
title: Add MCP server transport (stdio + HTTP)
status: To Do
assignee: []
created_date: '2026-07-23 00:49'
updated_date: '2026-07-24 01:12'
labels:
  - mcp
  - delivery
dependencies:
  - OKC-00002
references:
  - src/transport/cli.rs
  - docs/implementation-plan.md#7-mcp-server-transport-stdio--http
priority: high
type: feature
ordinal: 1400
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement MCP server with stdio transport for Claude Code and HTTP/SSE transport for web clients. Include tool definitions for all 9 AI operations.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 MCP server starts and registers 9 tools
- [ ] #2 stdio transport works with Claude Code
- [ ] #3 HTTP/SSE transport available on configurable port
- [ ] #4 Tool schemas match OKF spec (browse, get, section, search, query, links, backlinks, traverse, validate)
- [ ] #5 Serve subcommand runs a stdio-based MCP protocol server
- [ ] #6 Exposes search_bundles, get_bundle, validate_bundle, list_tags as MCP tools
- [ ] #7 MCP server uses &self shared-state pattern (Send + Sync) — thread-safe from day one
- [ ] #8 Client connects via npx/openai MCP agent and successfully calls every exposed tool
<!-- AC:END -->

## Comments

<!-- COMMENTS:BEGIN -->
created: 2026-07-23 06:50
---
Gap analysis finding: The `okf serve` CLI command already exists in src/transport/cli.rs as a stub (prints "MCP server coming soon" placeholder). The MCP transport directory `src/transport/` exists with cli.rs and mod.rs only — no actual MCP transport files. The task scope should include: (1) implementing the actual MCP protocol handler, (2) wiring it into the existing CLI serve command, (3) registering all 9 tool definitions via the mcp-server crate (or equivalent).
---
<!-- COMMENTS:END -->
