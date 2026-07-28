---
id: OKC-00052
title: 'Competitor Assessment: tribal + relay-knowledge'
status: Done
assignee:
  - '@research-agent'
created_date: '2026-07-27 18:02'
updated_date: '2026-07-27 20:30'
labels:
  - research
  - competitor
  - tribal
  - relay-knowledge
  - assessment
milestone: m-0
dependencies: []
references:
  - 'https://github.com/nicholasgriffintn/tribal-relay-knowledge'
  - 'https://www.npmjs.com/package/tribal-relay-knowledge'
documentation:
  - docs/competitors/
priority: medium
type: spike
ordinal: 49000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
**Competitors:** tribal, relay-knowledge
**Versions:** tribal 0.1.x, relay-knowledge 0.4.x
**Language:** Rust
**Links:** https://crates.io/crates/tribal | https://crates.io/crates/relay-knowledge

**What they do:**
**tribal** — Knowledge graph MCP server for AI-native code intelligence. Organizes code knowledge as a typed, queryable graph accessible to agents.
**relay-knowledge** — MCP server for knowledge graph management. Provides CRUD over knowledge triples with semantic queries.

**Why assess:** Both are Rust MCP servers exposing knowledge graph capabilities to agents — directly overlapping with OKC MCP server functionality. Understanding their graph models, query capabilities, and MCP tool design is essential.

**Assessment focus:**
1. Graph data model comparison (entity types, relationship types, properties)
2. MCP tool/resource design (what operations they expose)
3. Query capabilities (traversal, filtering, semantic search)
4. Integration patterns for agent consumption
5. Architecture and code quality
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Graph model comparison (entities, relations, properties vs OKC knowledge graph)
- [ ] #2 MCP server tool/resource inventory comparison
- [ ] #3 Query capability comparison
- [ ] #4 Architecture and code quality review
- [ ] #5 Feature gaps and OKC advantage opportunities
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Assessment written to docs/competitors/tribal-relay-knowledge-assessment.md
- [ ] #2 All ACs answered with evidence
<!-- DOD:END -->
