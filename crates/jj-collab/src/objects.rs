use crate::*;
use jj_agent::AgentObjectRef;
use jj_review::ReviewObjectRef;
use jj_store_model::portable::CodeEvidenceRefV1;
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
pub enum CollabObjectKind {
    IndexPage,
    ThreadVersion,
    ConversationMetadata,
    ThreadEvent,
    HumanMessageRevision,
    ThreadLink,
    Reaction,
    ReactionRetraction,
    ReactionSlot,
    CustomEmojiVersion,
    ImageAsset,
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
pub struct CollabObjectRef {
    pub kind: CollabObjectKind,
    pub id: CollabObjectId,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(
    tag = "type",
    content = "value",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum CollabObjectV1 {
    IndexPage(CollabIndexPageV1),
    ThreadVersion(ThreadVersionV1),
    ConversationMetadata(ConversationMetadataV1),
    ThreadEvent(ThreadEventV1),
    HumanMessageRevision(HumanMessageRevisionV1),
    ThreadLink(ThreadLinkV1),
    Reaction(ReactionV1),
    ReactionRetraction(ReactionRetractionV1),
    ReactionSlot(ReactionSlotV1),
    CustomEmojiVersion(CustomEmojiVersionV1),
    ImageAsset(ImageAssetV1),
}
impl CollabObjectV1 {
    pub fn kind(&self) -> CollabObjectKind {
        match self {
            Self::IndexPage(_) => CollabObjectKind::IndexPage,
            Self::ThreadVersion(_) => CollabObjectKind::ThreadVersion,
            Self::ConversationMetadata(_) => CollabObjectKind::ConversationMetadata,
            Self::ThreadEvent(_) => CollabObjectKind::ThreadEvent,
            Self::HumanMessageRevision(_) => CollabObjectKind::HumanMessageRevision,
            Self::ThreadLink(_) => CollabObjectKind::ThreadLink,
            Self::Reaction(_) => CollabObjectKind::Reaction,
            Self::ReactionRetraction(_) => CollabObjectKind::ReactionRetraction,
            Self::ReactionSlot(_) => CollabObjectKind::ReactionSlot,
            Self::CustomEmojiVersion(_) => CollabObjectKind::CustomEmojiVersion,
            Self::ImageAsset(_) => CollabObjectKind::ImageAsset,
        }
    }
    pub fn validate(&self) -> Result<()> {
        match self {
            Self::IndexPage(v) => v.validate(),
            Self::ThreadVersion(v) => v.validate(),
            Self::ConversationMetadata(v) => v.validate(),
            Self::ThreadEvent(v) => v.validate(),
            Self::HumanMessageRevision(v) => v.validate(),
            Self::ThreadLink(v) => v.validate(),
            Self::Reaction(v) => v.validate(),
            Self::ReactionRetraction(v) => v.validate(),
            Self::ReactionSlot(v) => v.validate(),
            Self::CustomEmojiVersion(v) => v.validate(),
            Self::ImageAsset(v) => v.validate(),
        }
    }
    pub fn reference(&self) -> Result<CollabObjectRef> {
        Ok(CollabObjectRef {
            kind: self.kind(),
            id: CollabObjectId::sha256(&encode_object(self)?),
        })
    }
    pub fn dependencies(&self) -> Result<BTreeSet<CollabObjectRef>> {
        self.validate()?;
        let refs = match self {
            Self::IndexPage(v) => v.dependencies(),
            Self::ConversationMetadata(v) => v.previous.into_iter().collect(),
            Self::ThreadVersion(v) => v
                .previous
                .into_iter()
                .chain([v.event])
                .chain(
                    v.comment_source
                        .iter()
                        .flat_map(|s| s.target.collab_dependencies()),
                )
                .collect(),
            Self::ThreadEvent(v) => v
                .previous
                .into_iter()
                .chain(match v.event {
                    ThreadEventKindV1::Message { revision } => Some(revision),
                    ThreadEventKindV1::Link { version }
                    | ThreadEventKindV1::Metadata { version } => Some(version),
                    _ => None,
                })
                .collect(),
            Self::HumanMessageRevision(v) => v
                .previous
                .into_iter()
                .chain(v.reply_to)
                .chain(v.emojis.values().copied())
                .collect(),
            Self::ThreadLink(v) => {
                let mut dependencies: BTreeSet<_> = v.previous.into_iter().collect();
                if let ThreadLinkTargetV1::CaptureGap { binding, .. } = &v.target {
                    dependencies.insert(binding.observed_thread);
                    dependencies.extend(binding.observed_assignment);
                }
                if let ThreadLinkTargetV1::CommentBatch {
                    comments,
                    message,
                    reply_to,
                    ..
                } = &v.target
                {
                    dependencies.extend(message);
                    dependencies.extend(reply_to);
                    dependencies.extend(comments.iter().flat_map(|c| c.collab_dependencies()));
                }
                if let ThreadLinkTargetV1::CommentReply { reply } = &v.target {
                    dependencies.extend(reply.targets().flat_map(|t| t.collab_dependencies()));
                }
                dependencies.into_iter().collect()
            }
            Self::Reaction(v) => v
                .target
                .collab_dependencies()
                .into_iter()
                .chain(match v.value {
                    ReactionValueV1::Custom { version, .. } => Some(version),
                    _ => None,
                })
                .collect(),
            Self::ReactionRetraction(v) => v
                .observed
                .iter()
                .copied()
                .chain(v.target.collab_dependencies())
                .collect(),
            Self::ReactionSlot(v) => v
                .additions
                .iter()
                .chain(&v.retractions)
                .copied()
                .chain(v.target.collab_dependencies())
                .collect(),
            Self::CustomEmojiVersion(v) => {
                v.previous.into_iter().chain([v.asset, v.preview]).collect()
            }
            Self::ImageAsset(_) => Vec::new(),
        };
        Ok(refs.into_iter().collect())
    }
    pub fn agent_dependencies(&self) -> BTreeSet<AgentObjectRef> {
        match self {
            Self::ConversationMetadata(v) => v
                .value
                .source_refs()
                .into_iter()
                .map(|s| s.snapshot)
                .collect(),
            Self::ThreadVersion(v) => v
                .agent_message
                .as_ref()
                .map(|t| t.agent_dependencies())
                .unwrap_or_default()
                .union(
                    &v.comment_source
                        .as_ref()
                        .map(|s| s.target.agent_dependencies())
                        .unwrap_or_default(),
                )
                .copied()
                .collect(),
            Self::ThreadLink(v) => match &v.target {
                ThreadLinkTargetV1::CaptureGap {
                    source_state,
                    event_batch,
                    ..
                } => BTreeSet::from([*source_state, *event_batch]),
                ThreadLinkTargetV1::Session { snapshot, .. } => BTreeSet::from([*snapshot]),
                ThreadLinkTargetV1::CommentReply { reply } => reply
                    .targets()
                    .flat_map(|t| t.agent_dependencies())
                    .collect(),
                ThreadLinkTargetV1::Code { evidence, .. } => evidence.iter().copied().collect(),
                _ => BTreeSet::new(),
            },
            Self::Reaction(v) => v.target.agent_dependencies(),
            Self::ReactionRetraction(v) => v.target.agent_dependencies(),
            Self::ReactionSlot(v) => v.target.agent_dependencies(),
            _ => BTreeSet::new(),
        }
    }
    pub fn review_dependencies(&self) -> BTreeSet<ReviewObjectRef> {
        match self {
            Self::ThreadVersion(v) => v
                .comment_source
                .as_ref()
                .map(|s| s.target.review_dependencies())
                .unwrap_or_default(),
            Self::ThreadLink(v) => match &v.target {
                ThreadLinkTargetV1::Review { snapshot, .. } => BTreeSet::from([*snapshot]),
                ThreadLinkTargetV1::CommentReply { reply } => reply
                    .targets()
                    .flat_map(|t| t.review_dependencies())
                    .collect(),
                ThreadLinkTargetV1::CommentBatch { comments, .. } => comments
                    .iter()
                    .flat_map(|c| c.review_dependencies())
                    .collect(),
                _ => BTreeSet::new(),
            },
            Self::Reaction(v) => v.target.review_dependencies(),
            Self::ReactionRetraction(v) => v.target.review_dependencies(),
            Self::ReactionSlot(v) => v.target.review_dependencies(),
            _ => BTreeSet::new(),
        }
    }
    pub fn native_dependencies(&self) -> BTreeSet<jj_store_model::ObjectKey> {
        match self {
            Self::ThreadVersion(v) => v
                .baseline
                .as_ref()
                .map(|a| a.native_dependencies())
                .unwrap_or_default(),
            Self::ThreadLink(v) => match &v.target {
                ThreadLinkTargetV1::Code { anchor, .. } => anchor.native_dependencies(),
                _ => Default::default(),
            },
            _ => Default::default(),
        }
    }
    pub fn code_evidence(&self) -> BTreeSet<CodeEvidenceRefV1> {
        match self {
            Self::ThreadVersion(v) => v
                .baseline
                .as_ref()
                .map(|a| a.evidence())
                .unwrap_or_default(),
            Self::ThreadLink(v) => match &v.target {
                ThreadLinkTargetV1::Code { anchor, .. } => anchor.evidence(),
                _ => BTreeSet::new(),
            },
            _ => BTreeSet::new(),
        }
    }
}
pub(crate) fn expect(reference: CollabObjectRef, kind: CollabObjectKind) -> Result<()> {
    if reference.kind == kind {
        Ok(())
    } else {
        Err(Error::Invalid("collaboration object kind"))
    }
}
