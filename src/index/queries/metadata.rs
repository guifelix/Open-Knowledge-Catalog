//! Metadata query operations.

use crate::index::database::RepositoryIndex;
use crate::model::document::MetadataQueryResponse;

/// Valid column names for the documents table to prevent SQL injection.
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

/// Default select fields when none are specified.
const DEFAULT_SELECT: &[&str] = &[
    "path",
    "title",
    "type",
    "description",
    "file_size",
    "modified_at",
    "parse_status",
];

/// Structured metadata query with filtering and projection.
///
/// - `filters`: Key-value pairs to match against front-matter fields
/// - `select`: Fields to return (empty = all default fields)
/// - `limit`: Maximum rows to return
pub fn query_metadata(
    index: &RepositoryIndex,
    filters: &std::collections::HashMap<String, serde_json::Value>,
    select: &[String],
    limit: usize,
) -> Result<MetadataQueryResponse, anyhow::Error> {
    let conn = index.pool().get()?;

    // Determine which fields to select
    let fields: Vec<String> = if select.is_empty() {
        DEFAULT_SELECT.iter().map(|s| s.to_string()).collect()
    } else {
        select.to_vec()
    };

    // Build SELECT columns with SQL-injection-safe validation
    let mut select_cols: Vec<String> = Vec::new();
    let mut custom_field_joins: Vec<(String, String)> = Vec::new();
    let mut custom_field_count = 0usize;

    for field in &fields {
        match field.as_str() {
            "owner" => {
                let alias = format!("mf_{}", custom_field_count);
                select_cols.push(format!("{}.value AS {}", alias, field));
                custom_field_joins.push(("owner".to_string(), alias));
                custom_field_count += 1;
            }
            "status" => {
                let alias = format!("mf_{}", custom_field_count);
                select_cols.push(format!("{}.value AS {}", alias, field));
                custom_field_joins.push(("status".to_string(), alias));
                custom_field_count += 1;
            }
            f if VALID_DOCUMENT_COLUMNS.contains(&f) => {
                select_cols.push(format!("d.{}", f));
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

    // Add custom field joins
    for (key, alias) in &custom_field_joins {
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
                let alias = format!("mf_{}", param_values.len());
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

    if filters.contains_key("tags_contains") {
        sql.push_str(" GROUP BY d.id");
    }

    sql.push_str(&format!(" LIMIT ?{}", param_values.len() + 1));
    param_values.push(Box::new(limit as i64));

    let params_refs: Vec<&dyn rusqlite::types::ToSql> =
        param_values.iter().map(|p| p.as_ref()).collect();

    let mut stmt = conn.prepare(&sql)?;
    let mut results = Vec::new();

    let rows = stmt.query_map(params_refs.as_slice(), |row| {
        let mut map = serde_json::Map::new();
        for (i, field) in fields.iter().enumerate() {
            match field.as_str() {
                "path" | "parse_status" => {
                    map.insert(
                        field.clone(),
                        serde_json::Value::String(row.get::<_, String>(i)?),
                    );
                }
                "title" | "type" | "description" | "content_hash" | "parent_path" => {
                    map.insert(
                        field.clone(),
                        serde_json::Value::String(
                            row.get::<_, Option<String>>(i)?.unwrap_or_default(),
                        ),
                    );
                }
                "file_size" => {
                    map.insert(
                        field.clone(),
                        serde_json::Value::Number(serde_json::Number::from(row.get::<_, i64>(i)?)),
                    );
                }
                "modified_at" => {
                    map.insert(
                        field.clone(),
                        serde_json::Value::Number(serde_json::Number::from(row.get::<_, i64>(i)?)),
                    );
                }
                "id" => {
                    map.insert(
                        field.clone(),
                        serde_json::Value::Number(serde_json::Number::from(row.get::<_, i64>(i)?)),
                    );
                }
                // Custom metadata fields (owner, status) — nullable text
                _ => {
                    if let Ok(v) = row.get::<_, Option<String>>(i) {
                        map.insert(
                            field.clone(),
                            serde_json::Value::String(v.unwrap_or_default()),
                        );
                    }
                }
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
