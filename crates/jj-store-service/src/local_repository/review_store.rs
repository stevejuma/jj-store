use super::{
    LocalError, LocalProfile, LocalRepository, LocalResult,
    records::GitRecords,
    review_io::{LocalReviewRootV1, ReviewIo},
};
use jj_store_core as wal;
use jj_store_jj::local_metadata::{Domain, GitOid, Snapshot};
use jj_store_model as m;
use jj_store_wal::publication::{
    ReviewCatalogEditor, ReviewObjectIo, ReviewReader, ReviewSnapshotRead,
};
use jj_store_wal::publication::{
    ReviewCatalogKey, ReviewCatalogKind, ReviewObjectRef, ReviewObjectV1, ReviewRootV1,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

#[derive(
    Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct LocalReviewSnapshot {
    pub profile: LocalProfile,
    pub repo_id: Option<m::RepositoryId>,
    pub metadata_commit: Option<GitOid>,
    pub review_state_version: u64,
    pub review_root: Option<m::RecordDigest>,
}
pub struct LocalReviewStore {
    pub snapshot: LocalReviewSnapshot,
    pub(crate) io: Arc<ReviewIo>,
    root: ReviewRootV1,
    objects: BTreeSet<ReviewObjectRef>,
}
impl LocalReviewStore {
    /// The anchor must be reachable from this committed ReviewStore selection.
    pub async fn file_source(&self, reference: ReviewObjectRef) -> LocalResult<Vec<u8>> {
        let ReviewObjectV1::Anchor(anchor) = self.object(reference).await? else {
            return Err(LocalError::Invalid);
        };
        let artifact = match &anchor.target {
            jj_review::ArtifactTargetV1::PortableCodeText(v) => &v.artifact,
            jj_review::ArtifactTargetV1::PortableCodeArtifact(v) => v.as_ref(),
            _ => return Err(LocalError::Unsupported),
        };
        Ok(self.io.portable_file(artifact)?)
    }
    pub fn reads(&self) -> ReviewReader<'_> {
        ReviewReader::new(self)
    }
    pub async fn object(&self, reference: ReviewObjectRef) -> LocalResult<ReviewObjectV1> {
        Ok(self.read_object(reference).await?)
    }
}
#[async_trait::async_trait]
impl ReviewSnapshotRead for LocalReviewStore {
    fn repository(&self) -> m::RepositoryId {
        self.io.store.repository
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
        ReviewCatalogEditor::new(self.io.as_ref())
            .lookup_review_catalog(root, key)
            .await
    }
    async fn read_object(&self, reference: ReviewObjectRef) -> wal::Result<ReviewObjectV1> {
        if !self.objects.contains(&reference) {
            return Err(wal::Error::Stale);
        }
        self.io.read_review_object(reference).await
    }
}
impl LocalRepository {
    pub async fn review_store(&self) -> LocalResult<LocalReviewStore> {
        let selected = self.metadata.read()?;
        self.check_identity(selected.as_ref())?;
        self.review_store_at(selected.as_ref(), m::RepositoryId::new([0; 16]))
            .await
    }
    pub(crate) async fn review_store_at(
        &self,
        selected: Option<&Snapshot>,
        proposed: m::RepositoryId,
    ) -> LocalResult<LocalReviewStore> {
        let repository = selected.map_or(proposed, |s| s.repository());
        let bytes = selected
            .map(|s| self.metadata.read_domain(s, Domain::Review))
            .transpose()?
            .flatten();
        let (records, reference, evidence) = if let Some(bytes) = bytes {
            let value = LocalReviewRootV1::decode(&bytes, repository)?;
            for blob in value.records.values() {
                self.metadata
                    .read_retained_blob(selected.ok_or(LocalError::Corrupt)?, *blob)?;
            }
            (value.records, Some(value.root), value.evidence)
        } else {
            (BTreeMap::new(), None, Vec::new())
        };
        let io = Arc::new(ReviewIo::with_evidence(
            Arc::new(GitRecords::new(repository, self.metadata.clone(), records)?),
            super::agent_evidence::EvidenceIo::new(self.metadata.clone(), selected, evidence)?,
        ));
        let root = match reference {
            Some(reference) => io.root(reference).await?,
            None => ReviewRootV1::empty(),
        };
        let objects = io.audit(&root).await?;
        Ok(LocalReviewStore {
            snapshot: LocalReviewSnapshot {
                profile: LocalProfile::LocalJjGit,
                repo_id: selected.map(|s| s.repository()),
                metadata_commit: selected.map(|s| s.commit()),
                review_state_version: selected
                    .and_then(|s| s.selection(Domain::Review))
                    .map_or(0, |s| s.version),
                review_root: reference.map(|r| r.digest),
            },
            io,
            root,
            objects,
        })
    }
}
