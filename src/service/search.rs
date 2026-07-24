use std::collections::HashMap;

use crate::model::{MetadataQueryResponse, SearchResponse};
use crate::service::OkcService;

impl OkcService {
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
        filters: &HashMap<String, String>,
        select: &[String],
        limit: usize,
    ) -> Result<MetadataQueryResponse, anyhow::Error> {
        self.index.query_metadata(filters, select, limit)
    }
}
