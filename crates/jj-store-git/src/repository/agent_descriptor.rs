use super::{LocalError, LocalResult, agent_evidence::EvidenceEntry};
use crate::metadata::GitOid;
use crate::records::RecordRef;
use jj_store_model::RepositoryId;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocalAgentRootV2 {
    pub version: u32,
    pub repository: RepositoryId,
    pub root: RecordRef,
    pub records: BTreeMap<String, GitOid>,
    pub evidence: Vec<EvidenceEntry>,
}
impl LocalAgentRootV2 {
    pub fn decode(bytes: &[u8], repository: RepositoryId) -> LocalResult<Self> {
        let v: Self = serde_json::from_slice(bytes).map_err(|_| LocalError::Corrupt)?;
        if v.version != 2
            || v.repository != repository
            || serde_json::to_vec(&v).map_err(|_| LocalError::Corrupt)? != bytes
        {
            return Err(LocalError::Corrupt);
        }
        Ok(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::records::RecordKind;
    use jj_store_model::RecordDigest;

    fn current() -> LocalAgentRootV2 {
        LocalAgentRootV2 {
            version: 2,
            repository: RepositoryId::new([1; 16]),
            root: RecordRef {
                kind: RecordKind::AgentRoot,
                digest: RecordDigest::new([2; 32]),
                attempt: None,
            },
            records: BTreeMap::new(),
            evidence: Vec::new(),
        }
    }

    #[test]
    fn rejects_legacy_agent_descriptor_without_in_memory_upgrade() {
        let current = current();
        let old = format!(
            r#"{{"version":1,"repository":"{}","root":{},"records":{{}}}}"#,
            current.repository,
            serde_json::to_string(&current.root).unwrap()
        );
        assert!(matches!(
            LocalAgentRootV2::decode(old.as_bytes(), current.repository),
            Err(LocalError::Corrupt)
        ));
    }

    #[test]
    fn current_agent_descriptor_requires_exact_version_identity_and_canonical_bytes() {
        let mut current = current();
        let bytes = serde_json::to_vec(&current).unwrap();
        let decoded = LocalAgentRootV2::decode(&bytes, current.repository).unwrap();
        assert_eq!(serde_json::to_vec(&decoded).unwrap(), bytes);
        assert!(LocalAgentRootV2::decode(&bytes, RepositoryId::new([3; 16])).is_err());
        let mut noncanonical = bytes;
        noncanonical.push(b' ');
        assert!(LocalAgentRootV2::decode(&noncanonical, current.repository).is_err());
        current.version = 1;
        assert!(
            LocalAgentRootV2::decode(&serde_json::to_vec(&current).unwrap(), current.repository)
                .is_err()
        );
        current.version = 3;
        assert!(
            LocalAgentRootV2::decode(&serde_json::to_vec(&current).unwrap(), current.repository)
                .is_err()
        );
    }
}
