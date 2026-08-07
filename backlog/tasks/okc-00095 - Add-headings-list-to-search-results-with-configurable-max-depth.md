---
id: OKC-00095
title: Add headings list to search results with configurable max/depth
status: Done
assignee:
  - '@backend-agent'
created_date: '2026-07-28 02:32'
updated_date: '2026-08-07 22:58'
labels:
  - backend
  - search
  - feature
  - headings
  - config
dependencies:
  - OKC-00042
references:
  - docs/references/okf-spec.md
documentation:
  - docs/architecture/data-flow.md
priority: high
type: feature
ordinal: 70000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Search results currently return `excerpt` from body text but no structured headings list. Headings provide rich, structured context — showing the document outline at a glance.

**No new MCP tool.** The existing `search` tool gains two optional params (`max_headings`, `heading_depth`) and returns `headings: Vec<String>` per result. An internal helper function queries the already-parsed `headings` table — not a new tool.

Both follow the same fallback chain:
  1. Per-request MCP param (if passed)
  2. TOML `[search]` section (if configured)
  3. Hard default = 1

**Budget interaction:** `max_headings` only counts headings at the allowed depth levels. `heading_depth=2, max_headings=3` → up to 3 headings from h1+h2 combined.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Only the existing `search` MCP tool is affected — no new MCP tool is added
- [ ] #2 Internal helper get_document_headings() returns empty vec when body is empty
- [ ] #3 Internal helper get_document_headings() returns empty vec when heading_depth=0
- [ ] #4 Internal helper get_document_headings() filters by heading depth (depth=2 → h1+h2 only)
- [ ] #5 Internal helper get_document_headings() respects max_headings cap (at most N)
- [ ] #6 Internal helper get_document_headings() with depth=1 returns only h1 headings
- [ ] #7 SearchResult includes headings: Vec<String> field (never null, empty vec when none found)
- [ ] #8 MCP SearchParams adds optional max_headings: Option<usize> and heading_depth: Option<u32>
- [ ] #9 MCP SearchResultOutput includes headings: Vec<String>
- [ ] #10 TOML config accepts [search] section with max_headings and heading_depth
- [ ] #11 Default OkcConfig has search.max_headings = 1 and search.heading_depth = 1
- [ ] #12 Config validation rejects heading_depth = 0 and max_headings = 0
- [ ] #13 Per-request param overrides config default; config overrides hard default
- [ ] #14 Both search paths (FTS5 + SQLite queries) populate the headings field
- [ ] #15 cargo fmt --check, cargo clippy -- -D warnings, cargo test all pass
- [ ] #16 Internal helper get_document_headings() ignores headings inside code blocks
- [ ] #17 max_headings only counts headings within the allowed heading_depth levels (depth=2, max=3 → up to 3 from h1+h2)
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Config unit tests: heading_depth=0 rejected, max_headings=0 rejected, env var override
- [ ] #2 Integration test: search tool returns headings in JSON response
- [ ] #3 cargo fmt --check, cargo clippy -- -D warnings, cargo test clean
- [ ] #4 Unit tests for internal helper: depth filtering, max cap, empty body, no headings, code block exclusion
- [ ] #5 Unit tests for budget interaction: max_headings only counts headings within allowed depth
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
## Implementation Plan for OKC-00095

### Phase 1: Config Layer
1. Add  struct in  with  (default 1) and  (default 1)
2. Add  section to  
3. Implement validation in  to reject  and 
4. Update  to handle env var overrides (OKC_SEARCH_MAX_HEADINGS, OKC_SEARCH_HEADING_DEPTH)
5. Add unit tests in  for validation and defaults

### Phase 2: Heading Extraction - Code Block Awareness
6. Modify  to track code block state:
   - Track  boolean when encountering  and 
   - Only push headings to result when 
7. Add unit test for code block heading exclusion

### Phase 3: Internal Helper
8. Create  in  or new module:
   - Query  table for document by path (join documents -> document_id -> headings)
   - Filter by 
   - Order by 
   - Take up to 
   - Return  of titles
   - Handle edge cases: empty body → empty vec, heading_depth=0 → empty vec

