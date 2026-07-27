//! Metadata field CRUD operations for SqliteDocumentStore.

use crate::index::traits::Result;
use rusqlite::{params, Connection, Transaction};
use std::collections::BTreeMap;

pub fn insert_metadata_fields(
    conn: &Connection,
    doc_id: i64,
    fields: &BTreeMap<String, serde_json::Value>,
) -> Result<()> {
    conn.execute(
        "DELETE FROM metadata_fields WHERE document_id = ?1",
        params![doc_id],
    )?;
    for (key, value) in fields {
        let val_str = serde_json::to_string(value).unwrap_or_else(|_| "null".to_string());
        conn.execute(
            "INSERT INTO metadata_fields (document_id, key, value) VALUES (?1, ?2, ?3)",
            params![doc_id, key, val_str],
        )?;
    }
    Ok(())
}

pub fn insert_metadata_fields_tx(
    tx: &Transaction,
    doc_id: i64,
    fields: &BTreeMap<String, serde_json::Value>,
) -> Result<()> {
    tx.execute(
        "DELETE FROM metadata_fields WHERE document_id = ?1",
        params![doc_id],
    )?;
    for (key, value) in fields {
        let val_str = serde_json::to_string(value).unwrap_or_else(|_| "null".to_string());
        tx.execute(
            "INSERT INTO metadata_fields (document_id, key, value) VALUES (?1, ?2, ?3)",
            params![doc_id, key, val_str],
        )?;
    }
    Ok(())
}

pub fn get_metadata_fields(
    conn: &Connection,
    doc_id: i64,
) -> Result<BTreeMap<String, serde_json::Value>> {
    let mut stmt = conn.prepare("SELECT key, value FROM metadata_fields WHERE document_id = ?1")?;
    let mut fields = BTreeMap::new();
    for row in stmt.query_map(params![doc_id], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    })? {
        let (k, v) = row?;
        if let Ok(val) = serde_json::from_str(&v) {
            fields.insert(k, val);
        }
    }
    Ok(fields)
}

pub fn delete_metadata_fields(conn: &Connection, doc_id: i64) -> Result<()> {
    conn.execute(
        "DELETE FROM metadata_fields WHERE document_id = ?1",
        params![doc_id],
    )?;
    Ok(())
}
