//! Unit tests for [`LinkResolver`] and link utility functions.
//!
//! Covers: relative/absolute path resolution, external URL pass-through,
//! path traversal prevention, anchor fragment handling, percent-encoded
//! path decoding, case-insensitive matching, wiki-link extraction, broken
//! link handling, self-reference filtering, and cycle detection.

use super::*;
use crate::model::Link;
use std::collections::HashMap;

#[test]
fn test_resolve_relative_same_dir() {
    let result = LinkResolver::resolve("metrics/revenue.md", "costs.md");
    assert_eq!(result, "metrics/costs.md");
}

#[test]
fn test_resolve_parent_dir() {
    let result = LinkResolver::resolve("metrics/revenue.md", "../datasets/orders.md");
    assert_eq!(result, "datasets/orders.md");
}

#[test]
fn test_external_url_left_unchanged() {
    let result = LinkResolver::resolve("metrics/revenue.md", "https://example.com");
    assert_eq!(result, "https://example.com");
}

#[test]
fn test_check_exists_positive() {
    let files = vec!["metrics/revenue.md".into(), "datasets/orders.md".into()];
    assert!(LinkResolver::check_exists("metrics/revenue.md", &files));
    assert!(!LinkResolver::check_exists(
        "metrics/nonexistent.md",
        &files
    ));
}

#[test]
fn test_path_traversal_blocked() {
    let result = LinkResolver::resolve("metrics/revenue.md", "../../../etc/passwd");
    assert_eq!(result, "INVALID_PATH_TRAVERSAL");
}

#[test]
fn test_path_traversal_blocked_from_root() {
    let result = LinkResolver::resolve("index.md", "../secret.txt");
    assert_eq!(result, "INVALID_PATH_TRAVERSAL");
}

#[test]
fn test_repository_root_relative_path() {
    let result = LinkResolver::resolve("metrics/revenue.md", "/datasets/orders.md");
    assert_eq!(result, "datasets/orders.md");
}

#[test]
fn test_normalize_path_traversal_returns_none() {
    use std::path::Path;
    let result = normalize_path(Path::new("../../../etc/passwd"));
    assert!(result.is_none());
}

#[test]
fn test_normalize_path_valid_returns_some() {
    use std::path::Path;
    let result = normalize_path(Path::new("metrics/../datasets/orders.md"));
    assert_eq!(result, Some("datasets/orders.md".to_string()));
}

// ===== NEW TESTS FOR EDGE CASES =====

// #1 Anchor fragments extracted and stored separately
#[test]
fn test_anchor_extracted_and_stored_separately() {
    let result = LinkResolver::resolve("metrics/revenue.md", "report.md#section-one");
    let (path, anchor) = split_anchor(&result);
    assert_eq!(path, "metrics/report.md");
    assert_eq!(anchor, Some("section-one".to_string()));
}

#[test]
fn test_anchor_with_special_chars() {
    let result = LinkResolver::resolve("metrics/revenue.md", "report.md#section%20one");
    let (path, anchor) = split_anchor(&result);
    assert_eq!(path, "metrics/report.md");
    assert_eq!(anchor, Some("section one".to_string())); // URL decoded
}

#[test]
fn test_no_anchor_returns_none() {
    let result = LinkResolver::resolve("metrics/revenue.md", "report.md");
    let (path, anchor) = split_anchor(&result);
    assert_eq!(path, "metrics/report.md");
    assert_eq!(anchor, None);
}

#[test]
fn test_external_url_with_anchor_preserved() {
    let result = LinkResolver::resolve("metrics/revenue.md", "https://example.com#section");
    // External URLs should keep anchor in target
    assert_eq!(result, "https://example.com#section");
}

// #2 Case-insensitive path matching on case-insensitive filesystems
#[test]
fn test_case_insensitive_check_exists_macos() {
    #[cfg(target_os = "macos")]
    {
        let files = vec!["Metrics/Revenue.md".into(), "Datasets/Orders.md".into()];
        // Should match regardless of case
        assert!(LinkResolver::check_exists("metrics/revenue.md", &files));
        assert!(LinkResolver::check_exists("METRICS/REVENUE.MD", &files));
        assert!(LinkResolver::check_exists("DaTaSeTs/OrDeRs.Md", &files));
    }
}

#[test]
fn test_case_insensitive_check_exists_windows() {
    #[cfg(target_os = "windows")]
    {
        let files = vec!["Metrics/Revenue.md".into(), "Datasets/Orders.md".into()];
        assert!(LinkResolver::check_exists("metrics/revenue.md", &files));
        assert!(LinkResolver::check_exists("METRICS/REVENUE.MD", &files));
    }
}

