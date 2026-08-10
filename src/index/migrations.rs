//! Database schema migrations for the repository index.
//!
//! This module manages the SQLite schema for the OKC index, including:
//! - `roots` - Configured repository roots with stable identities
//! - `documents` - Core document metadata and content
//! - `document_tags` - Document tag associations
//! - `headings` - Heading hierarchy for each document
//! - `links` - Internal and external link relationships
//! - `metadata_fields` - Custom front-matter fields
//! - `tables` - Extracted markdown tables with headers and rows
//! - `code_blocks` - Fenced code blocks with language and filename metadata
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

        CREATE TABLE IF NOT EXISTS roots (
            id INTEGER PRIMARY KEY,
            root_id TEXT NOT NULL UNIQUE,
            path TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        );

        CREATE TABLE IF NOT EXISTS documents (
            id INTEGER PRIMARY KEY,
            root_id INTEGER NOT NULL DEFAULT 1,
            path TEXT NOT NULL,
            parent_path TEXT NOT NULL DEFAULT '',
            title TEXT,
            type TEXT,
            description TEXT,
            body_text TEXT,
            file_size INTEGER NOT NULL DEFAULT 0,
            modified_at INTEGER NOT NULL DEFAULT 0,
            content_hash TEXT,
            parse_status TEXT NOT NULL DEFAULT 'ok',
            UNIQUE(root_id, path),
            FOREIGN KEY(root_id) REFERENCES roots(id) ON DELETE CASCADE
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
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            source_document_id INTEGER NOT NULL,
            source_root_id INTEGER NOT NULL DEFAULT 1,
            target_path TEXT,
            target_root_id INTEGER,
            target_anchor TEXT,
            external_url TEXT,
            exists_in_repository INTEGER NOT NULL DEFAULT 1,
            FOREIGN KEY(source_document_id) REFERENCES documents(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS tables (
            document_id INTEGER NOT NULL,
            section_heading TEXT,
            headers TEXT NOT NULL, -- JSON array of header strings
            rows TEXT NOT NULL, -- JSON array of row arrays
            alignments TEXT NOT NULL, -- JSON array of alignment strings
            position INTEGER,
            FOREIGN KEY(document_id) REFERENCES documents(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS code_blocks (
            document_id INTEGER NOT NULL,
            section_heading TEXT,
            language TEXT,
            filename TEXT,
            content TEXT NOT NULL,
            position INTEGER,
            FOREIGN KEY(document_id) REFERENCES documents(id) ON DELETE CASCADE
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
         CREATE INDEX IF NOT EXISTS idx_documents_root ON documents(root_id);
         CREATE INDEX IF NOT EXISTS idx_document_tags_tag ON document_tags(tag);
         CREATE INDEX IF NOT EXISTS idx_headings_doc ON headings(document_id);
         CREATE INDEX IF NOT EXISTS idx_links_source ON links(source_document_id);
         CREATE INDEX IF NOT EXISTS idx_links_target ON links(target_path);
         CREATE INDEX IF NOT EXISTS idx_links_source_root ON links(source_root_id);
         CREATE INDEX IF NOT EXISTS idx_links_target_root ON links(target_root_id);
         CREATE INDEX IF NOT EXISTS idx_metadata_fields_doc ON metadata_fields(document_id);
         CREATE INDEX IF NOT EXISTS idx_metadata_fields_key ON metadata_fields(key);
         CREATE INDEX IF NOT EXISTS idx_tables_doc ON tables(document_id);
         CREATE INDEX IF NOT EXISTS idx_code_blocks_doc ON code_blocks(document_id);",
    )?;

    conn.execute_batch(
        "CREATE VIRTUAL TABLE IF NOT EXISTS document_search USING fts5(
            root_id UNINDEXED,
            path UNINDEXED,
            title,
            description,
            headings,
            body,
            concept_type,
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

    // Migration 2: Add concept_type column to FTS5 table
    // FTS5 virtual tables cannot be altered, so we must drop and recreate
    if version < 2 {
        conn.execute("DROP TABLE IF EXISTS document_search", [])?;
        conn.execute_batch(
            "CREATE VIRTUAL TABLE document_search USING fts5(
                path UNINDEXED,
                title,
                description,
                headings,
                body,
                concept_type,
                tokenize='porter unicode61'
            );",
        )?;
        conn.execute("INSERT INTO schema_version (version) VALUES (2)", [])?;
    }

    // Migration 3: Add roots table and root_id column to documents for multi-root support
    if version < 3 {
        // Create roots table if not exists (idempotent)
        conn.execute(
            "CREATE TABLE IF NOT EXISTS roots (
                id INTEGER PRIMARY KEY,
                root_id TEXT NOT NULL UNIQUE,
                path TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT (datetime('now'))
            )",
            [],
        )?;

        // Check if root_id column exists in documents
        let has_root_id: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM pragma_table_info('documents') WHERE name = 'root_id'",
                [],
                |row| row.get(0),
            )
            .unwrap_or(0);

        // Add root_id column to documents if not exists
        if has_root_id == 0 {
            conn.execute(
                "ALTER TABLE documents ADD COLUMN root_id INTEGER DEFAULT 1",
                [],
            )?;
        }

        // Update unique constraint from path to (root_id, path)
        // SQLite doesn't support dropping unique constraint directly,
        // so we need to recreate the table. But for migration we'll handle this carefully.
        // First, populate root_id for existing documents
        conn.execute("UPDATE documents SET root_id = 1 WHERE root_id IS NULL", [])?;

        // Create index on root_id
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_documents_root ON documents(root_id)",
            [],
        )?;

        // Recreate document_search with root_id if needed (FTS5 can't be altered)
        // Check if root_id column exists in document_search
        let has_root_id_search: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM pragma_table_info('document_search') WHERE name = 'root_id'",
                [],
                |row| row.get(0),
            )
            .unwrap_or(0);

        if has_root_id_search == 0 {
            conn.execute("DROP TABLE IF EXISTS document_search", [])?;
            conn.execute_batch(
                "CREATE VIRTUAL TABLE document_search USING fts5(
                    root_id UNINDEXED,
                    path UNINDEXED,
                    title,
                    description,
                    headings,
                    body,
                    concept_type,
                    tokenize='porter unicode61'
                );",
            )?;
        }

        conn.execute("INSERT INTO schema_version (version) VALUES (3)", [])?;
    }

    Ok(())
}
