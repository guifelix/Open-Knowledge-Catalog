//! Document retrieval operations.

use crate::error::Result;
use crate::index::database::RepositoryIndex;
use crate::index::queries::suggest;
use std::collections::BTreeMap;

use crate::model::document::{
    BacklinkInfo, DocumentDetail, DocumentMetadata, HeadingInfo, LinkInfo, ParseError,
};
use rusqlite::params;

const VALID_INCLUDES: &[&str] = &[
    "metadata",
    "headings",
    "body",
    "custom",
    "content_hash",
    "parent_path",
    "links",
    "backlinks",
];

/// Check whether a document exists at the given path.
///
/// Lightweight existence probe used to distinguish a genuinely-missing
/// document (which should surface recovery hints) from an existing document
/// whose requested section was not found (which keeps the base shape).
pub fn document_exists(index: &RepositoryIndex, doc_path: &str) -> Result<bool> {
    let conn = index.pool().get()?;
    let exists = conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM documents WHERE path = ?1)",
        params![doc_path],
        |row| row.get::<_, bool>(0),
    )?;
    Ok(exists)
}

/// Get a document by path with optional section inclusion and truncation.
pub fn get_document(
    index: &RepositoryIndex,
    doc_path: &str,
    include: &[String],
    max_body_chars: usize,
) -> Result<DocumentDetail> {
    for value in include {
        if !VALID_INCLUDES.contains(&value.as_str()) {
            return Err(crate::error::OkfError::validation(
                format!(
                    "Unknown include value '{value}'. Valid values: {}",
                    VALID_INCLUDES.join(", ")
                ),
                Some("include".to_string()),
                Some(value.clone()),
            ));
        }
    }

    let conn = index.pool().get()?;
    let mut stmt = conn.prepare(
        "SELECT id, path, title, type, description, body_text, file_size, modified_at, parse_status,
                content_hash, parent_path
         FROM documents WHERE path = ?1",
    )?;

    let doc = match stmt.query_row(params![doc_path], |row| {
        let id: i64 = row.get(0)?;
        let path: String = row.get(1)?;
        let title: Option<String> = row.get(2)?;
        let ctype: Option<String> = row.get(3)?;
        let description: Option<String> = row.get(4)?;
        let body_text: String = row.get::<_, Option<String>>(5)?.unwrap_or_default();
        let file_size: i64 = row.get(6)?;
        let modified_at: i64 = row.get(7)?;
        let parse_status: String = row.get(8)?;
        let content_hash: Option<String> = row.get(9)?;
        let parent_path: String = row.get(10)?;
        Ok((
            id,
            path,
            title,
            ctype,
            description,
            body_text,
            file_size,
            modified_at,
            parse_status,
            content_hash,
            parent_path,
        ))
    }) {
        Ok(doc) => doc,
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            let hints = index
                .load_paths()
                .map(|paths| suggest::suggest_paths(doc_path, &paths, suggest::MAX_SUGGESTIONS))
                .unwrap_or_default();
            return Err(crate::error::OkfError::not_found_with_hints(
                "document",
                Some(std::path::PathBuf::from(doc_path)),
                hints,
            ));
        }
        Err(err) => {
            return Err(crate::error::OkfError::from(err));
        }
    };

    let (
        id,
        path,
        title,
        ctype,
        description,
        body_text,
        file_size,
        modified_at,
        parse_status,
        content_hash,
        parent_path,
    ) = doc;

    let mut tags = vec![];
    if includes(include, "metadata") {
        let mut tag_stmt =
            conn.prepare("SELECT tag FROM document_tags WHERE document_id = ?1 ORDER BY tag")?;
        tags = tag_stmt
            .query_map(params![id], |row| row.get::<_, String>(0))?
            .map(|r| r.map_err(crate::error::OkfError::from))
            .collect::<Result<Vec<_>>>()?;
    }

    let include_metadata = includes(include, "metadata");
    let include_custom = includes(include, "custom");
    let mut custom_fields = BTreeMap::new();
    if include_metadata || include_custom {
        let mut field_stmt = conn.prepare(
            "SELECT key, value FROM metadata_fields WHERE document_id = ?1 ORDER BY key",
        )?;
        for row in field_stmt.query_map(params![id], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })? {
            let (k, v) = row?;
            if let Ok(val) = serde_json::from_str(&v) {
                custom_fields.insert(k, val);
            }
        }
    }

    let mut headings = vec![];
    if includes(include, "headings") {
        let mut h_stmt = conn.prepare(
            "SELECT level, title, anchor FROM headings WHERE document_id = ?1 ORDER BY position",
        )?;
        headings = h_stmt
            .query_map(params![id], |row| {
                Ok(HeadingInfo {
                    level: row.get::<_, i32>(0)? as u32,
                    title: row.get(1)?,
                    anchor: row.get(2)?,
                })
            })?
            .map(|r| r.map_err(crate::error::OkfError::from))
            .collect::<Result<Vec<_>>>()?;
    }

    let mut errors = vec![];
    {
        let mut e_stmt =
            conn.prepare("SELECT stage, message, line FROM scan_errors WHERE path = ?1")?;
        errors = e_stmt
            .query_map(params![doc_path], |row| {
                Ok(ParseError {
                    stage: row.get(0)?,
                    message: row.get(1)?,
                    line: row.get::<_, Option<i64>>(2)?.map(|l| l as usize),
                })
            })?
            .map(|r| r.map_err(crate::error::OkfError::from))
            .collect::<Result<Vec<_>>>()?;
    }

    let links = if includes(include, "links") {
        let mut link_stmt = conn.prepare(
            "SELECT target_path, target_anchor, external_url, exists_in_repository, target_root_id
             FROM links WHERE source_document_id = ?1
             ORDER BY COALESCE(target_path, external_url), COALESCE(target_anchor, '')",
        )?;
        let items = link_stmt
            .query_map(params![id], |row| {
                Ok(LinkInfo {
                    target_path: row.get(0)?,
                    target_anchor: row.get(1)?,
                    external_url: row.get(2)?,
                    exists_in_repository: row.get::<_, i32>(3)? != 0,
                    target_root_id: row.get(4)?,
                })
            })?
            .map(|r| r.map_err(crate::error::OkfError::from))
            .collect::<Result<Vec<_>>>()?;
        Some(items)
    } else {
        None
    };

    let backlinks = if includes(include, "backlinks") {
        let mut backlink_stmt = conn.prepare(
            "SELECT source.path, l.target_anchor, l.exists_in_repository
             FROM links l
             JOIN documents source ON source.id = l.source_document_id
             WHERE l.target_path = ?1
             ORDER BY source.path, COALESCE(l.target_anchor, '')",
        )?;
        let items = backlink_stmt
            .query_map(params![doc_path], |row| {
                Ok(BacklinkInfo {
                    source_path: row.get(0)?,
                    target_anchor: row.get(1)?,
                    exists_in_repository: row.get::<_, i32>(2)? != 0,
                })
            })?
            .map(|r| r.map_err(crate::error::OkfError::from))
            .collect::<Result<Vec<_>>>()?;
        Some(items)
    } else {
        None
    };

    let truncated;
    let body = if includes(include, "body") {
        if body_text.chars().count() > max_body_chars {
            truncated = true;
            Some(body_text.chars().take(max_body_chars).collect())
        } else {
            truncated = false;
            Some(body_text)
        }
    } else {
        truncated = false;
        None
    };

    let mut detail = DocumentDetail {
        path,
        metadata: DocumentMetadata {
            title,
            concept_type: ctype,
            description,
            tags,
            custom: if include_metadata || include_custom {
                custom_fields
            } else {
                BTreeMap::new()
            },
            file_size: file_size as u64,
            modified_at,
            parse_status,
        },
        headings,
        body,
        truncated,
        errors,
        content_hash: if includes(include, "content_hash") {
            content_hash
        } else {
            None
        },
        parent_path: includes(include, "parent_path").then_some(parent_path),
        links,
        backlinks,
    };
    enforce_response_limit(&mut detail, index.config.max_response_chars)?;
    Ok(detail)
}

