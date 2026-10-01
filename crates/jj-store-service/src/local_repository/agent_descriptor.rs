use super::{LocalError, LocalResult, agent_evidence::EvidenceEntry};
use jj_store_jj::local_metadata::GitOid;
use jj_store_model::RepositoryId;
use jj_store_wal::records::RecordRef;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct V1 {
    version: u32,
    repository: RepositoryId,
    root: RecordRef,
    records: BTreeMap<String, GitOid>,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LocalAgentRootV2 {
    pub version: u32,
    pub repository: RepositoryId,
    pub root: RecordRef,
    pub records: BTreeMap<String, GitOid>,
    pub evidence: Vec<EvidenceEntry>,
}
impl LocalAgentRootV2 {
    pub fn decode(bytes: &[u8], repository: RepositoryId) -> LocalResult<Self> {
        if let Ok(v) = serde_json::from_slice::<V1>(bytes) {
            if v.version != 1
                || v.repository != repository
                || serde_json::to_vec(&v).map_err(|_| LocalError::Corrupt)? != bytes
            {
                return Err(LocalError::Corrupt);
            }
            return Ok(Self {
                version: 2,
                repository,
                root: v.root,
                records: v.records,
                evidence: Vec::new(),
            });
        }
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
