---
id: DRAFT-00017
title: Complete and harden MCP server (stdio + basic HTTP/SSE)
status: To Do
assignee:
  - '@backend-agent'
created_date: '2026-07-25 19:59'
labels:
  - mcp
  - agent-ux
  - p0
dependencies: []
documentation:
  - docs/ai-usage.md
priority: high
type: feature
ordinal: 38000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Finish the MCP transport so AI agents can call the 9 core tools without shelling out to the CLI. Stdio is the minimum viable path; HTTP/SSE is highly desirable for remote or multi-client use. This is the primary adoption gate for agent users.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 All 9 tools (browse_directory, get_document, get_section, search_documents, query_metadata, get_links, get_backlinks, traverse_graph, validate_repository) are exposed via MCP
- [ ] #2 Stdio transport works with Claude Desktop / Cursor / other common MCP clients
- [ ] #3 Tool schemas are accurate (schemars) and documented
- [ ] #4 Response size limits and path confinement are enforced on the MCP path
- [ ] #5 Basic HTTP/SSE transport optional but functional for local testing
- [ ] #6 Official MCP config snippet published in docs
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Integration tests against a real MCP client or mock
- [ ] #2 docs/ai-usage.md updated with copy-paste configs
- [ ] #3 Binary can be started as okc mcp (or equivalent)
<!-- DOD:END -->

## Implementation Plan
<!-- SECTION:PLAN:BEGIN -->
1. **Choose MCP framework**: Evaluate `rmcp` (official Rust SDK) vs `mcp-server` vs custom; recommend `rmcp` for spec compliance and schemars integration
2. **Define tool schemas**: Use `schemars` to derive JSON Schema from typed input/output structs for all 9 tools; ensure descriptions match CLI help
3. **Implement stdio transport**: 
   - Add `okc mcp` subcommand that spawns stdio server
   - Wire each tool to existing service layer (reuse `search_documents`, `get_document`, etc.)
   - Enforce response size limits (configurable, default 1MB) and path confinement per DRAFT-00018
4. **Add HTTP/SSE transport** (stretch):
   - Axum-based server with `/mcp` endpoint for SSE
   - Session management for multi-client support
   - CORS config for browser-based clients
5. **Integration testing**: 
   - Test against `mcp-inspector` and Claude Desktop config
   - Mock client tests for all 9 tools with various inputs
   - Stress test concurrent requests
6. **Documentation**: 
   - Add `okc mcp --help` output
   - Publish Claude Desktop / Cursor config snippets in `docs/ai-usage.md`
   - Document HTTP/SSE endpoint and auth (if any)
<!-- SECTION:PLAN:END -->
