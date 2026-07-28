---
id: OKC-00064
title: 'Competitor Assessment: Enterprise Knowledge Ecosystem (gnosis, dkp)'
status: Done
assignee:
  - '@agent'
created_date: '2026-07-27 19:38'
updated_date: '2026-07-27 20:46'
labels:
  - competitor-assessment
  - knowledge-management
  - enterprise
  - rust
milestone: m-0
dependencies: []
references:
  - 'https://crates.io/crates/gnosis'
  - 'https://crates.io/crates/dkp'
  - 'https://crates.io/crates/dkp-core'
priority: medium
type: spike
ordinal: 61000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Assess enterprise-grade knowledge management tools on crates.io:

**Crates to assess:**
1. **gnosis** (thanos/gnosis) — Enterprise knowledge compiler. Compiles knowledge bases from distributed sources.
2. **dkp / dkp-core / dkp-gen-core** (dkp-standard) — Domain Knowledge Pack: structured knowledge packaging with formal ontology.

**Assessment dimensions:**
- Knowledge model: ontology, schemas, typing system
- Architecture: compiler vs runtime, packaging vs serving
- Enterprise readiness: scale, performance, governance
- MCP/agent integration: protocol support, API surface
- Community: maturity, adoption, maintenance
- Differentiation from OKC: what unique capabilities do they offer?
- Threat level to OKC project\
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Cover gnosis with architecture and feature analysis
- [ ] #2 Cover dkp ecosystem (core, gen-core) with architecture analysis
- [ ] #3 Compare knowledge models against OKF/OKC approach
- [ ] #4 Assess enterprise readiness and agent-readiness
- [ ] #5 Assign threat levels with supporting evidence
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Assessment written to docs/competitors/enterprise-knowledge-ecosystem-assessment.md
- [ ] #2 Each project has own section with architecture, features, threat level
- [ ] #3 Comparison matrix: knowledge model, enterprise features, agent-readiness
- [ ] #4 Strategic recommendations section
<!-- DOD:END -->
