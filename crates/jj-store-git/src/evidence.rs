//! Trusted evidence verification is injected by the embedding JJ integration.
use crate::metadata::{Error, GitOid, Result, RetainedKind};
use jj_store_model::{self as m, portable::CodeEvidenceRefV1};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EvidenceCommit {
    pub change_id: m::ChangeId,
    pub root_tree: Vec<m::GitSha1Id>,
}
/// Facts returned only after the host's complete operation/view/capsule validation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EvidenceCapsule {
    pub profile: m::portable::CodeProfileV1,
    pub repository: m::RepositoryId,
    pub operation_id: m::OperationId,
    pub view_id: m::ViewId,
    pub commits: BTreeMap<m::GitSha1Id, EvidenceCommit>,
    pub bytes: Vec<u8>,
}
impl EvidenceCapsule {
    /// Exact bytes already admitted by the trusted source validator.
    pub fn encode(&self) -> Result<Vec<u8>> {
        Ok(self.bytes.clone())
    }

    pub fn retention(&self, blob: GitOid) -> Result<BTreeMap<GitOid, RetainedKind>> {
        let mut retained = BTreeMap::from([(blob, RetainedKind::Blob)]);
        for (id, commit) in &self.commits {
            if *id != m::GitSha1Id::new([0; 20]) {
                retained.insert(id.to_string().parse()?, RetainedKind::Commit);
            }
            for tree in &commit.root_tree {
                retained.insert(tree.to_string().parse()?, RetainedKind::Tree);
            }
        }
        Ok(retained)
    }
}
/// This is a trusted host capability, never deserialized repository data.
/// Implementations must verify complete native source identities and closure.
pub trait EvidenceValidator: Send + Sync + std::fmt::Debug {
    fn validate(&self, reference: &CodeEvidenceRefV1, bytes: &[u8]) -> Result<EvidenceCapsule>;
}
impl crate::metadata::MetadataStore {
    pub fn validate_evidence(
        &self,
        reference: &CodeEvidenceRefV1,
        bytes: &[u8],
    ) -> Result<EvidenceCapsule> {
        use sha2::{Digest as _, Sha256};
        if bytes.len() > 16 * 1024 * 1024 {
            return Err(Error::Limit);
        }
        if m::CodeCapsuleId::new(Sha256::digest(bytes).into()) != reference.capsule {
            return Err(Error::Invalid("capsule identity"));
        }
        let value = self
            .evidence_validator()
            .ok_or(Error::Invalid("qualified evidence validator required"))?
            .validate(reference, bytes)?;
        if value.profile != reference.profile
            || value.repository != reference.repository
            || value.operation_id != reference.operation
            || value.bytes != bytes
        {
            return Err(Error::Invalid("verified evidence binding"));
        }
        Ok(value)
    }
    pub fn read_evidence(
        &self,
        selected: &crate::metadata::Snapshot,
        source: m::RepositoryId,
        capsule: m::CodeCapsuleId,
        blob: GitOid,
    ) -> Result<EvidenceCapsule> {
        let bytes = self.read_retained_blob(selected, blob)?;
        // The selected inventory supplies the full reference; decoding only these
        // public fields does not substitute for the injected identity verifier.
        #[derive(serde::Deserialize)]
        struct Header {
            profile: m::portable::CodeProfileV1,
            operation_id: m::OperationId,
        }
        let header: Header = serde_json::from_slice(&bytes).map_err(Error::other)?;
        let reference = CodeEvidenceRefV1 {
            profile: header.profile,
            repository: source,
            operation: header.operation_id,
            capsule,
        };
        let value = self.validate_evidence(&reference, &bytes)?;
        for (id, kind) in value.retention(blob)? {
            if selected.retained().get(&id) != Some(&kind) {
                return Err(Error::Invalid("incomplete retained capsule closure"));
            }
        }
        Ok(value)
    }
}
