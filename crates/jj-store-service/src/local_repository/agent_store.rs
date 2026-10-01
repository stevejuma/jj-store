use super::{
    LocalError, LocalProfile, LocalRepository, LocalResult, agent_io::AgentIo, records::GitRecords,
};
use jj_store_jj::local_metadata::{Domain, GitOid, Snapshot};
use jj_store_model as m;
use jj_store_wal::{self as wal, publication::*, records::RecordRef};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LocalAgentRootV1 {
    pub version: u32,
    pub repository: m::RepositoryId,
    pub root: RecordRef,
    pub records: BTreeMap<String, GitOid>,
}
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
}
impl LocalAgentStore {
    pub fn reads(&self) -> AgentReader<'_> {
        AgentReader::new(self)
    }
    pub async fn object(&self, reference: AgentObjectRef) -> LocalResult<AgentObjectV1> {
        Ok(self.read_object(reference).await?)
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
    pub(crate) async fn agent_store_at(
        &self,
        selected: Option<&Snapshot>,
        proposed: m::RepositoryId,
    ) -> LocalResult<LocalAgentStore> {
        let repository = selected.map_or(proposed, |s| s.repository());
        let bytes = selected
            .map(|s| self.metadata.read_domain(s, Domain::Agent))
            .transpose()?
            .flatten();
        let (records, reference) = if let Some(bytes) = bytes {
            let value: LocalAgentRootV1 =
                serde_json::from_slice(&bytes).map_err(|_| LocalError::Corrupt)?;
            if value.version != 1
                || value.repository != repository
                || serde_json::to_vec(&value).map_err(|_| LocalError::Corrupt)? != bytes
            {
                return Err(LocalError::Corrupt);
            }
            for blob in value.records.values() {
                self.metadata
                    .read_retained_blob(selected.ok_or(LocalError::Corrupt)?, *blob)?;
            }
            (value.records, Some(value.root))
        } else {
            (BTreeMap::new(), None)
        };
        let io = Arc::new(AgentIo::new(Arc::new(GitRecords::new(
            repository,
            self.metadata.clone(),
            records,
        )?)));
        let root = match reference {
            Some(reference) => io.root(reference).await?,
            None => AgentRootV1::empty(),
        };
        let objects = io.audit(&root).await?;
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
        })
    }
}
