//! Root-pinned AgentStore reads and persistent typed catalog edits.
use super::agent::agent_error;
use super::*;
use futures::future::BoxFuture;
use jj_agent::{AgentCatalogKey, AgentCatalogKind, AgentIndexPageV1};

const LEAF_ENTRIES: usize = 32;

#[derive(Clone)]
pub struct AgentStoreSnapshot {
    coordinator: Coordinator,
    repository: RepositoryId,
    reference: RecordRef,
    root: AgentRootV1,
    objects: BTreeSet<AgentObjectRef>,
}
impl AgentStoreSnapshot {
    pub fn reference(&self) -> RecordRef {
        self.reference
    }
    pub fn root(&self) -> &AgentRootV1 {
        &self.root
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
}

impl Coordinator {
    pub async fn agent_store(&self, reference: RecordRef) -> Result<AgentStoreSnapshot> {
        let (root, native, objects) = self.read_agent_root_closure(reference).await?;
        self.verify_delta(None, native).await?;
        Ok(AgentStoreSnapshot {
            coordinator: self.clone(),
            repository: self.records.repository,
            reference,
            root,
            objects,
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
        for edit in &delta.edits {
            if base.lookup(edit.catalog, edit.key).await? != edit.expected {
                return Err(Error::Stale);
            }
        }
        let mut root = base.root.clone();
        for catalog in [
            AgentCatalogKind::Sessions,
            AgentCatalogKind::Aliases,
            AgentCatalogKind::Checkpoints,
            AgentCatalogKind::Associations,
            AgentCatalogKind::Relations,
            AgentCatalogKind::Sources,
            AgentCatalogKind::Projections,
        ] {
            let changes: BTreeMap<_, _> = delta
                .edits
                .iter()
                .filter(|edit| edit.catalog == catalog)
                .map(|edit| (edit.key, edit.replacement))
                .collect();
            if changes.is_empty() {
                continue;
            }
            let selected = self
                .rewrite_agent_catalog(catalog_root(&root, catalog), String::new(), changes)
                .await?;
            set_catalog_root(&mut root, catalog, selected);
        }
        self.stage_agent_root(&root).await
    }

    async fn lookup_agent_catalog(
        &self,
        mut page: Option<AgentObjectRef>,
        key: AgentCatalogKey,
    ) -> Result<Option<AgentObjectRef>> {
        let key_text = key.to_string();
        for _ in 0..=64 {
            let Some(reference) = page else {
                return Ok(None);
            };
            match self.read_agent_page(reference).await? {
                AgentIndexPageV1::Leaf { prefix, entries } => {
                    if !key_text.starts_with(&prefix) {
                        return Err(Error::Corrupt("agent catalog traversal"));
                    }
                    return Ok(entries.get(&key).copied());
                }
                AgentIndexPageV1::Branch { prefix, children } => {
                    if !key_text.starts_with(&prefix) || prefix.len() >= key_text.len() {
                        return Err(Error::Corrupt("agent catalog traversal"));
                    }
                    page = children
                        .get(&key_text[prefix.len()..=prefix.len()])
                        .copied();
                }
            }
        }
        Err(Error::Limit("agent catalog depth"))
    }

    fn rewrite_agent_catalog<'a>(
        &'a self,
        page: Option<AgentObjectRef>,
        prefix: String,
        changes: BTreeMap<AgentCatalogKey, Option<AgentObjectRef>>,
    ) -> BoxFuture<'a, Result<Option<AgentObjectRef>>> {
        Box::pin(async move {
            let Some(reference) = page else {
                let entries = changes
                    .into_iter()
                    .filter_map(|(key, value)| value.map(|value| (key, value)))
                    .collect();
                return self.build_agent_catalog(prefix, entries).await;
            };
            match self.read_agent_page(reference).await? {
                AgentIndexPageV1::Leaf {
                    prefix: stored,
                    mut entries,
                } => {
                    if stored != prefix {
                        return Err(Error::Corrupt("agent catalog prefix"));
                    }
                    for (key, value) in changes {
                        match value {
                            Some(value) => {
                                entries.insert(key, value);
                            }
                            None => {
                                entries.remove(&key);
                            }
                        }
                    }
                    self.build_agent_catalog(prefix, entries).await
                }
                AgentIndexPageV1::Branch {
                    prefix: stored,
                    mut children,
                } => {
                    if stored != prefix {
                        return Err(Error::Corrupt("agent catalog prefix"));
                    }
                    let mut grouped: BTreeMap<String, BTreeMap<_, _>> = BTreeMap::new();
                    for (key, value) in changes {
                        let text = key.to_string();
                        if !text.starts_with(&prefix) || prefix.len() >= text.len() {
                            return Err(Error::Corrupt("agent catalog edit prefix"));
                        }
                        grouped
                            .entry(text[prefix.len()..=prefix.len()].to_owned())
                            .or_default()
                            .insert(key, value);
                    }
                    for (nibble, changes) in grouped {
                        let child = children.get(&nibble).copied();
                        let next = self
                            .rewrite_agent_catalog(child, format!("{prefix}{nibble}"), changes)
                            .await?;
                        match next {
                            Some(next) => {
                                children.insert(nibble, next);
                            }
                            None => {
                                children.remove(&nibble);
                            }
                        }
                    }
                    if children.is_empty() {
                        Ok(None)
                    } else {
                        self.stage_agent_page(AgentIndexPageV1::Branch { prefix, children })
                            .await
                            .map(Some)
                    }
                }
            }
        })
    }

    fn build_agent_catalog<'a>(
        &'a self,
        prefix: String,
        entries: BTreeMap<AgentCatalogKey, AgentObjectRef>,
    ) -> BoxFuture<'a, Result<Option<AgentObjectRef>>> {
        Box::pin(async move {
            if entries.is_empty() {
                return Ok(None);
            }
            if entries.len() <= LEAF_ENTRIES {
                return self
                    .stage_agent_page(AgentIndexPageV1::Leaf { prefix, entries })
                    .await
                    .map(Some);
            }
            if prefix.len() >= 64 {
                return Err(Error::Corrupt("agent catalog key collision"));
            }
            let mut grouped: BTreeMap<String, BTreeMap<_, _>> = BTreeMap::new();
            for (key, value) in entries {
                let text = key.to_string();
                grouped
                    .entry(text[prefix.len()..=prefix.len()].to_owned())
                    .or_default()
                    .insert(key, value);
            }
            let mut children = BTreeMap::new();
            for (nibble, entries) in grouped {
                let child = self
                    .build_agent_catalog(format!("{prefix}{nibble}"), entries)
                    .await?
                    .ok_or(Error::Corrupt("empty agent catalog child"))?;
                children.insert(nibble, child);
            }
            self.stage_agent_page(AgentIndexPageV1::Branch { prefix, children })
                .await
                .map(Some)
        })
    }

    async fn read_agent_page(&self, reference: AgentObjectRef) -> Result<AgentIndexPageV1> {
        match self.read_agent_object(reference).await? {
            AgentObjectV1::IndexPage(page) => Ok(page),
            _ => Err(Error::Corrupt("agent catalog page kind")),
        }
    }
    async fn stage_agent_page(&self, page: AgentIndexPageV1) -> Result<AgentObjectRef> {
        self.stage_agent_object(&AgentObjectV1::IndexPage(page))
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
fn set_catalog_root(
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
