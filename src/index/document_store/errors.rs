//! Scan error CRUD operations for SqliteDocumentStore.

use crate::index::traits::Result;
use crate::model::document::ParseError;
use rusqlite::{params, Connection, Transaction};

pub fn insert_scan_errors(conn: &Connection, path: &str, errors: &[ParseError]) -> Result<()> {
    conn.execute("DELETE FROM scan_errors WHERE path = ?1", params![path])?;
    for err in errors {
        conn.execute(
            "INSERT INTO scan_errors (path, stage, message, line) VALUES (?1, ?2, ?3, ?4)",
            params![path, err.stage, err.message, err.line.map(|l| l as i64)],
        )?;
    }
    Ok(())
}

pub fn insert_scan_errors_tx(tx: &Transaction, path: &str, errors: &[ParseError]) -> Result<()> {
    tx.execute("DELETE FROM scan_errors WHERE path = ?1", params![path])?;
    for err in errors {
        tx.execute(
            "INSERT INTO scan_errors (path, stage, message, line) VALUES (?1, ?2, ?3, ?4)",
            params![path, err.stage, err.message, err.line.map(|l| l as i64)],
        )?;
    }
    Ok(())
}

pub fn get_scan_errors(conn: &Connection, path: &str) -> Result<Vec<ParseError>> {
    let mut stmt = conn.prepare("SELECT stage, message, line FROM scan_errors WHERE path = ?1")?;
    let errors = stmt
        .query_map(params![path], |row| {
            Ok(ParseError {
                stage: row.get(0)?,
                message: row.get(1)?,
                line: row.get::<_, Option<i64>>(2)?.map(|l| l as usize),
            })
        })?
        .filter_map(|r| r.ok())
        .collect();
    Ok(errors)
}

pub fn delete_scan_errors(conn: &Connection, path: &str) -> Result<()> {
    conn.execute("DELETE FROM scan_errors WHERE path = ?1", params![path])?;
    Ok(())
}
