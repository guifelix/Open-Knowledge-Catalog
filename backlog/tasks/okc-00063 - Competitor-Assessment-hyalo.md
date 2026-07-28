---
id: OKC-00063
title: 'Competitor Assessment: hyalo'
status: Done
assignee:
  - '@research-agent'
created_date: '2026-07-27 18:01'
updated_date: '2026-07-27 20:30'
labels:
  - research
  - competitor
  - hyalo
  - assessment
milestone: m-0
dependencies:
  - DRAFT-00005
  - DRAFT-00006
references:
  - 'https://github.com/ractive/hyalo'
  - 'https://crates.io/crates/hyalo'
documentation:
  - docs/competitors/
priority: high
type: spike
ordinal: 47000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
**Competitor:** hyalo (ractive/hyalo)
**Version:** 0.20.0 | **Language:** Rust
**Links:** https://crates.io/crates/hyalo | https://github.com/ractive/hyalo

**What it does:**
CLI for searching, filtering, bulk-editing, and reorganizing markdown knowledgebases with YAML frontmatter. Full-text BM25 search, regex, frontmatter filters, tag/section/task/symbol queries. Rename with auto link rewrite across vault. Linting for schema consistency (required/optional fields). Designed specifically for both humans and AI agent tooling.

**Why assess:** Closest pure-CLI competitor. Rich search and linting capabilities. Designed for AI agent consumption. hyalo serves as a standalone search/mutate layer for md vaults — overlaps with OKC's CLI search + parser + index operations.

**Assessment focus:**
1. Search capability comparison (BM25, regex, frontmatter filters)
2. Linting/schema validation features
3. Bulk edit / rename with link rewriting
4. CLI ergonomics and DX
5. Agent tooling integration patterns
6. Code quality and architecture
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Search feature comparison matrix (BM25, regex, filtering, ranking)
- [ ] #2 Linting and schema validation capability comparison
- [ ] #3 CLI ergonomics evaluation vs OKC CLI
- [ ] #4 Bulk operations (rename, rewrite links) comparison
- [ ] #5 Code architecture review
- [ ] #6 List of OKC improvement opportunities from hyalo strengths
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Assessment written to docs/competitors/hyalo-assessment.md
- [ ] #2 All ACs answered with evidence
- [ ] #3 Prioritized action items generated
<!-- DOD:END -->
