//! Metadata query operations.

use crate::index::database::RepositoryIndex;
use crate::model::document::MetadataQueryResponse;
use rusqlite::params;

/// Structured metadata query with filtering and projection.
///
/// - `filters`: Key-value pairs to match against front-matter fields
/// - `limit`: Maximum rows to return
pub fn query_metadata(
    index: &RepositoryIndex,
    filters: &std::collections::HashMap<String, serde_json::Value>,
    limit: usize,
) -> Result<MetadataQueryResponse, anyhow::Error> {
    let conn = index.pool().get()?;

    let mut sql = String::from("SELECT d.path, d.title, d.type, d.description, d.file_size, d.modified_at, d.parse_status FROM documents d");
    let mut conditions = Vec::new();
    let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

    for (key, value) in filters {
        match key.as_str() {
            "type" => {
                conditions.push(format!("d.type = ?{}", param_values.len() + 1));
                param_values.push(Box::new(value.as_str().unwrap_or("").to_string()));
            }
            "tags_contains" => {
                sql.push_str(" JOIN document_tags dt ON dt.document_id = d.id");
                conditions.push(format!("dt.tag = ?{}", param_values.len() + 1));
                param_values.push(Box::new(value.as_str().unwrap_or("").to_string()));
            }
            "title" => {
                conditions.push(format!("d.title = ?{}", param_values.len() + 1));
                param_values.push(Box::new(value.as_str().unwrap_or("").to_string()));
            }
            "parse_status" => {
                conditions.push(format!("d.parse_status = ?{}", param_values.len() + 1));
                param_values.push(Box::new(value.as_str().unwrap_or("").to_string()));
            }
            _ => {
                // Custom metadata field
                let alias = format!("mf_{}", conditions.len());
                sql.push_str(&format!(
                    " LEFT JOIN metadata_fields {} ON {}.document_id = d.id AND {}.key = ?{}",
                    alias,
                    alias,
                    alias,
                    param_values.len() + 1
                ));
                param_values.push(Box::new(key.clone()));
                conditions.push(format!("{}.value = ?{}", alias, param_values.len() + 1));
                param_values.push(Box::new(value.as_str().unwrap_or("").to_string()));
            }
        }
    }

    if !conditions.is_empty() {
        sql.push_str(&format!(" WHERE {}", conditions.join(" AND ")));
    }

    sql.push_str(&format!(" LIMIT ?{}", param_values.len() + 1));
    param_values.push(Box::new(limit as i64));

    let params_refs: Vec<&dyn rusqlite::types::ToSql> =
        param_values.iter().map(|p| p.as_ref()).collect();

    let mut stmt = conn.prepare(&sql)?;
    let mut results = Vec::new();

    let rows = stmt.query_map(params_refs.as_slice(), |row| {
        let mut map = serde_json::Map::new();
        map.insert("path".to_string(), serde_json::Value::String(row.get(0)?));
        map.insert(
            "title".to_string(),
            serde_json::Value::String(row.get::<_, Option<String>>(1)?.unwrap_or_default()),
        );
        map.insert(
            "type".to_string(),
            serde_json::Value::String(row.get::<_, Option<String>>(2)?.unwrap_or_default()),
        );
        map.insert(
            "description".to_string(),
            serde_json::Value::String(row.get::<_, Option<String>>(3)?.unwrap_or_default()),
        );
        map.insert(
            "file_size".to_string(),
            serde_json::Value::Number(serde_json::Number::from(row.get::<_, i64>(4)?)),
        );
        map.insert(
            "modified_at".to_string(),
            serde_json::Value::Number(serde_json::Number::from(row.get::<_, i64>(5)?)),
        );
        map.insert(
            "parse_status".to_string(),
            serde_json::Value::String(row.get(6)?),
        );
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
