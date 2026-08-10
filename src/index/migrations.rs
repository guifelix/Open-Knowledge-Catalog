//! Database schema migrations for the repository index.
//!
//! This module manages the SQLite schema for the OKC index, including:
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
            relation TEXT,
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
         CREATE INDEX IF NOT EXISTS idx_document_tags_tag ON document_tags(tag);
         CREATE INDEX IF NOT EXISTS idx_headings_doc ON headings(document_id);
         CREATE INDEX IF NOT EXISTS idx_links_source ON links(source_document_id);
         CREATE INDEX IF NOT EXISTS idx_links_target ON links(target_path);
         CREATE INDEX IF NOT EXISTS idx_metadata_fields_doc ON metadata_fields(document_id);
         CREATE INDEX IF NOT EXISTS idx_metadata_fields_key ON metadata_fields(key);
         CREATE INDEX IF NOT EXISTS idx_tables_doc ON tables(document_id);
         CREATE INDEX IF NOT EXISTS idx_code_blocks_doc ON code_blocks(document_id);",
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

    // Migration 3: Add optional relation column to links for typed_links.
    // Guarded so it is safe on fresh databases (column already present via
    // CREATE TABLE) and on existing ones (column added by ALTER TABLE).
    if version < 3 {
        let has_relation: bool = conn
            .query_row(
                "SELECT COUNT(*) FROM pragma_table_info('links') WHERE name = 'relation'",
                [],
                |row| row.get::<_, i64>(0),
            )
            .is_ok_and(|count| count > 0);
        if !has_relation {
            conn.execute("ALTER TABLE links ADD COLUMN relation TEXT", [])?;
        }
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_links_relation ON links(relation)",
            [],
        )?;
        conn.execute("INSERT INTO schema_version (version) VALUES (3)", [])?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn column_exists(conn: &Connection, column: &str) -> bool {
        conn.query_row(
            "SELECT COUNT(*) FROM pragma_table_info('links') WHERE name = ?1",
            [column],
            |row| row.get::<_, i64>(0),
        )
        .is_ok_and(|count| count > 0)
    }

    #[allow(clippy::expect_used)]
    fn latest_version(conn: &Connection) -> i64 {
        conn.query_row("SELECT MAX(version) FROM schema_version", [], |row| {
            row.get(0)
        })
        .expect("schema_version exists")
    }

    #[test]
    #[allow(clippy::expect_used)]
    fn fresh_database_has_relation_column_and_index() {
        let conn = Connection::open_in_memory().expect("open in-memory db");
        run(&conn).expect("migrations succeed on a fresh database");

        assert!(column_exists(&conn, "relation"));
        let index_exists: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master
                 WHERE type = 'index' AND name = 'idx_links_relation'",
                [],
                |row| row.get(0),
            )
            .expect("index lookup");
        assert_eq!(index_exists, 1);
        assert_eq!(latest_version(&conn), 3);
    }

    #[test]
    #[allow(clippy::expect_used)]
    fn migration_v3_upgrades_v2_database() {
        let conn = Connection::open_in_memory().expect("open in-memory db");
        conn.execute_batch(
            "
            CREATE TABLE schema_version (
                version INTEGER PRIMARY KEY,
                applied_at TEXT NOT NULL DEFAULT (datetime('now'))
            );
            INSERT INTO schema_version (version) VALUES (1);
            INSERT INTO schema_version (version) VALUES (2);
            CREATE TABLE documents (
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
            CREATE TABLE links (
                source_document_id INTEGER NOT NULL,
                target_path TEXT,
                target_anchor TEXT,
                external_url TEXT,
                exists_in_repository INTEGER NOT NULL DEFAULT 0,
                FOREIGN KEY(source_document_id) REFERENCES documents(id) ON DELETE CASCADE
            );
            CREATE INDEX idx_links_source ON links(source_document_id);
            CREATE INDEX idx_links_target ON links(target_path);
            ",
        )
        .expect("seed v2 schema with untyped links");

        run(&conn).expect("migrations upgrade v2 to v3");

        assert!(column_exists(&conn, "relation"), "relation column added");
        assert_eq!(latest_version(&conn), 3);
        let row_count: i64 = conn
            .query_row("SELECT COUNT(*) FROM links", [], |row| row.get(0))
            .expect("links preserved");
        assert_eq!(row_count, 0, "existing links rows remain untouched");
    }

    #[test]
    #[allow(clippy::expect_used)]
    fn run_is_idempotent() {
        let conn = Connection::open_in_memory().expect("open in-memory db");
        run(&conn).expect("first migration run");
        run(&conn).expect("second migration run is a no-op");
        assert_eq!(latest_version(&conn), 3);
    }
}
