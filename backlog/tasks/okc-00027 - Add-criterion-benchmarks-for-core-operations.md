---
id: OKC-00027
title: Add criterion benchmarks for core operations
status: Done
assignee:
  - '@felix'
created_date: '2026-07-23 19:04'
updated_date: '2026-07-24 00:22'
labels:
  - quality
dependencies: []
priority: high
type: enhancement
ordinal: 20000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add criterion benchmarks to measure and track performance of all core operations. Prevent regressions.\n\nBenchmark targets:\n- scan_bundle(bundle_size = 1/10/100 docs)\n- search_query(n_terms = 1/3/10, n_results = 10/100)\n- get_concept_graph(depth = 1/3/5)\n- validate_bundle(bundle_size = 1/10/100 docs)\n- export_bundle_json(bundle_size = same range)\n\nStore baseline results in .criterion/ and compare in CI.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
test parser::frontmatter::tests::test_basic_frontmatter ... ignored
test parser::frontmatter::tests::test_bom_handling ... ignored
test parser::frontmatter::tests::test_exceeds_max_size ... ignored
test parser::frontmatter::tests::test_missing_closing ... ignored
test parser::frontmatter::tests::test_no_frontmatter ... ignored
test parser::frontmatter::tests::test_windows_line_endings ... ignored
test parser::links::tests::test_check_exists_positive ... ignored
test parser::links::tests::test_external_url_left_unchanged ... ignored
test parser::links::tests::test_resolve_parent_dir ... ignored
test parser::links::tests::test_resolve_relative_same_dir ... ignored

test result: ok. 0 passed; 0 failed; 10 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 10 tests
test parser::frontmatter::tests::test_basic_frontmatter ... ignored
test parser::frontmatter::tests::test_bom_handling ... ignored
test parser::frontmatter::tests::test_exceeds_max_size ... ignored
test parser::frontmatter::tests::test_missing_closing ... ignored
test parser::frontmatter::tests::test_no_frontmatter ... ignored
test parser::frontmatter::tests::test_windows_line_endings ... ignored
test parser::links::tests::test_check_exists_positive ... ignored
test parser::links::tests::test_external_url_left_unchanged ... ignored
test parser::links::tests::test_resolve_parent_dir ... ignored
test parser::links::tests::test_resolve_relative_same_dir ... ignored

test result: ok. 0 passed; 0 failed; 10 ignored; 0 measured; 0 filtered out; finished in 0.00s succeeds with no errors

- [ ] #1 criterion benchmarks exist for: scan, search, get_concept_graph, validate, export
- [ ] #2 Benchmarks run different input sizes (1/10/100 docs) to measure scaling behavior
- [ ] #3 cargo bench succeeds with no errors
- [ ] #4 CI gate: PRs that regress >20% on any benchmark trigger a warning comment
- [ ] #5 Baseline results checked into .criterion-baseline/ (git-tracked)
<!-- AC:END -->