### Phase 4: Model Types
9. Add  to  in  (never null, empty vec default)
10. Add  to  in 
11. Add  and  to  in 

### Phase 5: Service Layer
12. Update  in  to accept  and  parameters
13. Apply fallback chain: per-request params > config defaults > hard defaults (1, 1)
14. For each search result, call  and populate  field
15. Update  in  to pass new params through

### Phase 6: MCP Transport Layer
16. Update  tool handler in  to extract  and  from params and pass to service

### Phase 7: Tests
17. Unit tests for : depth filtering, max cap, empty body, no headings, code block exclusion
18. Integration test: search tool returns headings in JSON response
19. Config unit tests: heading_depth=0 rejected, max_headings=0 rejected, env var override

### Phase 8: Verification
20. Run , , 
running 118 tests
test config::tests::test_bm25_config_default ... ok
test config::tests::test_env_overrides_bm25_body_weight ... ok
test config::tests::test_env_overrides_bm25_k1 ... ok
test config::tests::test_env_overrides_bm25_concept_type_weight ... ok
test config::tests::test_env_overrides_bm25_description_weight ... ok
test config::tests::test_env_overrides_db_path ... ok
test config::tests::test_env_overrides_bm25_b ... ok
test config::tests::test_create_default_config_file ... ok
test config::tests::test_env_overrides_watcher_debounce_ms ... ok
test config::tests::test_env_overrides_bm25_title_weight ... ok
test config::tests::test_env_overrides_watcher_reconcile_secs ... ok
test config::tests::test_env_overrides_follow_symlinks ... ok
test config::tests::test_env_overrides_exclude_patterns ... ok
test config::tests::test_env_overrides_max_file_size ... ok
test config::tests::test_invalid_env_max_front_matter_size ... ok
test config::tests::test_env_overrides_max_front_matter_size ... ok
test config::tests::test_env_overrides_max_graph_depth ... ok
test config::tests::test_env_overrides_max_graph_nodes ... ok
test config::tests::test_env_overrides_max_response_chars ... ok
test config::tests::test_invalid_env_max_yaml_input_size ... ok
test config::tests::test_env_overrides_max_yaml_input_size ... ok
test config::tests::test_load_config_file_not_found ... ok
test config::tests::test_env_overrides_require_index_files ... ok
test config::tests::test_okc_config_default ... ok
test index::content_hash::tests::test_deterministic_hash ... ok
test index::content_hash::tests::test_different_content_different_hash ... ok
test index::content_hash::tests::test_full_hash_small_content ... ok
test index::content_hash::tests::test_hash_config_custom ... ok
test index::queries::metadata::tests::escapes_path_prefix_like_metacharacters ... ok
test index::content_hash::tests::test_truncate_preserves_structure ... ok
test index::search_index::tests::correction_is_bounded_and_deterministic ... ok
test index::content_hash::tests::test_truncate_short_content_unchanged ... ok
test index::search_index::tests::edit_distance_handles_insertions_deletions_and_substitutions ... ok
test index::queries::metadata::tests::validates_filter_names_and_operators ... ok
test parser::frontmatter::tests::test_basic_frontmatter ... ok
test config::tests::test_load_config_from_file ... ok
test config::tests::test_env_overrides_bm25_headings_weight ... ok
test config::tests::test_invalid_env_bm25_title_weight ... ok
test config::tests::test_invalid_env_follow_symlinks ... ok
test parser::frontmatter::tests::test_missing_closing ... ok
test config::tests::test_invalid_env_max_file_size ... ok
test parser::frontmatter::tests::test_no_frontmatter ... ok
test parser::frontmatter::tests::test_exceeds_max_size ... ok
test config::tests::test_invalid_env_max_graph_depth ... ok
test parser::frontmatter::tests::test_windows_line_endings ... ok
test config::tests::test_invalid_env_max_graph_nodes ... ok
test config::tests::test_invalid_env_max_response_chars ... ok
test config::tests::test_env_overrides_roots ... ok
test parser::frontmatter::tests::test_bom_handling ... ok
test parser::links::tests::test_external_url_left_unchanged ... ok
test parser::links::tests::test_anchor_extracted_and_stored_separately ... ok
test parser::links::tests::test_external_url_with_anchor ... ok
test parser::links::tests::test_external_url_with_anchor_preserved ... ok
test parser::links::tests::test_broken_link_warning_and_marked ... ok
test parser::links::tests::test_case_insensitive_check_exists_macos ... ok
test parser::links::tests::test_no_anchor_returns_none ... ok
test parser::links::tests::test_case_insensitive_check_exists_windows ... ok
test parser::links::tests::test_normalize_path_traversal_returns_none ... ok
test parser::links::tests::test_normalize_path_valid_returns_some ... ok
test parser::links::tests::test_case_sensitive_check_exists_linux ... ok
test parser::links::tests::test_check_exists_positive ... ok
test parser::links::tests::test_cycle_detection ... ok
test parser::links::tests::test_path_traversal_blocked ... ok
test parser::links::tests::test_percent_encoded_path_decoded ... ok
test parser::links::tests::test_percent_encoded_anchor_decoded ... ok
test parser::links::tests::test_anchor_with_special_chars ... ok
test parser::links::tests::test_relative_markdown_link_parent_dir ... ok
test parser::links::tests::test_filter_self_references ... ok
test parser::links::tests::test_relative_markdown_link_same_dir ... ok
test parser::links::tests::test_resolve_relative_same_dir ... ok
test parser::links::tests::test_percent_encoded_slash_not_decoded_in_path ... ok
test parser::links::tests::test_path_traversal_blocked_from_root ... ok
test parser::links::tests::test_resolve_with_anchor_preserved_in_target ... ok
test parser::links::tests::test_round_trip_resolve_check_exists ... ok
test parser::links::tests::test_repository_root_relative_path ... ok
test parser::links::tests::test_split_anchor ... ok
test parser::links::tests::test_round_trip_with_anchor ... ok
test parser::links::tests::test_percent_encoded_special_chars_decoded ... ok
test parser::links::tests::test_resolve_parent_dir ... ok
test parser::links::tests::test_wiki_link_extraction_basic ... ok
test parser::links::tests::test_wiki_link_nested_brackets ... ok
test parser::links::tests::test_wiki_link_relative_path ... ok
test parser::links::tests::test_wiki_link_root_relative ... ok
test parser::links::tests::test_wiki_link_with_anchor ... ok
test parser::links::tests::test_wiki_link_with_anchor_and_display ... ok
test parser::links::tests::test_wiki_link_with_display_text ... ok
test parser::markdown::code_blocks::tests::test_extract_code_block_basic ... ok
test parser::markdown::code_blocks::tests::test_extract_code_block_with_filename ... ok
test scanner::watcher::tests::test_extract_paths_create ... ok
test parser::markdown::sections::tests::test_build_sections_basic ... ok
test parser::markdown::links::tests::test_extract_wiki_links ... ok
test parser::markdown::code_blocks::tests::test_extract_code_block_no_language ... ok
test scanner::watcher::tests::test_is_ignored_emacs_lock ... ok
test parser::markdown::headings::tests::test_slugify ... ok
test parser::yaml::tests::valid_documents_parse_unchanged_after_guard ... ok
test scanner::watcher::tests::test_extract_paths_remove ... ok
test scanner::watcher::tests::test_is_ignored_emacs_autosave ... ok
test parser::markdown::headings::tests::test_extract_headings ... ok
test parser::markdown::tables::tests::test_extract_table_with_alignments ... ok
test parser::markdown::tables::tests::test_extract_table_basic ... ok
test scanner::watcher::tests::test_is_ignored_target ... ok
test parser::yaml::tests::directive_without_trailing_newline_terminates ... ok
test scanner::watcher::tests::test_is_ignored_tilde_backup ... ok
test scanner::watcher::tests::test_is_ignored_tmp_file ... ok
test scanner::watcher::tests::test_is_ignored_git_dir ... ok
test scanner::watcher::tests::test_extract_paths_modify ... ok
test scanner::watcher::tests::test_is_ignored_hidden_file ... ok
test scanner::watcher::tests::test_is_ignored_node_modules ... ok
test scanner::watcher::tests::test_is_ignored_vendor ... ok
test scanner::watcher::tests::test_is_ignored_vim_swap ... ok
test parser::markdown::links::tests::test_extract_links_basic ... ok
test scanner::watcher::tests::test_is_not_ignored_markdown ... ok
test scanner::watcher::tests::test_is_not_ignored_non_markdown ... ok
test transport::mcp::tests::mcp_server_rejects_invalid_configuration ... ok
test transport::mcp::tests::metadata_filter_parser_rejects_malformed_and_duplicate_filters ... ok
test transport::mcp::types::tests::enriched_document_output_serializes_every_optional_field ... ok
test transport::mcp::tests::metadata_filter_parser_accepts_values_containing_equals ... ok
test index::content_hash::tests::test_sampled_hash_large_content ... ok