fn includes(include: &[String], value: &str) -> bool {
    include.iter().any(|candidate| candidate == value)
}

fn enforce_response_limit(detail: &mut DocumentDetail, max_chars: usize) -> Result<()> {
    if serialized_chars(detail)? <= max_chars {
        return Ok(());
    }

    detail.truncated = true;
    let body = detail.body.take();

    while serialized_chars(detail)? > max_chars {
        if detail
            .backlinks
            .as_mut()
            .is_some_and(|items| items.pop().is_some())
            || detail
                .links
                .as_mut()
                .is_some_and(|items| items.pop().is_some())
            || detail.headings.pop().is_some()
            || detail.errors.pop().is_some()
            || detail.metadata.custom.pop_last().is_some()
        {
            continue;
        }
        return Err(crate::error::OkfError::internal(
            format!(
                "max_response_chars ({max_chars}) is too small for the document response envelope"
            ),
            None,
        ));
    }

    if let Some(body) = body {
        let characters = body.chars().collect::<Vec<_>>();
        let mut low = 0;
        let mut high = characters.len();
        while low < high {
            let midpoint = low + (high - low).div_ceil(2);
            detail.body = Some(characters[..midpoint].iter().collect());
            if serialized_chars(detail)? <= max_chars {
                low = midpoint;
            } else {
                high = midpoint - 1;
            }
        }
        detail.body = Some(characters[..low].iter().collect());
    }
    Ok(())
}

