---
id: OKC-00066
title: >-
  Competitor Assessment: Core OKF Rust implementations (W4G1, mikevalstar,
  kathrinmotzkus)
status: Done
assignee:
  - '@agent'
created_date: '2026-07-27 19:39'
updated_date: '2026-07-27 20:28'
labels:
  - competitor-assessment
  - okf
  - rust
  - ecosystem
milestone: m-0
dependencies: []
references:
  - 'https://crates.io/crates/okf'
  - 'https://crates.io/crates/okf-permissive'
  - 'https://crates.io/crates/okf-open-knowledge-format'
documentation:
  - docs/competitors/okf-ecosystem-assessment.md
priority: medium
type: spike
ordinal: 58000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Assess all pure-Rust OKF implementations on crates.io beyond what OKC-00058 already covers:

**Crates to assess:**
1. **okf** (W4G1/okf) — Pure-Rust OKF v0.2 implementation. Structural OKF parsing/serialization.
2. **okf-permissive** (mikevalstar/okf-permissive) — Permissive fork of okf v0.1. Looser validation.
3. **okf-open-knowledge-format** (kathrinmotzkus/open-knowledge-format) — Core document model and repository API (separate from the kathrinmotzkus ecosystem assessed in OKC-00058).

**Assessment dimensions:**
- Architecture & design philosophy (file-first vs storage-first)
- Feature comparison: querying, validation, MCP support, export formats
- Rust API quality: type safety, ergonomics, documentation
- Agent-readiness: MCP servers, CLI tools, library integration
- Community health: maintainer activity, adoption, test coverage
- Differentiation from OKC: what does each do better/worse?
- Threat level to OKC project\
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Cover all 3 Rust OKF implementation crates with architecture analysis
- [ ] #2 Compare each crate against OKC on 10+ feature dimensions
- [ ] #3 Assess Rust API quality and agent-readiness for each
- [ ] #4 Assign threat level with supporting evidence
- [ ] #5 Document any unique features OKC should consider adopting
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Assessment written to docs/competitors/okf-rust-implementations-assessment.md
- [ ] #2 Each crate has its own section with architecture, features, threat level
- [ ] #3 Comparison matrix with OKC across all crates
- [ ] #4 Strategic recommendations section
<!-- DOD:END -->
