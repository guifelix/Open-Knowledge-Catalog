//! Document parsing module.
//!
//! This module provides parsing functionality for markdown documents,
//! including front-matter extraction, markdown parsing, and link resolution.

use std::collections::BTreeMap;
use std::path::Path;

use crate::config::OkcConfig;
use crate::index::content_hash::{compute_content_hash_default, HashConfig};
use crate::model::document::{
    FileRecord, FrontMatter, Heading, HeadingInfo, Link, LinkInfo, ParseError, ParseStatus,
    ProcessChangesResult, ScanResult,
};
use crate::parser::frontmatter::FrontMatterExtractor;
use crate::parser::links::LinkResolver;
use crate::parser::markdown::MarkdownParser;
use crate::parser::yaml::YamlParser;
use crate::scanner::changes::{ChangeDetector, FileChanges};

/// Parsed document data ready for storage.
#[derive(Debug, Clone)]
pub struct ParsedDocument {
    pub path: String,
    pub absolute_path: String,
    pub size: u64,
    pub modified_at: i64,
    pub front_matter: Option<FrontMatter>,
    pub parse_status: ParseStatus,
    pub parse_errors: Vec<ParseError>,
    pub markdown_body: String,
    pub headings: Vec<HeadingInfo>,
    pub links: Vec<LinkInfo>,
    pub body_text: String,
    pub content_hash: String,
    pub parent_path: String,
    pub title: Option<String>,
    pub concept_type: Option<String>,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub custom_fields: BTreeMap<String, serde_json::Value>,
}

/// Document parser that coordinates front-matter, markdown, and link parsing.
pub struct DocumentParser {
    extractor: FrontMatterExtractor,
    config: OkcConfig,
}

impl DocumentParser {
    /// Create a new document parser with the given configuration.
    pub fn new(config: &OkcConfig) -> Self {
        Self {
            extractor: FrontMatterExtractor::new(config.max_front_matter_size),
            config: config.clone(),
        }
    }

    /// Parse a single file into a ParsedDocument.
    pub fn parse_file(&self, file: &FileRecord, known_paths: &[String]) -> ParsedDocument {
        let path = Path::new(&file.absolute_path);
        let content = match std::fs::read(path) {
            Ok(c) => c,
            Err(e) => {
                return ParsedDocument {
                    path: file.path.clone(),
                    absolute_path: file.absolute_path.clone(),
                    size: file.size,
                    modified_at: file.modified_at,
                    front_matter: None,
                    parse_status: ParseStatus::Failed,
                    parse_errors: vec![ParseError {
                        stage: "read".to_string(),
                        message: e.to_string(),
                        line: None,
                    }],
                    markdown_body: String::new(),
                    headings: vec![],
                    links: vec![],
                    body_text: String::new(),
                    content_hash: String::new(),
                    parent_path: String::new(),
                    title: None,
                    concept_type: None,
                    description: None,
                    tags: vec![],
                    custom_fields: BTreeMap::new(),
                };
            }
        };

        let body = match String::from_utf8(content) {
            Ok(s) => s,
            Err(e) => {
                return ParsedDocument {
                    path: file.path.clone(),
                    absolute_path: file.absolute_path.clone(),
                    size: file.size,
                    modified_at: file.modified_at,
                    front_matter: None,
                    parse_status: ParseStatus::Failed,
                    parse_errors: vec![ParseError {
                        stage: "utf8".to_string(),
                        message: e.to_string(),
                        line: None,
                    }],
                    markdown_body: String::new(),
                    headings: vec![],
                    links: vec![],
                    body_text: String::new(),
                    content_hash: String::new(),
                    parent_path: String::new(),
                    title: None,
                    concept_type: None,
                    description: None,
                    tags: vec![],
                    custom_fields: BTreeMap::new(),
                };
            }
        };

        let (front_matter, parse_status, parse_errors, front_matter_end) = Self::parse_front_matter(
            &self.extractor,
            &file.path,
            &body,
            self.config.max_yaml_input_size,
        );

        let markdown_body = if front_matter_end < body.len() {
            body[front_matter_end..].trim_start()
        } else {
            ""
        };

        let (headings, links, body_text, _sections, _tables, _code_blocks) =
            MarkdownParser::parse(markdown_body);

        // Convert Heading to HeadingInfo
        let heading_infos: Vec<HeadingInfo> = headings
            .into_iter()
            .map(|h| HeadingInfo {
                level: h.level,
                title: h.title,
                anchor: h.anchor,
            })
            .collect();

        let resolved_links = LinkResolver::resolve_links(&file.path, &links, known_paths);

        // Convert Link to LinkInfo
        let mut link_infos: Vec<LinkInfo> = resolved_links
            .into_iter()
            .map(|l| {
                let target = l.target;
                LinkInfo {
                    target_path: if l.is_external {
                        None
                    } else {
                        Some(target.clone())
                    },
                    target_anchor: l.target_anchor,
                    external_url: if l.is_external { Some(target) } else { None },
                    exists_in_repository: if l.is_external {
                        true
                    } else {
                        l.exists_in_repository
                    },
                    relation: None,
                }
            })
            .collect();

        // Merge typed_links from front-matter into the link set.
        // Each typed link becomes a LinkInfo carrying its `relation`, so the
        // relationship survives storage and graph traversal. Broken internal
        // typed targets are tolerated (marked not-in-repository) per the OKF
        // broken-link tolerance conventions.
        if let Some(fm) = front_matter.as_ref() {
            for tl in &fm.typed_links {
                let is_external = tl.target.starts_with("http://")
                    || tl.target.starts_with("https://")
                    || tl.target.starts_with("mailto:");
                let exists = !is_external && known_paths.contains(&tl.target);
                link_infos.push(LinkInfo {
                    target_path: if is_external {
                        None
                    } else {
                        Some(tl.target.clone())
                    },
                    target_anchor: tl.anchor.clone(),
                    external_url: if is_external {
                        Some(tl.target.clone())
                    } else {
                        None
                    },
                    exists_in_repository: if is_external { true } else { exists },
                    relation: Some(tl.relation.clone()),
                });
            }
        }

        let content_hash = compute_content_hash_default(body.as_bytes());
        let parent_path = Path::new(&file.path)
            .parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();

        let (title, concept_type, description, tags, custom_fields) = match &front_matter {
            Some(fm) => {
                let tags_ref = &fm.tags;
                let custom_ref = &fm.custom;
                (
                    fm.title.as_deref().map(|s| s.to_string()),
                    fm.concept_type.as_deref().map(|s| s.to_string()),
                    fm.description.as_deref().map(|s| s.to_string()),
                    tags_ref.clone(),
                    custom_ref.clone(),
                )
            }
            None => (None, None, None, Vec::new(), BTreeMap::new()),
        };

        ParsedDocument {
            path: file.path.clone(),
            absolute_path: file.absolute_path.clone(),
            size: file.size,
            modified_at: file.modified_at,
            front_matter,
            parse_status,
            parse_errors,
            markdown_body: markdown_body.to_string(),
            headings: heading_infos,
            links: link_infos,
            body_text,
            content_hash,
            parent_path,
            title,
            concept_type,
            description,
            tags,
            custom_fields,
        }
    }

