//! Validation check implementations.
//!
//! Contains the actual check logic for validating the repository index,
//! including file validation, missing index file detection, and
//! incremental validation helpers.

use std::collections::{HashMap, HashSet};

use rusqlite::params;

use crate::index::database::RepositoryIndex;
use crate::model::document::ValidationIssue;
use crate::parser::frontmatter::FrontMatterExtractor;
use crate::parser::yaml::YamlParser;

/// Returns true if the path refers to a reserved OKF filename (index.md or log.md).
///
/// Reserved files are not concept documents (OKF v0.2 §3.1) and have
/// special frontmatter rules (OKF v0.2 §8, §9).
fn is_reserved_filename(path: &str) -> bool {
    let name = path.rsplit('/').next().unwrap_or(path);
    name == "index.md" || name == "log.md"
}

impl RepositoryIndex {
    pub(super) fn check_missing_index_files(&self) -> Vec<ValidationIssue> {
        if !self.config.require_index_files {
            return Vec::new();
        }
        let mut issues = Vec::new();
        let dirs: Vec<String> = self
            .pool()
            .get()
            .ok()
            .and_then(|conn| {
                conn.prepare("SELECT DISTINCT parent_path FROM documents WHERE parent_path != ''")
                    .ok()
                    .and_then(|mut stmt| {
                        stmt.query_map([], |row| row.get::<_, String>(0))
                            .ok()
                            .map(|rows| rows.filter_map(|r| r.ok()).collect())
                    })
            })
            .unwrap_or_default();

        for dir in dirs {
            let index_path = format!("{}/index.md", dir.trim_end_matches('/'));
            let exists: bool = self
                .pool()
                .get()
                .ok()
                .and_then(|conn| {
                    conn.query_row(
                        "SELECT COUNT(*) > 0 FROM documents WHERE path = ?1",
                        params![index_path],
                        |row| row.get(0),
                    )
                    .ok()
                })
                .unwrap_or(false);

            if !exists {
                issues.push(ValidationIssue {
                    path: dir.clone(),
                    severity: "warning".to_string(),
                    category: "missing_index_files".to_string(),
                    message: format!("Directory '{}' is missing an index.md file", dir),
                    line: None,
                });
            }
        }
        issues
    }

