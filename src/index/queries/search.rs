//! Search query operations.

use crate::index::database::RepositoryIndex;
use crate::model::document::{SearchResponse, SearchResult};
use rusqlite::params;

/// Full-text search across indexed documents.
///
/// - `query`: Search query string (FTS5 syntax supported)
/// - `path_prefix`: Optional path prefix to restrict search scope
/// - `types`: Optional concept types to filter by
/// - `tags`: Optional tags to filter by
/// - `limit`: Maximum results to return
pub fn search(
    index: &RepositoryIndex,
    query: &str,
    path_prefix: Option<&str>,
    types: Option<&[String]>,
    tags: Option<&[String]>,
    limit: usize,
) -> Result<SearchResponse, anyhow::Error> {
    let conn = index.pool().get()?;

    let sql = String::from(
        "SELECT ds.path, ds.title, d.type, d.description, ds.rank,
                ds.body, COUNT(*) OVER() as total_count
         FROM document_search ds
         JOIN documents d ON d.path = ds.path
         WHERE document_search MATCH ?1",
    );

    let mut conditions = vec![];
    let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = vec![];
    param_values.push(Box::new(query.to_string()));

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

    let mut stmt = conn.prepare(&full_sql)?;
    let rows = stmt.query_map(param_vec.as_slice(), |row| {
        let path: String = row.get(0)?;
        let title: Option<String> = row.get(1)?;
        let ctype: Option<String> = row.get(2)?;
        let description: Option<String> = row.get(3)?;
        let rank: f64 = row.get(4)?;
        let body: String = row.get::<_, Option<String>>(5)?.unwrap_or_default();
        let total: i64 = row.get(6)?;
        Ok((path, title, ctype, description, rank, body, total as usize))
    })?;

    let mut total_matches = 0usize;
    let mut results = Vec::new();
    for row in rows {
        let (path, title, ctype, _description, rank, body, total) = row?;
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
