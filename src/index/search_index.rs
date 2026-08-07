//! SQLite FTS5-backed full-text search index.
//!
//! [`SqliteSearchIndex`] implements the [`SearchIndex`] trait using SQLite's
//! FTS5 extension with Porter stemming and Unicode61 tokenization.
//!
//! Provides full-text search across document paths, titles, descriptions,
//! headings, and body content with support for path prefix filtering,
//! type/tag filtering, and relevance ranking.
//!
//! Thread Safety: Uses a connection pool (r2d2) for thread-safe access.

use crate::config::Bm25Config;
use crate::index::traits::{Result, SearchFilters, SearchIndex, SearchableDocument};
use crate::model::document::{derive_display_title, IndexStats, SearchResponse, SearchResult};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, params_from_iter, types::Value as SqlValue, Connection, Transaction};
use std::sync::{Arc, Mutex};

const MAX_FUZZY_QUERY_TOKENS: usize = 8;
const MAX_FUZZY_VOCABULARY_TERMS: usize = 50_000;

/// FTS5-based search index with thread-safe connection pool.
///
/// Uses SQLite's FTS5 virtual table for efficient full-text search.
/// The index is updated incrementally during document processing.
pub struct SqliteSearchIndex {
    pool: Arc<Pool<SqliteConnectionManager>>,
    bm25_config: Bm25Config,
    fuzzy_vocabulary: Mutex<Option<Arc<Vec<(String, i64)>>>>,
}

impl SqliteSearchIndex {
    /// Create a new search index with the given connection pool and BM25 configuration.
    #[allow(dead_code)]
    pub fn new(pool: Arc<Pool<SqliteConnectionManager>>, bm25_config: Bm25Config) -> Self {
        Self {
            pool,
            bm25_config,
            fuzzy_vocabulary: Mutex::new(None),
        }
    }

    fn get_conn(&self) -> Result<r2d2::PooledConnection<SqliteConnectionManager>> {
        Ok(self.pool.get()?)
    }

    /// Build the BM25 function call with configured weights.
    fn bm25_expr(&self) -> String {
        format!(
            "bm25(document_search, 0.0, {}, {}, {}, {}, {})",
            self.bm25_config.title_weight,
            self.bm25_config.description_weight,
            self.bm25_config.headings_weight,
            self.bm25_config.body_weight,
            self.bm25_config.concept_type_weight,
        )
    }

    fn invalidate_fuzzy_vocabulary(&self) {
        if let Ok(mut vocabulary) = self.fuzzy_vocabulary.lock() {
            *vocabulary = None;
        }
    }

    fn fuzzy_vocabulary(&self, conn: &Connection) -> Result<Arc<Vec<(String, i64)>>> {
        let mut cache = self
            .fuzzy_vocabulary
            .lock()
            .map_err(|_| anyhow::anyhow!("fuzzy vocabulary cache lock poisoned"))?;
        if let Some(vocabulary) = cache.as_ref() {
            return Ok(vocabulary.clone());
        }
        let mut statement = conn.prepare(
            "SELECT term, doc FROM document_search_vocab \
             ORDER BY doc DESC, term ASC LIMIT ?",
        )?;
        let vocabulary = Arc::new(
            statement
                .query_map(params![MAX_FUZZY_VOCABULARY_TERMS as i64], |row| {
                    Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
                })?
                .collect::<std::result::Result<Vec<_>, _>>()?,
        );
        *cache = Some(vocabulary.clone());
        Ok(vocabulary)
    }
}

impl SearchIndex for SqliteSearchIndex {
    fn init(&self) -> Result<()> {
        let conn = self.get_conn()?;
        conn.execute_batch(
            r#"
            CREATE VIRTUAL TABLE IF NOT EXISTS document_search USING fts5(
                path UNINDEXED,
                title,
                description,
                headings,
                body,
                concept_type,
                tokenize = 'porter unicode61'
            );
            CREATE VIRTUAL TABLE IF NOT EXISTS document_search_vocab
                USING fts5vocab(document_search, 'row');
            "#,
        )?;
        Ok(())
    }

