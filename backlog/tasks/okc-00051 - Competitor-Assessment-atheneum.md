---
id: OKC-00051
title: 'Competitor Assessment: atheneum'
status: Done
assignee:
  - '@research-agent'
created_date: '2026-07-27 18:02'
updated_date: '2026-07-27 20:30'
labels:
  - research
  - competitor
  - atheneum
  - assessment
milestone: m-0
dependencies: []
references:
  - 'https://crates.io/crates/atheneum'
  - 'https://github.com/chriskriley/atheneum'
documentation:
  - docs/competitors/
priority: medium
type: spike
ordinal: 48000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
**Competitor:** atheneum
**Language:** Rust
**Links:** https://crates.io/crates/atheneum

**What it does:**
Agent coordination graph database. Manages both episodic memory (task histories, tool calls, LLM interactions) and semantic memory (extracted knowledge) for multi-agent systems. Supports vector similarity search, temporal queries, relationship graphs. Designed for agents to persist and share context.

**Why assess:** Addresses the same "agent knowledge management" space but focused on internal agent memory (episodic + semantic) rather than user-facing knowledge files. Important to understand the adjacent space.

**Assessment focus:**
1. Episodic vs semantic memory architecture
2. Graph database capabilities (node types, relationships, queries)
3. Vector search integration
4. Multi-agent coordination patterns
5. Differentiation: agent memory vs knowledge base vs OKC
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Memory architecture comparison (episodic + semantic vs OKC knowledge graph)
- [ ] #2 Graph database capabilities comparison
- [ ] #3 Agent coordination patterns assessment
- [ ] #4 Boundary analysis: where atheneum ends and OKC begins
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Assessment written to docs/competitors/atheneum-assessment.md
- [ ] #2 Clear positioning guidance: complementary vs overlapping
<!-- DOD:END -->
