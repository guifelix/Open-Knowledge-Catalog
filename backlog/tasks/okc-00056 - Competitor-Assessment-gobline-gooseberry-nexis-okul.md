---
id: OKC-00056
title: 'Competitor Assessment: gobline + gooseberry + nexis + okul'
status: Done
assignee:
  - '@research-agent'
created_date: '2026-07-27 18:02'
updated_date: '2026-07-27 20:25'
labels:
  - research
  - competitor
  - gobline
  - gooseberry
  - nexis
  - okul
  - assessment
milestone: m-0
dependencies: []
documentation:
  - docs/competitors/
priority: low
type: spike
ordinal: 56000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
**Competitors (Knowledge Utilities):** gobline, gooseberry, nexis, okul
**Versions:** gobline 0.1.x, gooseberry 0.2.x, nexis 0.1.x, okul 0.4.x
**Language:** Rust
**Links:** https://crates.io/crates/gobline | https://crates.io/crates/gooseberry | https://crates.io/crates/nexis | https://crates.io/crates/okul

**What they do:**
**gobline** — Knowledge-based logic computation. Evaluates logical expressions against knowledge bases.
**gooseberry** — CLI tool that generates a knowledge base from Hypothesis web annotations (your bookmarked web highlights become structured markdown knowledge).
**nexis** — Analyzes wikilinks in markdown vaults (link graph analysis, orphan detection, backlink statistics).
**okul** — Spaced repetition system integrated with markdown knowledge files (generates review prompts from md content).

**Why assess:** These cover adjacent knowledge-management use cases that OKC might eventually want to support — logical inference, web annotation import, link analysis, and spaced repetition. Evaluating them helps decide what belongs in OKC vs what should be plugins/integrations.

**Assessment focus:**
1. Each tool functionality and quality assessment
2. Potential for integration with OKC
3. Feature gaps these fill that OKC should consider
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Functional assessment of each utility
- [ ] #2 Integration potential analysis with OKC
- [ ] #3 Roadmap recommendations (build, plugin, or ignore)
- [ ] #4 Code quality spot-check for each
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Assessment written to docs/competitors/knowledge-utilities-assessment.md
- [ ] #2 Recommendation for each: integrate, plugin, or out of scope
<!-- DOD:END -->
