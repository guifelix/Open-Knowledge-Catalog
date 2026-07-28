---
id: OKC-00054
title: 'Competitor Assessment: memcrate + ai-memory'
status: Done
assignee:
  - '@research-agent'
created_date: '2026-07-27 18:02'
updated_date: '2026-07-27 20:30'
labels:
  - research
  - competitor
  - memcrate
  - ai-memory
  - assessment
milestone: m-0
dependencies: []
references:
  - 'https://crates.io/crates/memcrate'
  - 'https://docs.rs/memcrate'
documentation:
  - docs/competitors/
priority: medium
type: spike
ordinal: 51000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
**Competitors:** memcrate, ai-memory
**Versions:** memcrate 0.3.3, ai-memory 0.1.x
**Language:** Rust
**Links:** https://crates.io/crates/memcrate | https://crates.io/crates/ai-memory

**What they do:**
**memcrate** — Personal context vault for AI tools. Markdown-native, three verbs: /save (snapshot context), /pin (persist important info), /load (restore for new session). Organizes as profiles/projects/state/sessions.
**ai-memory** — Persistent wiki for AI agent coding CLIs. Agents read/write structured wiki pages. Focuses on maintaining cross-session context for long-running coding projects.

**Why assess:** Both target the "AI agent persistent context" use case — memcrate as a personal vault for CLI tools, ai-memory as a wiki for agent sessions. Adjacent to OKC but focus on session-to-session context rather than knowledge base management. Important to understand the boundary.

**Assessment focus:**
1. Context persistence model (how data flows between sessions)
2. Storage format (markdown-native? structured? embedded DB?)
3. CLI/API design patterns
4. Integration with AI agent tooling
5. Boundary analysis: context vault vs knowledge base vs OKC
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Context persistence model comparison
- [ ] #2 Storage format and data model review
- [ ] #3 CLI design and DX comparison vs OKC
- [ ] #4 Boundary analysis: where context memory ends and OKC knowledge base begins
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Assessment written to docs/competitors/memcrate-ai-memory-assessment.md
- [ ] #2 Clear positioning guidance
<!-- DOD:END -->
