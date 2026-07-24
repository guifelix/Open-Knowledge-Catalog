use crate::index::RepositoryIndex;
use crate::model::*;
use anyhow::Result;

pub fn traverse(
    index: &RepositoryIndex,
    start: &str,
    relations: &[String],
    max_depth: usize,
    max_nodes: usize,
) -> Result<TraverseResponse> {
    index.traverse_graph(start, relations, max_depth, max_nodes)
}
