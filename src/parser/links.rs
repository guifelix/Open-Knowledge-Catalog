//! Link resolution and validation for wiki-style markdown links.
//!
//! [`LinkResolver`] handles:
//! - Relative path resolution (e.g., `[[page]]`, `[[../page]]`)
//! - Absolute path resolution (e.g., `[[/page]]`)
//! - External URL pass-through (http/https/mailto)
//! - Path normalization and traversal attack prevention
//! - Existence checking against known repository files

use std::path::Path;

use crate::model::Link;

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
    pub fn resolve(source_path: &str, target: &str) -> String {
        let source = Path::new(source_path);
        let parent = source.parent().unwrap_or(Path::new(""));

        let resolved = if let Some(stripped) = target.strip_prefix('/') {
            Path::new(stripped).to_path_buf()
        } else if target.starts_with("http://")
            || target.starts_with("https://")
            || target.starts_with("mailto:")
        {
            return target.to_string();
        } else {
            parent.join(target)
        };

        let normalized = normalize_path(&resolved).unwrap_or_else(|| {
            // Path traversal attempt detected - return a safe fallback
            // that will not match any known file
            "INVALID_PATH_TRAVERSAL".to_string()
        });
        normalized.replace('\\', "/")
    }

    /// Check if a link target exists in the repository.
    ///
    /// External URLs (http/https/mailto) always return true.
    /// Internal links are checked against the known files list (without anchor).
    pub fn check_exists(target: &str, known_files: &[String]) -> bool {
        if target.starts_with("http://")
            || target.starts_with("https://")
            || target.starts_with("mailto:")
        {
            return true;
        }
        let target_without_anchor = target.split('#').next().unwrap_or(target);
        known_files
            .iter()
            .any(|f| f.as_str() == target_without_anchor)
    }

    /// Resolve a batch of raw links against known repository files.
    ///
    /// For each link, resolves the target path and checks existence.
    /// Returns links with `target`, `target_anchor`, `is_external`,
    /// and `exists_in_repository` populated.
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
                let has_anchor = resolved.contains('#');
                let (target_path, target_anchor) = if has_anchor {
                    let mut parts = resolved.splitn(2, '#');
                    (
                        parts.next().unwrap_or(&resolved).to_string(),
                        parts.next().map(|s| s.to_string()),
                    )
                } else {
                    (resolved.clone(), None)
                };
                let exists = Self::check_exists(&target_path, known_files);
                Link {
                    raw: link.raw.clone(),
                    target: resolved,
                    target_anchor,
                    is_external: false,
                    exists_in_repository: exists,
                }
            })
            .collect()
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
        // Attempt to traverse outside repository root
        let result = LinkResolver::resolve("metrics/revenue.md", "../../../etc/passwd");
        assert_eq!(result, "INVALID_PATH_TRAVERSAL");
    }

    #[test]
    fn test_path_traversal_blocked_from_root() {
        // Attempt to traverse from root level
        let result = LinkResolver::resolve("index.md", "../secret.txt");
        assert_eq!(result, "INVALID_PATH_TRAVERSAL");
    }

    #[test]
    fn test_repository_root_relative_path() {
        // Paths starting with / are treated as repository-relative (wiki-style)
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
}
