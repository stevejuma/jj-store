//! Immutable ReviewStore capabilities; publication remains adapter-owned.
use super::*;
use jj_review::{ReviewCatalogKey, ReviewCatalogKind};

/// Implementations bind one repository and validate canonical identity, kind
/// and complete immutable dependencies before returning an object.
#[async_trait::async_trait]
pub trait ReviewObjectIo: Send + Sync {
    fn repository(&self) -> RepositoryId;
    async fn read_review_object(&self, reference: ReviewObjectRef) -> Result<ReviewObjectV1>;
    async fn stage_review_object(&self, object: &ReviewObjectV1) -> Result<ReviewObjectRef>;
}
/// A read facade for one audited root. Staged objects are not readable here.
#[async_trait::async_trait]
pub trait ReviewSnapshotRead: Send + Sync {
    fn repository(&self) -> RepositoryId;
    fn root(&self) -> &ReviewRootV1;
    async fn lookup(
        &self,
        catalog: ReviewCatalogKind,
        key: ReviewCatalogKey,
    ) -> Result<Option<ReviewObjectRef>>;
    async fn read_object(&self, reference: ReviewObjectRef) -> Result<ReviewObjectV1>;
}
