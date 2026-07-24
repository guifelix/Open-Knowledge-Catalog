use crate::model::directory::BrowseResponse;
use crate::service::OkcService;

impl OkcService {
    pub fn browse(
        &self,
        path: &str,
        depth: usize,
        limit: usize,
    ) -> Result<BrowseResponse, anyhow::Error> {
        self.index.browse_directory(path, depth, limit)
    }
}
