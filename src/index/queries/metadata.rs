//! Metadata query operations.

use std::collections::HashMap;

use anyhow::{anyhow, Context};
use rusqlite::{params_from_iter, types::Value as SqlValue};
use serde_json::{Map, Value};

use crate::index::database::RepositoryIndex;
use crate::model::document::MetadataQueryResponse;

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

const DEFAULT_SELECT: &[&str] = &[
    "path",
    "title",
    "type",
    "description",
    "file_size",
    "modified_at",
    "parse_status",
];

/// Structured metadata query with exact-match filtering and field projection.
pub fn query_metadata(
    index: &RepositoryIndex,
    filters: &HashMap<String, Value>,
    select: &[String],
    limit: usize,
) -> Result<MetadataQueryResponse, anyhow::Error> {
    validate_filters(filters)?;

    let fields = selected_fields(select)?;
    let (select_clause, select_params) = build_select(&fields);
    let (where_clause, filter_params) = build_filters(filters)?;
    let conn = index.pool().get()?;

    let count_sql = format!("SELECT COUNT(*) FROM documents d{where_clause}");
    let total_matches: usize = conn
        .query_row(&count_sql, params_from_iter(filter_params.iter()), |row| {
            row.get::<_, i64>(0)
        })?
        .try_into()
        .context("metadata match count does not fit in usize")?;

    let sql = format!(
        "SELECT {select_clause} FROM documents d{where_clause} \
         ORDER BY d.path ASC, d.id ASC LIMIT ?"
    );
    let mut query_params = select_params;
    query_params.extend(filter_params);
    query_params.push(SqlValue::Integer(
        limit
            .try_into()
            .context("metadata query limit exceeds SQLite integer range")?,
    ));

    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(params_from_iter(query_params.iter()), |row| {
        project_row(row, &fields)
    })?;
    let results = rows.collect::<Result<Vec<_>, _>>()?;

    Ok(MetadataQueryResponse {
        truncated: total_matches > results.len(),
        total_matches,
        results,
    })
}

fn selected_fields(select: &[String]) -> Result<Vec<String>, anyhow::Error> {
    let fields = if select.is_empty() {
        DEFAULT_SELECT
            .iter()
            .map(|field| (*field).to_string())
            .collect()
    } else {
        select.to_vec()
    };

    for field in &fields {
        validate_name(field, "select field")?;
    }
    Ok(fields)
}

fn validate_filters(filters: &HashMap<String, Value>) -> Result<(), anyhow::Error> {
    for (key, value) in filters {
        validate_name(key, "filter key")?;
        if key.ends_with("_contains") && key != "tags_contains" {
            return Err(anyhow!(
                "Invalid filter operator in '{key}': only tags_contains is supported"
            ));
        }
        if !value.is_string() {
            return Err(anyhow!(
                "Invalid filter value for '{key}': expected a string"
            ));
        }
    }
    Ok(())
}

fn validate_name(name: &str, kind: &str) -> Result<(), anyhow::Error> {
    let valid = !name.is_empty()
        && name.len() <= 128
        && name
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || "_-.".contains(character));
    if valid {
        Ok(())
    } else {
        Err(anyhow!(
            "Invalid {kind}: '{name}'. Use letters, numbers, '_', '-', or '.'"
        ))
    }
}

fn build_select(fields: &[String]) -> (String, Vec<SqlValue>) {
    let mut columns = Vec::with_capacity(fields.len());
    let mut params = Vec::new();

    for field in fields {
        if VALID_DOCUMENT_COLUMNS.contains(&field.as_str()) {
            columns.push(format!("d.{field}"));
        } else if field == "tags" {
            columns.push(
                "COALESCE((SELECT json_group_array(tag) FROM \
                 (SELECT tag FROM document_tags WHERE document_id = d.id ORDER BY tag)), '[]')"
                    .to_string(),
            );
        } else {
            columns.push(
                "(SELECT value FROM metadata_fields \
                 WHERE document_id = d.id AND key = ? LIMIT 1)"
                    .to_string(),
            );
            params.push(SqlValue::Text(field.clone()));
        }
    }

    (columns.join(", "), params)
}

fn build_filters(
    filters: &HashMap<String, Value>,
) -> Result<(String, Vec<SqlValue>), anyhow::Error> {
    let mut entries = filters.iter().collect::<Vec<_>>();
    entries.sort_by_key(|(key, _)| key.as_str());

    let mut conditions = Vec::with_capacity(entries.len());
    let mut params = Vec::new();
    for (key, value) in entries {
        let value = value
            .as_str()
            .ok_or_else(|| anyhow!("Invalid filter value for '{key}': expected a string"))?;
        match key.as_str() {
            "type" | "title" | "parse_status" => {
                conditions.push(format!("d.{key} = ?"));
                params.push(SqlValue::Text(value.to_string()));
            }
            "path_prefix" => {
                conditions.push("d.path LIKE (? || '%') ESCAPE '\\'".to_string());
                params.push(SqlValue::Text(escape_like(value)));
            }
            "tags_contains" => {
                conditions.push(
                    "EXISTS (SELECT 1 FROM document_tags dt \
                     WHERE dt.document_id = d.id AND dt.tag = ?)"
                        .to_string(),
                );
                params.push(SqlValue::Text(value.to_string()));
            }
            _ => {
                conditions.push(
                    "EXISTS (SELECT 1 FROM metadata_fields mf \
                     WHERE mf.document_id = d.id AND mf.key = ? AND mf.value = ?)"
                        .to_string(),
                );
                params.push(SqlValue::Text(key.clone()));
                params.push(SqlValue::Text(serde_json::to_string(value)?));
            }
        }
    }

    let clause = if conditions.is_empty() {
        String::new()
    } else {
        format!(" WHERE {}", conditions.join(" AND "))
    };
    Ok((clause, params))
}

fn escape_like(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('%', "\\%")
        .replace('_', "\\_")
}

fn project_row(row: &rusqlite::Row<'_>, fields: &[String]) -> rusqlite::Result<Value> {
    let mut result = Map::new();
    for (index, field) in fields.iter().enumerate() {
        let value = match field.as_str() {
            "path" | "parse_status" => Value::String(row.get(index)?),
            "title" | "type" | "description" | "content_hash" | "parent_path" => row
                .get::<_, Option<String>>(index)?
                .map_or(Value::Null, Value::String),
            "file_size" | "modified_at" | "id" => Value::Number(row.get::<_, i64>(index)?.into()),
            "tags" => row
                .get::<_, String>(index)
                .ok()
                .and_then(|json| serde_json::from_str(&json).ok())
                .unwrap_or_else(|| Value::Array(Vec::new())),
            _ => row
                .get::<_, Option<String>>(index)?
                .map_or(Value::Null, |json| {
                    serde_json::from_str(&json).unwrap_or(Value::String(json))
                }),
        };
        result.insert(field.clone(), value);
    }
    Ok(Value::Object(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_filter_names_and_operators() {
        assert!(validate_filters(&HashMap::from([(
            "owner".to_string(),
            Value::String("Analytics".to_string()),
        )]))
        .is_ok());
        assert!(validate_filters(&HashMap::from([(
            "type_contains".to_string(),
            Value::String("Metric".to_string()),
        )]))
        .is_err());
        assert!(validate_filters(&HashMap::from([(
            "type!".to_string(),
            Value::String("Metric".to_string()),
        )]))
        .is_err());
    }

    #[test]
    fn escapes_path_prefix_like_metacharacters() {
        assert_eq!(escape_like(r"metrics_100%\"), r"metrics\_100\%\\");
    }
}
