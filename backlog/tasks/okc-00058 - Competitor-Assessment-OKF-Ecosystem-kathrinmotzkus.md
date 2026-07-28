---
id: OKC-00058
title: 'Competitor Assessment: OKF Ecosystem (kathrinmotzkus)'
status: Done
assignee:
  - '@research-agent'
created_date: '2026-07-27 18:01'
updated_date: '2026-07-27 20:25'
labels:
  - research
  - competitor
  - okf
  - assessment
milestone: m-0
dependencies: []
documentation:
  - docs/competitors/
priority: medium
type: spike
ordinal: 53000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
**Competitor:** OKF Ecosystem by kathrinmotzkus
**Crates:** okf-open-knowledge-format (0.4.0), okf (0.2.1), okf-http (0.4.4), okf-cli (0.1.0), okf-lint (0.1.1), okq (0.5.2)
**Language:** Rust
**Links:** https://crates.io/users/kathrinmotzkus

**What they do:**
Ecosystem of Rust crates implementing the Open Knowledge Format spec. Core library (okf-open-knowledge-format) provides discovery, frontmatter parsing, relations. Separate crates for HTTP server, CLI, linter, and a dedicated query CLI (okq).

**Why assess:** These implement the same OKF specification that OKC is built on. Need to understand what their implementation covers, where it falls short, and what gaps OKC fills. Also need to evaluate potential for collaboration, integration, or differentiation.

**Assessment focus:**
1. OKF spec coverage comparison (what OKF features each implements)
2. Library API quality and ergonomics
3. HTTP server capabilities (okf-http)
4. Query capabilities (okq — deterministic search for humans + agents)
5. Community traction (downloads, contributors, maintenance)
6. Gaps that OKC fills or should fill
7. Opportunities for interop or collaboration
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 OKF spec feature coverage matrix (okf vs okf-open-knowledge-format vs OKC)
- [ ] #2 Library API quality and documentation review
- [ ] #3 HTTP server and query CLI capability comparison
- [ ] #4 Community health assessment (maintenance, downloads, contributors)
- [ ] #5 Gap analysis: what OKC has that OKF ecosystem doesn\'t
- [ ] #6 Opportunities for interop / co-marketing / spec contributions
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Assessment written to docs/competitors/okf-ecosystem-assessment.md
- [ ] #2 All ACs answered with evidence
- [ ] #3 Strategic recommendations (compete, collaborate, or differentiate)
<!-- DOD:END -->