    fn index_document(&self, doc: &SearchableDocument) -> Result<()> {
        self.invalidate_fuzzy_vocabulary();
        let conn = self.get_conn()?;
        conn.execute(
            r#"
            INSERT OR REPLACE INTO document_search (path, title, description, headings, body, concept_type)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            "#,
            params![
                doc.path,
                doc.title.clone().unwrap_or_default(),
                doc.description.clone().unwrap_or_default(),
                doc.headings,
                doc.body,
                doc.concept_type.clone().unwrap_or_default(),
            ],
        )?;
        Ok(())
    }

    fn index_document_tx(&self, tx: &Transaction, doc: &SearchableDocument) -> Result<()> {
        self.invalidate_fuzzy_vocabulary();
        tx.execute(
            r#"
            INSERT OR REPLACE INTO document_search (path, title, description, headings, body, concept_type)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            "#,
            params![
                doc.path,
                doc.title.clone().unwrap_or_default(),
                doc.description.clone().unwrap_or_default(),
                doc.headings,
                doc.body,
                doc.concept_type.clone().unwrap_or_default(),
            ],
        )?;
        Ok(())
    }

    fn remove_document(&self, path: &str) -> Result<()> {
        self.invalidate_fuzzy_vocabulary();
        let conn = self.get_conn()?;
        conn.execute("DELETE FROM document_search WHERE path = ?1", params![path])?;
        Ok(())
    }

    fn remove_document_tx(&self, tx: &Transaction, path: &str) -> Result<()> {
        self.invalidate_fuzzy_vocabulary();
        tx.execute("DELETE FROM document_search WHERE path = ?1", params![path])?;
        Ok(())
    }

    fn search(&self, query: &str, filters: &SearchFilters, limit: usize) -> Result<SearchResponse> {
        let bm25_expr = self.bm25_expr();
        let conn = self.get_conn()?;
        let primary = execute_search(&conn, query, filters, limit, &bm25_expr)?;
        if primary.total_matches > 0 {
            return Ok(primary);
        }

        let vocabulary = self.fuzzy_vocabulary(&conn)?;
        let Some(corrected_query) = corrected_query(query, &vocabulary) else {
            return Ok(primary);
        };

        execute_search(&conn, &corrected_query, filters, limit, &bm25_expr)
    }

