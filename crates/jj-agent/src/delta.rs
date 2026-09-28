use crate::{AgentCatalogKey, AgentObjectKind, AgentObjectRef, Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum AgentCatalogKind {
    Sessions,
    Aliases,
    Checkpoints,
    Associations,
    Relations,
    Sources,
    Projections,
}
impl AgentCatalogKind {
    fn accepts(self, value: AgentObjectRef) -> bool {
        matches!(
            (self, value.kind),
            (Self::Sessions, AgentObjectKind::SessionSnapshot)
                | (Self::Aliases, AgentObjectKind::Alias)
                | (Self::Checkpoints, AgentObjectKind::CheckpointVersion)
                | (Self::Associations, AgentObjectKind::Association)
                | (Self::Relations, AgentObjectKind::RelationPage)
                | (Self::Sources, AgentObjectKind::SourceState)
                | (Self::Projections, AgentObjectKind::ProjectionCatalog)
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AgentCatalogEditV1 {
    pub catalog: AgentCatalogKind,
    pub key: AgentCatalogKey,
    pub expected: Option<AgentObjectRef>,
    pub replacement: Option<AgentObjectRef>,
}
impl AgentCatalogEditV1 {
    fn validate(&self) -> Result<()> {
        if self.expected == self.replacement
            || self
                .expected
                .into_iter()
                .chain(self.replacement)
                .any(|value| !self.catalog.accepts(value))
        {
            return Err(Error::Invalid("agent catalog edit"));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AgentDeltaV1 {
    pub schema_version: u32,
    pub edits: Vec<AgentCatalogEditV1>,
}
impl AgentDeltaV1 {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != 1 || self.edits.is_empty() || self.edits.len() > 4096 {
            return Err(Error::Invalid("agent delta"));
        }
        let mut selected = BTreeSet::new();
        for edit in &self.edits {
            edit.validate()?;
            if !selected.insert((edit.catalog, edit.key)) {
                return Err(Error::Invalid("duplicate agent catalog edit"));
            }
        }
        Ok(())
    }
}
