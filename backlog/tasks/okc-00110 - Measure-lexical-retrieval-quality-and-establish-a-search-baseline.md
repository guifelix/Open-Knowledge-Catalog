---
id: OKC-00110
title: Measure lexical retrieval quality and establish a search baseline
status: Done
assignee:
  - '@codex'
created_date: '2026-08-06 19:55'
updated_date: '2026-08-06 23:03'
labels:
  - search
  - evaluation
  - bm25
  - architecture
  - quality
dependencies: []
references:
  - docs/roadmap.md
  - src/index/queries/search.rs
  - src/index/search_index.rs
documentation:
  - docs/search-baseline-v1.md
priority: high
type: spike
ordinal: 77000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Establish a reproducible relevance corpus and BM25 baseline before committing to embeddings, vector storage, fusion, reranking, or HyDE. The results must classify observed retrieval failures and determine whether lexical improvements, embedding reranking, or an independent vector-retrieval branch is justified.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 A versioned representative query corpus records expected relevant documents and includes exact, paraphrase, zero-hit, graph-assisted, and metadata-filtered cases
- [x] #2 The current production search path is measured for Recall@5 and Recall@10, MRR or nDCG@10, zero-required-evidence rate, and p50/p95 latency
- [x] #3 Results classify failures into lexical normalization, typo/fuzzy matching, ranking, semantic recall, graph expansion, and filter interaction categories
- [x] #4 The spike defines a predeclared improvement threshold and non-regression budget for any advanced retrieval proposal
- [x] #5 The conclusion recommends lexical enhancement, FTS-candidate reranking, independent vector retrieval, or no change based on measured evidence
- [x] #6 The evaluation identifies the canonical production query path and any duplicate search implementation that must be consolidated before advanced retrieval work
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add a versioned evaluation corpus over the representative fixture repository, covering exact, paraphrase, intentional zero-hit, graph-assisted, metadata-filtered, typo, normalization, and ranking cases with predeclared thresholds.
2. Build a deterministic evaluation harness against OkcService::search that reports Recall@5/10, MRR@10, zero-required-evidence rate, and p50/p95 latency, with metric unit tests.
3. Run the production path repeatedly, record results, and classify observed failures by the required taxonomy.
4. Document the canonical and duplicate search paths, measured recommendation, thresholds, and reproducible command; then run the full Rust quality gate and finalize the spike.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added versioned search-eval-v1 corpus and executable production-service evaluator. Baseline: Recall@5=0.6667, Recall@10=0.6667, MRR@10=0.6667, zero-required-evidence=0.3333, intentional-zero-hit accuracy=1.0; representative warm p50/p95=224/432 microseconds. Observed failures: typo/fuzzy=1, semantic recall=1, graph expansion=1; other required classes=0. Production audit found OkcService uses index::queries::search while configurable SqliteSearchIndex::search is a divergent unused duplicate.

Final validation: versioned evaluator rerun with stable relevance metrics and explicit taxonomy counts; cargo fmt --check; cargo test (278 unit/integration/property/evaluation tests plus 2 doc tests passed); cargo clippy -- -D warnings; git diff --check.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Established lexical search baseline v1 with a versioned judged corpus, executable production-path evaluator, relevance and latency metrics, fixed failure taxonomy, and predeclared proposal gate. Measured evidence recommends consolidating duplicate search paths and improving lexical typo handling plus graph expansion before considering embeddings; a future semantic experiment would require an independent vector branch, not FTS-only reranking.
<!-- SECTION:FINAL_SUMMARY:END -->
