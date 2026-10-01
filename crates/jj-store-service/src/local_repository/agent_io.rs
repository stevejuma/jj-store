use super::agent_evidence::EvidenceIo;
use super::records::GitRecords;
use jj_store_wal::{
    self as wal,
    publication::*,
    records::{RecordKind, RecordRef, Records},
};
use std::{collections::BTreeSet, sync::Arc};

pub(crate) struct AgentIo {
    pub store: Arc<GitRecords>,
    pub records: Records,
    pub evidence: EvidenceIo,
}
impl AgentIo {
    pub fn new(store: Arc<GitRecords>, evidence: EvidenceIo) -> Self {
        Self {
            records: Records::new(store.clone(), store.repository),
            store,
            evidence,
        }
    }
    pub async fn audit(&self, root: &AgentRootV1) -> wal::Result<BTreeSet<AgentObjectRef>> {
        let mut pending = root
            .dependencies()
            .map_err(|_| wal::Error::Corrupt("local agent root"))?;
        let mut seen = BTreeSet::new();
        while let Some(reference) = pending.pop() {
            if !seen.insert(reference) {
                continue;
            }
            if seen.len() > 4096 {
                return Err(wal::Error::Limit("local agent closure"));
            }
            let value = self.read_agent_object(reference).await?;
            pending.extend(
                value
                    .dependencies()
                    .map_err(|_| wal::Error::Corrupt("local agent dependencies"))?,
            );
        }
        Ok(seen)
    }
    pub async fn stage_root(&self, root: &AgentRootV1) -> wal::Result<RecordRef> {
        self.audit(root).await?;
        self.records.put(RecordKind::AgentRoot, None, root).await
    }
    pub async fn root(&self, reference: RecordRef) -> wal::Result<AgentRootV1> {
        let root = self.records.get(reference, RecordKind::AgentRoot).await?;
        self.audit(&root).await?;
        Ok(root)
    }
}
fn record(reference: AgentObjectRef) -> RecordRef {
    RecordRef {
        kind: RecordKind::AgentObject,
        digest: jj_store_model::RecordDigest::new(*reference.id.as_bytes()),
        attempt: None,
    }
}
#[async_trait::async_trait]
impl AgentObjectIo for AgentIo {
    fn repository(&self) -> jj_store_model::RepositoryId {
        self.store.repository
    }
    async fn read_agent_object(&self, reference: AgentObjectRef) -> wal::Result<AgentObjectV1> {
        let value: AgentObjectV1 = self
            .records
            .get(record(reference), RecordKind::AgentObject)
            .await?;
        if value.kind() != reference.kind {
            return Err(wal::Error::Corrupt("local agent object kind"));
        }
        value
            .validate()
            .map_err(|_| wal::Error::Corrupt("local agent object"))?;
        self.evidence.audit(&value)?;
        if !value
            .native_dependencies()
            .map_err(|_| wal::Error::Corrupt("local agent dependencies"))?
            .is_empty()
        {
            return Err(wal::Error::Unsupported(
                "native V1 code anchors require retained native evidence",
            ));
        }
        Ok(value)
    }
    async fn stage_agent_object(&self, object: &AgentObjectV1) -> wal::Result<AgentObjectRef> {
        object
            .validate()
            .map_err(|_| wal::Error::Corrupt("local agent object"))?;
        self.evidence.audit(object)?;
        if !object
            .native_dependencies()
            .map_err(|_| wal::Error::Corrupt("local agent dependencies"))?
            .is_empty()
        {
            return Err(wal::Error::Unsupported(
                "native V1 code anchors require retained native evidence",
            ));
        }
        for reference in object
            .dependencies()
            .map_err(|_| wal::Error::Corrupt("local agent dependencies"))?
        {
            self.read_agent_object(reference).await?;
        }
        let reference = self
            .records
            .put(RecordKind::AgentObject, None, object)
            .await?;
        Ok(AgentObjectRef::new(
            object.kind(),
            AgentObjectId::new(*reference.digest.as_bytes()),
        ))
    }
}
