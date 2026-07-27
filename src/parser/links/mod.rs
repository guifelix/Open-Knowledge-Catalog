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

use std::collections::HashMap;
use std::path::Path;

use crate::model::Link;
use crate::parser::link_utils::{
    extract_wiki_links, is_safe_path, normalize_case, normalize_path, split_anchor,
};
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
                let resolved = Self::resolve(source_path, &link.target);
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
        graph: &HashMap<String, Vec<String>>,
    ) -> bool {
        // Simple cycle detection: check if target can reach source
        let mut visited = HashMap::new();
        let mut stack = vec![target_path.to_string()];

        while let Some(current) = stack.pop() {
            if current == source_path {
                return true;
            }
            if visited.insert(current.clone(), true).is_none() {
                if let Some(neighbors) = graph.get(&current) {
                    stack.extend(neighbors.iter().cloned());
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests;
