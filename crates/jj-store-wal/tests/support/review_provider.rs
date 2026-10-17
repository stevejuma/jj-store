#![allow(dead_code)]
use jj_review::{
    ReviewCatalogKey, ReviewCatalogKind, ReviewObjectRef, ReviewObjectV1, ReviewRootV1,
};
use jj_store_core as wal;
use jj_store_model::RepositoryId;
use jj_store_object::memory::MemoryStore;
use jj_store_wal::{
    publication::{ReviewCatalogEditor, ReviewObjectIo, ReviewProjector, ReviewSnapshotRead},
    records::{RecordKind, RecordRef, Records},
};
use std::{collections::BTreeSet, sync::Arc};

pub struct Provider {
    pub records: Records,
    pub repository: RepositoryId,
}
impl Provider {
    pub fn new(repository: RepositoryId) -> Self {
        Self {
            records: Records::new(Arc::new(MemoryStore::default()), repository),
            repository,
        }
    }
    pub async fn pin(&self, root: ReviewRootV1) -> Pinned<'_> {
        let closure = ReviewProjector::new(self).audit_root(&root).await.unwrap();
        assert!(closure.native.is_empty());
        assert!(closure.agent_objects.is_empty());
        Pinned {
            provider: self,
            root,
            objects: closure.objects,
        }
    }
}
#[async_trait::async_trait]
impl ReviewObjectIo for Provider {
    fn repository(&self) -> RepositoryId {
        self.repository
    }
    async fn read_review_object(&self, reference: ReviewObjectRef) -> wal::Result<ReviewObjectV1> {
        let object: ReviewObjectV1 = self
            .records
            .get(
                RecordRef {
                    kind: RecordKind::ReviewObject,
                    digest: jj_store_model::RecordDigest::new(*reference.id.as_bytes()),
                    attempt: None,
                },
                RecordKind::ReviewObject,
            )
            .await?;
        if object.kind() != reference.kind {
            return Err(wal::Error::Corrupt("provider object kind"));
        }
        object
            .validate()
            .map_err(|_| wal::Error::Corrupt("provider object"))?;
        Ok(object)
    }
    async fn stage_review_object(&self, object: &ReviewObjectV1) -> wal::Result<ReviewObjectRef> {
        object
            .validate()
            .map_err(|_| wal::Error::Corrupt("provider object"))?;
        // This provider deliberately supports repository targets only.
        if !object.native_dependencies().unwrap().is_empty()
            || !object.agent_dependencies().unwrap().is_empty()
        {
            return Err(wal::Error::Corrupt("provider external evidence"));
        }
        for dependency in object.review_dependencies().unwrap() {
            self.read_review_object(dependency).await?;
        }
        let reference = self
            .records
            .put(RecordKind::ReviewObject, None, object)
            .await?;
        Ok(ReviewObjectRef::new(
            object.kind(),
            jj_review::ReviewObjectId::new(*reference.digest.as_bytes()),
        ))
    }
}
pub struct Pinned<'a> {
    provider: &'a Provider,
    root: ReviewRootV1,
    objects: BTreeSet<ReviewObjectRef>,
}
#[async_trait::async_trait]
impl ReviewSnapshotRead for Pinned<'_> {
    fn repository(&self) -> RepositoryId {
        self.provider.repository
    }
    fn root(&self) -> &ReviewRootV1 {
        &self.root
    }
    async fn lookup(
        &self,
        catalog: ReviewCatalogKind,
        key: ReviewCatalogKey,
    ) -> wal::Result<Option<ReviewObjectRef>> {
        let root = match catalog {
            ReviewCatalogKind::Threads => self.root.threads,
            ReviewCatalogKind::Comments => self.root.comments,
            ReviewCatalogKind::ThreadEvents => self.root.thread_events,
            ReviewCatalogKind::AnchorRelations => self.root.anchor_relations,
            ReviewCatalogKind::ArtifactRelations => self.root.artifact_relations,
            ReviewCatalogKind::Statuses => self.root.statuses,
        };
        ReviewCatalogEditor::new(self.provider)
            .lookup_review_catalog(root, key)
            .await
    }
    async fn read_object(&self, reference: ReviewObjectRef) -> wal::Result<ReviewObjectV1> {
        if !self.objects.contains(&reference) {
            return Err(wal::Error::Stale);
        }
        self.provider.read_review_object(reference).await
    }
}
