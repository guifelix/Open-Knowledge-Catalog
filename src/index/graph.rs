//! Link queries and graph traversal for RepositoryIndex.
//!
//! Public API: get_links, get_backlinks, traverse_graph.

use super::database::RepositoryIndex;
use crate::error::Result;
use crate::index::traits::relation_condition;
use crate::model::document::LinkInfo;
use crate::model::graph::{GraphEdge, TraverseNode, TraverseResponse};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, Row};

/// Parse a `LinkInfo` from a links-table row that selects, in order:
/// `target_path, target_anchor, external_url, exists_in_repository, relation`.
fn link_info_from_row(row: &Row<'_>) -> rusqlite::Result<LinkInfo> {
    Ok(LinkInfo {
        target_path: row.get(0)?,
        target_anchor: row.get(1)?,
        external_url: row.get(2)?,
        exists_in_repository: row.get::<_, i32>(3)? != 0,
        relation: row.get(4)?,
    })
}

impl RepositoryIndex {
    /// Get forward links from a document.
    ///
    /// Returns all links originating from the given document with
    /// resolution status (exists in repo, external, broken). When
    /// `relation_filter` is `Some`, only edges whose stored relation equals
    /// that value are returned (untyped NULL-relation links are excluded);
    /// `None` returns all edges including untyped links.
    pub fn get_links(
        &self,
        doc_path: &str,
        relation_filter: Option<&str>,
    ) -> Result<Vec<LinkInfo>> {
        let conn = self.pool().get()?;
        let mut sql = String::from(
            "SELECT l.target_path, l.target_anchor, l.external_url, l.exists_in_repository, l.relation
             FROM links l
             JOIN documents d ON d.id = l.source_document_id
             WHERE d.path = ?1",
        );
        if relation_filter.is_some() {
            sql.push_str(" AND l.relation = ?2");
        }

        let mut stmt = conn.prepare(&sql)?;
        let links = if let Some(relation) = relation_filter {
            stmt.query_map(params![doc_path, relation], link_info_from_row)?
                .filter_map(|r| r.ok())
                .collect()
        } else {
            stmt.query_map(params![doc_path], link_info_from_row)?
                .filter_map(|r| r.ok())
                .collect()
        };

        Ok(links)
    }

    /// Get backlinks to a document.
    ///
    /// Returns documents that link to the given path, limited by `limit`.
    /// When `relation_filter` is `Some`, only edges carrying that relation are
    /// returned; `None` returns all edges including untyped (NULL-relation)
    /// links.
    pub fn get_backlinks(
        &self,
        doc_path: &str,
        limit: usize,
        relation_filter: Option<&str>,
    ) -> Result<Vec<LinkInfo>> {
        let conn = self.pool().get()?;
        let mut sql = String::from(
            "SELECT l.target_path, l.target_anchor, l.external_url, l.exists_in_repository, l.relation
             FROM links l
             WHERE l.target_path = ?1",
        );
        let mut bound: Vec<Box<dyn rusqlite::ToSql>> = vec![Box::new(doc_path)];
        if let Some(relation) = relation_filter {
            sql.push_str(" AND l.relation = ?2");
            bound.push(Box::new(relation));
        }
        let limit_pos = bound.len() + 1;
        sql.push_str(&format!(" LIMIT ?{limit_pos}"));
        bound.push(Box::new(limit as i64));

        let mut stmt = conn.prepare(&sql)?;
        let backlinks = stmt
            .query_map(rusqlite::params_from_iter(bound.iter()), |row| {
                link_info_from_row(row)
            })?
            .filter_map(|r| r.ok())
            .collect();

        Ok(backlinks)
    }

    /// Traverse the link graph from a starting document.
    ///
    /// Performs breadth-first traversal following links matching the given
    /// relation types. Respects `max_depth` and `max_nodes` limits.
    pub fn traverse_graph(
        &self,
        start: &str,
        relations: &[String],
        max_depth: usize,
        max_nodes: usize,
    ) -> Result<TraverseResponse> {
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

            let conn = self.pool().get()?;
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

            if depth >= effective_max_depth {
                continue;
            }

            let link_condition = relation_condition(relations);

            let sql = format!(
                "SELECT target_path FROM links l
                 JOIN documents d ON d.id = l.source_document_id
                 WHERE d.path = ?1 AND target_path IS NOT NULL AND target_path != '' {link_condition}
                 LIMIT ?2"
            );

            // First query: forward links
            {
                let conn = self.pool().get()?;
                let mut stmt = conn.prepare(&sql)?;
                let rows = stmt.query_map(
                    params![current_path, (effective_max_nodes - visited.len()) as i64],
                    |row| row.get::<_, String>(0),
                )?;
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

            // Second query: backlinks
            {
                let conn = self.pool().get()?;
                let backlink_sql = format!(
                    "SELECT d.path FROM links l
                     JOIN documents d ON d.id = l.source_document_id
                     WHERE l.target_path = ?1 {link_condition}
                     LIMIT ?2"
                );
                let mut stmt = conn.prepare(&backlink_sql)?;
                let rows: Vec<String> = stmt
                    .query_map(
                        params![current_path, (effective_max_nodes - visited.len()) as i64],
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
        }

        let truncated = visited.len() >= effective_max_nodes;

        Ok(TraverseResponse {
            nodes,
            edges,
            truncated,
        })
    }
}
