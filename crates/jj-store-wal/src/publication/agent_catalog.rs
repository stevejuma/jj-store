//! Root-pinned AgentStore reads and persistent typed catalog edits.
use super::agent::agent_error;
use super::*;
use jj_agent::{AgentCatalogKey, AgentCatalogKind, AgentIndexPageV1};

/// Validated immutable selection with no coordinator, guard or credentials.
#[derive(Clone)]
pub struct AgentStoreSelection {
    repository: RepositoryId,
    reference: RecordRef,
    root: AgentRootV1,
    objects: Arc<BTreeSet<AgentObjectRef>>,
    transcripts: TranscriptAppendProofs,
}
impl AgentStoreSelection {
    pub fn object_count(&self) -> usize {
        self.objects.len()
    }
}

#[derive(Clone)]
pub struct AgentStoreSnapshot {
    coordinator: Coordinator,
    repository: RepositoryId,
    reference: RecordRef,
    root: AgentRootV1,
    objects: Arc<BTreeSet<AgentObjectRef>>,
    transcripts: TranscriptAppendProofs,
}
impl AgentStoreSnapshot {
    pub fn selection(&self) -> AgentStoreSelection {
        AgentStoreSelection {
            repository: self.repository,
            reference: self.reference,
            root: self.root.clone(),
            objects: self.objects.clone(),
            transcripts: self.transcripts.clone(),
        }
    }

    pub fn repository_id(&self) -> RepositoryId {
        self.repository
    }
    pub fn reference(&self) -> RecordRef {
        self.reference
    }
    pub fn root(&self) -> &AgentRootV1 {
        &self.root
    }
    pub fn transcript_proofs(&self) -> &TranscriptAppendProofs {
        &self.transcripts
    }
    pub fn object_count(&self) -> usize {
        self.objects.len()
    }
    pub async fn lookup(
        &self,
        catalog: AgentCatalogKind,
        key: AgentCatalogKey,
    ) -> Result<Option<AgentObjectRef>> {
        self.coordinator
            .lookup_agent_catalog(catalog_root(&self.root, catalog), key)
            .await
    }
    pub async fn read_object(&self, reference: AgentObjectRef) -> Result<AgentObjectV1> {
        if !self.objects.contains(&reference) {
            return Err(Error::Stale);
        }
        self.coordinator.read_agent_object(reference).await
    }

    pub(super) async fn catalog_entries(
        &self,
        catalog: AgentCatalogKind,
        limit: usize,
    ) -> Result<BTreeMap<AgentCatalogKey, AgentObjectRef>> {
        let mut pending = catalog_root(&self.root, catalog)
            .into_iter()
            .collect::<Vec<_>>();
        let mut seen = BTreeSet::new();
        let mut entries = BTreeMap::new();
        while let Some(reference) = pending.pop() {
            if !seen.insert(reference) {
                continue;
            }
            if seen.len().saturating_add(entries.len()) > limit {
                return Err(Error::Limit("agent baseline catalog"));
            }
            match self.coordinator.read_agent_page(reference).await? {
                AgentIndexPageV1::Leaf { entries: leaf, .. } => {
                    for (key, value) in leaf {
                        if entries.insert(key, value).is_some() {
                            return Err(Error::Corrupt("agent baseline duplicate"));
                        }
                    }
                }
                AgentIndexPageV1::Branch { children, .. } => {
                    pending.extend(children.into_values());
                }
            }
        }
        if seen.len().saturating_add(entries.len()) > limit {
            return Err(Error::Limit("agent baseline catalog"));
        }
        Ok(entries)
    }
}

impl Coordinator {
    /// Attach the current request reader; this does not authorize disclosure.
    pub fn selected_agent_reader(
        &self,
        selection: &AgentStoreSelection,
    ) -> Result<AgentStoreSnapshot> {
        if selection.repository != self.records.repository {
            return Err(Error::Denied);
        }
        Ok(AgentStoreSnapshot {
            coordinator: self.clone(),
            repository: selection.repository,
            reference: selection.reference,
            root: selection.root.clone(),
            objects: selection.objects.clone(),
            transcripts: selection.transcripts.clone(),
        })
    }

    pub async fn agent_store(&self, reference: RecordRef) -> Result<AgentStoreSnapshot> {
        let (root, native, objects, transcripts) = self.read_agent_root_snapshot(reference).await?;
        self.verify_delta(None, native).await?;
        Ok(AgentStoreSnapshot {
            coordinator: self.clone(),
            repository: self.records.repository,
            reference,
            root,
            objects: Arc::new(objects),
            transcripts,
        })
    }

    pub async fn stage_agent_delta(
        &self,
        base: &AgentStoreSnapshot,
        delta: &AgentDeltaV1,
    ) -> Result<RecordRef> {
        delta.validate().map_err(agent_error)?;
        if base.repository != self.records.repository {
            return Err(Error::Corrupt("agent store repository"));
        }
        let root = AgentCatalogEditor::new(self)
            .apply_delta(base, delta)
            .await?;
        self.stage_agent_root(&root).await
    }
    async fn lookup_agent_catalog(
        &self,
        page: Option<AgentObjectRef>,
        key: AgentCatalogKey,
    ) -> Result<Option<AgentObjectRef>> {
        AgentCatalogEditor::new(self)
            .lookup_agent_catalog(page, key)
            .await
    }
    async fn read_agent_page(&self, reference: AgentObjectRef) -> Result<AgentIndexPageV1> {
        AgentCatalogEditor::new(self)
            .read_agent_page(reference)
            .await
    }
}

pub(super) fn catalog_root(
    root: &AgentRootV1,
    catalog: AgentCatalogKind,
) -> Option<AgentObjectRef> {
    match catalog {
        AgentCatalogKind::Sessions => root.sessions,
        AgentCatalogKind::Aliases => root.aliases,
        AgentCatalogKind::Checkpoints => root.checkpoints,
        AgentCatalogKind::Associations => root.associations,
        AgentCatalogKind::Relations => root.relations,
        AgentCatalogKind::Sources => root.sources,
        AgentCatalogKind::Projections => root.projections,
    }
}
pub(super) fn set_catalog_root(
    root: &mut AgentRootV1,
    catalog: AgentCatalogKind,
    value: Option<AgentObjectRef>,
) {
    match catalog {
        AgentCatalogKind::Sessions => root.sessions = value,
        AgentCatalogKind::Aliases => root.aliases = value,
        AgentCatalogKind::Checkpoints => root.checkpoints = value,
        AgentCatalogKind::Associations => root.associations = value,
        AgentCatalogKind::Relations => root.relations = value,
        AgentCatalogKind::Sources => root.sources = value,
        AgentCatalogKind::Projections => root.projections = value,
    }
}
