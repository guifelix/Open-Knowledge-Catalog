---
id: OKC-00096
title: 'Competitor Assessment: fff (dmtrKovalenko/fff)'
status: Done
assignee: []
created_date: '2026-07-28 02:49'
updated_date: '2026-07-28 02:53'
labels:
  - competitor-assessment
  - file-search
  - mcp
  - rust
dependencies: []
references:
  - 'https://github.com/dmtrKovalenko/fff'
documentation:
  - docs/competitors/fff-assessment.md
priority: medium
type: spike
ordinal: 71000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Assess fff (dmtrKovalenko/fff) — an MIT-licensed Rust workspace (v0.10.1) providing frecency-ranked file search, multi-mode content grep (SIMD-accelerated), bigram-inverted path indexing, LMDB persistence, an MCP server (16 tools over stdio), and Neovim integration.

Where OKC is a structured knowledge catalog with typed OKF concepts, graph traversal, and agent-facing MCP tooling, fff is an unstructured file-system search engine optimised for developer workflow efficiency. This assessment evaluates its architecture (7 workspace members), frecency ranking pipeline, MCP surface, and competitive threat to OKC.

Key findings: fff is not a direct competitor to OKC (file finder vs knowledge catalog), but its MCP file-search/grep tools overlap with OKC's. fff's best-in-class bigram-indexed path search, frecency ranking, and 4-mode grep are worth adopting as patterns.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Assessment covers frecency ranking pipeline (exponential decay, git-boost, query-combo boost)
- [ ] #2 Assessment covers bigram-indexed path search with SIMD column merge
- [ ] #3 Assessment covers 4-mode grep engine (regex, aho-corasick, fuzzy, SIMD prefilter)
- [ ] #4 Assessment covers MCP surface (16 tools, cursor pagination, Perplexity-style output)
- [ ] #5 Assessment includes comparison matrix with OKC, strengths, weaknesses, threat level, and recommendations
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Completed fff (dmtrKovalenko/fff) competitor assessment. fff is an MIT-licensed Rust workspace (v0.10.1) providing frecency-ranked fuzzy file search, 4-mode SIMD grep, bigram-inverted path indexing, LMDB persistence, and an MCP server with 16 tools.

**Assessment verdict:** LOW threat — file-finder with MCP, not a knowledge catalog. Overlap only on file-search MCP tools. Best-in-class frecency ranking algorithm worth studying for OKC's own ranking. Key architectural patterns for OKC to adopt: exponential-decay frecency scoring with git-boost, bigram-inverted path index for fast prefix lookup, 4-mode grep engine (regex/aho-corasick/fuzzy/SIMD), and the MCP cursor-pagination pattern.

**Deliverables:**
- docs/competitors/fff-assessment.md — 9-section comprehensive assessment
- BACKLOG OKC-96 created
- COMPILATION.md updated: taxonomy count, Tier 2 entry, capability matrix (+fff column), LOW threat entry, appendix index
<!-- SECTION:FINAL_SUMMARY:END -->
