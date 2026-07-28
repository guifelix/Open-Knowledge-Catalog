---
id: OKC-00059
title: 'Competitor Assessment: mcp-knowledge-base (TF-IDF MCP)'
status: Done
assignee:
  - '@research-agent'
created_date: '2026-07-27 18:02'
updated_date: '2026-07-27 20:30'
labels:
  - research
  - competitor
  - mcp-knowledge-base
  - assessment
milestone: m-0
dependencies: []
documentation:
  - docs/competitors/
priority: medium
type: spike
ordinal: 54000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
**Competitor:** mcp-knowledge-base
**Version:** 1.2.0
**Language:** Rust
**Links:** https://crates.io/crates/mcp-knowledge-base

**What it does:**
MCP server for managing articles, policies, and known issues with TF-IDF search. Provides a CRUD knowledge base accessible to AI agents via MCP protocol. Organizes content as categorized documents with metadata.

**Why assess:** Pure MCP-server knowledge base in Rust — uses TF-IDF (not BM25, not embeddings) for search. Direct MCP-level competitor. Simple, focused, production-mature (v1.2.0).

**Assessment focus:**
1. MCP tool/resource design (CRUD operations, search, categories)
2. TF-IDF search quality vs OKC BM25
3. Knowledge model (document types, metadata, categories)
4. Production readiness and architecture quality
5. Feature gap analysis vs OKC
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 MCP server capability inventory vs OKC MCP
- [ ] #2 Search quality comparison (TF-IDF vs BM25)
- [ ] #3 Knowledge model and metadata comparison
- [ ] #4 Code architecture and production readiness review
- [ ] #5 OKC advantages and gaps
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Assessment written to docs/competitors/mcp-knowledge-base-assessment.md
- [ ] #2 All ACs answered with evidence
<!-- DOD:END -->
