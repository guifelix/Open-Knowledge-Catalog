---
id: OKC-00055
title: 'Competitor Assessment: wicked-knowledge-mcp + wicked-estate'
status: Done
assignee:
  - '@research-agent'
created_date: '2026-07-27 18:03'
updated_date: '2026-07-27 20:30'
labels:
  - research
  - competitor
  - wicked-knowledge
  - assessment
milestone: m-0
dependencies: []
references:
  - 'https://github.com/wicked-labs/wicked-knowledge-mcp'
documentation:
  - docs/competitors/
priority: low
type: spike
ordinal: 55000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
**Competitors:** wicked-knowledge-mcp (deprecated), wicked-estate
**Language:** TypeScript/Rust
**Links:** https://crates.io/crates/wicked-knowledge-mcp

**What they do:**
**wicked-knowledge-mcp** — MCP server for knowledge management with CRUD, semantic search, and tagging. Deprecated in favor of wicked-estate.
**wicked-estate** — Successor project with expanded scope beyond knowledge management.

**Why assess:** Represents an MCP-first knowledge management approach. Even though deprecated, understanding the architecture reveals patterns and pitfalls in MCP knowledge base design.

**Assessment focus:**
1. Knowledge model and MCP tool design
2. Reasons for deprecation/rebuild (what didnt work)
3. wicked-estate scope and architecture
4. Lessons for OKC MCP design
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Knowledge model and MCP tool design review
- [ ] #2 Deprecation lessons for OKC MCP design
- [ ] #3 wicked-estate architecture and scope assessment
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Assessment written to docs/competitors/wicked-knowledge-assessment.md
- [ ] #2 Key architectural lessons for OKC
<!-- DOD:END -->
