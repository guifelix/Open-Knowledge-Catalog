//! Link queries and graph traversal for RepositoryIndex.
//!
//! Public API: get_links, get_backlinks, traverse_graph.

use rusqlite::params;

use super::database::RepositoryIndex;
use crate::model::document::LinkInfo;
use crate::model::graph::{GraphEdge, TraverseNode, TraverseResponse};

impl RepositoryIndex {
    pub fn get_links(&self, doc_path: &str) -> Result<Vec<LinkInfo>, anyhow::Error> {
        let mut stmt = self.conn.prepare(
            "SELECT l.target_path, l.target_anchor, l.external_url, l.exists_in_repository
             FROM links l
             JOIN documents d ON d.id = l.source_document_id
             WHERE d.path = ?1",
        )?;

        let links = stmt
            .query_map(params![doc_path], |row| {
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

    pub fn get_backlinks(
        &self,
        doc_path: &str,
        limit: usize,
    ) -> Result<Vec<LinkInfo>, anyhow::Error> {
        let mut stmt = self.conn.prepare(
            "SELECT l.target_path, l.target_anchor, l.external_url, l.exists_in_repository
             FROM links l
             WHERE l.target_path = ?1
             LIMIT ?2",
        )?;

        let backlinks = stmt
            .query_map(params![doc_path, limit as i64], |row| {
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

    pub fn traverse_graph(
        &self,
        start: &str,
        relations: &[String],
        max_depth: usize,
        max_nodes: usize,
    ) -> Result<TraverseResponse, anyhow::Error> {
        let mut visited = std::collections::HashSet::new();
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((start.to_string(), 0usize));

        let effective_max_depth = max_depth.min(self.config.max_graph_depth);
        let effective_max_nodes = max_nodes.min(self.config.max_graph_nodes);

        while let Some((current_path, depth)) = queue.pop_front() {
            if visited.len() >= effective_max_nodes {
                break;
            }
            if !visited.insert(current_path.clone()) {
                continue;
            }

            let title = self
                .conn
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

            if depth >= effective_max_depth {
                continue;
            }

            let link_condition = if relations.is_empty() {
                "1=1".to_string()
            } else {
                let _or_conds: Vec<String> = relations
                    .iter()
                    .map(|r| format!("'{}'", r.replace('\'', "''")))
                    .collect();
                "target_path IS NOT NULL AND target_path != ''".to_string()
            };

            let sql = format!(
                "SELECT target_path FROM links l
                 JOIN documents d ON d.id = l.source_document_id
                 WHERE d.path = ?1 AND {}
                 LIMIT ?2",
                link_condition
            );

            if let Ok(mut stmt) = self.conn.prepare(&sql) {
                if let Ok(rows) = stmt.query_map(
                    params![current_path, (effective_max_nodes - visited.len()) as i64],
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
            }

            if let Ok(mut stmt) = self.conn.prepare(
                "SELECT d.path FROM links l
                 JOIN documents d ON d.id = l.source_document_id
                 WHERE l.target_path = ?1
                 LIMIT ?2",
            ) {
                if let Ok(rows) = stmt.query_map(
                    params![current_path, (effective_max_nodes - visited.len()) as i64],
                    |row| row.get::<_, String>(0),
                ) {
                    for source in rows.flatten() {
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
            }
        }

        let truncated = visited.len() >= effective_max_nodes;

        Ok(TraverseResponse {
            nodes,
            edges,
            truncated,
        })
    }
}
