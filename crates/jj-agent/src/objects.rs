use crate::{
    AgentAliasV1, AgentCatalogConflictV1, AgentCheckpointVersionV1, AgentContentV1,
    AgentEventBatchV1, AgentIndexPageV1, AgentObjectId, AgentProjectionCatalogV1,
    AgentRelationPageV1, AgentSessionSnapshotV1, CaptureSourceStateV1, CodeAssociationV1, Result,
    SanitizedObservationBatchV1, ToolInvocationSnapshotV1, TranscriptChunkV1, TranscriptManifestV1,
};
use jj_store_model::ObjectKey;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

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
    Alias,
    RelationPage,
    SourceState,
    ProjectionCatalog,
    CatalogConflict,
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
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum AgentObjectV1 {
    IndexPage(AgentIndexPageV1),
    SessionSnapshot(AgentSessionSnapshotV1),
    ToolSnapshot(ToolInvocationSnapshotV1),
    ObservationBatch(SanitizedObservationBatchV1),
    EventBatch(AgentEventBatchV1),
    TranscriptManifest(TranscriptManifestV1),
    TranscriptTailPage(crate::TranscriptTailPageV2),
    TranscriptChunk(TranscriptChunkV1),
    Content(AgentContentV1),
    CheckpointVersion(AgentCheckpointVersionV1),
    Association(CodeAssociationV1),
    PortableCheckpointVersion(crate::AgentCheckpointVersionV2),
    PortableAssociation(crate::CodeAssociationV2),
    Alias(AgentAliasV1),
    RelationPage(AgentRelationPageV1),
    SourceState(CaptureSourceStateV1),
    ProjectionCatalog(AgentProjectionCatalogV1),
    CatalogConflict(AgentCatalogConflictV1),
}
impl AgentObjectV1 {
    pub const fn kind(&self) -> AgentObjectKind {
        match self {
            Self::IndexPage(_) => AgentObjectKind::IndexPage,
            Self::SessionSnapshot(_) => AgentObjectKind::SessionSnapshot,
            Self::ToolSnapshot(_) => AgentObjectKind::ToolSnapshot,
            Self::ObservationBatch(_) => AgentObjectKind::ObservationBatch,
            Self::EventBatch(_) => AgentObjectKind::EventBatch,
            Self::TranscriptManifest(_) => AgentObjectKind::TranscriptManifest,
            Self::TranscriptTailPage(_) => AgentObjectKind::TranscriptPage,
            Self::TranscriptChunk(_) => AgentObjectKind::TranscriptChunk,
            Self::Content(_) => AgentObjectKind::Content,
            Self::CheckpointVersion(_) => AgentObjectKind::CheckpointVersion,
            Self::Association(_) => AgentObjectKind::Association,
            Self::PortableCheckpointVersion(_) => AgentObjectKind::CheckpointVersion,
            Self::PortableAssociation(_) => AgentObjectKind::Association,
            Self::Alias(_) => AgentObjectKind::Alias,
            Self::RelationPage(_) => AgentObjectKind::RelationPage,
            Self::SourceState(_) => AgentObjectKind::SourceState,
            Self::ProjectionCatalog(_) => AgentObjectKind::ProjectionCatalog,
            Self::CatalogConflict(_) => AgentObjectKind::CatalogConflict,
        }
    }
    pub fn validate(&self) -> Result<()> {
        match self {
            Self::IndexPage(value) => value.validate(),
            Self::SessionSnapshot(value) => value.validate(),
            Self::ToolSnapshot(value) => value.validate(),
            Self::ObservationBatch(value) => value.validate(),
            Self::EventBatch(value) => value.validate(),
            Self::TranscriptManifest(value) => value.validate(),
            Self::TranscriptTailPage(value) => value.validate(),
            Self::TranscriptChunk(value) => value.validate(),
            Self::Content(value) => value.validate(),
            Self::CheckpointVersion(value) => value.validate(),
            Self::Association(value) => value.validate(),
            Self::PortableCheckpointVersion(value) => value.validate(),
            Self::PortableAssociation(value) => value.validate(),
            Self::Alias(value) => value.validate(),
            Self::RelationPage(value) => value.validate(),
            Self::SourceState(value) => value.validate(),
            Self::ProjectionCatalog(value) => value.validate(),
            Self::CatalogConflict(value) => value.validate(),
        }
    }
    pub fn dependencies(&self) -> Result<Vec<AgentObjectRef>> {
        self.validate()?;
        Ok(match self {
            Self::IndexPage(value) => value.dependencies(),
            Self::SessionSnapshot(value) => value.dependencies()?.into_iter().collect(),
            Self::ToolSnapshot(value) => value.dependencies()?.into_iter().collect(),
            Self::ObservationBatch(_) | Self::TranscriptChunk(_) | Self::Content(_) => Vec::new(),
            Self::EventBatch(value) => value.dependencies()?.into_iter().collect(),
            Self::TranscriptManifest(value) => value.dependencies()?.into_iter().collect(),
            Self::TranscriptTailPage(value) => value.dependencies()?.into_iter().collect(),
            Self::CheckpointVersion(value) => value.dependencies()?.into_iter().collect(),
            Self::Association(value) => value.dependencies()?.into_iter().collect(),
            Self::PortableCheckpointVersion(value) => value.dependencies()?.into_iter().collect(),
            Self::PortableAssociation(value) => value.dependencies()?.into_iter().collect(),
            Self::Alias(_) | Self::SourceState(_) => Vec::new(),
            Self::RelationPage(value) => value.dependencies()?.into_iter().collect(),
            Self::ProjectionCatalog(value) => value.dependencies()?.into_iter().collect(),
            Self::CatalogConflict(value) => value.dependencies()?,
        })
    }
    pub fn native_dependencies(&self) -> Result<BTreeSet<ObjectKey>> {
        self.validate()?;
        match self {
            Self::CheckpointVersion(value) => value.native_dependencies(),
            Self::Association(value) => value.native_dependencies(),
            Self::CatalogConflict(_) => Ok(BTreeSet::new()),
            _ => Ok(BTreeSet::new()),
        }
    }
    pub fn code_evidence(&self) -> Result<BTreeSet<jj_store_model::portable::CodeEvidenceRefV1>> {
        self.validate()?;
        match self {
            Self::PortableCheckpointVersion(value) => value.code_evidence(),
            Self::PortableAssociation(value) => value.code_evidence(),
            Self::RelationPage(value) => match &value.subject {
                crate::AgentRelationSubjectV1::PortableOperation { evidence }
                | crate::AgentRelationSubjectV1::PortableRevision { evidence, .. } => {
                    Ok(BTreeSet::from([evidence.clone()]))
                }
                _ => Ok(BTreeSet::new()),
            },
            _ => Ok(BTreeSet::new()),
        }
    }
}
