---
id: OKC-00053
title: 'Competitor Assessment: semantic-memory-mcp'
status: Done
assignee:
  - '@research-agent'
created_date: '2026-07-27 18:02'
updated_date: '2026-07-27 20:30'
labels:
  - research
  - competitor
  - semantic-memory
  - assessment
milestone: m-0
dependencies: []
references:
  - 'https://www.npmjs.com/package/@hokify/semantic-memory-mcp'
  - 'https://github.com/hokify/semantic-memory-mcp'
documentation:
  - docs/competitors/
priority: medium
type: spike
ordinal: 50000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
**Competitor:** semantic-memory-mcp
**Language:** TypeScript/Rust
**Links:** https://crates.io/crates/semantic-memory-mcp

**What it does:**
Local-first knowledge management MCP server. Stores knowledge as semantic memories with evidence scores, confidence ratings, and source attribution. Supports querying by relevance, recency, and confidence. Designed for AI agents to build and query persistent knowledge bases during coding sessions.

**Why assess:** Directly competes with OKC MCP server for the "agent knowledge base" use case. Unique features include evidence-based confidence scoring and source attribution — important differentiators to understand.

**Assessment focus:**
1. Memory/knowledge data model (evidence scores, confidence, attribution)
2. MCP server capabilities (tools, resources, resource templates)
3. Search and retrieval capabilities
4. Architecture and integration patterns
5. Feature comparison with OKC MCP server
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Knowledge model comparison (evidence scoring, confidence, attribution)
- [ ] #2 MCP server capability inventory
- [ ] #3 Search and retrieval quality comparison
- [ ] #4 Architecture review
- [ ] #5 OKC improvement opportunities from semantic-memory patterns
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Assessment written to docs/competitors/semantic-memory-mcp-assessment.md
- [ ] #2 All ACs answered with evidence
<!-- DOD:END -->
