use super::{LocalError, LocalResult, records::GitRecords};
use crate::domain::{ArtifactTargetV1, ReviewObjectRef, ReviewObjectV1, ReviewRootV1};
use crate::{
    domain::{ReviewObjectIo, ReviewProjector},
    records::{RecordKind, RecordRef, Records},
};
use jj_store_core as wal;
use std::{collections::BTreeSet, sync::Arc};

pub struct ReviewIo {
    pub store: Arc<GitRecords>,
    records: Records,
    pub evidence: Option<super::agent_evidence::EvidenceIo>,
}
impl ReviewIo {
    pub fn new(store: Arc<GitRecords>) -> Self {
        Self {
            records: Records::new(store.clone(), store.repository),
            store,
            evidence: None,
        }
    }
    pub fn with_evidence(
        store: Arc<GitRecords>,
        evidence: super::agent_evidence::EvidenceIo,
    ) -> Self {
        let mut io = Self::new(store);
        io.evidence = Some(evidence);
        io
    }
    pub fn portable_file(
        &self,
        anchor: &jj_review::PortableCodeArtifactV2,
    ) -> wal::Result<Vec<u8>> {
        let io = self
            .evidence
            .as_ref()
            .ok_or(wal::Error::Unsupported("portable review evidence"))?;
        let capsule = io
            .read(&anchor.revision.evidence)
            .map_err(|_| wal::Error::Corrupt("review capsule"))?;
        let jj_store_model::portable::CodeCommitIdV1::GitSha1Jj045(commit) = anchor.revision.commit
        else {
            return Err(wal::Error::Unsupported("review code profile"));
        };
        let saved = capsule
            .commits
            .get(&commit)
            .ok_or(wal::Error::Corrupt("review commit absent"))?;
        if anchor
            .revision
            .change
            .is_some_and(|id| id != saved.change_id)
        {
            return Err(wal::Error::Corrupt("review change identity"));
        }
        let (id, bytes) = io
            .file(
                &anchor.revision.evidence,
                &anchor.revision.commit,
                &anchor.path,
            )
            .map_err(|_| wal::Error::Corrupt("review file source"))?;
        if anchor.object
            != (jj_store_model::portable::CodeObjectIdV1::GitSha1Jj045 {
                kind: jj_store_model::portable::GitObjectKindV1::Blob,
                id,
            })
            || !matches!(
                anchor.value_kind,
                jj_review::CodeValueKind::File
                    | jj_review::CodeValueKind::BinaryFile
                    | jj_review::CodeValueKind::EmptyFile
            )
            || (anchor.value_kind == jj_review::CodeValueKind::EmptyFile && !bytes.is_empty())
        {
            return Err(wal::Error::Corrupt("review file identity"));
        }
        if anchor.value_kind == jj_review::CodeValueKind::File
            && (bytes.contains(&0) || std::str::from_utf8(&bytes).is_err())
        {
            return Err(wal::Error::Unsupported("review file is not text"));
        }
        Ok(bytes)
    }
    fn admit(&self, value: &ReviewObjectV1) -> wal::Result<()> {
        value
            .validate()
            .map_err(|_| wal::Error::Corrupt("local review object"))?;
        if let ReviewObjectV1::Anchor(anchor) = value {
            if anchor.repository_id != self.store.repository {
                return Err(wal::Error::Corrupt("local review anchor repository"));
            }
            match &anchor.target {
                ArtifactTargetV1::Repository => {}
                ArtifactTargetV1::PortableCodeText(v) => {
                    let bytes = self.portable_file(&v.artifact)?;
                    v.text
                        .validate_bytes(&bytes)
                        .map_err(|_| wal::Error::Corrupt("review text selection"))?;
                    if let Some(diff) = &v.diff_context {
                        let io = self
                            .evidence
                            .as_ref()
                            .ok_or(wal::Error::Unsupported("review evidence"))?;
                        for (revision, path) in [
                            (&diff.before, &diff.before_path),
                            (&diff.after, &diff.after_path),
                        ] {
                            io.commit(&revision.evidence, &revision.commit)?;
                            let capsule = io
                                .read(&revision.evidence)
                                .map_err(|_| wal::Error::Corrupt("review diff capsule"))?;
                            let jj_store_model::portable::CodeCommitIdV1::GitSha1Jj045(commit) =
                                revision.commit
                            else {
                                return Err(wal::Error::Unsupported("review diff profile"));
                            };
                            if revision
                                .change
                                .is_some_and(|id| id != capsule.commits[&commit].change_id)
                            {
                                return Err(wal::Error::Corrupt("review diff change"));
                            }
                            if let Some(path) = path {
                                io.file(&revision.evidence, &revision.commit, path)
                                    .map_err(|_| wal::Error::Corrupt("review diff path"))?;
                            }
                        }
                    }
                }
                ArtifactTargetV1::PortableCodeArtifact(v) => {
                    self.portable_file(v)?;
                }
                _ => {
                    return Err(wal::Error::Unsupported(
                        "local review anchor evidence is not qualified",
                    ));
                }
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
            crate::domain::ReviewObjectId::new(*reference.digest.as_bytes()),
        ))
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocalReviewRootV1 {
    pub version: u32,
    pub repository: jj_store_model::RepositoryId,
    pub root: RecordRef,
    pub records: std::collections::BTreeMap<String, crate::metadata::GitOid>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence: Vec<super::agent_evidence::EvidenceEntry>,
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

fn missing_record(error: impl Into<wal::Error>) -> wal::Error {
    match error.into() {
        wal::Error::NotFound => wal::Error::Corrupt("missing local review record"),
        other => other,
    }
}
