use super::{
    LocalError, LocalProfile, LocalRepository, LocalResult, agent_io::AgentIo, records::GitRecords,
};
use super::{agent_descriptor::LocalAgentRootV2, agent_evidence::EvidenceIo};
use crate::domain::*;
use crate::metadata::{Domain, GitOid, Snapshot};
use jj_store_core as wal;
use jj_store_model as m;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct LocalAgentSnapshot {
    pub profile: LocalProfile,
    pub repo_id: Option<m::RepositoryId>,
    pub metadata_commit: Option<GitOid>,
    pub agent_state_version: u64,
    pub agent_root: Option<m::RecordDigest>,
}
pub struct LocalAgentStore {
    pub snapshot: LocalAgentSnapshot,
    pub(crate) io: Arc<AgentIo>,
    root: AgentRootV1,
    objects: BTreeSet<AgentObjectRef>,
    transcripts: TranscriptAppendProofs,
    evidence: BTreeSet<m::portable::CodeEvidenceRefV1>,
}
impl LocalAgentStore {
    pub fn reads(&self) -> AgentReader<'_> {
        AgentReader::new(self)
    }
    pub async fn object(&self, reference: AgentObjectRef) -> LocalResult<AgentObjectV1> {
        Ok(self.read_object(reference).await?)
    }
    pub fn code_evidence(
        &self,
        reference: &m::portable::CodeEvidenceRefV1,
    ) -> LocalResult<crate::evidence::EvidenceCapsule> {
        if !self.evidence.contains(reference) {
            return Err(LocalError::Stale);
        }
        self.io.evidence.read(reference)
    }
}
#[async_trait::async_trait]
impl AgentSnapshotRead for LocalAgentStore {
    fn repository(&self) -> m::RepositoryId {
        self.io.store.repository
    }
    fn root(&self) -> &AgentRootV1 {
        &self.root
    }
    fn transcript_boundary(&self, root: AgentObjectRef) -> Option<TranscriptAppendBoundary> {
        self.transcripts.boundary(root)
    }
    async fn lookup(
        &self,
        catalog: AgentCatalogKind,
        key: AgentCatalogKey,
    ) -> wal::Result<Option<AgentObjectRef>> {
        let page = match catalog {
            AgentCatalogKind::Sessions => self.root.sessions,
            AgentCatalogKind::Aliases => self.root.aliases,
            AgentCatalogKind::Checkpoints => self.root.checkpoints,
            AgentCatalogKind::Associations => self.root.associations,
            AgentCatalogKind::Relations => self.root.relations,
            AgentCatalogKind::Sources => self.root.sources,
            AgentCatalogKind::Projections => self.root.projections,
        };
        AgentCatalogEditor::new(self.io.as_ref())
            .lookup_agent_catalog(page, key)
            .await
    }
    async fn read_object(&self, reference: AgentObjectRef) -> wal::Result<AgentObjectV1> {
        if !self.objects.contains(&reference) {
            return Err(wal::Error::Stale);
        }
        self.io.read_agent_object(reference).await
    }
}
impl LocalRepository {
    pub async fn agent_store(&self) -> LocalResult<LocalAgentStore> {
        let selected = self.metadata.read()?;
        self.agent_store_at(selected.as_ref(), m::RepositoryId::new([0; 16]))
            .await
    }
    pub async fn agent_store_at(
        &self,
        selected: Option<&Snapshot>,
        proposed: m::RepositoryId,
    ) -> LocalResult<LocalAgentStore> {
        self.check_selected(selected)?;
        let repository = selected.map_or(proposed, |s| s.repository());
        let bytes = selected
            .map(|s| self.metadata.read_domain(s, Domain::Agent))
            .transpose()?
            .flatten();
        let (records, reference, entries) = if let Some(bytes) = bytes {
            let value = LocalAgentRootV2::decode(&bytes, repository)?;
            for blob in value.records.values() {
                self.metadata
                    .read_retained_blob(selected.ok_or(LocalError::Corrupt)?, *blob)?;
            }
            (value.records, Some(value.root), value.evidence)
        } else {
            (BTreeMap::new(), None, Vec::new())
        };
        let io = Arc::new(AgentIo::new(
            Arc::new(GitRecords::new(repository, self.metadata.clone(), records)?),
            EvidenceIo::new(self.metadata.clone(), selected, entries)?,
        ));
        let root = match reference {
            Some(reference) => io.root(reference).await?,
            None => AgentRootV1::empty(),
        };
        let objects = io.audit(&root).await?;
        let mut evidence = BTreeSet::new();
        let mut transcripts = TranscriptAppendAudit::default();
        for reference in &objects {
            let object = io.read_agent_object(*reference).await?;
            transcripts.observe(*reference, &object);
            evidence.extend(object.code_evidence().map_err(|_| LocalError::Corrupt)?);
        }
        Ok(LocalAgentStore {
            snapshot: LocalAgentSnapshot {
                profile: LocalProfile::LocalJjGit,
                repo_id: selected.map(|s| s.repository()),
                metadata_commit: selected.map(|s| s.commit()),
                agent_state_version: selected
                    .and_then(|s| s.selection(Domain::Agent))
                    .map_or(0, |s| s.version),
                agent_root: reference.map(|r| r.digest),
            },
            io,
            root,
            objects,
            transcripts: transcripts.finish(),
            evidence,
        })
    }
}
