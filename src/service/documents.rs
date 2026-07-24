use crate::model::document::{DocumentDetail, DocumentSummary};
use crate::service::OkcService;

impl OkcService {
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

    #[allow(dead_code)]
    pub fn get_recently_modified(
        &self,
        limit: usize,
    ) -> Result<Vec<DocumentSummary>, anyhow::Error> {
        self.index.get_recently_modified(limit)
    }
}
