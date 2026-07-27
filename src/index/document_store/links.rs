//! Link CRUD operations for SqliteDocumentStore.

use crate::index::traits::Result;
use crate::model::document::LinkInfo;
use rusqlite::{params, Connection, Transaction};

pub fn insert_links(conn: &Connection, doc_id: i64, links: &[LinkInfo]) -> Result<()> {
    conn.execute(
        "DELETE FROM links WHERE source_document_id = ?1",
        params![doc_id],
    )?;
    for link in links {
        let is_external = link.external_url.is_some();
        conn.execute(
            r#"
            INSERT INTO links (source_document_id, target_path, target_anchor, external_url, exists_in_repository)
            VALUES (?1, ?2, ?3, ?4, ?5)
            "#,
            params![
                doc_id,
                if is_external { None } else { link.target_path.clone() },
                link.target_anchor.clone(),
                if is_external { link.target_path.clone() } else { link.external_url.clone() },
                if is_external { 1 } else { link.exists_in_repository as i32 },
            ],
        )?;
    }
    Ok(())
}

pub fn insert_links_tx(tx: &Transaction, doc_id: i64, links: &[LinkInfo]) -> Result<()> {
    tx.execute(
        "DELETE FROM links WHERE source_document_id = ?1",
        params![doc_id],
    )?;
    for link in links {
        let is_external = link.external_url.is_some();
        tx.execute(
            r#"
            INSERT INTO links (source_document_id, target_path, target_anchor, external_url, exists_in_repository)
            VALUES (?1, ?2, ?3, ?4, ?5)
            "#,
            params![
                doc_id,
                if is_external { None } else { link.target_path.clone() },
                link.target_anchor.clone(),
                if is_external { link.target_path.clone() } else { link.external_url.clone() },
                if is_external { 1 } else { link.exists_in_repository as i32 },
            ],
        )?;
    }
    Ok(())
}

pub fn get_links(conn: &Connection, doc_id: i64) -> Result<Vec<LinkInfo>> {
    let mut stmt = conn.prepare(
        "SELECT target_path, target_anchor, external_url, exists_in_repository FROM links WHERE source_document_id = ?1"
    )?;
    let links = stmt
        .query_map(params![doc_id], |row| {
            Ok(LinkInfo {
                target_path: row.get(0)?,
                target_anchor: row.get(1)?,
                external_url: row.get(2)?,
                exists_in_repository: row.get::<_, i32>(3)? != 0,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();
    Ok(links)
}

pub fn delete_links(conn: &Connection, doc_id: i64) -> Result<()> {
    conn.execute(
        "DELETE FROM links WHERE source_document_id = ?1",
        params![doc_id],
    )?;
    Ok(())
}
