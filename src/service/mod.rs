use crate::config::OkcConfig;
use crate::index::RepositoryIndex;
use crate::model::*;

pub struct OkcService {
    index: RepositoryIndex,
}

impl OkcService {
    pub fn open(config: &OkcConfig) -> Result<Self, anyhow::Error> {
        let index = RepositoryIndex::open(config)?;
        Ok(Self { index })
    }

    #[allow(dead_code)]
    pub fn open_in_memory(config: &OkcConfig) -> Result<Self, anyhow::Error> {
        let index = RepositoryIndex::open_in_memory(config)?;
        Ok(Self { index })
    }

    pub fn scan(&mut self) -> Result<ScanResult, anyhow::Error> {
        self.index.scan()
    }

    pub fn browse(
        &self,
        path: &str,
        depth: usize,
        limit: usize,
    ) -> Result<BrowseResponse, anyhow::Error> {
        self.index.browse_directory(path, depth, limit)
    }

    pub fn get_document(
        &self,
        path: &str,
        include: &[String],
        max_chars: usize,
    ) -> Result<DocumentDetail, anyhow::Error> {
        self.index.get_document(path, include, max_chars)
    }

    pub fn get_section(
        &self,
        path: &str,
        heading: &str,
        max_chars: usize,
    ) -> Result<Option<(String, String)>, anyhow::Error> {
        self.index.get_section(path, heading, max_chars)
    }

    pub fn search(
        &self,
        query: &str,
        path_prefix: Option<&str>,
        types: Option<&[String]>,
        tags: Option<&[String]>,
        limit: usize,
    ) -> Result<SearchResponse, anyhow::Error> {
        self.index.search(query, path_prefix, types, tags, limit)
    }

    pub fn query_metadata(
        &self,
        filters: &std::collections::HashMap<String, String>,
        select: &[String],
        limit: usize,
    ) -> Result<MetadataQueryResponse, anyhow::Error> {
        self.index.query_metadata(filters, select, limit)
    }

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

    pub fn validate(&self) -> Result<Vec<ValidationIssue>, anyhow::Error> {
        self.index.validate()
    }

    #[allow(dead_code)]
    pub fn get_recently_modified(
        &self,
        limit: usize,
    ) -> Result<Vec<DocumentSummary>, anyhow::Error> {
        self.index.get_recently_modified(limit)
    }

    pub fn get_stats(&self) -> Result<IndexStats, anyhow::Error> {
        self.index.get_stats()
    }

    #[allow(dead_code)]
    pub fn export_to_json(&self) -> Result<serde_json::Value, anyhow::Error> {
        self.index.export_to_json()
    }
}
