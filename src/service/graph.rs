use crate::model::document::LinkInfo;
use crate::model::graph::TraverseResponse;
use crate::service::OkcService;

impl OkcService {
    pub fn get_links(&self, path: &str) -> Result<Vec<LinkInfo>, anyhow::Error> {
        self.index.get_links(path)
    }

    pub fn get_backlinks(&self, path: &str, limit: usize) -> Result<Vec<LinkInfo>, anyhow::Error> {
        self.index.get_backlinks(path, limit)
    }

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
