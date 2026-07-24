pub mod browse;
pub mod documents;
pub mod graph;
pub mod search;
pub mod stats;
pub mod validation;

use crate::config::OkcConfig;
use crate::index::RepositoryIndex;
use crate::model::*;
use crate::service::browse::browse as browse_impl;
use crate::service::browse::scan as scan_impl;
use crate::service::documents::*;
use crate::service::graph::traverse as traverse_impl;
use crate::service::search::*;
use crate::service::stats::get_stats as get_stats_impl;
use crate::service::validation::*;

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
        scan_impl(&mut self.index)
    }

    pub fn browse(
        &self,
        path: &str,
        depth: usize,
        limit: usize,
    ) -> Result<BrowseResponse, anyhow::Error> {
        browse_impl(&self.index, path, depth, limit)
    }

    pub fn get_document(
        &self,
        path: &str,
        include: &[String],
        max_chars: usize,
    ) -> Result<DocumentDetail, anyhow::Error> {
        get_document(&self.index, path, include, max_chars)
    }

    pub fn get_section(
        &self,
        path: &str,
        heading: &str,
        max_chars: usize,
    ) -> Result<Option<(String, String)>, anyhow::Error> {
        get_section(&self.index, path, heading, max_chars)
    }

    pub fn search(
        &self,
        query: &str,
        path_prefix: Option<&str>,
        types: Option<&[String]>,
        tags: Option<&[String]>,
        limit: usize,
    ) -> Result<SearchResponse, anyhow::Error> {
        search(&self.index, query, path_prefix, types, tags, limit)
    }

    pub fn query_metadata(
        &self,
        filters: &std::collections::HashMap<String, String>,
        select: &[String],
        limit: usize,
    ) -> Result<MetadataQueryResponse, anyhow::Error> {
        query_metadata(&self.index, filters, select, limit)
    }

    pub fn get_links(&self, path: &str) -> Result<Vec<LinkInfo>, anyhow::Error> {
        get_links(&self.index, path)
    }

    pub fn get_backlinks(&self, path: &str, limit: usize) -> Result<Vec<LinkInfo>, anyhow::Error> {
        get_backlinks(&self.index, path, limit)
    }

    pub fn traverse(
        &self,
        start: &str,
        relations: &[String],
        max_depth: usize,
        max_nodes: usize,
    ) -> Result<TraverseResponse, anyhow::Error> {
        traverse_impl(&self.index, start, relations, max_depth, max_nodes)
    }

    pub fn validate(&self) -> Result<Vec<ValidationIssue>, anyhow::Error> {
        validate(&self.index)
    }

    pub fn validate_report(&self) -> Result<ValidationReport, anyhow::Error> {
        validate_report(&self.index)
    }

    #[allow(dead_code)]
    pub fn get_recently_modified(
        &self,
        limit: usize,
    ) -> Result<Vec<DocumentSummary>, anyhow::Error> {
        get_recently_modified(&self.index, limit)
    }

    pub fn get_stats(&self) -> Result<IndexStats, anyhow::Error> {
        get_stats_impl(&self.index)
    }

    #[allow(dead_code)]
    pub fn export_to_json(&self) -> Result<serde_json::Value, anyhow::Error> {
        export_to_json(&self.index)
    }
}
