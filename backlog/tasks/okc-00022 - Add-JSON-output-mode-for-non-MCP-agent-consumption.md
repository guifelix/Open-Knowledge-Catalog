---
id: OKC-00022
title: Add JSON output mode for non-MCP agent consumption
status: To Do
assignee: []
created_date: '2026-07-23 19:03'
updated_date: '2026-07-23 19:04'
labels:
  - agent-ux
dependencies: []
priority: high
type: feature
ordinal: 8400
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add --json flag to all CLI subcommands so AI agents consuming okf via CLI (not MCP) get structured, machine-parseable output instead of human-only text.\n\nCurrent state: CLI output is pure text tables and prose. Agents must regex-parse output.\n\nRequired: Every subcommand (search, get, browse, query, links, backlinks, traverse, validate, summary, export) supports --json flag or JSON_LINES mode for agent consumption.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Every CLI subcommand supports --json flag
- [ ] #2 JSON output follows a consistent schema: {status: "ok"|"error", data: {...}, meta: {duration_ms, result_count}}
- [ ] #3 Error output is also JSON: {status: "error", error: {code, message}}
- [ ] #4 Pipe-friendly: JSON output on stdout, human errors on stderr
- [ ] #5 okf search --json "query" | jq works without extra flags
<!-- AC:END -->
