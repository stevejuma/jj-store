use crate::{AgentCatalogKey, AgentObjectId, Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    schemars::JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum AgentObjectKind {
    IndexPage,
    SessionSnapshot,
    TurnSnapshot,
    MessageSnapshot,
    ToolSnapshot,
    Intent,
    ObservationBatch,
    EventBatch,
    TranscriptManifest,
    TranscriptPage,
    TranscriptChunk,
    Content,
    CheckpointVersion,
    Association,
}

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    schemars::JsonSchema,
)]
#[serde(deny_unknown_fields)]
pub struct AgentObjectRef {
    pub kind: AgentObjectKind,
    pub id: AgentObjectId,
}
impl AgentObjectRef {
    pub const fn new(kind: AgentObjectKind, id: AgentObjectId) -> Self {
        Self { kind, id }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AgentRootV1 {
    pub schema_version: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sessions: Option<AgentObjectRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aliases: Option<AgentObjectRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkpoints: Option<AgentObjectRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub associations: Option<AgentObjectRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relations: Option<AgentObjectRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sources: Option<AgentObjectRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub projections: Option<AgentObjectRef>,
}
impl AgentRootV1 {
    pub const fn empty() -> Self {
        Self {
            schema_version: 1,
            sessions: None,
            aliases: None,
            checkpoints: None,
            associations: None,
            relations: None,
            sources: None,
            projections: None,
        }
    }
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != 1 {
            return Err(Error::Invalid("agent root schema"));
        }
        if self
            .dependencies_unchecked()
            .iter()
            .any(|reference| reference.kind != AgentObjectKind::IndexPage)
        {
            return Err(Error::Invalid("agent root catalog kind"));
        }
        Ok(())
    }
    pub fn dependencies(&self) -> Result<Vec<AgentObjectRef>> {
        self.validate()?;
        Ok(self.dependencies_unchecked())
    }
    fn dependencies_unchecked(&self) -> Vec<AgentObjectRef> {
        [
            self.sessions,
            self.aliases,
            self.checkpoints,
            self.associations,
            self.relations,
            self.sources,
            self.projections,
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(tag = "page", rename_all = "snake_case", deny_unknown_fields)]
pub enum AgentIndexPageV1 {
    Leaf {
        prefix: String,
        entries: BTreeMap<AgentCatalogKey, AgentObjectRef>,
    },
    Branch {
        prefix: String,
        children: BTreeMap<String, AgentObjectRef>,
    },
}
impl AgentIndexPageV1 {
    fn validate(&self) -> Result<()> {
        match self {
            Self::Leaf { prefix, entries } => {
                validate_prefix(prefix)?;
                if entries.len() > 32
                    || (entries.is_empty() && !prefix.is_empty())
                    || entries
                        .keys()
                        .any(|key| !key.to_string().starts_with(prefix))
                {
                    return Err(Error::Invalid("agent catalog leaf"));
                }
            }
            Self::Branch { prefix, children } => {
                validate_prefix(prefix)?;
                if prefix.len() >= 64
                    || children.is_empty()
                    || children.len() > 16
                    || children.iter().any(|(nibble, child)| {
                        nibble.len() != 1
                            || !nibble.bytes().all(is_hex)
                            || child.kind != AgentObjectKind::IndexPage
                    })
                {
                    return Err(Error::Invalid("agent catalog branch"));
                }
            }
        }
        Ok(())
    }
    fn dependencies(&self) -> BTreeSet<AgentObjectRef> {
        match self {
            Self::Leaf { entries, .. } => entries.values().copied().collect(),
            Self::Branch { children, .. } => children.values().copied().collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum AgentObjectV1 {
    IndexPage(AgentIndexPageV1),
}
impl AgentObjectV1 {
    pub const fn kind(&self) -> AgentObjectKind {
        match self {
            Self::IndexPage(_) => AgentObjectKind::IndexPage,
        }
    }
    pub fn validate(&self) -> Result<()> {
        match self {
            Self::IndexPage(page) => page.validate(),
        }
    }
    pub fn dependencies(&self) -> Result<Vec<AgentObjectRef>> {
        self.validate()?;
        Ok(match self {
            Self::IndexPage(page) => page.dependencies().into_iter().collect(),
        })
    }
}

fn validate_prefix(prefix: &str) -> Result<()> {
    if prefix.len() > 64 || !prefix.bytes().all(is_hex) {
        return Err(Error::Invalid("agent catalog prefix"));
    }
    Ok(())
}
fn is_hex(byte: u8) -> bool {
    byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)
}