test result: ok. 118 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

running 118 tests
test config::tests::test_bm25_config_default ... ok
test config::tests::test_create_default_config_file ... ok
test config::tests::test_env_overrides_max_front_matter_size ... ok
test config::tests::test_env_overrides_bm25_concept_type_weight ... ok
test config::tests::test_env_overrides_max_graph_nodes ... ok
test config::tests::test_env_overrides_bm25_body_weight ... ok
test config::tests::test_env_overrides_max_response_chars ... ok
test config::tests::test_env_overrides_bm25_headings_weight ... ok
test config::tests::test_env_overrides_bm25_k1 ... ok
test config::tests::test_invalid_env_bm25_title_weight ... ok
test config::tests::test_env_overrides_db_path ... ok
test config::tests::test_invalid_env_max_file_size ... ok
test config::tests::test_invalid_env_follow_symlinks ... ok
test config::tests::test_env_overrides_follow_symlinks ... ok
test config::tests::test_invalid_env_max_front_matter_size ... ok
test config::tests::test_invalid_env_max_response_chars ... ok
test config::tests::test_env_overrides_bm25_b ... ok
test config::tests::test_env_overrides_max_graph_depth ... ok
test config::tests::test_load_config_file_not_found ... ok
test config::tests::test_invalid_env_max_yaml_input_size ... ok
test config::tests::test_okc_config_default ... ok
test index::content_hash::tests::test_deterministic_hash ... ok
test index::content_hash::tests::test_different_content_different_hash ... ok
test index::content_hash::tests::test_full_hash_small_content ... ok
test index::content_hash::tests::test_hash_config_custom ... ok
test index::content_hash::tests::test_truncate_preserves_structure ... ok
test config::tests::test_load_config_from_file ... ok
test config::tests::test_env_overrides_max_yaml_input_size ... ok
test config::tests::test_env_overrides_roots ... ok
test config::tests::test_env_overrides_watcher_debounce_ms ... ok
test index::queries::metadata::tests::escapes_path_prefix_like_metacharacters ... ok
test index::queries::metadata::tests::validates_filter_names_and_operators ... ok
test config::tests::test_env_overrides_watcher_reconcile_secs ... ok
test config::tests::test_env_overrides_bm25_title_weight ... ok
test index::search_index::tests::correction_is_bounded_and_deterministic ... ok
test index::content_hash::tests::test_truncate_short_content_unchanged ... ok
test config::tests::test_env_overrides_exclude_patterns ... ok
test index::search_index::tests::edit_distance_handles_insertions_deletions_and_substitutions ... ok
test parser::frontmatter::tests::test_exceeds_max_size ... ok
test parser::frontmatter::tests::test_basic_frontmatter ... ok
test config::tests::test_env_overrides_max_file_size ... ok
test config::tests::test_invalid_env_max_graph_depth ... ok
test config::tests::test_invalid_env_max_graph_nodes ... ok
test parser::frontmatter::tests::test_missing_closing ... ok
test parser::links::tests::test_anchor_extracted_and_stored_separately ... ok
test parser::frontmatter::tests::test_bom_handling ... ok
test parser::frontmatter::tests::test_no_frontmatter ... ok
test parser::frontmatter::tests::test_windows_line_endings ... ok
test config::tests::test_env_overrides_bm25_description_weight ... ok
test parser::links::tests::test_case_insensitive_check_exists_macos ... ok
test config::tests::test_env_overrides_require_index_files ... ok
test parser::links::tests::test_case_insensitive_check_exists_windows ... ok
test parser::links::tests::test_check_exists_positive ... ok
test parser::links::tests::test_broken_link_warning_and_marked ... ok
test parser::links::tests::test_case_sensitive_check_exists_linux ... ok
test parser::links::tests::test_anchor_with_special_chars ... ok
test parser::links::tests::test_cycle_detection ... ok
test parser::links::tests::test_normalize_path_valid_returns_some ... ok
test parser::links::tests::test_external_url_left_unchanged ... ok
test parser::links::tests::test_external_url_with_anchor ... ok
test parser::links::tests::test_filter_self_references ... ok
test parser::links::tests::test_external_url_with_anchor_preserved ... ok
test parser::links::tests::test_normalize_path_traversal_returns_none ... ok
test parser::links::tests::test_no_anchor_returns_none ... ok
test parser::links::tests::test_path_traversal_blocked_from_root ... ok
test parser::links::tests::test_percent_encoded_slash_not_decoded_in_path ... ok
test parser::links::tests::test_percent_encoded_anchor_decoded ... ok
test parser::links::tests::test_percent_encoded_path_decoded ... ok
test parser::links::tests::test_path_traversal_blocked ... ok
test parser::links::tests::test_repository_root_relative_path ... ok
test parser::links::tests::test_resolve_parent_dir ... ok
test parser::links::tests::test_resolve_relative_same_dir ... ok
test parser::links::tests::test_relative_markdown_link_same_dir ... ok
test parser::links::tests::test_relative_markdown_link_parent_dir ... ok
test parser::links::tests::test_percent_encoded_special_chars_decoded ... ok
test parser::links::tests::test_round_trip_with_anchor ... ok
test parser::links::tests::test_split_anchor ... ok
test parser::links::tests::test_resolve_with_anchor_preserved_in_target ... ok
test parser::links::tests::test_round_trip_resolve_check_exists ... ok
test parser::links::tests::test_wiki_link_with_display_text ... ok
test parser::links::tests::test_wiki_link_with_anchor ... ok
test parser::links::tests::test_wiki_link_relative_path ... ok
test parser::links::tests::test_wiki_link_root_relative ... ok
test parser::links::tests::test_wiki_link_extraction_basic ... ok
test parser::links::tests::test_wiki_link_with_anchor_and_display ... ok
test parser::links::tests::test_wiki_link_nested_brackets ... ok
test parser::markdown::headings::tests::test_slugify ... ok
test parser::markdown::code_blocks::tests::test_extract_code_block_basic ... ok
test parser::markdown::links::tests::test_extract_links_basic ... ok
test parser::markdown::code_blocks::tests::test_extract_code_block_no_language ... ok
test parser::markdown::tables::tests::test_extract_table_basic ... ok
test scanner::watcher::tests::test_is_ignored_git_dir ... ok
test parser::markdown::sections::tests::test_build_sections_basic ... ok
test parser::markdown::links::tests::test_extract_wiki_links ... ok
test parser::markdown::tables::tests::test_extract_table_with_alignments ... ok
test scanner::watcher::tests::test_extract_paths_create ... ok
test scanner::watcher::tests::test_extract_paths_modify ... ok
test scanner::watcher::tests::test_extract_paths_remove ... ok
test scanner::watcher::tests::test_is_ignored_hidden_file ... ok
test scanner::watcher::tests::test_is_ignored_emacs_autosave ... ok
test scanner::watcher::tests::test_is_ignored_emacs_lock ... ok
test parser::markdown::headings::tests::test_extract_headings ... ok
test parser::yaml::tests::directive_without_trailing_newline_terminates ... ok
test parser::markdown::code_blocks::tests::test_extract_code_block_with_filename ... ok
test scanner::watcher::tests::test_is_ignored_tilde_backup ... ok
test scanner::watcher::tests::test_is_ignored_target ... ok
test parser::yaml::tests::valid_documents_parse_unchanged_after_guard ... ok
test scanner::watcher::tests::test_is_ignored_node_modules ... ok
test scanner::watcher::tests::test_is_ignored_vim_swap ... ok
test scanner::watcher::tests::test_is_ignored_tmp_file ... ok
test transport::mcp::tests::mcp_server_rejects_invalid_configuration ... ok
test scanner::watcher::tests::test_is_not_ignored_markdown ... ok
test transport::mcp::tests::metadata_filter_parser_rejects_malformed_and_duplicate_filters ... ok
test scanner::watcher::tests::test_is_ignored_vendor ... ok
test scanner::watcher::tests::test_is_not_ignored_non_markdown ... ok
test transport::mcp::tests::metadata_filter_parser_accepts_values_containing_equals ... ok
test transport::mcp::types::tests::enriched_document_output_serializes_every_optional_field ... ok
test index::content_hash::tests::test_sampled_hash_large_content ... ok

