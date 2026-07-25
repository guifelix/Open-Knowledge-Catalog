//! SQLite-backed document storage implementation.
//!
//! This module provides [`SqliteDocumentStore`], an implementation of the
//! [`DocumentStore`] trait that persists documents, headings, links, tags,
//! and metadata fields to a SQLite database.
//!
//! The store uses a mutex-protected connection for thread safety and provides
//! CRUD operations for all document-related data.

use crate::index::traits::{DocumentRecord, DocumentStore, Result};
use crate::model::document::{
    HeadingInfo, IndexStats, LinkInfo, MetadataQueryResponse, ParseError,
};
use rusqlite::{params, Connection, OptionalExtension};
use std::collections::{BTreeMap, HashMap};
use std::sync::{Mutex, PoisonError};

/// SQLite-backed document store with thread-safe connection.
///
/// Implements the [`DocumentStore`] trait for persistent document storage.
/// All operations are serialized through a mutex to ensure thread safety.
pub struct SqliteDocumentStore {
    conn: Mutex<Connection>,
}

impl SqliteDocumentStore {
    /// Create a new document store with the given database connection.
    #[allow(dead_code)]
    pub fn new(conn: Connection) -> Self {
        Self {
            conn: Mutex::new(conn),
        }
    }

    #[allow(dead_code)]
    fn get_doc_id(&self, path: &str) -> Result<Option<i64>> {
        let conn = self
            .conn
            .lock()
            .map_err(|e: PoisonError<_>| anyhow::anyhow!("Mutex poisoned: {}", e))?;
        conn.query_row(
            "SELECT id FROM documents WHERE path = ?1",
            params![path],
            |row| row.get::<_, i64>(0),
        )
        .optional()
        .map_err(anyhow::Error::from)
    }
}