    fn parse_front_matter(
        extractor: &FrontMatterExtractor,
        _path: &str,
        body: &str,
        max_yaml_input_size: usize,
    ) -> (Option<FrontMatter>, ParseStatus, Vec<ParseError>, usize) {
        let mut errors = Vec::new();

        let extracted = match extractor.extract(body.as_bytes()) {
            Ok(r) => r,
            Err(e) => {
                errors.push(e);
                return (None, ParseStatus::Partial, errors, 0);
            }
        };

        let (front_matter_end, raw_yaml) = match extracted {
            Some(r) => r,
            None => return (None, ParseStatus::Ok, errors, 0),
        };

        match YamlParser::parse(&raw_yaml, max_yaml_input_size) {
            Ok(fm) => (Some(fm), ParseStatus::Ok, errors, front_matter_end),
            Err(e) => {
                errors.push(e);
                (None, ParseStatus::Partial, errors, front_matter_end)
            }
        }
    }
}

/// Process file changes and return parsed documents ready for storage.
pub fn process_changes(
    config: &OkcConfig,
    changes: &FileChanges,
    known_paths: &[String],
) -> (Vec<ParsedDocument>, ProcessChangesResult) {
    let parser = DocumentParser::new(config);
    let mut parse_failures = 0;
    let mut total_links = 0;
    let mut broken_links = 0;
    let mut parsed_docs = Vec::new();

    for file in changes.added.iter().chain(changes.modified.iter()) {
        let parsed = parser.parse_file(file, known_paths);

        total_links += parsed.links.len();
        broken_links += parsed
            .links
            .iter()
            .filter(|l| l.external_url.is_none() && !l.exists_in_repository)
            .count();

        if parsed.parse_status == ParseStatus::Failed {
            parse_failures += 1;
        }

        parsed_docs.push(parsed);
    }

    let result = ProcessChangesResult {
        files_added: changes.added.len(),
        files_modified: changes.modified.len(),
        files_deleted: changes.deleted.len(),
        parse_failures,
        broken_links,
        total_links,
    };

    (parsed_docs, result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::unwrap_used)]
    fn parse_file_merges_typed_links_with_relation() {
        let dir = std::env::temp_dir().join(format!("okc_parser_test_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("typed.md");
        std::fs::write(
            &path,
            r#"---
title: Typed
typed_links:
  version: 1
  links:
    - target: metrics/costs.md
      relation: depends_on
    - target: https://example.com
      relation: references
    - target: missing.md
      relation: related
---
# Body

See [costs](metrics/costs.md).
"#,
        )
        .unwrap();

        let file = FileRecord {
            path: "typed.md".to_string(),
            absolute_path: path.to_string_lossy().to_string(),
            size: std::fs::metadata(&path).unwrap().len(),
            modified_at: 0,
        };
        let config = OkcConfig::default();

        let parsed =
            DocumentParser::new(&config).parse_file(&file, &["metrics/costs.md".to_string()]);

        // Markdown link plus the three typed links.
        assert!(parsed.links.len() >= 4);
        let typed: Vec<&LinkInfo> = parsed
            .links
            .iter()
            .filter(|l| l.relation.is_some())
            .collect();
        assert_eq!(typed.len(), 3);

        let internal = typed
            .iter()
            .find(|l| l.target_path.as_deref() == Some("metrics/costs.md"))
            .unwrap();
        assert_eq!(internal.relation.as_deref(), Some("depends_on"));
        assert!(internal.exists_in_repository);

        let external = typed
            .iter()
            .find(|l| l.external_url.as_deref() == Some("https://example.com"))
            .unwrap();
        assert_eq!(external.relation.as_deref(), Some("references"));
        assert!(external.exists_in_repository);

        let broken = typed
            .iter()
            .find(|l| l.target_path.as_deref() == Some("missing.md"))
            .unwrap();
        assert_eq!(broken.relation.as_deref(), Some("related"));
        assert!(!broken.exists_in_repository);

        std::fs::remove_dir_all(&dir).unwrap();
    }
}
