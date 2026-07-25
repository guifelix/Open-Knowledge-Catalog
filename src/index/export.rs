//! Export and format operations for RepositoryIndex.
//!
//! Provides JSON export functionality for the entire repository index.
//! Used by the CLI `--json` flag and for benchmarking.
//!
//! Exported data includes:
//! - Document metadata (path, title, type, description, tags)
//! - Heading hierarchy
//! - Body text content
//! - Links (internal and external)
//! - Custom front-matter fields

use rusqlite::params;

use super::database::RepositoryIndex;

impl RepositoryIndex {
    /// Export all documents in the index as a JSON array.
    ///
    /// Each entry contains: path, title, type, description, tags,
    /// headings, body_text, links, and front-matter custom fields.
    #[allow(dead_code)]
    pub fn export_to_json(&self) -> Result<serde_json::Value, anyhow::Error> {
        let mut stmt = self.conn.prepare(
            "SELECT path, title, type, description, body_text, file_size, modified_at, parse_status
             FROM documents
             ORDER BY path",
        )?;

        let rows = stmt.query_map([], |row| {
            Ok(serde_json::json!({
                "path": row.get::<_, String>(0)?,
                "title": row.get::<_, Option<String>>(1)?,
                "type": row.get::<_, Option<String>>(2)?,
                "description": row.get::<_, Option<String>>(3)?,
                "body_text": row.get::<_, String>(4)?,
                "file_size": row.get::<_, i64>(5)?,
                "modified_at": row.get::<_, i64>(6)?,
                "parse_status": row.get::<_, String>(7)?,
            }))
        })?;

        let mut docs: Vec<serde_json::Value> = Vec::new();
        for row in rows {
            let mut doc = row?;

            let path = doc["path"].as_str().unwrap_or("").to_string();

            // Attach tags
            let mut tag_stmt = self.conn.prepare(
                "SELECT tag FROM document_tags dt
                 JOIN documents d ON d.id = dt.document_id
                 WHERE d.path = ?1",
            )?;
            let tags: Vec<String> = tag_stmt
                .query_map(params![&path], |r| r.get::<_, String>(0))?
                .filter_map(|r| r.ok())
                .collect();
            doc.as_object_mut()
                .map(|m| m.insert("tags".to_string(), serde_json::json!(tags)));

            // Attach headings
            let mut h_stmt = self.conn.prepare(
                "SELECT h.level, h.title, h.anchor FROM headings h
                 JOIN documents d ON d.id = h.document_id
                 WHERE d.path = ?1
                 ORDER BY h.position",
            )?;
            let headings: Vec<serde_json::Value> = h_stmt
                .query_map(params![&path], |r| {
                    Ok(serde_json::json!({
                        "level": r.get::<_, i32>(0)?,
                        "title": r.get::<_, String>(1)?,
                        "anchor": r.get::<_, Option<String>>(2)?,
                    }))
                })?
                .filter_map(|r| r.ok())
                .collect();
            doc.as_object_mut()
                .map(|m| m.insert("headings".to_string(), serde_json::json!(headings)));

            // Attach metadata custom fields
            let mut m_stmt = self.conn.prepare(
                "SELECT key, value FROM metadata_fields mf
                 JOIN documents d ON d.id = mf.document_id
                 WHERE d.path = ?1",
            )?;
            let custom: serde_json::Map<String, serde_json::Value> = m_stmt
                .query_map(params![&path], |r| {
                    let key: String = r.get(0)?;
                    let val: String = r.get(1)?;
                    Ok((
                        key,
                        serde_json::from_str(&val).unwrap_or(serde_json::Value::String(val)),
                    ))
                })?
                .filter_map(|r| r.ok())
                .collect();
            doc.as_object_mut()
                .map(|m| m.insert("custom".to_string(), serde_json::Value::Object(custom)));

            // Attach links
            let mut l_stmt = self.conn.prepare(
                "SELECT l.target_path, l.target_anchor, l.external_url, l.exists_in_repository
                 FROM links l
                 JOIN documents d ON d.id = l.source_document_id
                 WHERE d.path = ?1",
            )?;
            let links: Vec<serde_json::Value> = l_stmt
                .query_map(params![&path], |r| {
                    Ok(serde_json::json!({
                        "target_path": r.get::<_, Option<String>>(0)?,
                        "target_anchor": r.get::<_, Option<String>>(1)?,
                        "external_url": r.get::<_, Option<String>>(2)?,
                        "exists_in_repository": r.get::<_, i32>(3)? != 0,
                    }))
                })?
                .filter_map(|r| r.ok())
                .collect();
            doc.as_object_mut()
                .map(|m| m.insert("links".to_string(), serde_json::json!(links)));

            docs.push(doc);
        }

        Ok(serde_json::Value::Array(docs))
    }
}