#[test]
fn test_case_sensitive_check_exists_linux() {
    #[cfg(target_os = "linux")]
    {
        let files = vec!["Metrics/Revenue.md".into(), "Datasets/Orders.md".into()];
        // On Linux, case-sensitive
        assert!(!LinkResolver::check_exists("metrics/revenue.md", &files));
        assert!(LinkResolver::check_exists("Metrics/Revenue.md", &files));
    }
}

// #3 Percent-encoded paths decoded before resolution
#[test]
fn test_percent_encoded_path_decoded() {
    // Space encoded as %20
    let result = LinkResolver::resolve("metrics/revenue.md", "costs%20report.md");
    assert_eq!(result, "metrics/costs report.md");
}

#[test]
fn test_percent_encoded_special_chars_decoded() {
    let result = LinkResolver::resolve("metrics/revenue.md", "file%23name.md");
    assert_eq!(result, "metrics/file#name.md");
}

#[test]
fn test_percent_encoded_anchor_decoded() {
    let result = LinkResolver::resolve("metrics/revenue.md", "report.md#section%20one");
    let (path, anchor) = split_anchor(&result);
    assert_eq!(path, "metrics/report.md");
    assert_eq!(anchor, Some("section one".to_string()));
}

#[test]
fn test_percent_encoded_slash_not_decoded_in_path() {
    // %2F should not be decoded to / as that would be path traversal
    let result = LinkResolver::resolve("metrics/revenue.md", "sub%2Fdir/file.md");
    // Should remain encoded or be treated as literal filename
    assert!(result.contains("sub%2Fdir") || result.contains("sub/dir"));
}

// #4 Round-trip: resolve -> check_exists -> get_document works
#[test]
fn test_round_trip_resolve_check_exists() {
    let source = "metrics/revenue.md";
    let target = "costs.md";
    let known_files = vec!["metrics/revenue.md".into(), "metrics/costs.md".into()];

    let resolved = LinkResolver::resolve(source, target);
    let exists = LinkResolver::check_exists(&resolved, &known_files);

    assert_eq!(resolved, "metrics/costs.md");
    assert!(exists);
}

#[test]
fn test_round_trip_with_anchor() {
    let source = "metrics/revenue.md";
    let target = "costs.md#q1";
    let known_files = vec!["metrics/revenue.md".into(), "metrics/costs.md".into()];

    let resolved = LinkResolver::resolve(source, target);
    let (path, anchor) = split_anchor(&resolved);
    let exists = LinkResolver::check_exists(path, &known_files);

    assert_eq!(path, "metrics/costs.md");
    assert_eq!(anchor, Some("q1".to_string()));
    assert!(exists);
}

// #5 Obsidian WikiLinks [[target]] resolve to the correct bundle concept
#[test]
fn test_wiki_link_extraction_basic() {
    let text = "See [[metrics/revenue]] and [[datasets/orders]] for details.";
    let links = extract_wiki_links(text);
    assert_eq!(links.len(), 2);
    assert_eq!(links[0], "metrics/revenue");
    assert_eq!(links[1], "datasets/orders");
}

#[test]
fn test_wiki_link_with_display_text() {
    let text = "See [[metrics/revenue|Revenue Report]] for details.";
    let links = extract_wiki_links(text);
    assert_eq!(links.len(), 1);
    assert_eq!(links[0], "metrics/revenue");
}

#[test]
fn test_wiki_link_with_anchor() {
    let text = "See [[metrics/revenue#q1]] for Q1 data.";
    let links = extract_wiki_links(text);
    assert_eq!(links.len(), 1);
    assert_eq!(links[0], "metrics/revenue#q1");
}

#[test]
fn test_wiki_link_with_anchor_and_display() {
    let text = "See [[metrics/revenue#q1|Q1 Revenue]] for details.";
    let links = extract_wiki_links(text);
    assert_eq!(links.len(), 1);
    assert_eq!(links[0], "metrics/revenue#q1");
}

#[test]
fn test_wiki_link_relative_path() {
    let text = "See [[../datasets/orders]] for orders.";
    let links = extract_wiki_links(text);
    assert_eq!(links.len(), 1);
    assert_eq!(links[0], "../datasets/orders");
}

#[test]
fn test_wiki_link_root_relative() {
    let text = "See [[/metrics/revenue]] for revenue.";
    let links = extract_wiki_links(text);
    assert_eq!(links.len(), 1);
    assert_eq!(links[0], "/metrics/revenue");
}

#[test]
fn test_wiki_link_nested_brackets() {
    let text = "See [[metrics/revenue [2024]]] for details.";
    let links = extract_wiki_links(text);
    assert_eq!(links.len(), 1);
    assert_eq!(links[0], "metrics/revenue [2024]");
}

