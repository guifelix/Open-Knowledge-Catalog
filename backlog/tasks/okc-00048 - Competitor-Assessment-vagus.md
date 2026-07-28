---
id: OKC-00048
title: 'Competitor Assessment: vagus'
status: Done
assignee:
  - '@research-agent'
created_date: '2026-07-27 18:01'
updated_date: '2026-07-27 20:30'
labels:
  - research
  - competitor
  - vagus
  - assessment
  - search
milestone: m-0
dependencies:
  - OKC-00013
references:
  - 'https://crates.io/crates/vagus'
  - 'https://github.com/vasovagal/vagus'
documentation:
  - docs/competitors/
priority: high
type: spike
ordinal: 22500
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
**Competitor:** vagus (vasovagal/vagus)
**Language:** Rust
**Links:** https://github.com/vasovagal/vagus

**What it does:**
Hybrid search engine for PARA-style markdown knowledge management. Uses Tantivy for BM25 + local ONNX embeddings with Reciprocal Rank Fusion. Optional reranking with cross-encoder, query expansion/HyDE. Obsidian-compatible. Has Claude Code skill integration.

**Why assess:** vagus represents the hybrid search gold standard (BM25 + embeddings + RRF + reranking). If OKC doesn't have embeddings, vagus shows what the state of the art looks like. Understanding vagus's architecture is critical for OKC's search roadmap (OKC-00013).

**Assessment focus:**
1. Hybrid search pipeline architecture (Tantivy BM25 + ONNX embeddings + RRF)
2. Embedding model selection and performance
3. Reranking pipeline (cross-encoder)
4. Query expansion / HyDE implementation
5. PARA organization patterns
6. Claude Code skill integration
7. Architecture and code quality
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Hybrid search pipeline comparison (BM25, embeddings, RRF fusion strategy)
- [ ] #2 Embedding model choices and performance characteristics
- [ ] #3 Reranking and query expansion capabilities
- [ ] #4 PARA organization patterns — applicability to OKC
- [ ] #5 Code architecture and integration patterns
- [ ] #6 Concrete recommendations for OKC-00013 (semantic embeddings)
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Assessment written to docs/competitors/vagus-assessment.md
- [ ] #2 All ACs answered with evidence
- [ ] #3 Architecture diagram of vagus search pipeline
- [ ] #4 Prioritized recommendations for OKC hybrid search implementation
<!-- DOD:END -->
