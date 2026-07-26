//! Query and search operations for RepositoryIndex.
//!
//! Public API: browse_directory, get_document, get_section, search,
//! query_metadata, get_recently_modified, get_stats.

use std::collections::HashMap;

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;

use super::database::RepositoryIndex;
use crate::index::traits::DocumentStore;
use crate::model::directory::{BrowseResponse, DirectoryDocument};
use crate::model::document::{
    DocumentDetail, DocumentMetadata, DocumentSummary, HeadingInfo, IndexStats,
    MetadataQueryResponse, ParseError, SearchResponse, SearchResult,
};
use crate::parser::markdown::MarkdownParser;

impl RepositoryIndex {
    /// Browse a directory in the knowledge base.
    ///
    /// Returns subdirectories and documents at the given path with optional
    /// index document summary. Depth controls recursion into subdirectories.
    pub fn browse_directory(
        &self,
        path: &str,
        depth: usize,
        limit: usize,
    ) -> Result<BrowseResponse, anyhow::Error> {
        let prefix = if path.is_empty() || path == "/" {
            String::new()
        } else {
            format!("{}/", path.trim_start_matches('/'))
        };

        let subdirs = if depth > 0 {
            let conn = self.pool().get()?;
            let mut stmt = conn.prepare(
                "SELECT DISTINCT parent_path FROM documents
                 WHERE parent_path LIKE ?1 AND parent_path != ?2
                 ORDER BY parent_path LIMIT ?3",
            )?;
            let pattern = format!("{}%", prefix);
            let dirs: Vec<String> = stmt
                .query_map(params![pattern, prefix, limit as i64], |row| {
                    row.get::<_, String>(0)
                })?
                .filter_map(|r| r.ok())
                .filter(|p| {
                    let remaining = p.strip_prefix(&prefix).unwrap_or(p);
                    remaining.split('/').count() <= depth + 1 && !remaining.is_empty()
                })
                .collect();
            dirs
        } else {
            vec![]
        };

        let conn = self.pool().get()?;
        let mut stmt = conn.prepare(
            "SELECT path, title, type, description FROM documents
             WHERE parent_path = ?1 ORDER BY path LIMIT ?2",
        )?;
        let docs: Vec<DirectoryDocument> = stmt
            .query_map(params![prefix.trim_end_matches('/'), limit as i64], |row| {
                Ok(DirectoryDocument {
                    path: row.get(0)?,
                    title: row.get(1)?,
                    concept_type: row.get(2)?,
                    description: row.get(3)?,
                })
            })?
            .filter_map(|r| r.ok())
            .collect();

        let truncated = subdirs.len() + docs.len() > limit;

        let index_path = if prefix.is_empty() {
            None
        } else {
            Some(format!("{}index.md", prefix))
        };

        Ok(BrowseResponse {
            path: path.to_string(),
            summary_document: index_path,
            directories: subdirs,
            documents: docs,
            truncated,
        })
    }

    /// Get a document by path with optional section inclusion and truncation.
    ///
    /// - `include`: Section names to include (empty = all)
    /// - `max_body_chars`: Maximum characters for body content
    pub fn get_document(
        &self,
        doc_path: &str,
        include: &[String],
        max_body_chars: usize,
    ) -> Result<DocumentDetail, anyhow::Error> {
        let conn = self.pool().get()?;
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
            let conn = self.pool().get()?;
            let mut tag_stmt =
                conn.prepare("SELECT tag FROM document_tags WHERE document_id = ?1")?;
            tags = tag_stmt
                .query_map(params![id], |row| row.get::<_, String>(0))?
                .filter_map(|r| r.ok())
                .collect();
        }

