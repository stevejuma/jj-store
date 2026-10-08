use crate::*;
use jj_agent::{AgentObjectRef, AgentSessionId, CodeAnchorV1, CodeAnchorV2, EvidenceMethod};
use jj_review::{ReviewObjectRef, ReviewThreadId};
use jj_store_model::PrincipalId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ThreadStatusV1 {
    Created,
    Active,
    Idle,
    Archived,
    Tombstoned,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ThreadActionV1 {
    Activate,
    Idle,
    Archive,
    Tombstone,
    Restore,
}
impl ThreadStatusV1 {
    /// Restore authorization belongs to the publication provider, not the state machine.
    pub fn transition(self, action: ThreadActionV1) -> Result<Self> {
        use ThreadActionV1 as A;
        use ThreadStatusV1 as S;
        match (self, action) {
            (S::Created | S::Idle, A::Activate) => Ok(S::Active),
            (S::Active, A::Idle) => Ok(S::Idle),
            (S::Active | S::Idle, A::Archive) => Ok(S::Archived),
            (S::Active | S::Idle | S::Archived, A::Tombstone) => Ok(S::Tombstoned),
            (S::Archived | S::Tombstoned, A::Restore) => Ok(S::Idle),
            _ => Err(Error::Invalid("Thread lifecycle transition")),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ThreadVersionV1 {
    pub schema_version: u32,
    pub thread_id: ThreadId,
    pub title: String,
    pub status: ThreadStatusV1,
    pub actor: PrincipalId,
    pub accepted_at_millis: i64,
    pub previous: Option<CollabObjectRef>,
    pub event: CollabObjectRef,
    pub baseline: Option<ThreadCodeAnchorV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_message: Option<ReactionTargetV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment_source: Option<ConversationSourceV1>,
}
impl ThreadVersionV1 {
    pub fn validate(&self) -> Result<()> {
        schema(self.schema_version)?;
        if let Some(source) = &self.comment_source {
            source.validate()?;
        }
        if let Some(target) = &self.agent_message {
            validate_agent_discussion(target)?;
        }
        if !label(&self.title, 1024) || self.accepted_at_millis < 0 {
            return Err(Error::Invalid("Thread version"));
        }
        expect(self.event, CollabObjectKind::ThreadEvent)?;
        if let Some(prior) = self.previous {
            expect(prior, CollabObjectKind::ThreadVersion)?;
        }
        if let Some(anchor) = &self.baseline {
            anchor
                .validate()
                .map_err(|_| Error::Invalid("Thread baseline"))?;
        }
        Ok(())
    }
}
pub(crate) fn validate_agent_discussion(target: &ReactionTargetV1) -> Result<()> {
    if !matches!(
        target,
        ReactionTargetV1::AgentMessage { .. }
            | ReactionTargetV1::ToolInvocation { .. }
            | ReactionTargetV1::ToolOutput { .. }
    ) {
        return Err(Error::Invalid("agent discussion target"));
    }
    target.validate()
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum ThreadEventKindV1 {
    Created,
    Renamed {
        previous_title: String,
    },
    Lifecycle {
        previous_status: ThreadStatusV1,
        action: ThreadActionV1,
    },
    Message {
        revision: CollabObjectRef,
    },
    Link {
        version: CollabObjectRef,
    },
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ThreadEventV1 {
    pub schema_version: u32,
    pub id: ThreadEventId,
    pub thread_id: ThreadId,
    pub actor: PrincipalId,
    pub accepted_at_millis: i64,
    pub previous: Option<CollabObjectRef>,
    pub event: ThreadEventKindV1,
}
impl ThreadEventV1 {
    pub fn validate(&self) -> Result<()> {
        schema(self.schema_version)?;
        if self.accepted_at_millis < 0 {
            return Err(Error::Invalid("Thread event time"));
        }
        if let Some(prior) = self.previous {
            expect(prior, CollabObjectKind::ThreadEvent)?;
        }
        match &self.event {
            ThreadEventKindV1::Created if self.previous.is_some() => {
                return Err(Error::Invalid("Thread creation history"));
            }
            ThreadEventKindV1::Renamed { previous_title } if !label(previous_title, 1024) => {
                return Err(Error::Invalid("Thread prior title"));
            }
            ThreadEventKindV1::Lifecycle {
                previous_status,
                action,
            } => {
                previous_status.transition(*action)?;
            }
            ThreadEventKindV1::Message { revision } => {
                expect(*revision, CollabObjectKind::HumanMessageRevision)?
            }
            ThreadEventKindV1::Link { version } => expect(*version, CollabObjectKind::ThreadLink)?,
            _ => {}
        }
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct HumanMessageRevisionV1 {
    pub schema_version: u32,
    pub message_id: HumanMessageId,
    pub thread_id: ThreadId,
    pub actor: PrincipalId,
    pub accepted_at_millis: i64,
    pub previous: Option<CollabObjectRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<CollabObjectRef>,
    pub body: Option<String>,
}
impl HumanMessageRevisionV1 {
    pub fn validate(&self) -> Result<()> {
        schema(self.schema_version)?;
        if let Some(parent) = self.reply_to {
            expect(parent, CollabObjectKind::HumanMessageRevision)?;
        }
        if self.accepted_at_millis < 0
            || self
                .body
                .as_ref()
                .is_some_and(|b| b.trim().is_empty() || b.len() > 256 * 1024 || b.contains('\0'))
        {
            return Err(Error::Invalid("human message"));
        }
        if let Some(prior) = self.previous {
            expect(prior, CollabObjectKind::HumanMessageRevision)?;
        } else if self.body.is_none() {
            return Err(Error::Invalid("new tombstoned message"));
        }
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum ThreadLinkTargetV1 {
    CommentReply {
        reply: CommentReplyLinkV1,
    },
    CommentBatch {
        request_id: jj_store_model::RequestId,
        comments: Vec<ReactionTargetV1>,
        message: Option<CollabObjectRef>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        reply_to: Option<CollabObjectRef>,
    },
    CaptureGap {
        source_state: AgentObjectRef,
        event_batch: AgentObjectRef,
        binding: ThreadBindingV1,
    },
    Session {
        session_id: AgentSessionId,
        snapshot: AgentObjectRef,
        primary: bool,
    },
    Code {
        anchor: ThreadCodeAnchorV1,
        evidence_method: EvidenceMethod,
        evidence: Option<AgentObjectRef>,
    },
    Review {
        review_id: ReviewThreadId,
        snapshot: ReviewObjectRef,
    },
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ThreadLinkV1 {
    pub schema_version: u32,
    pub id: ThreadLinkId,
    pub thread_id: ThreadId,
    pub actor: PrincipalId,
    pub accepted_at_millis: i64,
    pub previous: Option<CollabObjectRef>,
    pub active: bool,
    pub target: ThreadLinkTargetV1,
}
impl ThreadLinkV1 {
    pub fn validate(&self) -> Result<()> {
        schema(self.schema_version)?;
        if self.accepted_at_millis < 0 {
            return Err(Error::Invalid("Thread link time"));
        }
        if let Some(prior) = self.previous {
            expect(prior, CollabObjectKind::ThreadLink)?;
        } else if !self.active {
            return Err(Error::Invalid("new inactive Thread link"));
        }
        match &self.target {
            ThreadLinkTargetV1::CommentReply { reply } => reply.validate()?,
            ThreadLinkTargetV1::CommentBatch {
                comments,
                message,
                reply_to,
                ..
            } => {
                if comments.len() > 128 {
                    return Err(Error::Limit);
                }
                let mut seen = std::collections::BTreeSet::new();
                for target in comments {
                    validate_comment_reference(target)?;
                    if !seen.insert(target.exact_key()?) {
                        return Err(Error::Invalid("repeated batch comment"));
                    }
                }
                if let Some(r) = reply_to {
                    expect(*r, CollabObjectKind::HumanMessageRevision)?;
                }
                if let Some(r) = message {
                    expect(*r, CollabObjectKind::HumanMessageRevision)?;
                }
            }
            ThreadLinkTargetV1::CaptureGap {
                source_state,
                event_batch,
                binding,
            } => {
                binding.validate()?;
                if binding.thread_id != self.thread_id
                    || source_state.kind != jj_agent::AgentObjectKind::SourceState
                    || event_batch.kind != jj_agent::AgentObjectKind::EventBatch
                {
                    return Err(Error::Invalid("capture gap link"));
                }
            }
            ThreadLinkTargetV1::Session { snapshot, .. }
                if snapshot.kind != jj_agent::AgentObjectKind::SessionSnapshot =>
            {
                return Err(Error::Invalid("session snapshot"));
            }
            ThreadLinkTargetV1::Review { snapshot, .. }
                if snapshot.kind != jj_review::ReviewObjectKind::ThreadSnapshot =>
            {
                return Err(Error::Invalid("review snapshot"));
            }
            ThreadLinkTargetV1::Code {
                anchor,
                evidence_method,
                evidence,
            } => {
                anchor
                    .validate()
                    .map_err(|_| Error::Invalid("code link anchor"))?;
                if *evidence_method != EvidenceMethod::ManualAssertion && evidence.is_none() {
                    return Err(Error::Invalid("code link evidence"));
                }
            }
            _ => {}
        }
        Ok(())
    }
}
/// Capture adapters may bind explicitly. Absence is the supported unlinked state.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ThreadBindingV1 {
    pub thread_id: ThreadId,
    pub observed_thread: CollabObjectRef,
    pub observed_assignment: Option<CollabObjectRef>,
}
impl ThreadBindingV1 {
    pub fn validate(&self) -> Result<()> {
        expect(self.observed_thread, CollabObjectKind::ThreadVersion)?;
        if let Some(reference) = self.observed_assignment {
            expect(reference, CollabObjectKind::ThreadLink)?;
        }
        Ok(())
    }
}

/// Native IDs and portable evidence remain distinct; neither is a device path.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(
    tag = "profile",
    content = "anchor",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum ThreadCodeAnchorV1 {
    Native(CodeAnchorV1),
    Portable(CodeAnchorV2),
}
impl ThreadCodeAnchorV1 {
    pub fn validate(&self) -> Result<()> {
        match self {
            Self::Native(v)
                if v.operations.len() > 256
                    || v.commits.len() > 256
                    || (v.operations.is_empty() && v.commits.is_empty()) =>
            {
                Err(Error::Invalid("native Thread anchor"))
            }
            Self::Portable(v) => {
                v.validate()
                    .map_err(|_| Error::Invalid("portable Thread anchor"))?;
                if v.is_empty() {
                    return Err(Error::Invalid("empty portable Thread anchor"));
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }
    pub fn evidence(
        &self,
    ) -> std::collections::BTreeSet<jj_store_model::portable::CodeEvidenceRefV1> {
        match self {
            Self::Native(_) => Default::default(),
            Self::Portable(v) => v.evidence(),
        }
    }
    pub fn native_dependencies(&self) -> std::collections::BTreeSet<jj_store_model::ObjectKey> {
        use jj_store_model::ObjectKey;
        match self {
            Self::Native(v) => v
                .operations
                .iter()
                .copied()
                .map(ObjectKey::Operation)
                .chain(v.commits.iter().copied().map(ObjectKey::Commit))
                .collect(),
            Self::Portable(_) => Default::default(),
        }
    }
}
