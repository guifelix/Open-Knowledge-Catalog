use crate::index::traits::*;
use crate::model::*;
use rusqlite::{params, Connection, OptionalExtension};
use std::collections::VecDeque;
use std::sync::Mutex;

pub struct SqliteGraphStore {
    conn: Mutex<Connection>,
}

impl SqliteGraphStore {
    pub fn new(conn: Connection) -> Self {
        Self {
            conn: Mutex::new(conn),
        }
    }
}

impl GraphStore for SqliteGraphStore {
    fn init(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(
            r#"
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
            "#,
        )?;
        Ok(())
    }

    fn store_links(&self, source_path: &str, links: &[Link]) -> Result<()> {
        let mut conn = self.conn.lock().unwrap();
        let source_id = conn
            .query_row(
                "SELECT id FROM documents WHERE path = ?1",
                params![source_path],
                |row| row.get::<_, i64>(0),
            )
            .optional()?;

        let Some(source_id) = source_id else {
            return Ok(());
        };

        let tx = conn.transaction()?;
        tx.execute(
            "DELETE FROM links WHERE source_document_id = ?1",
            params![source_id],
        )?;

        for link in links {
            tx.execute(
                r#"
                INSERT INTO links (source_document_id, target_path, target_anchor, external_url, exists_in_repository)
                VALUES (?1, ?2, ?3, ?4, ?5)
                "#,
                params![
                    source_id,
                    if link.is_external { None } else { Some(&link.target) },
                    link.target_anchor.clone(),
                    if link.is_external { Some(&link.target) } else { None },
                    if link.is_external { 1 } else { link.exists_in_repository as i32 },
                ],
            )?;
        }
        tx.commit()?;
        Ok(())
    }

