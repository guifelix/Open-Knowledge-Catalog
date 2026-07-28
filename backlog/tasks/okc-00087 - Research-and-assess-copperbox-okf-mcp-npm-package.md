---
id: OKC-00087
title: Research and assess @copperbox/okf-mcp npm package
status: Done
assignee:
  - '@research-agent'
created_date: '2026-07-27 20:46'
updated_date: '2026-07-27 20:46'
labels:
  - competitor
  - okf
  - npm
  - research
  - spike
dependencies: []
references:
  - 'https://www.npmjs.com/package/@copperbox/okf-mcp'
  - 'https://github.com/copperbox/okf-mcp'
  - docs/competitors/tribal-relay-knowledge-assessment.md
documentation:
  - docs/competitors/copperbox-okf-mcp-assessment.md
priority: medium
type: spike
ordinal: 62000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Research the @copperbox/okf-mcp npm package and create a competitor assessment.

**Research tasks:**
1. Fetch package metadata from npm (version, downloads, description, license, repository)
2. Visit GitHub repo (https://github.com/copperbox/okf-mcp) for architecture, features, MCP tools
3. Determine OKF version support, language (TypeScript), MCP transport support
4. Assess code quality: stars, commits, CI, tests, documentation
5. Compare with OKC: MCP tools, search, graph traversal, file watching, remote bundles

**Deliverable:** Create assessment file at docs/competitors/copperbox-okf-mcp-assessment.md (or update existing)
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Package metadata fetched from npm (version, downloads, license, repo URL)
- [ ] #2 GitHub repo analyzed for architecture, MCP tools, OKF version support
- [ ] #3 Code quality assessed (stars, commits, CI, tests, docs)
- [ ] #4 Feature comparison table vs OKC created with ✅/❌/⚠️ markers
- [ ] #5 Assessment file created/updated at docs/competitors/copperbox-okf-mcp-assessment.md
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Assessment follows reference format (tribal-relay-knowledge-assessment.md)
- [ ] #2 All sections present: Overview, Feature Comparison, Architecture, MCP Inventory, Strengths, Weaknesses, Opportunities, Threat Level, Verdict
- [ ] #3 Threat level explicitly stated with rationale
<!-- DOD:END -->
