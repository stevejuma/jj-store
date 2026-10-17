//! Selected capsule inventory and staging, independent of code publication.
use super::{LocalError, LocalResult};
use crate::domain::*;
use crate::{
    evidence::EvidenceCapsule as GitEvidenceCapsuleV1,
    metadata::{GitOid, MetadataStore, RetainedKind, Snapshot},
};
use jj_store_core as wal;
use jj_store_model::{CodeCapsuleId, portable::*};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, sync::Mutex};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct LocalCodeEvidenceV1 {
    pub reference: CodeEvidenceRefV1,
    pub bytes: Vec<u8>,
}
impl LocalCodeEvidenceV1 {
    /// Structural reference admission. The injected JJ validator additionally
    /// verifies canonical operation/view identities and the complete closure.
    pub fn validate(&self) -> LocalResult<()> {
        if self.bytes.len() > 16 * 1024 * 1024 {
            return Err(LocalError::Limit);
        }
        use sha2::{Digest as _, Sha256};
        if CodeCapsuleId::new(Sha256::digest(&self.bytes).into()) != self.reference.capsule {
            return Err(LocalError::Invalid);
        }
        #[derive(Deserialize)]
        struct Header {
            profile: CodeProfileV1,
            repository: jj_store_model::RepositoryId,
            operation_id: jj_store_model::OperationId,
        }
        let header: Header =
            serde_json::from_slice(&self.bytes).map_err(|_| LocalError::Invalid)?;
        if header.profile != self.reference.profile
            || header.repository != self.reference.repository
            || header.operation_id != self.reference.operation
        {
            return Err(LocalError::Invalid);
        }
        Ok(())
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceEntry {
    pub reference: CodeEvidenceRefV1,
    pub blob: GitOid,
}
pub struct EvidenceIo {
    metadata: MetadataStore,
    entries: Mutex<BTreeMap<CodeCapsuleId, EvidenceEntry>>,
    retained: Mutex<BTreeMap<GitOid, RetainedKind>>,
}
impl EvidenceIo {
    pub fn new(
        metadata: MetadataStore,
        selected: Option<&Snapshot>,
        entries: Vec<EvidenceEntry>,
    ) -> LocalResult<Self> {
        if entries.len() > 4096
            || entries
                .windows(2)
                .any(|v| v[0].reference.capsule >= v[1].reference.capsule)
        {
            return Err(LocalError::Corrupt);
        }
        let value = Self {
            metadata,
            entries: Mutex::new(BTreeMap::new()),
            retained: Mutex::new(BTreeMap::new()),
        };
        for entry in entries {
            let capsule = value.metadata.read_evidence(
                selected.ok_or(LocalError::Corrupt)?,
                entry.reference.repository,
                entry.reference.capsule,
                entry.blob,
            )?;
            value.check(&entry.reference, &capsule)?;
            value
                .retained
                .lock()
                .unwrap()
                .extend(capsule.retention(entry.blob)?);
            value
                .entries
                .lock()
                .unwrap()
                .insert(entry.reference.capsule, entry);
        }
        Ok(value)
    }
    fn check(
        &self,
        reference: &CodeEvidenceRefV1,
        value: &GitEvidenceCapsuleV1,
    ) -> LocalResult<()> {
        if value.profile != reference.profile
            || value.repository != reference.repository
            || value.operation_id != reference.operation
        {
            return Err(LocalError::Corrupt);
        }
        Ok(())
    }
    pub fn stage(&self, values: &[LocalCodeEvidenceV1]) -> LocalResult<()> {
        for value in values {
            let capsule = self
                .metadata
                .validate_evidence(&value.reference, &value.bytes)?;
            let blob = self.metadata.stage_blob(&value.bytes)?;
            let mut entries = self.entries.lock().unwrap();
            if entries.len() >= 4096 && !entries.contains_key(&value.reference.capsule) {
                return Err(LocalError::Limit);
            }
            self.retained
                .lock()
                .unwrap()
                .extend(capsule.retention(blob)?);
            entries.insert(
                value.reference.capsule,
                EvidenceEntry {
                    reference: value.reference.clone(),
                    blob,
                },
            );
        }
        Ok(())
    }
    pub fn file(
        &self,
        reference: &CodeEvidenceRefV1,
        commit: &CodeCommitIdV1,
        path: &str,
    ) -> LocalResult<(jj_store_model::GitSha1Id, Vec<u8>)> {
        let capsule = self.read(reference)?;
        let CodeCommitIdV1::GitSha1Jj045(commit) = commit else {
            return Err(LocalError::Unsupported);
        };
        Ok(self.metadata.read_capsule_file(&capsule, *commit, path)?)
    }
    pub fn inventory(&self) -> Vec<EvidenceEntry> {
        self.entries.lock().unwrap().values().cloned().collect()
    }
    pub fn retained(&self) -> BTreeMap<GitOid, RetainedKind> {
        self.retained.lock().unwrap().clone()
    }
    pub fn read(&self, reference: &CodeEvidenceRefV1) -> LocalResult<GitEvidenceCapsuleV1> {
        let entry = self
            .entries
            .lock()
            .unwrap()
            .get(&reference.capsule)
            .cloned()
            .ok_or(LocalError::Stale)?;
        if &entry.reference != reference {
            return Err(LocalError::Corrupt);
        }
        let value = self
            .metadata
            .validate_evidence(reference, &self.metadata.read_staged_blob(entry.blob)?)?;
        self.check(reference, &value)?;
        Ok(value)
    }
    pub fn audit(&self, object: &AgentObjectV1) -> wal::Result<()> {
        for reference in object
            .code_evidence()
            .map_err(|_| wal::Error::Corrupt("agent code evidence"))?
        {
            self.read(&reference)
                .map_err(|_| wal::Error::Corrupt("agent capsule evidence"))?;
        }
        if let AgentObjectV1::RelationPage(page) = object
            && let crate::domain::AgentRelationSubjectV1::PortableRevision { evidence, commit } =
                &page.subject
        {
            self.commit(evidence, commit)?;
        }
        let anchors = match object {
            AgentObjectV1::PortableAssociation(v) => [&v.before, &v.after],
            AgentObjectV1::PortableCheckpointVersion(v) => [&v.before, &v.after],
            _ => return Ok(()),
        };
        for anchor in anchors {
            for reference in anchor.evidence() {
                self.read(&reference)
                    .map_err(|_| wal::Error::Corrupt("agent capsule evidence"))?;
            }
            for commit in &anchor.commits {
                self.commit(&commit.evidence, &commit.commit)?;
            }
        }
        Ok(())
    }
    pub(crate) fn commit(
        &self,
        evidence: &CodeEvidenceRefV1,
        commit: &CodeCommitIdV1,
    ) -> wal::Result<()> {
        let capsule = self
            .read(evidence)
            .map_err(|_| wal::Error::Corrupt("agent capsule evidence"))?;
        let CodeCommitIdV1::GitSha1Jj045(id) = commit else {
            return Err(wal::Error::Unsupported("local native capsule"));
        };
        if !capsule.commits.contains_key(id) {
            return Err(wal::Error::Corrupt("agent commit absent from capsule"));
        }
        Ok(())
    }
}
