//! Data-only collaboration reducers for imported accepted catalog facts.
use super::*;

impl Coordinator {
    pub(super) async fn baseline_agent_delta(
        &self,
        authored_root: RecordRef,
    ) -> Result<Option<AgentDeltaV1>> {
        let authored = self.agent_store(authored_root).await?;
        let mut edits = Vec::new();
        for catalog in [
            AgentCatalogKind::Sessions,
            AgentCatalogKind::Aliases,
            AgentCatalogKind::Checkpoints,
            AgentCatalogKind::Associations,
            AgentCatalogKind::Relations,
            AgentCatalogKind::Sources,
            AgentCatalogKind::Projections,
        ] {
            edits.extend(
                authored
                    .catalog_entries(catalog, 4096)
                    .await?
                    .into_iter()
                    .map(|(key, replacement)| AgentCatalogEditV1 {
                        catalog,
                        key,
                        expected: None,
                        replacement: Some(replacement),
                    }),
            );
        }
        if edits.is_empty() {
            Ok(None)
        } else {
            let delta = AgentDeltaV1 {
                schema_version: 1,
                edits,
            };
            delta
                .validate()
                .map_err(|_| Error::Limit("agent baseline facts"))?;
            Ok(Some(delta))
        }
    }

    pub(super) async fn reduce_imported_agent_facts(
        &self,
        current_root: RecordRef,
        authored_root: RecordRef,
        delta: &AgentDeltaV1,
    ) -> Result<RecordRef> {
        let authored = self.agent_store(authored_root).await?;
        for edit in &delta.edits {
            if authored.lookup(edit.catalog, edit.key).await? != edit.replacement {
                return Err(Error::Corrupt("origin agent facts selection"));
            }
            if let Some(replacement) = edit.replacement {
                authored.read_object(replacement).await?;
            }
        }
        let current = self.agent_store(current_root).await?;
        let mut edits = Vec::new();
        for edit in &delta.edits {
            let selected = current.lookup(edit.catalog, edit.key).await?;
            if selected == edit.replacement {
                continue;
            }
            let replacement = if selected == edit.expected {
                edit.replacement
            } else {
                let (Some(selected), Some(incoming)) = (selected, edit.replacement) else {
                    return Err(Error::Corrupt("concurrent agent deletion conflict"));
                };
                Some(
                    self.reduce_agent_catalog_value(
                        &current,
                        &authored,
                        edit.catalog,
                        edit.key,
                        selected,
                        incoming,
                    )
                    .await?,
                )
            };
            edits.push(AgentCatalogEditV1 {
                catalog: edit.catalog,
                key: edit.key,
                expected: selected,
                replacement,
            });
        }
        if edits.is_empty() {
            Ok(current_root)
        } else {
            self.stage_agent_delta(
                &current,
                &AgentDeltaV1 {
                    schema_version: 1,
                    edits,
                },
            )
            .await
        }
    }

    pub(super) async fn agent_conflict_alternatives(
        &self,
        store: &AgentStoreSnapshot,
        catalog: AgentCatalogKind,
        key: AgentCatalogKey,
        reference: AgentObjectRef,
    ) -> Result<BTreeSet<AgentObjectRef>> {
        match store.read_object(reference).await? {
            AgentObjectV1::CatalogConflict(conflict) => {
                if conflict.catalog != catalog || conflict.key != key {
                    return Err(Error::Corrupt("agent catalog conflict identity"));
                }
                Ok(conflict.alternatives.into_iter().collect())
            }
            _ => Ok(BTreeSet::from([reference])),
        }
    }
}
