use crate::index::RepositoryIndex;
use crate::model::*;
use anyhow::Result;

pub fn get_document(
    index: &RepositoryIndex,
    path: &str,
    include: &[String],
    max_chars: usize,
) -> Result<DocumentDetail> {
    index.get_document(path, include, max_chars)
}

pub fn get_section(
    index: &RepositoryIndex,
    path: &str,
    heading: &str,
    max_chars: usize,
) -> Result<Option<(String, String)>> {
    index.get_section(path, heading, max_chars)
}

pub fn get_links(index: &RepositoryIndex, path: &str) -> Result<Vec<LinkInfo>> {
    index.get_links(path)
}

pub fn get_backlinks(index: &RepositoryIndex, path: &str, limit: usize) -> Result<Vec<LinkInfo>> {
    index.get_backlinks(path, limit)
}

pub fn get_recently_modified(
    index: &RepositoryIndex,
    limit: usize,
) -> Result<Vec<DocumentSummary>> {
    index.get_recently_modified(limit)
}

pub fn export_to_json(index: &RepositoryIndex) -> Result<serde_json::Value> {
    index.export_to_json()
}
