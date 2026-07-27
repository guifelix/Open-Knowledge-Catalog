//! Tag operations for SqliteDocumentStore.

use crate::index::traits::Result;
use rusqlite::{params, Connection, Transaction};

pub fn insert_tags(conn: &Connection, doc_id: i64, tags: &[String]) -> Result<()> {
    conn.execute(
        "DELETE FROM document_tags WHERE document_id = ?1",
        params![doc_id],
    )?;
    for tag in tags {
        conn.execute(
            "INSERT OR IGNORE INTO document_tags (document_id, tag) VALUES (?1, ?2)",
            params![doc_id, tag],
        )?;
    }
    Ok(())
}

pub fn insert_tags_tx(tx: &Transaction, doc_id: i64, tags: &[String]) -> Result<()> {
    tx.execute(
        "DELETE FROM document_tags WHERE document_id = ?1",
        params![doc_id],
    )?;
    for tag in tags {
        tx.execute(
            "INSERT OR IGNORE INTO document_tags (document_id, tag) VALUES (?1, ?2)",
            params![doc_id, tag],
        )?;
    }
    Ok(())
}

pub fn get_tags(conn: &Connection, doc_id: i64) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT tag FROM document_tags WHERE document_id = ?1")?;
    let tags = stmt
        .query_map(params![doc_id], |row| row.get::<_, String>(0))?
        .filter_map(|r| r.ok())
        .collect();
    Ok(tags)
}

pub fn delete_tags(conn: &Connection, doc_id: i64) -> Result<()> {
    conn.execute(
        "DELETE FROM document_tags WHERE document_id = ?1",
        params![doc_id],
    )?;
    Ok(())
}
