use super::*;
use jj_review::{ReviewCatalogKey, ReviewCatalogKind, ReviewThreadId};
impl ReviewStoreSnapshot {
    pub async fn lookup_thread(
        &self,
        thread_id: ReviewThreadId,
    ) -> Result<Option<ReviewObjectRef>> {
        let key = ReviewCatalogKey::derive("thread", thread_id.as_bytes())
            .map_err(super::review::review_error)?;
        self.lookup(ReviewCatalogKind::Threads, key).await
    }
}