fn serialized_chars(detail: &DocumentDetail) -> Result<usize> {
    Ok(serde_json::to_string(detail)
        .map_err(|e| crate::error::OkfError::serde(e.to_string()))?
        .chars()
        .count())
}

/// Get a specific section from a document by heading title or anchor slug.
pub fn get_section(
    index: &RepositoryIndex,
    doc_path: &str,
    heading: &str,
    max_chars: usize,
) -> Result<Option<(String, String)>> {
    let conn = index.pool().get()?;
    let mut stmt = conn.prepare("SELECT body_text, title FROM documents WHERE path = ?1")?;

    let (body_text, _doc_title): (String, Option<String>) =
        match stmt.query_row(params![doc_path], |row| {
            Ok((
                row.get::<_, Option<String>>(0)?.unwrap_or_default(),
                row.get(1)?,
            ))
        }) {
            Ok(r) => r,
            Err(rusqlite::Error::QueryReturnedNoRows) => return Ok(None),
            Err(e) => return Err(e.into()),
        };

    let (_, _, _, sections, _tables, _code_blocks) =
        crate::parser::markdown::MarkdownParser::parse(&body_text);

    for section in &sections {
        if section.heading.eq_ignore_ascii_case(heading) {
            let content = if section.content.len() > max_chars {
                section.content.chars().take(max_chars).collect()
            } else {
                section.content.clone()
            };
            return Ok(Some((section.heading.clone(), content)));
        }
    }

    let heading_lower = heading.to_lowercase();
    for section in &sections {
        let slug = slugify(&section.heading);
        if slug == heading_lower {
            let content = if section.content.len() > max_chars {
                section.content.chars().take(max_chars).collect()
            } else {
                section.content.clone()
            };
            return Ok(Some((section.heading.clone(), content)));
        }
    }

    // Third pass: prefix match (case-insensitive) — for when AI agents approximate
    for section in &sections {
        if section.heading.to_lowercase().starts_with(&heading_lower) {
            let content = if section.content.len() > max_chars {
                section.content.chars().take(max_chars).collect()
            } else {
                section.content.clone()
            };
            return Ok(Some((section.heading.clone(), content)));
        }
    }

    Ok(None)
}

fn slugify(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == ' ' || *c == '-' || *c == '_')
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("-")
}
