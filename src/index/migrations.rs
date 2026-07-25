//! Database schema migrations for the repository index.
//!
//! This module manages the SQLite schema for the OKC index, including:
//! - `documents` - Core document metadata and content
//! - `document_tags` - Document tag associations
//! - `headings` - Heading hierarchy for each document
//! - `links` - Internal and external link relationships
//! - `metadata_fields` - Custom front-matter fields
//! - `document_search` - FTS5 full-text search index
//! - `scan_errors` - Parse error tracking
//! - `schema_version` - Migration version tracking

use rusqlite::Connection;

/// Run all pending migrations on the given connection.
///
/// Creates tables and indexes if they don't exist. This is idempotent
/// and safe to call on every startup.
pub fn run(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS schema_version (
            version INTEGER PRIMARY KEY,
            applied_at TEXT NOT NULL DEFAULT (datetime('now'))
        );

        CREATE TABLE IF NOT EXISTS documents (
            id INTEGER PRIMARY KEY,
            path TEXT NOT NULL UNIQUE,
            parent_path TEXT NOT NULL DEFAULT '',
            title TEXT,
            type TEXT,
            description TEXT,
            body_text TEXT,
            file_size INTEGER NOT NULL DEFAULT 0,
            modified_at INTEGER NOT NULL DEFAULT 0,
            content_hash TEXT,
            parse_status TEXT NOT NULL DEFAULT 'ok'
        );

        CREATE TABLE IF NOT EXISTS document_tags (
            document_id INTEGER NOT NULL,
            tag TEXT NOT NULL,
            FOREIGN KEY(document_id) REFERENCES documents(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS headings (
            document_id INTEGER NOT NULL,
            level INTEGER NOT NULL,
            title TEXT NOT NULL,
            anchor TEXT,
            position INTEGER,
            FOREIGN KEY(document_id) REFERENCES documents(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS links (
            source_document_id INTEGER NOT NULL,
            target_path TEXT,
            target_anchor TEXT,
            external_url TEXT,
            exists_in_repository INTEGER NOT NULL DEFAULT 0,
            FOREIGN KEY(source_document_id) REFERENCES documents(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS metadata_fields (
            document_id INTEGER NOT NULL,
            key TEXT NOT NULL,
            value TEXT,
            FOREIGN KEY(document_id) REFERENCES documents(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS scan_errors (
            id INTEGER PRIMARY KEY,
            path TEXT NOT NULL,
            stage TEXT NOT NULL,
            message TEXT NOT NULL,
            line INTEGER
        );
        ",
    )?;

    conn.execute_batch(
        "CREATE INDEX IF NOT EXISTS idx_documents_parent ON documents(parent_path);
         CREATE INDEX IF NOT EXISTS idx_documents_type ON documents(type);
         CREATE INDEX IF NOT EXISTS idx_document_tags_tag ON document_tags(tag);
         CREATE INDEX IF NOT EXISTS idx_headings_doc ON headings(document_id);
         CREATE INDEX IF NOT EXISTS idx_links_source ON links(source_document_id);
         CREATE INDEX IF NOT EXISTS idx_links_target ON links(target_path);
         CREATE INDEX IF NOT EXISTS idx_metadata_fields_doc ON metadata_fields(document_id);
         CREATE INDEX IF NOT EXISTS idx_metadata_fields_key ON metadata_fields(key);",
    )?;

    conn.execute_batch(
        "CREATE VIRTUAL TABLE IF NOT EXISTS document_search USING fts5(
            path,
            title,
            description,
            headings,
            body,
            tokenize='porter unicode61'
        );",
    )?;

    let version: i64 = conn
        .query_row(
            "SELECT COALESCE(MAX(version), 0) FROM schema_version",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);

    if version < 1 {
        conn.execute("INSERT INTO schema_version (version) VALUES (1)", [])?;
    }

    Ok(())
}
