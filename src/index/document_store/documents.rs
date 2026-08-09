//! Document CRUD operations for SqliteDocumentStore.

use crate::index::traits::{DocumentRecord, Result};
use rusqlite::{params, Connection, OptionalExtension, Transaction};

pub fn init(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS documents (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            root_id INTEGER NOT NULL DEFAULT 1,
            path TEXT NOT NULL,
            parent_path TEXT NOT NULL DEFAULT '',
            title TEXT,
            type TEXT,
            description TEXT,
            body_text TEXT NOT NULL DEFAULT '',
            file_size INTEGER NOT NULL DEFAULT 0,
            modified_at INTEGER NOT NULL DEFAULT 0,
            content_hash TEXT NOT NULL DEFAULT '',
            parse_status TEXT NOT NULL DEFAULT 'ok',
            UNIQUE(root_id, path)
        );
        CREATE INDEX IF NOT EXISTS idx_documents_path ON documents(path);
        CREATE INDEX IF NOT EXISTS idx_documents_root ON documents(root_id);
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

        CREATE TABLE IF NOT EXISTS tables (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            document_id INTEGER NOT NULL,
            section_heading TEXT,
            headers TEXT NOT NULL,
            rows TEXT NOT NULL,
            alignments TEXT NOT NULL,
            position INTEGER,
            FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE
        );
        CREATE INDEX IF NOT EXISTS idx_tables_document_id ON tables(document_id);

        CREATE TABLE IF NOT EXISTS code_blocks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            document_id INTEGER NOT NULL,
            section_heading TEXT,
            language TEXT,
            filename TEXT,
            content TEXT NOT NULL,
            position INTEGER,
            FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE
        );
        CREATE INDEX IF NOT EXISTS idx_code_blocks_document_id ON code_blocks(document_id);

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
            root_id UNINDEXED,
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

pub fn upsert_document(conn: &Connection, doc: &DocumentRecord) -> Result<()> {
    conn.execute(
        "INSERT OR REPLACE INTO documents
         (root_id, path, parent_path, title, type, description, body_text, file_size, modified_at, content_hash, parse_status)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        params![
            doc.root_id,
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

pub fn upsert_document_tx(tx: &Transaction, doc: &DocumentRecord) -> Result<()> {
    tx.execute(
        "INSERT OR REPLACE INTO documents
         (root_id, path, parent_path, title, type, description, body_text, file_size, modified_at, content_hash, parse_status)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        params![
            doc.root_id,
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

pub fn get_document(
    conn: &Connection,
    path: &str,
    root_id: Option<i64>,
) -> Result<Option<DocumentRecord>> {
    let root_id = root_id.unwrap_or(1);
    let mut stmt = conn.prepare(
        "SELECT id, root_id, path, parent_path, title, type, description, body_text, file_size, modified_at, content_hash, parse_status
         FROM documents WHERE root_id = ?1 AND path = ?2"
    )?;
    let doc = stmt
        .query_row(params![root_id, path], |row| {
            Ok(DocumentRecord {
                id: row.get(0)?,
                root_id: row.get(1)?,
                path: row.get(2)?,
                parent_path: row.get(3)?,
                title: row.get(4)?,
                concept_type: row.get(5)?,
                description: row.get(6)?,
                body_text: row.get(7)?,
                file_size: row.get::<_, i64>(8)? as u64,
                modified_at: row.get(9)?,
                content_hash: row.get(10)?,
                parse_status: row.get(11)?,
            })
        })
        .optional()?;
    Ok(doc)
}

pub fn delete_document(conn: &Connection, path: &str, root_id: Option<i64>) -> Result<()> {
    let root_id = root_id.unwrap_or(1);
    conn.execute(
        "DELETE FROM documents WHERE root_id = ?1 AND path = ?2",
        params![root_id, path],
    )?;
    Ok(())
}

pub fn delete_document_tx(tx: &Transaction, path: &str, root_id: Option<i64>) -> Result<()> {
    let root_id = root_id.unwrap_or(1);
    tx.execute(
        "DELETE FROM documents WHERE root_id = ?1 AND path = ?2",
        params![root_id, path],
    )?;
    Ok(())
}

pub fn get_doc_id_tx(tx: &Transaction, path: &str, root_id: Option<i64>) -> Result<i64> {
    let root_id = root_id.unwrap_or(1);
    let mut stmt = tx.prepare("SELECT id FROM documents WHERE root_id = ?1 AND path = ?2")?;
    let id: i64 = stmt.query_row(params![root_id, path], |row| row.get(0))?;
    Ok(id)
}

pub fn list_documents(
    conn: &Connection,
    path_prefix: Option<&str>,
    limit: Option<usize>,
    root_id: Option<i64>,
) -> Result<Vec<DocumentRecord>> {
    let root_id = root_id.unwrap_or(1);
    let mut sql = String::from(
        "SELECT id, root_id, path, parent_path, title, type, description, body_text, file_size, modified_at, content_hash, parse_status
         FROM documents"
    );
    let mut params_vec: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

    sql.push_str(" WHERE root_id = ?1");
    params_vec.push(Box::new(root_id));

    if let Some(prefix) = path_prefix {
        if !prefix.is_empty() {
            sql.push_str(" AND path LIKE ?2");
            params_vec.push(Box::new(format!("{}%", prefix)));
        }
    }

    sql.push_str(&format!(" ORDER BY path LIMIT ?{}", params_vec.len() + 1));
    params_vec.push(Box::new(limit.unwrap_or(0) as i64));

    let params_refs: Vec<&dyn rusqlite::types::ToSql> =
        params_vec.iter().map(|p| p.as_ref()).collect();
    let mut stmt = conn.prepare(&sql)?;
    let docs = stmt
        .query_map(params_refs.as_slice(), |row| {
            Ok(DocumentRecord {
                id: row.get(0)?,
                root_id: row.get(1)?,
                path: row.get(2)?,
                parent_path: row.get(3)?,
                title: row.get(4)?,
                concept_type: row.get(5)?,
                description: row.get(6)?,
                body_text: row.get(7)?,
                file_size: row.get::<_, i64>(8)? as u64,
                modified_at: row.get(9)?,
                content_hash: row.get(10)?,
                parse_status: row.get(11)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();
    Ok(docs)
}
