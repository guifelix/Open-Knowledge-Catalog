mod browse;
mod documents;
mod graph;
mod search;
mod validation;
mod watch;

use crate::config::OkcConfig;
use crate::index::RepositoryIndex;
use crate::model::*;

pub struct OkcService {
    pub(crate) index: RepositoryIndex,
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

    #[allow(dead_code)]
    pub fn export_to_json(&self) -> Result<serde_json::Value, anyhow::Error> {
        self.index.export_to_json()
    }
}
