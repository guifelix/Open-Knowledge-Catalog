---
id: OKC-00067
title: >-
  Competitor Assessment: OKF CLI & Developer Tooling (okf-http, okf-cli,
  okf-lint, okq, okftool, scrinium)
status: Done
assignee:
  - '@agent'
created_date: '2026-07-27 19:39'
updated_date: '2026-07-27 20:28'
labels:
  - competitor-assessment
  - okf
  - cli
  - tooling
  - rust
milestone: m-0
dependencies: []
references:
  - 'https://crates.io/crates/okf-http'
  - 'https://crates.io/crates/okf-cli'
  - 'https://crates.io/crates/okf-lint'
  - 'https://crates.io/crates/okq'
  - 'https://crates.io/crates/okftool-core'
  - 'https://crates.io/crates/scrinium'
priority: medium
type: spike
ordinal: 57000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Assess all OKF-related CLI tools and developer utilities on crates.io:

**Crates to assess:**
1. **okf-http** (kathrinmotzkus/open-knowledge-format) — HTTP server for OKF repositories
2. **okf-cli** (raimannma12/okf-cli) — Cross-platform CLI tool for OKF bundles
3. **okf-lint** (rpmoore/okf-lint) — CLI linter for OKF bundles, validation rules
4. **okq** (mikevalstar/okq) — Fast CLI for searching/navigating OKF bundles
5. **okftool-core / okftool-cli** (ryansann/okftool) — OKF parser, validator, lint engine
6. **scrinium / scrinium-core** (UtakataKyosui/scrinium) — OKF CLI with TUI editor

**Assessment dimensions:**
- CLI ergonomics: subcommands, flags, output format
- Feature depth: validation, linting, search, HTTP serving
- MCP server support (if any)
- Agent-readiness: machine-parseable output, pipeability
- Quality: documentation, error messages, tests
- Community: maintenance activity, release cadence
- Comparison with OKC CLI feature set
- Threat level to OKC project\
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Cover all 8 OKF CLI/tooling crates with individual analysis
- [ ] #2 Compare CLI ergonomics and feature depth against OKC
- [ ] #3 Assess agent-readiness (JSON output, pipeability, MCP) for each
- [ ] #4 Identify any features OKC should adopt
- [ ] #5 Assign threat level per crate with evidence
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Assessment written to docs/competitors/okf-cli-tooling-assessment.md
- [ ] #2 Each crate has own section with analysis and CLI examples
- [ ] #3 Comparison matrix: CLI features, agent-readiness, community health
- [ ] #4 Strategic recommendations and threat assessment
<!-- DOD:END -->
