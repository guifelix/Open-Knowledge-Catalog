---
id: OKC-00101
title: Return structured MCP content instead of text-wrapped JSON
status: Done
assignee:
  - '@codex'
created_date: '2026-08-06 00:47'
updated_date: '2026-08-06 20:56'
labels:
  - mcp
  - backend
  - enhancement
  - high-priority
dependencies: []
references:
  - src/transport/mcp.rs
  - rmcp crate documentation
  - 'MCP spec: structuredContent'
documentation:
  - docs/ai-usage.md#mcp-tools
priority: high
type: enhancement
ordinal: 75000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
All MCP tools currently return JSON as a TEXT string inside MCP content, forcing agents to double-parse.

**Current response format:**
```json
{"content":[{"type":"text","text":"{"results":[{"path":"ai-usage.md",...}]}"}],"isError":false}
```

**Problems:**
- Agent must parse JSON from text, then parse again
- No structuredContent field in MCP response
- Wastes tokens, error-prone, breaks schema validation
- Inconsistent with MCP spec which supports structuredContent

**Expected:**
- Return proper structuredContent with typed schema
- Keep text fallback for compatibility
- Define JSON schemas for each tool response
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 search tool returns structuredContent with typed results array
- [x] #2 get_document returns structuredContent with document object
- [x] #3 query_metadata returns structuredContent with results array
- [x] #4 All 11 tools return structuredContent matching their output schema
- [x] #5 text fallback still present for backward compatibility
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 MCP schema validation passes for all tool responses
- [x] #2 Integration test verifies structuredContent is parseable without double-parsing
- [x] #3 No regression in existing CLI or MCP text output
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add packaged MCP tests that require object-root output schemas, structuredContent, and valid compatibility JSON for every tool.
2. Return typed rmcp Json<T> results for object-shaped responses; use explicit schema wrappers for section/link responses while preserving their historical text shapes.
3. Document the response contract and run the focused MCP suite plus the full fmt/test/clippy gate.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added packaged-binary red tests requiring object-root outputSchema and structuredContent for all eleven tools. Converted handlers to typed rmcp Json<T> results; get_section and link tools use schema-compliant object wrappers while preserving their legacy text JSON shapes. Documented the structured response contract in AI usage, features, and the MCP transport ADR.

Final validation: packaged MCP E2E tests pass for all eleven tools; every advertised outputSchema has an object root and expected typed property; structuredContent is directly consumed; all text fallbacks remain valid JSON, with exact legacy section/link shapes asserted. cargo fmt --check, cargo test (260 tests), cargo clippy -- -D warnings, and git diff --check pass.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Implemented typed MCP structuredContent and advertised outputSchema for all eleven tools. Object-shaped handlers use rmcp Json<T>; section and link handlers use explicit object wrappers while retaining their historical JSON text fallback shapes. Updated MCP documentation and verified the packaged protocol end to end. Validation passed: cargo fmt --check, cargo test (260 tests), cargo clippy -- -D warnings, and git diff --check.
<!-- SECTION:FINAL_SUMMARY:END -->
