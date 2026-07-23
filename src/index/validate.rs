//! Repository validation logic for RepositoryIndex.
//!
//! Public API: validate.

use std::collections::HashMap;

use rusqlite::params;

use super::database::RepositoryIndex;
use crate::index::traits::GraphStore;
use crate::model::*;
use crate::parser::frontmatter::FrontMatterExtractor;
use crate::parser::yaml::YamlParser;

impl RepositoryIndex {
    pub fn validate(&self) -> Result<Vec<ValidationIssue>, anyhow::Error> {
        let mut issues = Vec::new();

        if let Some(ref gs) = self.graph_store {
            issues.extend(gs.validate_links()?);
        }

        if self.config.require_index_files {
            let mut stmt = self.conn.prepare(
                "SELECT DISTINCT parent_path FROM documents
                 WHERE parent_path != ''",
            )?;
            let dirs: Vec<String> = stmt
                .query_map([], |row| row.get::<_, String>(0))?
                .filter_map(|r| r.ok())
                .collect();

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
                        category: "missing_index".to_string(),
                        message: format!("Directory '{}' is missing an index.md file", dir),
                        line: None,
                    });
                }
            }
        }

        let mut stmt = self
            .conn
            .prepare("SELECT path FROM documents ORDER BY path")?;
        let paths: Vec<String> = stmt
            .query_map([], |row| row.get::<_, String>(0))?
            .filter_map(|r| r.ok())
            .collect();

        let extractor = FrontMatterExtractor::new(self.config.max_front_matter_size);
        let mut seen_concepts: HashMap<(String, String), String> = HashMap::new();

        for path in &paths {
            let abs_path = self.config.roots.iter().map(|root| root.join(path)).find(|p| p.exists());

            let abs_path = match abs_path {
                Some(p) => p,
                None => continue,
            };

            let content = match std::fs::read(&abs_path) {
                Ok(c) => c,
                Err(_) => continue,
            };

            let body = match String::from_utf8(content) {
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

            let extracted = match extractor.extract(body.as_bytes()) {
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

            let (_, raw_yaml) = match extracted {
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
        }

        Ok(issues)
    }
}
