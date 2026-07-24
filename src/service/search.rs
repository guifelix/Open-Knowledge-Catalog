use crate::index::RepositoryIndex;
use crate::model::*;
use anyhow::Result;

pub fn search(
    index: &RepositoryIndex,
    query: &str,
    path_prefix: Option<&str>,
    types: Option<&[String]>,
    tags: Option<&[String]>,
    limit: usize,
) -> Result<SearchResponse> {
    index.search(query, path_prefix, types, tags, limit)
}

pub fn query_metadata(
    index: &RepositoryIndex,
    filters: &std::collections::HashMap<String, String>,
    select: &[String],
    limit: usize,
) -> Result<MetadataQueryResponse> {
    index.query_metadata(filters, select, limit)
}
