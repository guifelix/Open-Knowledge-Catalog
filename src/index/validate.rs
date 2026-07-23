use std::collections::{HashMap, HashSet};

use rusqlite::params;

use super::database::RepositoryIndex;
use crate::index::traits::GraphStore;
use crate::model::*;
use crate::parser::frontmatter::FrontMatterExtractor;
use crate::parser::links::LinkResolver;
use crate::parser::markdown::MarkdownParser;
use crate::parser::yaml::YamlParser;

const CHECKS: &[&str] = &[
    "broken_links",
    "scan_errors",
    "missing_index_files",
    "unsupported_encoding",
    "oversized_frontmatter",
    "invalid_yaml",
    "missing_metadata",
    "duplicate_concept",
    "duplicate_content",
    "circular_references",
    "malformed_links",
];

impl RepositoryIndex {
    pub fn validate(&self) -> Result<Vec<ValidationIssue>, anyhow::Error> {
        Ok(self.validate_report()?.issues)
    }

    pub fn validate_report(&self) -> Result<ValidationReport, anyhow::Error> {
        let mut issues = Vec::new();

        if let Some(ref gs) = self.graph_store {
            issues.extend(gs.validate_links()?);
        }

        issues.extend(self.check_missing_index_files());

        issues.extend(self.validate_files());

        if let Some(ref gs) = self.graph_store {
            issues.extend(gs.detect_circular_references()?);
        }

        Ok(self.build_report(issues))
    }

    pub fn validate_incremental(
        &self,
        previous_hashes: Option<&HashMap<String, String>>,
    ) -> Result<ValidationReport, anyhow::Error> {
        let mut issues = Vec::new();

        if let Some(ref gs) = self.graph_store {
            issues.extend(gs.validate_links()?);
            issues.extend(gs.detect_circular_references()?);
        }

        issues.extend(self.check_missing_index_files());

        issues.extend(self.validate_files_changed(previous_hashes));

        Ok(self.build_report(issues))
    }

    fn build_report(&self, issues: Vec<ValidationIssue>) -> ValidationReport {
        let total = issues.len();
        let errors = issues.iter().filter(|i| i.severity == "error").count();
        let warnings = issues.iter().filter(|i| i.severity == "warning").count();
        let infos = issues.iter().filter(|i| i.severity == "info").count();

        let checks: Vec<CheckResult> = CHECKS
            .iter()
            .map(|name| {
                let count = issues.iter().filter(|i| i.category.as_str() == *name).count();
                CheckResult {
                    check_name: name.to_string(),
                    status: if count == 0 {
                        CheckStatus::Pass
                    } else if *name == "broken_links"
                        || *name == "missing_index_files"
                        || *name == "missing_metadata"
                        || *name == "circular_references"
                    {
                        CheckStatus::Warn
                    } else {
                        CheckStatus::Fail
                    },
                    issue_count: count,
                }
            })
            .collect();

        ValidationReport {
            summary: ValidationSummary {
                total_issues: total,
                errors,
                warnings,
                infos,
                checks,
            },
            issues,
        }
    }

    fn check_missing_index_files(&self) -> Vec<ValidationIssue> {
        if !self.config.require_index_files {
            return Vec::new();
        }
        let mut issues = Vec::new();
        let mut stmt = self
            .conn
            .prepare("SELECT DISTINCT parent_path FROM documents WHERE parent_path != ''")
            .ok();
        let Some(mut stmt) = stmt else {
            return issues;
        };
        let dirs: Vec<String> = stmt
            .query_map([], |row| row.get::<_, String>(0))
            .ok()
            .map(|rows| rows.filter_map(|r| r.ok()).collect())
            .unwrap_or_default();

        for dir in dirs {
            let index_path = format!("{}/index.md", dir.trim_end_matches('/'));
            let exists: bool = self
                .conn
                .query_row(
                    "SELECT COUNT(*) > 0 FROM documents WHERE path = ?1",
                    params![index_path],
                    |row| row.get(0),
                )
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

    fn validate_files(&self) -> Vec<ValidationIssue> {
        let paths: Vec<String> = self
            .conn
            .prepare("SELECT path FROM documents ORDER BY path")
            .ok()
            .and_then(|mut stmt| {
                stmt.query_map([], |row| row.get::<_, String>(0))
                    .ok()
                    .map(|rows| rows.filter_map(|r| r.ok()).collect())
            })
            .unwrap_or_default();

        let extractor = FrontMatterExtractor::new(self.config.max_front_matter_size);
        let mut seen_concepts: HashMap<(String, String), String> = HashMap::new();
        let mut seen_hashes: HashMap<String, Vec<String>> = HashMap::new();
        let mut issues = Vec::new();

        let content_hashes: HashMap<String, String> = self
            .conn
            .prepare("SELECT path, content_hash FROM documents")
            .ok()
            .and_then(|mut stmt| {
                stmt.query_map([], |row| {
                    Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
                })
                .ok()
                .map(|rows| rows.filter_map(|r| r.ok()).collect())
            })
            .unwrap_or_default();

        for path in &paths {
            let abs_path = self
                .config
                .roots
                .iter()
                .map(|root| root.join(path))
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
                    let category = if e.message.contains("exceeds") {
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

            let (body_start, raw_yaml) = match extracted {
                Some(r) => r,
                None => continue,
            };

            let fm = match YamlParser::parse(&raw_yaml) {
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

            if fm.title.as_deref().unwrap_or("").trim().is_empty() {
                issues.push(ValidationIssue {
                    path: path.clone(),
                    severity: "warning".to_string(),
                    category: "missing_metadata".to_string(),
                    message: "Missing required metadata: 'title'".to_string(),
                    line: None,
                });
            }
            if fm.concept_type.as_deref().unwrap_or("").trim().is_empty() {
                issues.push(ValidationIssue {
                    path: path.clone(),
                    severity: "warning".to_string(),
                    category: "missing_metadata".to_string(),
                    message: "Missing required metadata: 'type'".to_string(),
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

            let body_text = &full_text[body_start..];
            let (_, raw_links, _, _) = MarkdownParser::parse(body_text);
            let known_files: Vec<String> = paths.clone();
            let resolved = LinkResolver::resolve_links(path, &raw_links, &known_files);

            for link in &resolved {
                if link.is_external {
                    continue;
                }
                if link.target.is_empty() {
                    issues.push(ValidationIssue {
                        path: path.clone(),
                        severity: "warning".to_string(),
                        category: "malformed_links".to_string(),
                        message: format!("Empty link target"),
                        line: None,
                    });
                    continue;
                }
                if !link.target.starts_with('#') && !link.exists_in_repository {
                    issues.push(ValidationIssue {
                        path: path.clone(),
                        severity: "warning".to_string(),
                        category: "malformed_links".to_string(),
                        message: format!("Broken link to '{}'", link.target),
                        line: None,
                    });
                }
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

    fn validate_files_changed(
        &self,
        previous_hashes: Option<&HashMap<String, String>>,
    ) -> Vec<ValidationIssue> {
        let previous = match previous_hashes {
            Some(h) => h,
            None => return self.validate_files(),
        };

        let mut stmt = match self
            .conn
            .prepare("SELECT path, content_hash FROM documents")
        {
            Ok(s) => s,
            Err(_) => return self.validate_files(),
        };

        let current: HashMap<String, String> = match stmt
            .query_map([], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            }) {
            Ok(rows) => rows.filter_map(|r| r.ok()).collect(),
            Err(_) => return self.validate_files(),
        };

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