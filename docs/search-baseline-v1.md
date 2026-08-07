---
type: Documentation
title: Lexical Search Baseline v1
description: Reproducible relevance and latency baseline for the production FTS5 search path
tags: [search, evaluation, bm25, quality]
owner: Engineering Team
status: published
---

# Lexical Search Baseline v1

Recorded on 2026-08-06 against the 11-document
`tests/fixtures/simple` repository. The versioned judgments and proposal gate are
in `tests/fixtures/search-eval-v1.json`; the executable evaluator is
`tests/search_evaluation.rs`.

Reproduce the baseline with:

```bash
cargo test --test search_evaluation production_lexical_search_baseline_v1 -- --exact --nocapture
```

## Predeclared proposal gate

An advanced retrieval proposal must satisfy all of these conditions against the
same corpus before it replaces or augments production search:

- improve absolute Recall@5 by at least 0.10;
- improve absolute MRR@10 by at least 0.05;
- add no more than 25% to p95 query latency;
- introduce no recall regression on exact-query cases.

The corpus must be expanded before a production vector decision. These initial
11 documents and 10 queries are a regression baseline, not a claim of general
search quality.

## Results

Nine queries require evidence; one query intentionally expects no result.
Metrics are macro-averaged across the nine judged queries.

| Metric | Original baseline | Bounded typo fallback |
|---|---:|---:|
| Recall@5 | 0.6667 | 0.7778 |
| Recall@10 | 0.6667 | 0.7778 |
| MRR@10 | 0.6667 | 0.7222 |
| Zero-required-evidence rate | 0.3333 | 0.2222 |
| Intentional zero-hit accuracy | 1.0000 | 1.0000 |
| Warm p50 latency | 224 µs | 291 µs |
| Warm p95 latency | 432 µs | 488 µs |

Latency used 25 in-process samples per query after scan in the Rust test profile.
It is useful for relative regression checks on the same machine, not as a
cross-machine service-level objective.

The regression test's absolute p95 cap is 900 µs (`MAX_P95_LATENCY_MICROS` in
`tests/search_evaluation.rs`). The earlier 540 µs cap (432 µs baseline + 25%)
was too tight on shared CI runners, which routinely show 550–600 µs p95 spikes.
The 900 µs cap preserves a genuine order-of-magnitude regression guard while
absorbing runner variance. The proportional proposal gate (≤25% in
`proposal_gate.maximum_p95_latency_regression_percent`) is unchanged and
applies to comparisons between retrieval proposals measured in one run.

| Case | Outcome | Classification |
|---|---|---|
| Exact monthly recurring revenue | 2/2 relevant documents in top 5 | Pass |
| Exact customer orders | Relevant dataset ranked first | Pass |
| Customer-cancellation paraphrase | Churn metric ranked first | Pass |
| Subscription-income-rules paraphrase | No result | Semantic recall |
| Intentional unrelated query | No result | Pass |
| Monthly-revenue source dataset | Required dataset absent | Graph expansion |
| Filtered customer metrics | 2/2 relevant metrics in top 5 | Pass |
| Misspelled monthly revenue | Relevant metric retrieved in top 5 | Pass |
| Porter subscription normalization | 2/2 relevant documents in top 5 | Pass |
| Revenue ranking | 3/3 relevant documents in top 5 | Pass |

Observed failure counts by the fixed taxonomy:

| Failure class | Count |
|---|---:|
| Lexical normalization | 0 |
| Typo/fuzzy matching | 0 |
| Ranking | 0 |
| Semantic recall | 1 |
| Graph expansion | 1 |
| Filter interaction | 0 |

## Production path audit and resolution

The baseline originally found that CLI, service, and MCP called
`index::queries::search::search`, while `SqliteSearchIndex::search` contained a
second query implementation with different weighting, filtering, and count
behavior.

OKC-00111 resolved that prerequisite. The canonical path is now:

```text
OkcService::search
  -> RepositoryIndex::search
  -> SearchIndex::search
  -> SqliteSearchIndex::search
  -> SQLite document_search MATCH
```

`RepositoryIndex::search` only translates public filter arguments into
`SearchFilters`; all ranking, filtering, counting, ordering, and excerpt behavior
lives in `SqliteSearchIndex::search`. The former `index::queries::search` module
was removed. Production now applies configured BM25 field weights, uses
deterministic path ordering for score ties, and computes counts independently of
the requested page limit.

## Bounded typo fallback

OKC-00113 adds one correction attempt only when the primary filtered FTS query
has zero matches. Plain ASCII queries are eligible when they contain one to
eight tokens and every token has at least five characters. Candidate vocabulary
terms must start with the same character and have edit distance at most one for
five-character tokens or two for longer tokens. The vocabulary snapshot is
capped at 50,000 terms, cached after indexing, and invalidated on every search
index mutation. The corrected query reuses the same filters, ranking, limit,
count, and deterministic ordering code. Successful primary queries never enter
the fallback, and at most one corrected FTS query is executed.

## Recommendation

The bounded lexical typo fallback passes the predeclared relevance and latency
gate. The next evidence-backed retrieval experiment is graph-assisted expansion
as a distinct stage. Exact, normalization, filtering, ranking, typo, and
intentional-zero-hit cases now pass; the remaining failures are semantic recall
and graph expansion.

Do not add embeddings yet. The one semantic zero-candidate failure is evidence
that FTS-candidate reranking alone cannot solve every case, but this corpus is too
small to justify vector storage and operational complexity. If an expanded
corpus continues to show semantic zero-candidate failures and an experiment
passes the proposal gate, evaluate an independent vector-retrieval branch fused
with FTS5; reranking only existing FTS candidates would not address that failure
mode.