test result: ok. 118 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

running 20 tests
test test_in_memory_service_rejects_invalid_configuration ... ok
test test_service_rejects_invalid_configuration_before_opening_storage ... ok
test test_relationship_reasoning ... ok
test test_get_document_enriched_response_respects_configured_limit ... ok
test test_get_section ... ok
test test_exact_metadata_query ... ok
test test_hierarchical_browsing ... ok
test test_circular_links_handled ... ok
test test_backlinks ... ok
test test_metadata_query_filters_projection_order_and_counts ... ok
test test_search_combined_filters_counts_and_stable_pages ... ok
test test_repository_validation ... ok
test test_direct_concept_lookup ... ok
test test_get_document_opt_in_enriched_context_and_validation ... ok
test test_get_document_with_metadata ... ok
test test_stats ... ok
test test_search_with_filters ... ok
test test_validation_oversized_frontmatter ... ok
test test_search_uses_configured_bm25_field_weights ... ok
test test_validation_missing_type ... ok

test result: ok. 20 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.21s

running 8 tests
test test_mcp_stdio_packaged_binary_reports_invalid_root ... ok
test test_mcp_scan_rejects_invalid_configuration_before_storage ... ok
test test_mcp_enriched_document_respects_configured_response_limit ... ok
test test_mcp_error_invalid_path ... ok
test test_mcp_error_missing_document ... ok
test test_mcp_http_transport_packaged_binary ... ok
test test_all_mcp_tools_covered ... ok
test test_mcp_stdio_transport_all_tools_packaged_binary ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.35s

