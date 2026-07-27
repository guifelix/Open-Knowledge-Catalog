//! Code block CRUD operations for SqliteDocumentStore.

use crate::index::traits::Result;
use rusqlite::{params, Connection};

pub fn insert_code_blocks(
    conn: &Connection,
    doc_id: i64,
    code_blocks: &[crate::model::document::CodeBlock],
) -> Result<()> {
    conn.execute(
        "DELETE FROM code_blocks WHERE document_id = ?1",
        params![doc_id],
    )?;
    for (idx, cb) in code_blocks.iter().enumerate() {
        conn.execute(
            "INSERT INTO code_blocks (document_id, section_heading, language, filename, content, position) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![doc_id, None::<String>, cb.language, cb.filename, cb.content, idx as i64],
        )?;
    }
    Ok(())
}

pub fn get_code_blocks(
    conn: &Connection,
    doc_id: i64,
) -> Result<Vec<crate::model::document::CodeBlock>> {
    let mut stmt = conn.prepare(
        "SELECT language, filename, content FROM code_blocks WHERE document_id = ?1 ORDER BY position",
    )?;
    let code_blocks = stmt
        .query_map(params![doc_id], |row| {
            Ok(crate::model::document::CodeBlock {
                language: row.get(0)?,
                filename: row.get(1)?,
                content: row.get(2)?,
                position: 0,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();
    Ok(code_blocks)
}

pub fn delete_code_blocks(conn: &Connection, doc_id: i64) -> Result<()> {
    conn.execute(
        "DELETE FROM code_blocks WHERE document_id = ?1",
        params![doc_id],
    )?;
    Ok(())
}