    fn remove_links(&self, source_path: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "DELETE FROM links WHERE source_document_id = (SELECT id FROM documents WHERE path = ?1)",
            params![source_path],
        )?;
        Ok(())
    }

    fn get_links(&self, path: &str) -> Result<Vec<LinkInfo>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            r#"
            SELECT l.target_path, l.target_anchor, l.external_url, l.exists_in_repository
            FROM links l
            JOIN documents d ON d.id = l.source_document_id
            WHERE d.path = ?1
            "#,
        )?;

        let links = stmt
            .query_map(params![path], |row| {
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

    fn get_backlinks(&self, path: &str, limit: usize) -> Result<Vec<LinkInfo>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            r#"
            SELECT l.target_path, l.target_anchor, l.external_url, l.exists_in_repository
            FROM links l
            WHERE l.target_path = ?1
            LIMIT ?2
            "#,
        )?;

        let backlinks = stmt
            .query_map(params![path, limit as i64], |row| {
                Ok(LinkInfo {
                    target_path: row.get(0)?,
                    target_anchor: row.get(1)?,
                    external_url: row.get(2)?,
                    exists_in_repository: row.get::<_, i32>(3)? != 0,
                })
            })?
            .filter_map(|r| r.ok())
            .collect();

        Ok(backlinks)
    }

    fn traverse(
        &self,
        start: &str,
        _relations: &[String],
        max_depth: usize,
        max_nodes: usize,
    ) -> Result<TraverseResponse> {
        let conn = self.conn.lock().unwrap();
        let mut visited = std::collections::HashSet::new();
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back((start.to_string(), 0usize));

        while let Some((current_path, depth)) = queue.pop_front() {
            if visited.len() >= max_nodes {
                break;
            }
            if !visited.insert(current_path.clone()) {
                continue;
            }

            let title = conn
                .query_row(
                    "SELECT title, type FROM documents WHERE path = ?1",
                    params![current_path],
                    |row| {
                        Ok((
                            row.get::<_, Option<String>>(0)?,
                            row.get::<_, Option<String>>(1)?,
                        ))
                    },
                )
                .ok();

            let (title, ctype) = title.unwrap_or((None, None));

            nodes.push(TraverseNode {
                path: current_path.clone(),
                title,
                concept_type: ctype,
                depth,
            });

            if depth >= max_depth {
                continue;
            }

            let mut stmt = conn.prepare(
                r#"
                SELECT l.target_path
                FROM links l
                JOIN documents d ON d.id = l.source_document_id
                WHERE d.path = ?1 AND l.target_path IS NOT NULL AND l.target_path != ''
                LIMIT ?2
                "#,
            )?;

            if let Ok(rows) = stmt.query_map(
                params![current_path, (max_nodes - visited.len()) as i64],
                |row| row.get::<_, String>(0),
            ) {
                for target in rows.flatten() {
                    if !visited.contains(&target) {
                        edges.push(GraphEdge {
                            source: current_path.clone(),
                            target: target.clone(),
                            relation: "links_to".to_string(),
                        });
                        queue.push_back((target, depth + 1));
                    }
                }
            }

            let mut stmt = conn.prepare(
                r#"
                SELECT d.path
                FROM links l
                JOIN documents d ON d.id = l.source_document_id
                WHERE l.target_path = ?1
                LIMIT ?2
                "#,
            )?;
            let rows: Vec<String> = stmt
                .query_map(
                    params![current_path, (max_nodes - visited.len()) as i64],
                    |row| row.get::<_, String>(0),
                )?
                .filter_map(|r| r.ok())
                .collect();

            for source in rows {
                if !visited.contains(&source) {
                    edges.push(GraphEdge {
                        source: source.clone(),
                        target: current_path.clone(),
                        relation: "linked_from".to_string(),
                    });
                    queue.push_back((source, depth + 1));
                }
            }
        }

        let truncated = visited.len() >= max_nodes;

        Ok(TraverseResponse {
            nodes,
            edges,
            truncated,
        })
    }

    fn validate_links(&self) -> Result<Vec<ValidationIssue>> {
        let conn = self.conn.lock().unwrap();
        let mut issues = Vec::new();

        // Broken links
        let mut stmt = conn.prepare(
            r#"
            SELECT d.path, l.target_path
            FROM links l
            JOIN documents d ON d.id = l.source_document_id
            WHERE l.exists_in_repository = 0 AND l.external_url IS NULL
            "#,
        )?;
        for row in stmt.query_map([], |row| {
            Ok(ValidationIssue {
                path: row.get(0)?,
                severity: "warning".to_string(),
                category: "broken_link".to_string(),
                message: format!("Broken internal link to '{}'", row.get::<_, String>(1)?),
                line: None,
            })
        })? {
            issues.push(row?);
        }

        // Scan errors
        let mut stmt =
            conn.prepare("SELECT path, stage, message, line FROM scan_errors ORDER BY path")?;
        for row in stmt.query_map([], |row| {
            Ok(ValidationIssue {
                path: row.get(0)?,
                severity: "error".to_string(),
                category: row.get::<_, String>(1)?,
                message: row.get(2)?,
                line: row.get::<_, Option<i64>>(3)?.map(|l| l as usize),
            })
        })? {
            issues.push(row?);
        }

        Ok(issues)
    }

    fn detect_circular_references(&self) -> Result<Vec<ValidationIssue>> {
        let conn = self.conn.lock().unwrap();

        let mut graph: std::collections::HashMap<String, Vec<String>> =
            std::collections::HashMap::new();
        let mut nodes: std::collections::HashSet<String> =
            std::collections::HashSet::new();

        let mut stmt = conn.prepare(
            "SELECT d.path, l.target_path
             FROM links l
             JOIN documents d ON d.id = l.source_document_id
             WHERE l.target_path IS NOT NULL AND l.external_url IS NULL AND l.exists_in_repository = 1"
        )?;

        for row in stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })? {
            let (source, target) = row?;
            graph.entry(source.clone()).or_default().push(target.clone());
            nodes.insert(source);
            nodes.insert(target);
        }

        let mut color: std::collections::HashMap<String, u8> =
            nodes.iter().map(|n| (n.clone(), 0)).collect();
        let mut issues = Vec::new();

        for start in &nodes {
            if color[start] != 0 {
                continue;
            }

            color.insert(start.clone(), 1);
            let mut stack: Vec<(String, usize, Vec<String>)> =
                vec![(start.clone(), 0, vec![start.clone()])];

            while let Some((node, idx, path)) = stack.last_mut() {
                let neighbors = graph.get(node).cloned().unwrap_or_default();

                if *idx >= neighbors.len() {
                    color.insert(node.clone(), 2);
                    stack.pop();
                    continue;
                }

                let neighbor = neighbors[*idx].clone();
                *idx += 1;

                match color.get(&neighbor).copied().unwrap_or(2) {
                    0 => {
                        color.insert(neighbor.clone(), 1);
                        let mut new_path = path.clone();
                        new_path.push(neighbor.clone());
                        stack.push((neighbor.clone(), 0, new_path));
                    }
                    1 => {
                        if let Some(cycle_start) = path.iter().position(|n| *n == neighbor) {
                            let cycle: Vec<&str> = path[cycle_start..]
                                .iter().map(|s| s.as_str()).collect();
                            issues.push(ValidationIssue {
                                path: node.clone(),
                                severity: "warning".to_string(),
                                category: "circular_references".to_string(),
                                message: format!("Circular reference: {}", cycle.join(" -> ")),
                                line: None,
                            });
                        }
                    }
                    _ => {}
                }
            }
        }

        issues.dedup_by(|a, b| a.message == b.message);
        Ok(issues)
    }
}