impl DocumentStore for SqliteDocumentStore {
    fn init(&self) -> Result<()> {
        let conn = self
            .conn
            .lock()
            .map_err(|e: PoisonError<_>| anyhow::anyhow!("Mutex poisoned: {}", e))?;
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS documents (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                path TEXT NOT NULL UNIQUE,
                parent_path TEXT NOT NULL DEFAULT '',
                title TEXT,
                type TEXT,
                description TEXT,
                body_text TEXT NOT NULL DEFAULT '',
                file_size INTEGER NOT NULL DEFAULT 0,
                modified_at INTEGER NOT NULL DEFAULT 0,
                content_hash TEXT NOT NULL DEFAULT '',
                parse_status TEXT NOT NULL DEFAULT 'ok'
            );
            CREATE INDEX IF NOT EXISTS idx_documents_path ON documents(path);
            CREATE INDEX IF NOT EXISTS idx_documents_parent_path ON documents(parent_path);
            CREATE INDEX IF NOT EXISTS idx_documents_type ON documents(type);
            CREATE INDEX IF NOT EXISTS idx_documents_parse_status ON documents(parse_status);
            CREATE INDEX IF NOT EXISTS idx_documents_content_hash ON documents(content_hash);

            CREATE TABLE IF NOT EXISTS document_tags (
                document_id INTEGER NOT NULL,
                tag TEXT NOT NULL,
                PRIMARY KEY (document_id, tag),
                FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE
            );
            CREATE INDEX IF NOT EXISTS idx_document_tags_document_id ON document_tags(document_id);
            CREATE INDEX IF NOT EXISTS idx_document_tags_tag ON document_tags(tag);

            CREATE TABLE IF NOT EXISTS headings (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                document_id INTEGER NOT NULL,
                level INTEGER NOT NULL,
                title TEXT NOT NULL,
                anchor TEXT,
                position INTEGER NOT NULL DEFAULT 0,
                FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE
            );
            CREATE INDEX IF NOT EXISTS idx_headings_document_id ON headings(document_id);

            CREATE TABLE IF NOT EXISTS links (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                source_document_id INTEGER NOT NULL,
                target_path TEXT,
                target_anchor TEXT,
                external_url TEXT,
                exists_in_repository INTEGER NOT NULL DEFAULT 1,
                FOREIGN KEY (source_document_id) REFERENCES documents(id) ON DELETE CASCADE
            );
            CREATE INDEX IF NOT EXISTS idx_links_source ON links(source_document_id);
            CREATE INDEX IF NOT EXISTS idx_links_target_path ON links(target_path);

            CREATE TABLE IF NOT EXISTS metadata_fields (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                document_id INTEGER NOT NULL,
                key TEXT NOT NULL,
                value TEXT NOT NULL,
                FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE
            );
            CREATE INDEX IF NOT EXISTS idx_metadata_fields_document_id ON metadata_fields(document_id);
            CREATE INDEX IF NOT EXISTS idx_metadata_fields_key ON metadata_fields(key);

            CREATE TABLE IF NOT EXISTS scan_errors (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                path TEXT NOT NULL,
                stage TEXT NOT NULL,
                message TEXT NOT NULL,
                line INTEGER,
                FOREIGN KEY (path) REFERENCES documents(path) ON DELETE CASCADE
            );
            CREATE INDEX IF NOT EXISTS idx_scan_errors_path ON scan_errors(path);

            CREATE VIRTUAL TABLE IF NOT EXISTS document_search USING fts5(
                path UNINDEXED,
                title,
                description,
                headings,
                body,
                tokenize = 'porter unicode61'
            );
            "#,
        )?;
        Ok(())
    }

    fn upsert_document(&self, doc: &DocumentRecord) -> Result<()> {
        let conn = self
            .conn
            .lock()
            .map_err(|e: PoisonError<_>| anyhow::anyhow!("Mutex poisoned: {}", e))?;
        conn.execute(
            "INSERT OR REPLACE INTO documents
             (path, parent_path, title, type, description, body_text, file_size, modified_at, content_hash, parse_status)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                doc.path,
                doc.parent_path,
                doc.title,
                doc.concept_type,
                doc.description,
                doc.body_text,
                doc.file_size as i64,
                doc.modified_at,
                doc.content_hash,
                doc.parse_status,
            ],
        )?;
        Ok(())
    }

    fn get_document(&self, path: &str) -> Result<Option<DocumentRecord>> {
        let conn = self
            .conn
            .lock()
            .map_err(|e: PoisonError<_>| anyhow::anyhow!("Mutex poisoned: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, path, parent_path, title, type, description, body_text, file_size, modified_at, content_hash, parse_status
             FROM documents WHERE path = ?1"
        )?;
        let doc = stmt
            .query_row(params![path], |row| {
                Ok(DocumentRecord {
                    id: row.get(0)?,
                    path: row.get(1)?,
                    parent_path: row.get(2)?,
                    title: row.get(3)?,
                    concept_type: row.get(4)?,
                    description: row.get(5)?,
                    body_text: row.get(6)?,
                    file_size: row.get::<_, i64>(7)? as u64,
                    modified_at: row.get(8)?,
                    content_hash: row.get(9)?,
                    parse_status: row.get(10)?,
                })
            })
            .optional()?;
        Ok(doc)
    }

    fn delete_document(&self, path: &str) -> Result<()> {
        let conn = self
            .conn
            .lock()
            .map_err(|e: PoisonError<_>| anyhow::anyhow!("Mutex poisoned: {}", e))?;
        conn.execute("DELETE FROM documents WHERE path = ?1", params![path])?;
        Ok(())
    }

    fn list_documents(
        &self,
        path_prefix: Option<&str>,
        limit: usize,
    ) -> Result<Vec<DocumentRecord>> {
        let conn = self
            .conn
            .lock()
            .map_err(|e: PoisonError<_>| anyhow::anyhow!("Mutex poisoned: {}", e))?;
        let mut sql = String::from(
            "SELECT id, path, parent_path, title, type, description, body_text, file_size, modified_at, content_hash, parse_status
             FROM documents"
        );
        let mut params_vec: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

        if let Some(prefix) = path_prefix {
            if !prefix.is_empty() {
                sql.push_str(" WHERE path LIKE ?1");
                params_vec.push(Box::new(format!("{}%", prefix)));
            }
        }

        sql.push_str(&format!(" ORDER BY path LIMIT ?{}", params_vec.len() + 1));
        params_vec.push(Box::new(limit as i64));

        let params_refs: Vec<&dyn rusqlite::types::ToSql> =
            params_vec.iter().map(|p| p.as_ref()).collect();
        let mut stmt = conn.prepare(&sql)?;
        let docs = stmt
            .query_map(params_refs.as_slice(), |row| {
                Ok(DocumentRecord {
                    id: row.get(0)?,
                    path: row.get(1)?,
                    parent_path: row.get(2)?,
                    title: row.get(3)?,
                    concept_type: row.get(4)?,
                    description: row.get(5)?,
                    body_text: row.get(6)?,
                    file_size: row.get::<_, i64>(7)? as u64,
                    modified_at: row.get(8)?,
                    content_hash: row.get(9)?,
                    parse_status: row.get(10)?,
                })
            })?
            .filter_map(|r| r.ok())
            .collect();
        Ok(docs)
    }

    fn insert_tags(&self, doc_id: i64, tags: &[String]) -> Result<()> {
        let conn = self
            .conn
            .lock()
            .map_err(|e: PoisonError<_>| anyhow::anyhow!("Mutex poisoned: {}", e))?;
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

    fn get_tags(&self, doc_id: i64) -> Result<Vec<String>> {
        let conn = self
            .conn
            .lock()
            .map_err(|e: PoisonError<_>| anyhow::anyhow!("Mutex poisoned: {}", e))?;
        let mut stmt = conn.prepare("SELECT tag FROM document_tags WHERE document_id = ?1")?;
        let tags = stmt
            .query_map(params![doc_id], |row| row.get::<_, String>(0))?
            .filter_map(|r| r.ok())
            .collect();
        Ok(tags)
    }

    fn delete_tags(&self, doc_id: i64) -> Result<()> {
        let conn = self
            .conn
            .lock()
            .map_err(|e: PoisonError<_>| anyhow::anyhow!("Mutex poisoned: {}", e))?;
        conn.execute(
            "DELETE FROM document_tags WHERE document_id = ?1",
            params![doc_id],
        )?;
        Ok(())
    }

    fn insert_headings(&self, doc_id: i64, headings: &[HeadingInfo]) -> Result<()> {
        let conn = self
            .conn
            .lock()
            .map_err(|e: PoisonError<_>| anyhow::anyhow!("Mutex poisoned: {}", e))?;
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

    fn get_headings(&self, doc_id: i64) -> Result<Vec<HeadingInfo>> {
        let conn = self
            .conn
            .lock()
            .map_err(|e: PoisonError<_>| anyhow::anyhow!("Mutex poisoned: {}", e))?;
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

    fn delete_headings(&self, doc_id: i64) -> Result<()> {
        let conn = self
            .conn
            .lock()
            .map_err(|e: PoisonError<_>| anyhow::anyhow!("Mutex poisoned: {}", e))?;
        conn.execute(
            "DELETE FROM headings WHERE document_id = ?1",
            params![doc_id],
        )?;
        Ok(())
    }

    fn insert_links(&self, doc_id: i64, links: &[LinkInfo]) -> Result<()> {
        let conn = self
            .conn
            .lock()
            .map_err(|e: PoisonError<_>| anyhow::anyhow!("Mutex poisoned: {}", e))?;
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

    fn get_links(&self, doc_id: i64) -> Result<Vec<LinkInfo>> {
        let conn = self
            .conn
            .lock()
            .map_err(|e: PoisonError<_>| anyhow::anyhow!("Mutex poisoned: {}", e))?;
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

    fn delete_links(&self, doc_id: i64) -> Result<()> {
        let conn = self
            .conn
            .lock()
            .map_err(|e: PoisonError<_>| anyhow::anyhow!("Mutex poisoned: {}", e))?;
        conn.execute(
            "DELETE FROM links WHERE source_document_id = ?1",
            params![doc_id],
        )?;
        Ok(())
    }

    fn insert_metadata_fields(
        &self,
        doc_id: i64,
        fields: &BTreeMap<String, serde_json::Value>,
    ) -> Result<()> {
        let conn = self
            .conn
            .lock()
            .map_err(|e: PoisonError<_>| anyhow::anyhow!("Mutex poisoned: {}", e))?;
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

    fn get_metadata_fields(&self, doc_id: i64) -> Result<BTreeMap<String, serde_json::Value>> {
        let conn = self
            .conn
            .lock()
            .map_err(|e: PoisonError<_>| anyhow::anyhow!("Mutex poisoned: {}", e))?;
        let mut stmt =
            conn.prepare("SELECT key, value FROM metadata_fields WHERE document_id = ?1")?;
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

    fn delete_metadata_fields(&self, doc_id: i64) -> Result<()> {
        let conn = self
            .conn
            .lock()
            .map_err(|e: PoisonError<_>| anyhow::anyhow!("Mutex poisoned: {}", e))?;
        conn.execute(
            "DELETE FROM metadata_fields WHERE document_id = ?1",
            params![doc_id],
        )?;
        Ok(())
    }

    fn insert_scan_errors(&self, path: &str, errors: &[ParseError]) -> Result<()> {
        let conn = self
            .conn
            .lock()
            .map_err(|e: PoisonError<_>| anyhow::anyhow!("Mutex poisoned: {}", e))?;
        conn.execute("DELETE FROM scan_errors WHERE path = ?1", params![path])?;
        for err in errors {
            conn.execute(
                "INSERT INTO scan_errors (path, stage, message, line) VALUES (?1, ?2, ?3, ?4)",
                params![path, err.stage, err.message, err.line.map(|l| l as i64)],
            )?;
        }
        Ok(())
    }

    fn get_scan_errors(&self, path: &str) -> Result<Vec<ParseError>> {
        let conn = self
            .conn
            .lock()
            .map_err(|e: PoisonError<_>| anyhow::anyhow!("Mutex poisoned: {}", e))?;
        let mut stmt =
            conn.prepare("SELECT stage, message, line FROM scan_errors WHERE path = ?1")?;
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

    fn delete_scan_errors(&self, path: &str) -> Result<()> {
        let conn = self
            .conn
            .lock()
            .map_err(|e: PoisonError<_>| anyhow::anyhow!("Mutex poisoned: {}", e))?;
        conn.execute("DELETE FROM scan_errors WHERE path = ?1", params![path])?;
        Ok(())
    }

    fn query_metadata(
        &self,
        _filters: &HashMap<String, String>,
        _select: &[String],
        _limit: usize,
    ) -> Result<MetadataQueryResponse> {
        // Simplified implementation
        Ok(MetadataQueryResponse {
            results: vec![],
            total_matches: 0,
            truncated: false,
        })
    }

    fn get_stats(&self) -> Result<IndexStats> {
        let conn = self
            .conn
            .lock()
            .map_err(|e: PoisonError<_>| anyhow::anyhow!("Mutex poisoned: {}", e))?;
        let doc_count: i64 =
            conn.query_row("SELECT COUNT(*) FROM documents", [], |row| row.get(0))?;
        let error_count: i64 =
            conn.query_row("SELECT COUNT(*) FROM scan_errors", [], |row| row.get(0))?;
        let link_count: i64 = conn.query_row("SELECT COUNT(*) FROM links", [], |row| row.get(0))?;
        let heading_count: i64 =
            conn.query_row("SELECT COUNT(*) FROM headings", [], |row| row.get(0))?;

        Ok(IndexStats {
            document_count: doc_count as usize,
            error_count: error_count as usize,
            link_count: link_count as usize,
            heading_count: heading_count as usize,
        })
    }
}