    fn stats(&self) -> Result<IndexStats> {
        let conn = self.get_conn()?;
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

fn execute_search(
    conn: &Connection,
    query: &str,
    filters: &SearchFilters,
    limit: usize,
    bm25_expr: &str,
) -> Result<SearchResponse> {
    let from = " FROM document_search ds JOIN documents d ON d.path = ds.path";
    let mut conditions = vec!["document_search MATCH ?".to_string()];
    let mut params = vec![SqlValue::Text(query.to_string())];

    if let Some(prefix) = &filters.path_prefix {
        let prefix = prefix.trim_matches('/');
        if !prefix.is_empty() {
            conditions.push("(d.parent_path = ? OR d.parent_path LIKE (? || '/%'))".to_string());
            params.push(SqlValue::Text(prefix.to_string()));
            params.push(SqlValue::Text(prefix.to_string()));
        }
    }

    if let Some(types) = &filters.concept_types {
        if !types.is_empty() {
            let placeholders = vec!["?"; types.len()];
            conditions.push(format!("d.type IN ({})", placeholders.join(",")));
            params.extend(types.iter().cloned().map(SqlValue::Text));
        }
    }

    if let Some(tags) = &filters.tags {
        let tags = tags
            .iter()
            .filter(|tag| !tag.is_empty())
            .collect::<Vec<_>>();
        if !tags.is_empty() {
            let placeholders = vec!["?"; tags.len()];
            conditions.push(format!(
                "EXISTS (SELECT 1 FROM document_tags dt \
                     WHERE dt.document_id = d.id AND dt.tag IN ({}))",
                placeholders.join(",")
            ));
            params.extend(tags.into_iter().cloned().map(SqlValue::Text));
        }
    }

    let where_clause = format!(" WHERE {}", conditions.join(" AND "));
    let count_sql = format!("SELECT COUNT(*){from}{where_clause}");
    let total_matches: usize = conn
        .query_row(&count_sql, params_from_iter(params.iter()), |row| {
            row.get::<_, i64>(0)
        })?
        .try_into()?;
    if total_matches == 0 {
        return Ok(SearchResponse {
            results: Vec::new(),
            total_matches: 0,
            truncated: false,
        });
    }

    let sql = format!(
        "SELECT ds.path, ds.title, d.type, {bm25_expr}, ds.body \
             {from}{where_clause} ORDER BY {bm25_expr} ASC, d.path ASC LIMIT ?"
    );
    params.push(SqlValue::Integer(limit.try_into()?));
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(params_from_iter(params.iter()), |row| {
        let path: String = row.get(0)?;
        let title: Option<String> = row.get(1)?;
        let ctype: Option<String> = row.get(2)?;
        let rank: f64 = row.get(3)?;
        let body: String = row.get::<_, Option<String>>(4)?.unwrap_or_default();
        Ok((path, title, ctype, rank, body))
    })?;
    let mut results = Vec::new();
    for row in rows {
        let (path, title, concept_type, rank, body) = row?;
        results.push(SearchResult {
            path: path.clone(),
            title: title.clone(),
            display_title: derive_display_title(&path, title.as_deref()),
            concept_type,
            score: -rank,
            matching_section: None,
            excerpt: extract_excerpt(&body, query, 200),
        });
    }

    Ok(SearchResponse {
        truncated: total_matches > results.len(),
        total_matches,
        results,
    })
}

fn corrected_query(query: &str, vocabulary: &[(String, i64)]) -> Option<String> {
    let tokens = query.split_whitespace().collect::<Vec<_>>();
    if tokens.is_empty()
        || tokens.len() > MAX_FUZZY_QUERY_TOKENS
        || tokens
            .iter()
            .any(|token| token.len() < 5 || !token.bytes().all(|byte| byte.is_ascii_alphabetic()))
    {
        return None;
    }

    let mut corrected = Vec::with_capacity(tokens.len());
    for token in tokens {
        let token = token.to_ascii_lowercase();
        let max_distance = if token.len() >= 6 { 2 } else { 1 };
        let first = char::from(token.as_bytes()[0]);
        let min_len = token.len().saturating_sub(max_distance);
        let max_len = token.len() + max_distance;

        let mut best: Option<(usize, i64, String)> = None;
        for (term, document_frequency) in vocabulary {
            if term.len() < min_len || term.len() > max_len || !term.starts_with(first) {
                continue;
            }
            let distance = edit_distance(&token, term);
            if distance > max_distance {
                continue;
            }
            let rank = (distance, -*document_frequency, term.clone());
            if best
                .as_ref()
                .is_none_or(|current| rank < (current.0, -current.1, current.2.clone()))
            {
                best = Some((distance, *document_frequency, term.clone()));
            }
        }
        let (_, _, term) = best?;
        corrected.push(format!("\"{}\"", term.replace('"', "\"\"")));
    }

    Some(corrected.join(" "))
}

fn edit_distance(left: &str, right: &str) -> usize {
    let mut previous = (0..=right.len()).collect::<Vec<_>>();
    let mut current = vec![0; right.len() + 1];
    for (left_index, left_byte) in left.bytes().enumerate() {
        current[0] = left_index + 1;
        for (right_index, right_byte) in right.bytes().enumerate() {
            current[right_index + 1] = (previous[right_index + 1] + 1)
                .min(current[right_index] + 1)
                .min(previous[right_index] + usize::from(left_byte != right_byte));
        }
        std::mem::swap(&mut previous, &mut current);
    }
    previous[right.len()]
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

#[cfg(test)]
mod tests {
    use super::{corrected_query, edit_distance};

    #[test]
    fn edit_distance_handles_insertions_deletions_and_substitutions() {
        assert_eq!(edit_distance("revenue", "revenue"), 0);
        assert_eq!(edit_distance("reveneu", "revenu"), 1);
        assert_eq!(edit_distance("montly", "monthli"), 2);
    }

    #[test]
    fn correction_is_bounded_and_deterministic() {
        let vocabulary = vec![
            ("monthli".to_string(), 4),
            ("revenu".to_string(), 7),
            ("retain".to_string(), 10),
        ];
        assert_eq!(
            corrected_query("montly reveneu", &vocabulary).as_deref(),
            Some("\"monthli\" \"revenu\"")
        );
        assert_eq!(corrected_query("quantum entanglement", &vocabulary), None);
        assert_eq!(corrected_query("tiny typo", &vocabulary), None);
    }
}
