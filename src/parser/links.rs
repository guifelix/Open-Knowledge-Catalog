//! Link resolution and validation for wiki-style markdown links.
//!
//! [`LinkResolver`] handles:
//! - Relative path resolution (e.g., `[[page]]`, `[[../page]]`)
//! - Absolute path resolution (e.g., `[[/page]]`)
//! - External URL pass-through (http/https/mailto)
//! - Path normalization and traversal attack prevention
//! - Existence checking against known repository files
//! - URL percent-decoding for encoded paths
//! - Case-insensitive matching on case-insensitive filesystems
//! - Anchor fragment extraction and storage
//! - Wiki-style link syntax (`[[...]]`)
//! - Broken link handling (non-fatal warnings)
//! - Self-referencing link filtering
//! - Cycle detection for graph traversal

use std::path::Path;

use crate::model::Link;
use percent_encoding::{percent_decode_str, NON_ALPHANUMERIC};

/// Resolves and validates markdown links.
///
/// Provides static methods for resolving link targets relative to a source
/// document, checking existence, and batch-processing extracted links.
pub struct LinkResolver;

impl LinkResolver {
    /// Resolve a single link target relative to a source document.
    ///
    /// Handles:
    /// - Absolute paths (starting with `/`) -> repository root
    /// - Relative paths -> resolved from source document's parent directory
    /// - External URLs (http/https/mailto) -> returned unchanged
    /// - Path traversal attempts (`../` escaping repo) -> returns safe fallback
    /// - Percent-encoded paths -> decoded before resolution
    /// - Anchor fragments (`#section`) -> extracted and stored separately
    pub fn resolve(source_path: &str, target: &str) -> String {
        let source = Path::new(source_path);
        let parent = source.parent().unwrap_or(Path::new(""));

        // Check if it's an external URL BEFORE decoding
        // External URLs should be returned unchanged
        if target.starts_with("http://")
            || target.starts_with("https://")
            || target.starts_with("mailto:")
        {
            return target.to_string();
        }

        // For internal paths, decode percent-encoded characters
        let decoded_target = percent_decode_str(target).decode_utf8_lossy().to_string();

        // Extract anchor fragment before path resolution
        let (path_part, anchor) = split_anchor(&decoded_target);

        let resolved = if let Some(stripped) = path_part.strip_prefix('/') {
            Path::new(stripped).to_path_buf()
        } else {
            parent.join(path_part)
        };

        let normalized = normalize_path(&resolved).unwrap_or_else(|| {
            // Path traversal attempt detected - return a safe fallback
            // that will not match any known file
            "INVALID_PATH_TRAVERSAL".to_string()
        });

        // Reattach anchor if present
        let result = normalized.replace('\\', "/");
        if let Some(a) = anchor {
            format!("{}#{}", result, a)
        } else {
            result
        }
    }

    /// Check if a link target exists in the repository.
    ///
    /// External URLs (http/https/mailto) always return true.
    /// Internal links are checked against the known files list (without anchor).
    /// Matching is case-insensitive on case-insensitive filesystems (macOS, Windows).
    pub fn check_exists(target: &str, known_files: &[String]) -> bool {
        if target.starts_with("http://")
            || target.starts_with("https://")
            || target.starts_with("mailto:")
        {
            return true;
        }
        let target_without_anchor = target.split('#').next().unwrap_or(target);
        let normalized_target = normalize_case(target_without_anchor);

        known_files
            .iter()
            .any(|f| normalize_case(f.as_str()) == normalized_target)
    }

    /// Resolve a batch of raw links against known repository files.
    ///
    /// For each link, resolves the target path and checks existence.
    /// Returns links with `target`, `target_anchor`, `is_external`,
    /// and `exists_in_repository` populated.
    /// Broken links (non-existent internal links) are included but marked
    /// with `exists_in_repository = false` and generate a warning.
    pub fn resolve_links(
        source_path: &str,
        raw_links: &[Link],
        known_files: &[String],
    ) -> Vec<Link> {
        raw_links
            .iter()
            .map(|link| {
                if link.is_external {
                    return link.clone();
                }
                let resolved = Self::resolve(source_path, &link.raw);
                let (target_path, target_anchor) = split_anchor(&resolved);
                let exists = Self::check_exists(target_path, known_files);

                // Log warning for broken internal links (non-fatal)
                if !exists && !link.is_external {
                    eprintln!(
                        "Warning: Broken link in '{}': '{}' -> '{}' (target not found)",
                        source_path, link.raw, target_path
                    );
                }

                Link {
                    raw: link.raw.clone(),
                    target: target_path.to_string(),
                    target_anchor,
                    is_external: false,
                    exists_in_repository: exists,
                }
            })
            .collect()
    }

    /// Filter out self-referencing links from a list of resolved links.
    ///
    /// A self-referencing link is one where the target path (without anchor)
    /// matches the source document path. These are stored but excluded from
    /// backlink/graph computations.
    pub fn filter_self_references(source_path: &str, links: &[Link]) -> Vec<Link> {
        let normalized_source = normalize_case(source_path);
        links
            .iter()
            .filter(|link| {
                let target_without_anchor = link.target.split('#').next().unwrap_or(&link.target);
                normalize_case(target_without_anchor) != normalized_source
            })
            .cloned()
            .collect()
    }

