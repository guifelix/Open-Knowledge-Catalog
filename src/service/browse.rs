use crate::index::RepositoryIndex;
use crate::model::*;
use anyhow::Result;

pub fn browse(
    index: &RepositoryIndex,
    path: &str,
    depth: usize,
    limit: usize,
) -> Result<BrowseResponse> {
    index.browse_directory(path, depth, limit)
}

pub fn scan(index: &mut RepositoryIndex) -> Result<ScanResult> {
    index.scan()
}