        let mut custom = std::collections::BTreeMap::new();
        if include.contains(&"metadata".to_string()) {
            let conn = self.pool().get()?;
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
            let conn = self.pool().get()?;
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
            let conn = self.pool().get()?;
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
    ///
    /// Returns `(heading_title, section_content)` if found, truncated to `max_chars`.
    pub fn get_section(
        &self,
        doc_path: &str,
        heading: &str,
        max_chars: usize,
    ) -> Result<Option<(String, String)>, anyhow::Error> {
        let conn = self.pool().get()?;
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

        let (_, _, _, sections, _tables, _code_blocks) = MarkdownParser::parse(&body_text);

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

    /// Full-text search across indexed documents.
    ///
    /// - `query`: Search query string (FTS5 syntax supported)
    /// - `path_prefix`: Optional path prefix to restrict search scope
    /// - `types`: Optional concept types to filter by
    /// - `tags`: Optional tags to filter by
    /// - `limit`: Maximum results to return
    pub fn search(
        &self,
        query: &str,
        path_prefix: Option<&str>,
        types: Option<&[String]>,
        tags: Option<&[String]>,
        limit: usize,
    ) -> Result<SearchResponse, anyhow::Error> {
        let escaped = query.replace('\'', "''");

        let sql = "
            SELECT ds.path, ds.title, d.type, ds.rank, ds.body,
                   COUNT(*) OVER() as total_count
            FROM document_search ds
            JOIN documents d ON d.path = ds.path
            WHERE document_search MATCH ?1
        ";

        let mut conditions = vec![];
        let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = vec![];
        param_values.push(Box::new(escaped.clone()));

        if let Some(prefix) = path_prefix {
            let p = if prefix.is_empty() || prefix == "/" {
                String::new()
            } else {
                prefix.trim_start_matches('/').to_string()
            };
            if !p.is_empty() {
                let param_idx = param_values.len();
                param_values.push(Box::new(p.clone()));
                param_values.push(Box::new(format!("{}%", p)));
                conditions.push(format!(
                    "(d.parent_path = ?{} OR d.parent_path LIKE ?{})",
                    param_idx + 1,
                    param_idx + 2
                ));
            }
        }

        if let Some(types) = types {
            if !types.is_empty() {
                let placeholders: Vec<String> = (0..types.len())
                    .map(|i| format!("?{}", param_values.len() + 1 + i))
                    .collect();
                conditions.push(format!("d.type IN ({})", placeholders.join(",")));
                for t in types {
                    param_values.push(Box::new(t.clone()));
                }
            }
        }

        if let Some(tags) = tags {
            if let Some(first) = tags.first() {
                if !first.is_empty() || tags.len() > 1 {
                    let placeholders: Vec<String> = (0..tags.len())
                        .map(|i| format!("?{}", param_values.len() + 1 + i))
                        .collect();
                    conditions.push(format!(
                        "d.id IN (SELECT document_id FROM document_tags WHERE tag IN ({}))",
                        placeholders.join(",")
                    ));
                    for t in tags {
                        param_values.push(Box::new(t.clone()));
                    }
                }
            }
        }

        let params_refs: Vec<&dyn rusqlite::types::ToSql> =
            param_values.iter().map(|p| p.as_ref()).collect();

        let full_sql = if conditions.is_empty() {
            format!("{} ORDER BY rank LIMIT ?{}", sql, param_values.len() + 1)
        } else {
            format!(
                "{} AND {} ORDER BY rank LIMIT ?{}",
                sql,
                conditions.join(" AND "),
                param_values.len() + 1
            )
        };

        let limit_val = limit as i64;
        let mut param_vec = params_refs.clone();
        param_vec.push(&limit_val);

        let conn = self.pool().get()?;
        let mut stmt = conn.prepare(&full_sql)?;
        let rows = stmt.query_map(param_vec.as_slice(), |row| {
            let path: String = row.get(0)?;
            let title: Option<String> = row.get(1)?;
            let ctype: Option<String> = row.get(2)?;
            let rank: f64 = row.get(3)?;
            let body: String = row.get::<_, Option<String>>(4)?.unwrap_or_default();
            let total: i64 = row.get(5)?;
            Ok((path, title, ctype, rank, body, total as usize))
        })?;

        let mut total_matches = 0usize;
        let mut results = Vec::new();
        for row in rows {
            let (path, title, ctype, rank, body, total) = row?;
            total_matches = total;
            let excerpt = extract_excerpt(&body, query, 200);
            results.push(SearchResult {
                path,
                title,
                concept_type: ctype,
                score: -rank,
                matching_section: None,
                excerpt,
            });
        }

        let truncated = total_matches > results.len();
        Ok(SearchResponse {
            results,
            total_matches,
            truncated,
        })
    }

    /// Structured metadata query with filtering and projection.
    ///
    /// - `filters`: Key-value pairs to match against front-matter fields
    /// - `select_fields`: Fields to include in results (empty = all)
    /// - `limit`: Maximum rows to return
    pub fn query_metadata(
        &self,
        filters: &HashMap<String, String>,
        select_fields: &[String],
        limit: usize,
    ) -> Result<MetadataQueryResponse, anyhow::Error> {
        let base_select = if select_fields.is_empty() {
            vec!["path".to_string()]
        } else {
            select_fields.to_vec()
        };

        let mut select_cols = Vec::new();
        let mut custom_field_joins = Vec::new();
        let mut custom_field_count = 0;

        // Valid column names for the documents table to prevent SQL injection
        const VALID_DOCUMENT_COLUMNS: &[&str] = &[
            "path",
            "title",
            "type",
            "description",
            "file_size",
            "modified_at",
            "content_hash",
            "parse_status",
            "parent_path",
            "id",
        ];

        for field in &base_select {
            match field.as_str() {
                "path" => select_cols.push("d.path".to_string()),
                "title" => select_cols.push("d.title".to_string()),
                "type" => select_cols.push("d.type".to_string()),
                "description" => select_cols.push("d.description".to_string()),
                "owner" => {
                    let alias = format!("mf_{}", custom_field_count);
                    select_cols.push(format!("{}.value", alias));
                    custom_field_joins.push(("owner".to_string(), alias));
                    custom_field_count += 1;
                }
                "status" => {
                    let alias = format!("mf_{}", custom_field_count);
                    select_cols.push(format!("{}.value", alias));
                    custom_field_joins.push(("status".to_string(), alias));
                    custom_field_count += 1;
                }
                field if VALID_DOCUMENT_COLUMNS.contains(&field) => {
                    select_cols.push(format!("d.{}", field));
                }
                _ => {
                    return Err(anyhow::anyhow!(
                        "Invalid select field: '{}'. Valid fields: {}",
                        field,
                        VALID_DOCUMENT_COLUMNS.join(", ")
                    ));
                }
            }
        }

        let mut sql = format!("SELECT {} FROM documents d", select_cols.join(", "));

        for (key, alias) in &custom_field_joins {
            // key is validated to be "owner" or "status" above, so safe to interpolate
            sql.push_str(&format!(
                " LEFT JOIN metadata_fields {} ON {}.document_id = d.id AND {}.key = '{}'",
                alias, alias, alias, key
            ));
        }

        let mut conditions = Vec::new();
        let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

        for (key, value) in filters {
            match key.as_str() {
                "type" => {
                    conditions.push(format!("d.type = ?{}", param_values.len() + 1));
                    param_values.push(Box::new(value.clone()));
                }
                "tags_contains" => {
                    sql.push_str(" JOIN document_tags dt ON dt.document_id = d.id");
                    conditions.push(format!("dt.tag = ?{}", param_values.len() + 1));
                    param_values.push(Box::new(value.clone()));
                }
                "title" => {
                    conditions.push(format!("d.title = ?{}", param_values.len() + 1));
                    param_values.push(Box::new(value.clone()));
                }
                "parse_status" => {
                    conditions.push(format!("d.parse_status = ?{}", param_values.len() + 1));
                    param_values.push(Box::new(value.clone()));
                }
                _ => {
                    let alias = format!("mf_{}", custom_field_count);
                    sql.push_str(&format!(
                        " LEFT JOIN metadata_fields {} ON {}.document_id = d.id AND {}.key = ?{}",
                        alias,
                        alias,
                        alias,
                        param_values.len() + 1
                    ));
                    param_values.push(Box::new(key.clone()));

                    conditions.push(format!("{}.value = ?{}", alias, param_values.len() + 1));
                    param_values.push(Box::new(value.clone()));
                    custom_field_count += 1;
                }
            }
        }

        if !conditions.is_empty() {
            sql.push_str(&format!(" WHERE {}", conditions.join(" AND ")));
        }

        if filters.contains_key("tags_contains") {
            sql.push_str(" GROUP BY d.id");
        }

        sql.push_str(&format!(" LIMIT ?{}", param_values.len() + 1));
        param_values.push(Box::new(limit as i64));

        let params_refs: Vec<&dyn rusqlite::types::ToSql> =
            param_values.iter().map(|p| p.as_ref()).collect();

        let conn = self.pool().get()?;
        let mut stmt = conn.prepare(&sql)?;
        let mut results = Vec::new();

        let rows = stmt.query_map(params_refs.as_slice(), |row| {
            let mut map = serde_json::Map::new();
            for (i, field) in base_select.iter().enumerate() {
                let val: Result<String, _> = row.get(i);
                if let Ok(v) = val {
                    map.insert(field.clone(), serde_json::Value::String(v));
                }
            }
            Ok(serde_json::Value::Object(map))
        })?;

        for row in rows {
            results.push(row?);
        }

        let total_matches = results.len();
        let truncated = total_matches > limit;

        Ok(MetadataQueryResponse {
            results: results.into_iter().take(limit).collect(),
            total_matches,
            truncated,
        })
    }

    /// Get recently modified documents.
    ///
    /// Returns lightweight summaries sorted by modification time (newest first).
    #[allow(dead_code)]
    pub fn get_recently_modified(
        &self,
        limit: usize,
    ) -> Result<Vec<DocumentSummary>, anyhow::Error> {
        let conn = self.pool().get()?;
        let mut stmt = conn.prepare(
            "SELECT path, title, type, description FROM documents
             ORDER BY modified_at DESC LIMIT ?1",
        )?;

        let docs = stmt
            .query_map(params![limit as i64], |row| {
                Ok(DocumentSummary {
                    path: row.get(0)?,
                    title: row.get(1)?,
                    concept_type: row.get(2)?,
                    description: row.get(3)?,
                    tags: vec![],
                })
            })?
            .filter_map(|r| r.ok())
            .collect();

        Ok(docs)
    }

    /// Get index statistics (document count, link count, etc.).
    pub fn get_stats(&self) -> Result<IndexStats, anyhow::Error> {
        self.document_store.get_stats()
    }
}

fn extract_excerpt(body: &str, query: &str, context_chars: usize) -> String {
    let body_lower = body.to_lowercase();
    let query_lower = query.to_lowercase();

    if let Some(pos) = body_lower.find(&query_lower) {
        let start = pos.saturating_sub(context_chars / 2);
        let end = (pos + query_lower.len() + context_chars / 2).min(body.len());

        let excerpt: String = body[start..end].chars().collect();
        if start > 0 {
            format!("...{}...", excerpt)
        } else {
            format!("{}...", excerpt)
        }
    } else {
        let preview: String = body.chars().take(context_chars).collect();
        format!("{}...", preview)
    }
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
