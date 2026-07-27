//! Heading CRUD operations for SqliteDocumentStore.

use crate::index::traits::Result;
use crate::model::document::HeadingInfo;
use rusqlite::{params, Connection, Transaction};

pub fn insert_headings(conn: &Connection, doc_id: i64, headings: &[HeadingInfo]) -> Result<()> {
    conn.execute(
        "DELETE FROM headings WHERE document_id = ?1",
        params![doc_id],
    )?;
    for heading in headings {
        conn.execute(
            "INSERT INTO headings (document_id, level, title, anchor, position) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![doc_id, heading.level as i32, heading.title, heading.anchor, 0],
        )?;
    }
    Ok(())
}

pub fn insert_headings_tx(tx: &Transaction, doc_id: i64, headings: &[HeadingInfo]) -> Result<()> {
    tx.execute(
        "DELETE FROM headings WHERE document_id = ?1",
        params![doc_id],
    )?;
    for heading in headings {
        tx.execute(
            "INSERT INTO headings (document_id, level, title, anchor, position) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![doc_id, heading.level as i32, heading.title, heading.anchor, 0],
        )?;
    }
    Ok(())
}

pub fn get_headings(conn: &Connection, doc_id: i64) -> Result<Vec<HeadingInfo>> {
    let mut stmt = conn.prepare(
        "SELECT level, title, anchor FROM headings WHERE document_id = ?1 ORDER BY position",
    )?;
    let headings = stmt
        .query_map(params![doc_id], |row| {
            Ok(HeadingInfo {
                level: row.get::<_, i32>(0)? as u32,
                title: row.get(1)?,
                anchor: row.get(2)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();
    Ok(headings)
}

pub fn delete_headings(conn: &Connection, doc_id: i64) -> Result<()> {
    conn.execute(
        "DELETE FROM headings WHERE document_id = ?1",
        params![doc_id],
    )?;
    Ok(())
}
