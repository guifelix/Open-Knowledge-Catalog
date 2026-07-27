//! Document retrieval operations.

use crate::index::database::RepositoryIndex;
use crate::model::document::{DocumentDetail, DocumentMetadata, HeadingInfo, ParseError};
use rusqlite::params;

/// Get a document by path with optional section inclusion and truncation.
pub fn get_document(
    index: &RepositoryIndex,
    doc_path: &str,
    include: &[String],
    max_body_chars: usize,
) -> Result<DocumentDetail, anyhow::Error> {
    let conn = index.pool().get()?;
    let mut stmt = conn.prepare(
        "SELECT id, path, title, type, description, body_text, file_size, modified_at, parse_status
         FROM documents WHERE path = ?1",
    )?;

    let doc = stmt.query_row(params![doc_path], |row| {
        let id: i64 = row.get(0)?;
        let path: String = row.get(1)?;
        let title: Option<String> = row.get(2)?;
        let ctype: Option<String> = row.get(3)?;
        let description: Option<String> = row.get(4)?;
        let body_text: String = row.get::<_, Option<String>>(5)?.unwrap_or_default();
        let file_size: i64 = row.get(6)?;
        let modified_at: i64 = row.get(7)?;
        let parse_status: String = row.get(8)?;
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
        ))
    })?;

    let (id, path, title, ctype, description, body_text, file_size, modified_at, parse_status) =
        doc;

    let mut tags = vec![];
    if include.contains(&"metadata".to_string()) {
        let conn = index.pool().get()?;
        let mut tag_stmt = conn.prepare("SELECT tag FROM document_tags WHERE document_id = ?1")?;
        tags = tag_stmt
            .query_map(params![id], |row| row.get::<_, String>(0))?
            .filter_map(|r| r.ok())
            .collect();
    }

    let mut custom = std::collections::BTreeMap::new();
    if include.contains(&"metadata".to_string()) {
        let conn = index.pool().get()?;
        let mut field_stmt =
            conn.prepare("SELECT key, value FROM metadata_fields WHERE document_id = ?1")?;
        for row in field_stmt.query_map(params![id], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })? {
            let (k, v) = row?;
            if let Ok(val) = serde_json::from_str(&v) {
                custom.insert(k, val);
            }
        }
    }

    let mut headings = vec![];
    if include.contains(&"headings".to_string()) {
        let conn = index.pool().get()?;
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
            .filter_map(|r| r.ok())
            .collect();
    }

    let mut errors = vec![];
    {
        let conn = index.pool().get()?;
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
            .filter_map(|r| r.ok())
            .collect();
    }

    let truncated;
    let body = if include.contains(&"body".to_string()) {
        if body_text.len() > max_body_chars {
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

    Ok(DocumentDetail {
        path,
        metadata: DocumentMetadata {
            title,
            concept_type: ctype,
            description,
            tags,
            custom,
            file_size: file_size as u64,
            modified_at,
            parse_status,
        },
        headings,
        body,
        truncated,
        errors,
    })
}

/// Get a specific section from a document by heading title or anchor slug.
pub fn get_section(
    index: &RepositoryIndex,
    doc_path: &str,
    heading: &str,
    max_chars: usize,
) -> Result<Option<(String, String)>, anyhow::Error> {
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
