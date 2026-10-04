use super::{LocalError, LocalResult, records::GitRecords};
use jj_store_wal::publication::{ArtifactTargetV1, ReviewObjectRef, ReviewObjectV1, ReviewRootV1};
use jj_store_wal::{
    self as wal,
    publication::{ReviewObjectIo, ReviewProjector},
    records::{RecordKind, RecordRef, Records},
};
use std::{collections::BTreeSet, sync::Arc};

pub(crate) struct ReviewIo {
    pub store: Arc<GitRecords>,
    records: Records,
}
impl ReviewIo {
    pub fn new(store: Arc<GitRecords>) -> Self {
        Self {
            records: Records::new(store.clone(), store.repository),
            store,
        }
    }
    fn admit(&self, value: &ReviewObjectV1) -> wal::Result<()> {
        value
            .validate()
            .map_err(|_| wal::Error::Corrupt("local review object"))?;
        if let ReviewObjectV1::Anchor(anchor) = value {
            if anchor.repository_id != self.store.repository {
                return Err(wal::Error::Corrupt("local review anchor repository"));
            }
            if !matches!(anchor.target, ArtifactTargetV1::Repository) {
                return Err(wal::Error::Unsupported(
                    "local review anchor evidence is not qualified",
                ));
            }
        }
        if !value
            .native_dependencies()
            .map_err(|_| wal::Error::Corrupt("local review native dependencies"))?
            .is_empty()
            || !value
                .agent_dependencies()
                .map_err(|_| wal::Error::Corrupt("local review agent dependencies"))?
                .is_empty()
        {
            return Err(wal::Error::Unsupported(
                "local review external evidence is not qualified",
            ));
        }
        Ok(())
    }
    pub async fn audit(&self, root: &ReviewRootV1) -> wal::Result<BTreeSet<ReviewObjectRef>> {
        let closure = ReviewProjector::new(self).audit_root(root).await?;
        if !closure.native.is_empty() || !closure.agent_objects.is_empty() {
            return Err(wal::Error::Unsupported("local review external closure"));
        }
        Ok(closure.objects)
    }
    pub async fn stage_root(&self, root: &ReviewRootV1) -> wal::Result<RecordRef> {
        self.audit(root).await?;
        self.records.put(RecordKind::ReviewRoot, None, root).await
    }
    pub async fn root(&self, reference: RecordRef) -> wal::Result<ReviewRootV1> {
        let root = self.records.get(reference, RecordKind::ReviewRoot).await?;
        self.audit(&root).await?;
        Ok(root)
    }
}
fn record(reference: ReviewObjectRef) -> RecordRef {
    RecordRef {
        kind: RecordKind::ReviewObject,
        digest: jj_store_model::RecordDigest::new(*reference.id.as_bytes()),
        attempt: None,
    }
}
#[async_trait::async_trait]
impl ReviewObjectIo for ReviewIo {
    fn repository(&self) -> jj_store_model::RepositoryId {
        self.store.repository
    }
    async fn read_review_object(&self, reference: ReviewObjectRef) -> wal::Result<ReviewObjectV1> {
        let value: ReviewObjectV1 = self
            .records
            .get(record(reference), RecordKind::ReviewObject)
            .await
            .map_err(missing_record)?;
        if value.kind() != reference.kind {
            return Err(wal::Error::Corrupt("local review object kind"));
        }
        self.admit(&value)?;
        Ok(value)
    }
    async fn stage_review_object(&self, object: &ReviewObjectV1) -> wal::Result<ReviewObjectRef> {
        self.admit(object)?;
        for reference in object
            .review_dependencies()
            .map_err(|_| wal::Error::Corrupt("local review dependencies"))?
        {
            self.read_review_object(reference).await?;
        }
        let reference = self
            .records
            .put(RecordKind::ReviewObject, None, object)
            .await?;
        Ok(ReviewObjectRef::new(
            object.kind(),
            jj_store_wal::publication::ReviewObjectId::new(*reference.digest.as_bytes()),
        ))
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LocalReviewRootV1 {
    pub version: u32,
    pub repository: jj_store_model::RepositoryId,
    pub root: RecordRef,
    pub records: std::collections::BTreeMap<String, jj_store_jj::local_metadata::GitOid>,
}
impl LocalReviewRootV1 {
    pub fn decode(bytes: &[u8], repository: jj_store_model::RepositoryId) -> LocalResult<Self> {
        let value: Self = serde_json::from_slice(bytes).map_err(|_| LocalError::Corrupt)?;
        if value.version != 1
            || value.repository != repository
            || value.root.kind != RecordKind::ReviewRoot
            || value.root.attempt.is_some()
            || !value.records.contains_key(
                value
                    .root
                    .key(repository)
                    .map_err(|_| LocalError::Corrupt)?
                    .as_str(),
            )
            || serde_json::to_vec(&value).map_err(|_| LocalError::Corrupt)? != bytes
            || value
                .records
                .keys()
                .any(|key| !key.starts_with(&format!("repos/{repository}/review-")))
        {
            return Err(LocalError::Corrupt);
        }
        Ok(value)
    }
}

fn missing_record(error: wal::Error) -> wal::Error {
    match error {
        wal::Error::Storage(jj_store_object::Error::NotFound) => {
            wal::Error::Corrupt("missing local review record")
        }
        other => other,
    }
}
