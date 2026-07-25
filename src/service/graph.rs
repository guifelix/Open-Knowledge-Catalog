//! Graph traversal and link navigation service.
//!
//! Provides link graph operations: forward links, backlinks, and
//! multi-hop traversal with depth and node limits.

use crate::model::document::LinkInfo;
use crate::model::graph::TraverseResponse;
use crate::service::OkcService;

impl OkcService {
    /// Get forward links from a document.
    ///
    /// Returns all links originating from the given document with
    /// resolution status (exists in repo, external, broken).
    pub fn get_links(&self, path: &str) -> Result<Vec<LinkInfo>, anyhow::Error> {
        self.index.get_links(path)
    }

    /// Get backlinks to a document.
    ///
    /// Returns documents that link to the given path, limited by `limit`.
    pub fn get_backlinks(&self, path: &str, limit: usize) -> Result<Vec<LinkInfo>, anyhow::Error> {
        self.index.get_backlinks(path, limit)
    }

    /// Traverse the link graph from a starting document.
    ///
    /// - `relations`: Link relation types to follow (empty = all)
    /// - `max_depth`: Maximum hops from start
    /// - `max_nodes`: Maximum total nodes to visit
    pub fn traverse(
        &self,
        start: &str,
        relations: &[String],
        max_depth: usize,
        max_nodes: usize,
    ) -> Result<TraverseResponse, anyhow::Error> {
        self.index
            .traverse_graph(start, relations, max_depth, max_nodes)
    }
}