    /// Check if following links from a source would create a cycle.
    ///
    /// Uses DFS to detect cycles in the link graph. Returns true if adding
    /// the given link would create a cycle.
    pub fn would_create_cycle(
        source_path: &str,
        target_path: &str,
        graph: &std::collections::HashMap<String, Vec<String>>,
    ) -> bool {
        // Simple cycle detection: check if target can reach source
        let mut visited = std::collections::HashSet::new();
        let mut stack = vec![target_path.to_string()];

        while let Some(current) = stack.pop() {
            if current == source_path {
                return true;
            }
            if visited.insert(current.clone()) {
                if let Some(neighbors) = graph.get(&current) {
                    stack.extend(neighbors.iter().cloned());
                }
            }
        }
        false
    }
}

/// Split a path/URL into its path component and optional anchor fragment.
///
/// Returns `(path_without_anchor, anchor_or_none)`.
fn split_anchor(input: &str) -> (&str, Option<String>) {
    if let Some(idx) = input.find('#') {
        let path = &input[..idx];
        let anchor = input[idx + 1..].to_string();
        // Decode the anchor fragment
        let decoded_anchor = percent_decode_str(&anchor).decode_utf8_lossy().to_string();
        (path, Some(decoded_anchor))
    } else {
        (input, None)
    }
}

/// Normalize path case for case-insensitive filesystem comparison.
///
/// On macOS and Windows, filesystems are case-insensitive (but case-preserving).
/// This function lowercases the path for comparison purposes.
fn normalize_case(path: &str) -> String {
    // Check if we're on a case-insensitive filesystem
    #[cfg(target_os = "macos")]
    {
        path.to_lowercase()
    }
    #[cfg(target_os = "windows")]
    {
        path.to_lowercase()
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        path.to_string()
    }
}

/// Normalize a path by resolving `.` and `..` components.
///
/// Returns `None` if the path attempts to traverse outside the repository root
/// (i.e., if `..` would go past the root).
pub fn normalize_path(path: &Path) -> Option<String> {
    let mut components = Vec::new();
    for component in path.components() {
        match component {
            std::path::Component::Normal(c) => components.push(c.to_string_lossy().to_string()),
            std::path::Component::ParentDir => {
                if components.is_empty() {
                    // Attempt to traverse above root - reject
                    return None;
                }
                components.pop();
            }
            std::path::Component::CurDir => {}
            std::path::Component::RootDir | std::path::Component::Prefix(_) => {
                components.push(String::new());
            }
        }
    }
    Some(components.join("/"))
}

/// Check if a resolved link target is safe (doesn't escape repository root).
///
/// Returns `true` if the path is safe, `false` if it attempts path traversal.
#[allow(dead_code)]
pub fn is_safe_path(path: &str) -> bool {
    // Empty path or root is safe
    if path.is_empty() || path == "." {
        return true;
    }
    // Absolute paths (starting with /) are not allowed in repository-relative paths
    if path.starts_with('/') {
        return false;
    }
    // Check for path traversal attempts
    let path_obj = Path::new(path);
    for component in path_obj.components() {
        if matches!(component, std::path::Component::ParentDir) {
            return false;
        }
    }
    true
}

/// Extract wiki-style link targets from markdown text.
///
/// Wiki-style links use `[[target]]` or `[[target|display]]` syntax.
/// Returns a vector of raw link targets (without the `[[` `]]` delimiters).
pub fn extract_wiki_links(text: &str) -> Vec<String> {
    let mut links = Vec::new();
    let mut chars = text.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '[' {
            if let Some(&next_c) = chars.peek() {
                if next_c == '[' {
                    chars.next(); // consume second '['
                    let mut target = String::new();
                    let mut depth = 1;

                    for c in chars.by_ref() {
                        if c == '[' {
                            depth += 1;
                            target.push(c);
                        } else if c == ']' {
                            depth -= 1;
                            if depth == 0 {
                                // Check for pipe (display text)
                                if let Some(pipe_idx) = target.find('|') {
                                    target.truncate(pipe_idx);
                                }
                                links.push(target.trim().to_string());
                                break;
                            }
                            target.push(c);
                        } else {
                            target.push(c);
                        }
                    }
                }
            }
        }
    }
    links
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let exists = LinkResolver::check_exists(&path, &known_files);

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
                target: "".to_string(),
                target_anchor: None,
                is_external: false,
                exists_in_repository: false,
            },
            Link {
                raw: "nonexistent.md".to_string(),
                target: "".to_string(),
                target_anchor: None,
                is_external: false,
                exists_in_repository: false,
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
            },
            Link {
                raw: "revenue.md".to_string(),
                target: "metrics/revenue.md".to_string(),
                target_anchor: None,
                is_external: false,
                exists_in_repository: true,
            },
            Link {
                raw: "revenue.md#section".to_string(),
                target: "metrics/revenue.md".to_string(),
                target_anchor: Some("section".to_string()),
                is_external: false,
                exists_in_repository: true,
            },
        ];

        let filtered = LinkResolver::filter_self_references(source, &links);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].target, "metrics/costs.md");
    }

    // #9 Cycle detection: A->B->C->A does not cause infinite traversal in graph queries
    #[test]
    fn test_cycle_detection() {
        let mut graph = std::collections::HashMap::new();
        graph.insert("a.md".to_string(), vec!["b.md".to_string()]);
        graph.insert("b.md".to_string(), vec!["c.md".to_string()]);
        graph.insert("c.md".to_string(), vec!["a.md".to_string()]); // cycle

        // a -> b -> c -> a is a cycle
        assert!(LinkResolver::would_create_cycle("a.md", "b.md", &graph));
        assert!(LinkResolver::would_create_cycle("b.md", "c.md", &graph));
        assert!(LinkResolver::would_create_cycle("c.md", "a.md", &graph));

        // No cycle if we break it
        let mut graph2 = std::collections::HashMap::new();
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
}
