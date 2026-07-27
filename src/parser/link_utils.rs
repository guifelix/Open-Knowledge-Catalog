//! Standalone link utility functions for path normalization, safety checks, and wiki-link extraction.

use percent_encoding::{percent_decode_str, NON_ALPHANUMERIC};
use std::path::Path;

/// Split a path/URL into its path component and optional anchor fragment.
///
/// Returns `(path_without_anchor, anchor_or_none)`.
/// The anchor fragment is URL-decoded.
pub fn split_anchor(input: &str) -> (&str, Option<String>) {
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
pub fn normalize_case(path: &str) -> String {
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
