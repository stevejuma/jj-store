use super::*;
use jj_review::{ReviewCatalogKey, ReviewCatalogKind};
#[async_trait::async_trait]
impl ReviewObjectIo for Coordinator {
    fn repository(&self) -> RepositoryId {
        self.records.repository
    }
    async fn read_review_object(
        &self,
        reference: ReviewObjectRef,
    ) -> jj_store_core::Result<ReviewObjectV1> {
        Coordinator::read_review_object(self, reference)
            .await
            .map_err(Into::into)
    }
    async fn stage_review_object(
        &self,
        object: &ReviewObjectV1,
    ) -> jj_store_core::Result<ReviewObjectRef> {
        Coordinator::stage_review_object(self, object)
            .await
            .map_err(Into::into)
    }
}
#[async_trait::async_trait]
impl ReviewSnapshotRead for ReviewStoreSnapshot {
    fn repository(&self) -> RepositoryId {
        self.repository_id()
    }
    fn root(&self) -> &ReviewRootV1 {
        ReviewStoreSnapshot::root(self)
    }
    async fn lookup(
        &self,
        catalog: ReviewCatalogKind,
        key: ReviewCatalogKey,
    ) -> jj_store_core::Result<Option<ReviewObjectRef>> {
        ReviewStoreSnapshot::lookup(self, catalog, key)
            .await
            .map_err(Into::into)
    }
    async fn read_object(
        &self,
        reference: ReviewObjectRef,
    ) -> jj_store_core::Result<ReviewObjectV1> {
        ReviewStoreSnapshot::read_object(self, reference)
            .await
            .map_err(Into::into)
    }
}
