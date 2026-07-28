---
id: OKC-00061
title: 'Competitor Assessment: arkouda + memstead'
status: Done
assignee:
  - '@research-agent'
created_date: '2026-07-27 18:03'
updated_date: '2026-07-27 20:30'
labels:
  - research
  - competitor
  - arkouda
  - memstead
  - assessment
milestone: m-0
dependencies: []
documentation:
  - docs/competitors/
priority: low
type: spike
ordinal: 58000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
**Competitors:** arkouda, memstead
**Versions:** arkouda 0.1.x, memstead 0.1.x
**Language:** Rust
**Links:** https://crates.io/crates/arkouda | https://crates.io/crates/memstead

**What they do:**
**arkouda** — AI-native CLI for Architecture Decision Records (ADRs). Helps developers and AI agents create, manage, and query ADRs.
**memstead** — Typed graph of entities stored in markdown files. Manages entities, relationships, and properties using markdown as the storage format.

**Why assess:** Both are Rust CLI tools that treat structured content (ADRs, entity graphs) as markdown — adjacent to OKC. arkouda overlaps with document management, memstead overlaps with knowledge graph representation.

**Assessment focus:**
1. arkouda ADR model and CLI design
2. memstead entity graph model and markdown storage
3. Integration potential with OKC
4. Feature overlap and differentiation
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Functional assessment of each tool
- [ ] #2 Markdown storage model comparison
- [ ] #3 Integration potential with OKC
- [ ] #4 Boundary and differentiation analysis
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Assessment written to docs/competitors/arkouda-memstead-assessment.md
<!-- DOD:END -->
