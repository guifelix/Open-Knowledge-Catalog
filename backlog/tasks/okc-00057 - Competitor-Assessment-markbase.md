---
id: OKC-00057
title: 'Competitor Assessment: markbase'
status: Done
assignee:
  - '@research-agent'
created_date: '2026-07-27 18:01'
updated_date: '2026-07-27 20:30'
labels:
  - research
  - competitor
  - markbase
  - assessment
milestone: m-0
dependencies: []
references:
  - 'https://crates.io/crates/markbase'
documentation:
  - docs/competitors/
priority: medium
type: spike
ordinal: 52000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
**Competitor:** markbase (flyisland/markbase)
**Version:** 0.9.6 | **Stars:** 1 | **Language:** Rust
**Links:** https://crates.io/crates/markbase | https://github.com/flyisland/markbase

**What it does:**
Obsidian-compatible CLI with a DuckDB-backed template system for schema consistency in markdown notes. `note verify` checks notes against YAML-defined templates — validates required fields, value types, allowed values. Designed as a three-way contract: Human (intent) → Agent (fill templates) → markbase (enforce).

**Why assess:** Unique angle — schema/template enforcement for AI-written notes. If OKC plans to support AI agents writing knowledge files, template validation is a critical feature. Completely different DB backend (DuckDB) vs OKC's Tantivy/graph approach.

**Assessment focus:**
1. Template definition format and validation capabilities
2. DuckDB integration patterns and query capabilities
3. Three-way contract concept (Human→Agent→System)
4. Obsidian compatibility layer
5. Code architecture and test coverage
6. Evaluate whether template validation should be an OKC feature
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Template system design comparison (schema format, validation rules, ergonomics)
- [ ] #2 DuckDB integration patterns — pros/cons vs Tantivy
- [ ] #3 Assessment of template validation as potential OKC feature
- [ ] #4 Code architecture review
- [ ] #5 Obsidian compatibility comparison
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Assessment written to docs/competitors/markbase-assessment.md
- [ ] #2 All ACs answered with evidence
- [ ] #3 Recommendation on template validation for OKC roadmap
<!-- DOD:END -->