running 28 tests
test frontmatter_extractor_never_panics ... ok
test frontmatter_extractor_small_limit ... ok
test external_urls_unchanged ... ok
test link_anchor_handling ... ok
test link_resolution_never_panics ... ok
test bom_handling ... ok
test multiple_delimiters ... ok
test custom_fields_preserved ... ok
test parent_directory_traversal ... ok
test utf8_validation_in_frontmatter ... ok
test path_normalization_dots ... ok
test nested_path_resolution ... ok
test link_existence_check ... ok
test path_normalization_never_panics ... ok
test yaml_parser_never_panics ... ok
test relative_path_resolution ... ok
test yaml_tags_sequence ... ok
test windows_line_endings ... ok
test change_detector_new_files_added ... ok
test size_limit_enforcement ... ok
test change_detector_deleted_files ... ok
test change_detector_empty_current ... ok
test change_detector_identical_files_unchanged ... ok
test change_detector_modified_files ... ok
test change_detector_size_only_change ... ok
test change_detector_empty_previous ... ok
test change_detector_mtime_only_change ... ok
test change_detector_deterministic ... ok

test result: ok. 28 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.38s

running 2 tests
test metric_tests::metrics_use_judged_relevance_and_ignore_intentional_empty_queries ... ok
test production_lexical_search_baseline_v1 ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.64s

running 2 tests
test src/lib.rs - (line 21) - compile ... ok
test src/config.rs - config::OkcConfig (line 38) ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.56s - all must pass
<!-- SECTION:PLAN:END -->
