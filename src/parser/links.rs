use std::path::Path;

use crate::model::Link;

pub struct LinkResolver;

impl LinkResolver {
    pub fn resolve(source_path: &str, target: &str) -> String {
        let source = Path::new(source_path);
        let parent = source.parent().unwrap_or(Path::new(""));

        let resolved = if target.starts_with('/') {
            Path::new(&target[1..]).to_path_buf()
        } else if target.starts_with("http://")
            || target.starts_with("https://")
            || target.starts_with("mailto:")
        {
            return target.to_string();
        } else {
            parent.join(target)
        };

        let normalized = normalize_path(&resolved);
        normalized.replace('\\', "/")
    }

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

pub fn normalize_path(path: &Path) -> String {
    let mut components = Vec::new();
    for component in path.components() {
        match component {
            std::path::Component::Normal(c) => components.push(c.to_string_lossy().to_string()),
            std::path::Component::ParentDir => {
                components.pop();
            }
            std::path::Component::CurDir => {}
            std::path::Component::RootDir | std::path::Component::Prefix(_) => {
                components.push(String::new());
            }
        }
    }
    components.join("/")
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
}
