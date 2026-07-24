use crate::index::traits::{Result, SearchFilters, SearchIndex, SearchableDocument};
use crate::model::document::{IndexStats, SearchResponse, SearchResult};
use rusqlite::{params, Connection};
use std::sync::{Mutex, PoisonError};

pub struct SqliteSearchIndex {
    conn: Mutex<Connection>,
}

impl SqliteSearchIndex {
    #[allow(dead_code)]
    pub fn new(conn: Connection) -> Self {
        Self {
            conn: Mutex::new(conn),
        }
    }
}

impl SearchIndex for SqliteSearchIndex {
    fn init(&self) -> Result<()> {
        let conn = self
            .conn
            .lock()
            .map_err(|e: PoisonError<_>| anyhow::anyhow!("Mutex poisoned: {}", e))?;
        conn.execute_batch(
            r#"
            CREATE VIRTUAL TABLE IF NOT EXISTS document_search USING fts5(
                path UNINDEXED,
                title,
                description,
                headings,
                body,
                tokenize = 'porter unicode61'
            );
            "#,
        )?;
        Ok(())
    }

    fn index_document(&self, doc: &SearchableDocument) -> Result<()> {
        let conn = self
            .conn
            .lock()
            .map_err(|e: PoisonError<_>| anyhow::anyhow!("Mutex poisoned: {}", e))?;
        conn.execute(
            r#"
            INSERT OR REPLACE INTO document_search (path, title, description, headings, body)
            VALUES (?1, ?2, ?3, ?4, ?5)
            "#,
            params![
                doc.path,
                doc.title.clone().unwrap_or_default(),
                doc.description.clone().unwrap_or_default(),
                doc.headings,
                doc.body,
            ],
        )?;
        Ok(())
    }

    fn remove_document(&self, path: &str) -> Result<()> {
        let conn = self
            .conn
            .lock()
            .map_err(|e: PoisonError<_>| anyhow::anyhow!("Mutex poisoned: {}", e))?;
        conn.execute("DELETE FROM document_search WHERE path = ?1", params![path])?;
        Ok(())
    }

    fn search(&self, query: &str, filters: &SearchFilters, limit: usize) -> Result<SearchResponse> {
        let escaped = query.replace('\'', "''");

        let mut sql = String::from(
            "SELECT ds.path, ds.title, d.type, ds.rank, ds.body
             FROM document_search ds
             JOIN documents d ON d.path = ds.path
             WHERE document_search MATCH ?1",
        );

        let mut conditions = Vec::new();
        let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> =
            vec![Box::new(escaped.clone())];

        if let Some(prefix) = &filters.path_prefix {
            if !prefix.is_empty() {
                let p = prefix.trim_start_matches('/').to_string();
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
        }

        if let Some(types) = &filters.concept_types {
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

        if let Some(tags) = &filters.tags {
            if !tags.is_empty() {
                sql.push_str(" JOIN document_tags dt ON dt.document_id = d.id");
                let placeholders: Vec<String> = (0..tags.len())
                    .map(|i| format!("?{}", param_values.len() + 1 + i))
                    .collect();
                conditions.push(format!("dt.tag IN ({})", placeholders.join(",")));
                for t in tags {
                    param_values.push(Box::new(t.clone()));
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

        let conn = self
            .conn
            .lock()
            .map_err(|e: PoisonError<_>| anyhow::anyhow!("Mutex poisoned: {}", e))?;
        let mut stmt = conn.prepare(&full_sql)?;
        let results: Vec<SearchResult> = stmt
            .query_map(param_vec.as_slice(), |row| {
                let path: String = row.get(0)?;
                let title: Option<String> = row.get(1)?;
                let ctype: Option<String> = row.get(2)?;
                let rank: f64 = row.get(3)?;
                let body: String = row.get::<_, Option<String>>(4)?.unwrap_or_default();
                Ok((path, title, ctype, rank, body))
            })?
            .filter_map(|r| r.ok())
            .map(|(path, title, ctype, rank, body)| {
                let excerpt = extract_excerpt(&body, query, 200);
                SearchResult {
                    path,
                    title,
                    concept_type: ctype,
                    score: -rank,
                    matching_section: None,
                    excerpt,
                }
            })
            .collect();

        let total = results.len();
        let truncated = total > limit;

        Ok(SearchResponse {
            results: results.into_iter().take(limit).collect(),
            total_matches: total,
            truncated,
        })
    }

    fn stats(&self) -> Result<IndexStats> {
        let conn = self
            .conn
            .lock()
            .map_err(|e: PoisonError<_>| anyhow::anyhow!("Mutex poisoned: {}", e))?;
        let doc_count: i64 =
            conn.query_row("SELECT COUNT(*) FROM documents", [], |row| row.get(0))?;
        let error_count: i64 =
            conn.query_row("SELECT COUNT(*) FROM scan_errors", [], |row| row.get(0))?;
        let link_count: i64 = conn.query_row("SELECT COUNT(*) FROM links", [], |row| row.get(0))?;
        let heading_count: i64 =
            conn.query_row("SELECT COUNT(*) FROM headings", [], |row| row.get(0))?;

        Ok(IndexStats {
            document_count: doc_count as usize,
            error_count: error_count as usize,
            link_count: link_count as usize,
            heading_count: heading_count as usize,
        })
    }
}

fn extract_excerpt(body: &str, query: &str, max_len: usize) -> String {
    let query_lower = query.to_lowercase();
    let body_lower = body.to_lowercase();

    if let Some(pos) = body_lower.find(&query_lower) {
        let start = pos.saturating_sub(50);
        let end = (pos + query.len() + 150).min(body.len());
        let excerpt = &body[start..end];
        if excerpt.len() > max_len {
            format!("...{}", &excerpt[excerpt.len() - max_len..])
        } else {
            excerpt.to_string()
        }
    } else {
        body.chars().take(max_len).collect()
    }
}
