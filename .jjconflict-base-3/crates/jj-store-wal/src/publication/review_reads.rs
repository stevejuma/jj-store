//! Native wrappers preserve the public root-pinned read facade.
pub use super::review_read_engine::ReviewCatalogPage;
use super::*;
use jj_review::{ReviewCatalogKey, ReviewCatalogKind};
impl ReviewStoreSnapshot {
    pub async fn list_threads(
        &self,
        after: Option<ReviewCatalogKey>,
        limit: usize,
    ) -> Result<ReviewCatalogPage> {
        ReviewReader::new(self).list_threads(after, limit).await
    }
    pub async fn list_catalog(
        &self,
        catalog: ReviewCatalogKind,
        after: Option<ReviewCatalogKey>,
        limit: usize,
    ) -> Result<ReviewCatalogPage> {
        ReviewReader::new(self)
            .list_catalog(catalog, after, limit)
            .await
    }
}
