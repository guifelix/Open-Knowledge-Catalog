use crate::index::RepositoryIndex;
use crate::model::*;
use anyhow::Result;

pub fn get_stats(index: &RepositoryIndex) -> Result<IndexStats> {
    index.get_stats()
}
