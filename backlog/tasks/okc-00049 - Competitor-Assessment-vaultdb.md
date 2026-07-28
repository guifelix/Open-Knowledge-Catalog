---
id: OKC-00049
title: 'Competitor Assessment: vaultdb'
status: Done
assignee:
  - '@research-agent'
created_date: '2026-07-27 18:01'
updated_date: '2026-07-27 20:30'
labels:
  - research
  - competitor
  - vaultdb
  - assessment
milestone: m-0
dependencies: []
references:
  - 'https://crates.io/crates/vaultdb'
  - 'https://github.com/doingtheprocess/vaultdb'
documentation:
  - docs/competitors/
priority: high
type: spike
ordinal: 45000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
**Competitor:** vaultdb (rusenbb/vaultdb)
**Version:** 1.6.1 | **Stars:** 0 | **Language:** Rust
**Links:** https://crates.io/crates/vaultdb | https://github.com/rusenbb/vaultdb

**What it does:**
Library + CLI + MCP server that treats folders of .md files with YAML frontmatter as a queryable relational database. Supports joins, graph traversal (BFS), bulk mutations, wiki-link renaming. No daemon, reads files directly. Obsidian-compatible.

**Why assess:** Most direct competitor — same trinity architecture (lib+CLI+MCP), same target use case, same tech stack (Rust). Highest overlap with OKC.

**Assessment focus:**
1. Feature comparison vs OKC (search, graph, MCP, export)
2. Architecture quality (code structure, error handling, tests)
3. Search quality (BM25, hybrid, filtering)
4. MCP server capabilities
5. Performance benchmarks
6. Community & documentation quality
7. Unique features OKC should match or beat
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Feature comparison matrix covering search, MCP, graph, export, CLI, library
- [ ] #2 Architecture review: code quality, test coverage, error handling
- [ ] #3 Search quality comparison against OKC benchmarks
- [ ] #4 MCP server capability comparison (tools, resources, prompts)
- [ ] #5 List of OKC must-beat features with concrete targets
- [ ] #6 Performance benchmarks on same dataset
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Assessment written to docs/competitors/vaultdb-assessment.md
- [ ] #2 All ACs answered with evidence (code refs, test results)
- [ ] #3 Prioritized action items for OKC improvement
<!-- DOD:END -->