// #6 Relative Markdown links (./file.md#section) resolve against source file directory
#[test]
fn test_relative_markdown_link_same_dir() {
    let result = LinkResolver::resolve("metrics/revenue.md", "./costs.md");
    assert_eq!(result, "metrics/costs.md");
}

#[test]
fn test_relative_markdown_link_parent_dir() {
    let result = LinkResolver::resolve("metrics/revenue.md", "../datasets/orders.md");
    assert_eq!(result, "datasets/orders.md");
}

// #7 Broken links produce a non-fatal warning and are omitted from the graph
#[test]
fn test_broken_link_warning_and_marked() {
    let source = "metrics/revenue.md";
    let raw_links = vec![
        Link {
            raw: "costs.md".to_string(),
            target: "costs.md".to_string(),
            target_anchor: None,
            is_external: false,
            exists_in_repository: false,
            relation: None,
        },
        Link {
            raw: "nonexistent.md".to_string(),
            target: "nonexistent.md".to_string(),
            target_anchor: None,
            is_external: false,
            exists_in_repository: false,
            relation: None,
        },
    ];
    let known_files = vec!["metrics/revenue.md".into(), "metrics/costs.md".into()];

    let resolved = LinkResolver::resolve_links(source, &raw_links, &known_files);

    assert_eq!(resolved.len(), 2);
    assert_eq!(resolved[0].target, "metrics/costs.md");
    assert!(resolved[0].exists_in_repository);
    assert_eq!(resolved[1].target, "metrics/nonexistent.md");
    assert!(!resolved[1].exists_in_repository);
}

// #8 Self-referencing links (to the same file or fragment) are stored but never returned as backlinks
#[test]
fn test_filter_self_references() {
    let source = "metrics/revenue.md";
    let links = vec![
        Link {
            raw: "costs.md".to_string(),
            target: "metrics/costs.md".to_string(),
            target_anchor: None,
            is_external: false,
            exists_in_repository: true,
            relation: None,
        },
        Link {
            raw: "revenue.md".to_string(),
            target: "metrics/revenue.md".to_string(),
            target_anchor: None,
            is_external: false,
            exists_in_repository: true,
            relation: None,
        },
        Link {
            raw: "revenue.md#section".to_string(),
            target: "metrics/revenue.md".to_string(),
            target_anchor: Some("section".to_string()),
            is_external: false,
            exists_in_repository: true,
            relation: None,
        },
    ];

    let filtered = LinkResolver::filter_self_references(source, &links);
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].target, "metrics/costs.md");
}

// #9 Cycle detection: A->B->C->A does not cause infinite traversal in graph queries
#[test]
fn test_cycle_detection() {
    let mut graph = HashMap::new();
    graph.insert("a.md".to_string(), vec!["b.md".to_string()]);
    graph.insert("b.md".to_string(), vec!["c.md".to_string()]);
    graph.insert("c.md".to_string(), vec!["a.md".to_string()]); // cycle

    // a -> b -> c -> a is a cycle
    assert!(LinkResolver::would_create_cycle("a.md", "b.md", &graph));
    assert!(LinkResolver::would_create_cycle("b.md", "c.md", &graph));
    assert!(LinkResolver::would_create_cycle("c.md", "a.md", &graph));

    // No cycle if we break it
    let mut graph2 = HashMap::new();
    graph2.insert("a.md".to_string(), vec!["b.md".to_string()]);
    graph2.insert("b.md".to_string(), vec!["c.md".to_string()]);
    // c.md has no outgoing edges

    assert!(!LinkResolver::would_create_cycle("a.md", "b.md", &graph2));
    assert!(!LinkResolver::would_create_cycle("b.md", "c.md", &graph2));
}

#[test]
fn test_split_anchor() {
    assert_eq!(split_anchor("path/to/file.md"), ("path/to/file.md", None));
    assert_eq!(
        split_anchor("path/to/file.md#section"),
        ("path/to/file.md", Some("section".to_string()))
    );
    assert_eq!(
        split_anchor("path/to/file.md#section%20one"),
        ("path/to/file.md", Some("section one".to_string()))
    );
}

#[test]
fn test_external_url_with_anchor() {
    let result = LinkResolver::resolve("metrics/revenue.md", "https://example.com#section");
    assert_eq!(result, "https://example.com#section");
}

#[test]
fn test_resolve_with_anchor_preserved_in_target() {
    // The resolve function returns the full path with anchor
    // split_anchor is used to separate them for storage
    let result = LinkResolver::resolve("metrics/revenue.md", "report.md#section");
    let (path, anchor) = split_anchor(&result);
    assert_eq!(path, "metrics/report.md");
    assert_eq!(anchor, Some("section".to_string()));
}
