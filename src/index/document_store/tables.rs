//! Table CRUD operations for SqliteDocumentStore.

use crate::index::traits::Result;
use rusqlite::{params, Connection};

pub fn insert_tables(
    conn: &Connection,
    doc_id: i64,
    tables: &[crate::model::document::Table],
) -> Result<()> {
    conn.execute("DELETE FROM tables WHERE document_id = ?1", params![doc_id])?;
    for (idx, table) in tables.iter().enumerate() {
        let headers_json = serde_json::to_string(&table.headers)?;
        let rows_json = serde_json::to_string(&table.rows)?;
        let alignments_json = serde_json::to_string(
            &table
                .alignments
                .iter()
                .map(|a| format!("{:?}", a))
                .collect::<Vec<_>>(),
        )?;
        conn.execute(
            "INSERT INTO tables (document_id, section_heading, headers, rows, alignments, position) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![doc_id, None::<String>, headers_json, rows_json, alignments_json, idx as i64],
        )?;
    }
    Ok(())
}

pub fn get_tables(conn: &Connection, doc_id: i64) -> Result<Vec<crate::model::document::Table>> {
    let mut stmt = conn.prepare(
        "SELECT headers, rows, alignments FROM tables WHERE document_id = ?1 ORDER BY position",
    )?;
    let tables = stmt
        .query_map(params![doc_id], |row| {
            let headers_json: String = row.get(0)?;
            let rows_json: String = row.get(1)?;
            let alignments_json: String = row.get(2)?;
            let headers: Vec<String> = serde_json::from_str(&headers_json).unwrap_or_default();
            let rows: Vec<Vec<String>> = serde_json::from_str(&rows_json).unwrap_or_default();
            let alignments_str: Vec<String> =
                serde_json::from_str(&alignments_json).unwrap_or_default();
            let alignments = alignments_str
                .iter()
                .map(|s| match s.as_str() {
                    "Left" => crate::model::document::TableAlignment::Left,
                    "Center" => crate::model::document::TableAlignment::Center,
                    "Right" => crate::model::document::TableAlignment::Right,
                    _ => crate::model::document::TableAlignment::None,
                })
                .collect();
            Ok(crate::model::document::Table {
                headers,
                rows,
                alignments,
                position: 0,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();
    Ok(tables)
}

pub fn delete_tables(conn: &Connection, doc_id: i64) -> Result<()> {
    conn.execute("DELETE FROM tables WHERE document_id = ?1", params![doc_id])?;
    Ok(())
}