    pub(super) fn validate_files(&self) -> Vec<ValidationIssue> {
        let paths: Vec<String> = self
            .pool()
            .get()
            .ok()
            .and_then(|conn| {
                conn.prepare("SELECT path FROM documents ORDER BY path")
                    .ok()
                    .and_then(|mut stmt| {
                        stmt.query_map([], |row| row.get::<_, String>(0))
                            .ok()
                            .map(|rows| rows.filter_map(|r| r.ok()).collect())
                    })
            })
            .unwrap_or_default();

        let extractor = FrontMatterExtractor::new(self.config.max_front_matter_size);
        let mut seen_concepts: HashMap<(String, String), String> = HashMap::new();
        let mut seen_hashes: HashMap<String, Vec<String>> = HashMap::new();
        let mut issues = Vec::new();

        let content_hashes: HashMap<String, String> = self
            .pool()
            .get()
            .ok()
            .and_then(|conn| {
                conn.prepare("SELECT path, content_hash FROM documents")
                    .ok()
                    .and_then(|mut stmt| {
                        stmt.query_map([], |row| {
                            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
                        })
                        .ok()
                        .map(|rows| rows.filter_map(|r| r.ok()).collect())
                    })
            })
            .unwrap_or_default();

        for path in &paths {
            let abs_path = self
                .config
                .roots
                .iter()
                .map(|root| root.path.join(path))
                .find(|p| p.exists());

            let abs_path = match abs_path {
                Some(p) => p,
                None => continue,
            };

            let content = match std::fs::read(&abs_path) {
                Ok(c) => c,
                Err(_) => continue,
            };

            let full_text = match String::from_utf8(content) {
                Ok(s) => s,
                Err(e) => {
                    issues.push(ValidationIssue {
                        path: path.clone(),
                        severity: "error".to_string(),
                        category: "unsupported_encoding".to_string(),
                        message: format!("Invalid UTF-8 encoding: {}", e),
                        line: None,
                    });
                    continue;
                }
            };

            let extracted = match extractor.extract(full_text.as_bytes()) {
                Ok(r) => r,
                Err(e) => {
                    let category = if e.message.contains("exceed") {
                        "oversized_frontmatter"
                    } else {
                        "invalid_frontmatter"
                    };
                    issues.push(ValidationIssue {
                        path: path.clone(),
                        severity: "error".to_string(),
                        category: category.to_string(),
                        message: e.message.clone(),
                        line: None,
                    });
                    continue;
                }
            };

            let (_body_start, raw_yaml) = match extracted {
                Some(r) => r,
                None => continue,
            };

            let fm = match YamlParser::parse(&raw_yaml, self.config.max_yaml_input_size) {
                Ok(fm) => fm,
                Err(e) => {
                    issues.push(ValidationIssue {
                        path: path.clone(),
                        severity: "error".to_string(),
                        category: "invalid_yaml".to_string(),
                        message: e.message.clone(),
                        line: None,
                    });
                    continue;
                }
            };

            // OKF v0.2 §8, §9: Reserved files (index.md, log.md) have
            // restricted frontmatter rules and are not concept documents.
            if is_reserved_filename(path) {
                let basename = path.rsplit('/').next().unwrap_or(path);
                let is_root = !path.contains('/');

                if is_root {
                    // Root index.md: only okf_version allowed (§8 exception)
                    let mut violations: Vec<String> = Vec::new();
                    if fm.title.is_some() {
                        violations.push("title".into());
                    }
                    if fm.concept_type.is_some() {
                        violations.push("type".into());
                    }
                    if fm.description.is_some() {
                        violations.push("description".into());
                    }
                    if !fm.tags.is_empty() {
                        violations.push("tags".into());
                    }
                    let extra_custom: Vec<&str> = fm
                        .custom
                        .keys()
                        .filter(|k| *k != "okf_version")
                        .map(|k| k.as_str())
                        .collect();
                    if !violations.is_empty() || !extra_custom.is_empty() {
                        let mut msg = String::from(
                            "Root index.md may only carry 'okf_version' in frontmatter",
                        );
                        if !violations.is_empty() {
                            msg.push_str(&format!(
                                "; prohibited fields: {}",
                                violations.join(", ")
                            ));
                        }
                        if !extra_custom.is_empty() {
                            msg.push_str(&format!("; unknown keys: {}", extra_custom.join(", ")));
                        }
                        issues.push(ValidationIssue {
                            path: path.clone(),
                            severity: "error".to_string(),
                            category: "reserved_file_frontmatter".to_string(),
                            message: msg,
                            line: None,
                        });
                    }
                } else {
                    // Non-root reserved file: no frontmatter allowed (§8, §9)
                    issues.push(ValidationIssue {
                        path: path.clone(),
                        severity: "error".to_string(),
                        category: "reserved_file_frontmatter".to_string(),
                        message: format!(
                            "Reserved file '{}' must not contain frontmatter",
                            basename
                        ),
                        line: None,
                    });
                }
                continue;
            }

            // Concept-document checks (non-reserved files only)
            // OKF v0.2 §4.1: 'type' is required
            if fm.concept_type.as_deref().unwrap_or("").trim().is_empty() {
                issues.push(ValidationIssue {
                    path: path.clone(),
                    severity: "error".to_string(),
                    category: "missing_type".to_string(),
                    message: "Missing required field 'type' in frontmatter".to_string(),
                    line: None,
                });
            }

            if let (Some(ct), Some(t)) = (&fm.concept_type, &fm.title) {
                let key = (ct.clone(), t.clone());
                if let Some(existing_path) = seen_concepts.get(&key) {
                    issues.push(ValidationIssue {
                        path: path.clone(),
                        severity: "error".to_string(),
                        category: "duplicate_concept".to_string(),
                        message: format!(
                            "Duplicate concept: '{}' (type: '{}') also appears at '{}'",
                            t, ct, existing_path
                        ),
                        line: None,
                    });
                } else {
                    seen_concepts.insert(key, path.clone());
                }
            }

            if let Some(content_hash) = content_hashes.get(path) {
                seen_hashes
                    .entry(content_hash.clone())
                    .or_default()
                    .push(path.clone());
            }
        }

        for (hash, dup_paths) in &seen_hashes {
            if dup_paths.len() > 1 {
                for p in dup_paths {
                    let others: Vec<&str> = dup_paths
                        .iter()
                        .filter(|x| *x != p)
                        .map(|x| x.as_str())
                        .collect();
                    issues.push(ValidationIssue {
                        path: p.clone(),
                        severity: "warning".to_string(),
                        category: "duplicate_content".to_string(),
                        message: format!(
                            "Duplicate content (hash: {}): matches {}",
                            &hash[..8.min(hash.len())],
                            others.join(", ")
                        ),
                        line: None,
                    });
                }
            }
        }

        issues
    }

    pub(super) fn validate_files_changed(
        &self,
        previous_hashes: Option<&HashMap<String, String>>,
    ) -> Vec<ValidationIssue> {
        let previous = match previous_hashes {
            Some(h) => h,
            None => return self.validate_files(),
        };

        let current: HashMap<String, String> = self
            .pool()
            .get()
            .ok()
            .and_then(|conn| {
                conn.prepare("SELECT path, content_hash FROM documents")
                    .ok()
                    .and_then(|mut stmt| {
                        stmt.query_map([], |row| {
                            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
                        })
                        .ok()
                        .map(|rows| rows.filter_map(|r| r.ok()).collect())
                    })
            })
            .unwrap_or_default();

        let changed: HashSet<String> = current
            .into_iter()
            .filter(|(path, hash)| previous.get(path) != Some(hash))
            .map(|(path, _)| path)
            .collect();

        let all_issues = self.validate_files();
        all_issues
            .into_iter()
            .filter(|issue| changed.contains(&issue.path))
            .collect()
    }
}
