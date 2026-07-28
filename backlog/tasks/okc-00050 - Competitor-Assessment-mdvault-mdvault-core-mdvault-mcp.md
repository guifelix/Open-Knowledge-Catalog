---
id: OKC-00050
title: 'Competitor Assessment: mdvault (mdvault-core, mdvault-mcp)'
status: Done
assignee:
  - '@research-agent'
created_date: '2026-07-27 18:02'
updated_date: '2026-07-27 20:30'
labels:
  - research
  - competitor
  - mdvault
  - assessment
milestone: m-0
dependencies: []
references:
  - 'https://github.com/TN1ck/mdvault'
  - 'https://pypi.org/project/mdvault/'
  - 'https://www.npmjs.com/package/@tn1ck/mdvault-mcp'
documentation:
  - docs/competitors/
priority: high
type: spike
ordinal: 46000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
**Competitor:** mdvault ecosystem
**Versions:** mdvault 0.7.2, mdvault-core 0.4.0, mdvault-mcp 0.3.0
**Language:** Rust
**Links:** https://crates.io/crates/mdvault | https://crates.io/crates/mdvault-core | https://crates.io/crates/mdvault-mcp

**What it does:**
CLI tool + library + MCP server for managing markdown vaults with structured notes, validation, search. mdvault-core provides the library layer. mdvault-mcp provides MCP server for agent consumption.

**Why assess:** Same Rust stack, same three-component architecture (lib+CLI+MCP) as OKC, same target (markdown vaults). One of the closest structural competitors.

**Assessment focus:**
1. Feature scope vs OKC (search, graph, validation, export)
2. mdvault-mcp MCP server capabilities
3. Code architecture, test coverage, documentation
4. Search implementation quality
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Complete feature comparison matrix vs OKC
- [ ] #2 MCP server capability comparison
- [ ] #3 Code architecture review
- [ ] #4 Differentiation analysis: what OKC does better, what mdvault does better
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Assessment written to docs/competitors/mdvault-assessment.md
- [ ] #2 All ACs answered with evidence
<!-- DOD:END -->
