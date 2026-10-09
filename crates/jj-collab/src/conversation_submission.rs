//! One receipt-backed ThreadStore projection for a prompt's human contributions.
use crate::*;
use jj_store_model::{PrincipalId, RequestId};
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ConversationCommentDraftV1 {
    pub thread_id: ThreadId,
    pub expected_thread: Option<CollabObjectRef>,
    pub source: ConversationSourceV1,
    pub message_id: HumanMessageId,
    pub expected_revision: Option<CollabObjectRef>,
    pub reply_to: Option<CollabObjectRef>,
    pub body: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ConversationSubmissionV1 {
    pub thread_id: ThreadId,
    pub expected_thread: CollabObjectRef,
    /// Ordinary thread messages only; agent prompt text remains in AgentStore.
    pub message_id: Option<HumanMessageId>,
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<CollabObjectRef>,
    pub comments: Vec<ConversationCommentDraftV1>,
    pub references: Vec<ReactionTargetV1>,
}
impl ConversationSubmissionV1 {
    pub fn validate(&self) -> Result<()> {
        expect(self.expected_thread, CollabObjectKind::ThreadVersion)?;
        if self.comments.len() + self.references.len() > 128
            || self
                .text
                .as_ref()
                .is_some_and(|s| s.trim().is_empty() || s.len() > 256 * 1024 || s.contains('\0'))
            || self.text.is_some() != self.message_id.is_some()
            || (self.text.is_none() && self.comments.is_empty() && self.references.is_empty())
        {
            return Err(Error::Invalid("conversation submission"));
        }
        if let Some(reply) = self.reply_to {
            expect(reply, CollabObjectKind::HumanMessageRevision)?;
        }
        let mut ids = BTreeSet::new();
        let mut observations = BTreeMap::new();
        let mut size = 0usize;
        for c in &self.comments {
            c.source.validate()?;
            if c.thread_id == self.thread_id
                || !ids.insert(c.message_id)
                || Some(c.message_id) == self.message_id
                || c.body.trim().is_empty()
                || c.body.contains('\0')
            {
                return Err(Error::Invalid("conversation comment"));
            }
            size = size
                .checked_add(c.body.len() + c.source.quote.len())
                .ok_or(Error::Limit)?;
            if size > 32768 {
                return Err(Error::Limit);
            }
            if let Some(old) = observations.insert(c.thread_id, (&c.source, c.expected_thread)) {
                if old != (&c.source, c.expected_thread) {
                    return Err(Error::Invalid("inconsistent discussion observations"));
                }
            }
            if let Some(r) = c.expected_thread {
                expect(r, CollabObjectKind::ThreadVersion)?;
            }
            if let Some(r) = c.expected_revision {
                expect(r, CollabObjectKind::HumanMessageRevision)?;
            }
            if let Some(r) = c.reply_to {
                expect(r, CollabObjectKind::HumanMessageRevision)?;
            }
            if c.expected_thread.is_none()
                && (c.reply_to.is_some() || c.expected_revision.is_some())
            {
                return Err(Error::Invalid("new discussion history"));
            }
        }
        let mut refs = BTreeSet::new();
        for target in &self.references {
            validate_comment_reference(target)?;
            if !refs.insert(target.exact_key()?) {
                return Err(Error::Invalid("duplicate comment reference"));
            }
        }
        Ok(())
    }
}
pub fn validate_comment_reference(target: &ReactionTargetV1) -> Result<()> {
    if !matches!(
        target,
        ReactionTargetV1::HumanMessage { .. }
            | ReactionTargetV1::ReviewComment { .. }
            | ReactionTargetV1::RepositoryReviewComment { .. }
    ) {
        return Err(Error::Invalid("comment reference kind"));
    }
    target.validate()
}

impl ThreadStateV1 {
    pub fn submit_conversation(
        &self,
        submission: &ConversationSubmissionV1,
        actor: PrincipalId,
        request: RequestId,
        at: i64,
        graph: &mut CollabGraphV1,
    ) -> Result<(ThreadRootV1, ThreadCommandResultV1)> {
        submission.validate()?;
        if self.threads.get(&submission.thread_id) != Some(&submission.expected_thread) {
            return Err(Error::Stale);
        }
        let CollabObjectV1::ThreadVersion(parent) = graph.get(submission.expected_thread)? else {
            return Err(Error::Invalid("submission thread"));
        };
        if matches!(
            parent.status,
            ThreadStatusV1::Archived | ThreadStatusV1::Tombstoned
        ) {
            return Err(Error::Invalid("closed conversation"));
        }
        if let Some(reference) = submission.reply_to {
            let CollabObjectV1::HumanMessageRevision(message) = graph.get(reference)? else {
                return Err(Error::Invalid("submission reply kind"));
            };
            if message.thread_id != submission.thread_id || message.body.is_none() {
                return Err(Error::Invalid("submission reply identity"));
            }
        }
        for c in &submission.comments {
            if self.threads.get(&c.thread_id).copied() != c.expected_thread
                || self.messages.get(&c.message_id).copied() != c.expected_revision
            {
                return Err(Error::Stale);
            }
            if let Some(reference) = c.expected_thread {
                let CollabObjectV1::ThreadVersion(thread) = graph.get(reference)? else {
                    return Err(Error::Invalid("discussion kind"));
                };
                let legacy_whole = thread.comment_source.is_none()
                    && c.source.selection.is_none()
                    && c.source.part.is_none()
                    && thread.agent_message.as_ref() == Some(&c.source.target);
                if thread.comment_source.as_ref() != Some(&c.source) && !legacy_whole {
                    return Err(Error::Invalid("discussion source changed"));
                }
            }
        }
        // Work on a clone: refusal cannot leave even an in-memory partial batch.
        let mut staged = graph.clone();
        let mut state = self.clone();
        let mut step = 0u64;
        let mut apply = |state: &mut ThreadStateV1,
                         command: ThreadCommandV1,
                         graph: &mut CollabGraphV1|
         -> Result<ThreadCommandResultV1> {
            let mut hash = Sha256::new_with_prefix(b"jj-store/conversation-step/v1\0");
            hash.update(request.as_bytes());
            hash.update(step.to_be_bytes());
            step += 1;
            let id: [u8; 32] = hash.finalize().into();
            let (root, result) = state.apply(
                &command,
                actor,
                RequestId::new(id[..16].try_into().unwrap()),
                at,
                graph,
            )?;
            *state = ThreadStateV1::read(root, graph)?;
            Ok(result)
        };
        let mut comments = submission.references.clone();
        for c in &submission.comments {
            if !state.threads.contains_key(&c.thread_id) {
                apply(
                    &mut state,
                    ThreadCommandV1::CreateCommentDiscussion {
                        thread_id: c.thread_id,
                        title: "Inline discussion".into(),
                        source: c.source.clone(),
                    },
                    &mut staged,
                )?;
            }
            let expected_thread = state.threads[&c.thread_id];
            let result = apply(
                &mut state,
                ThreadCommandV1::Message {
                    thread_id: c.thread_id,
                    expected_thread,
                    message_id: c.message_id,
                    expected_revision: c.expected_revision,
                    reply_to: c.reply_to,
                    body: Some(c.body.clone()),
                },
                &mut staged,
            )?;
            comments.push(ReactionTargetV1::HumanMessage {
                thread_id: c.thread_id,
                message_id: c.message_id,
                revision: result.message.ok_or(Error::Invalid("submitted comment"))?,
            });
        }
        let message =
            if let (Some(message_id), Some(text)) = (submission.message_id, &submission.text) {
                let expected_thread = state.threads[&submission.thread_id];
                apply(
                    &mut state,
                    ThreadCommandV1::Message {
                        thread_id: submission.thread_id,
                        expected_thread,
                        message_id,
                        expected_revision: None,
                        reply_to: submission.reply_to,
                        body: Some(text.clone()),
                    },
                    &mut staged,
                )?
                .message
            } else {
                None
            };
        let expected_thread = state.threads[&submission.thread_id];
        let result = apply(
            &mut state,
            ThreadCommandV1::Link {
                thread_id: submission.thread_id,
                expected_thread,
                link_id: ThreadLinkId::new(*request.as_bytes()),
                target: ThreadLinkTargetV1::CommentBatch {
                    request_id: request,
                    reply_to: submission.reply_to,
                    comments,
                    message,
                },
                observed_primary: None,
            },
            &mut staged,
        )?;
        *graph = staged;
        Ok((state.root, result))
    }
}

impl ThreadCommandV1 {
    pub fn conversation_sources(&self) -> Vec<&ConversationSourceV1> {
        match self {
            Self::CreateCommentDiscussion { source, .. } => vec![source],
            Self::Submit { submission } => submission.comments.iter().map(|c| &c.source).collect(),
            _ => vec![],
        }
    }
    pub fn comment_references(&self) -> Vec<&ReactionTargetV1> {
        match self {
            Self::Submit { submission } => submission.references.iter().collect(),
            Self::Link {
                target: ThreadLinkTargetV1::CommentBatch { comments, .. },
                ..
            } => comments.iter().collect(),
            Self::Link {
                target: ThreadLinkTargetV1::CommentReply { reply },
                ..
            } => reply.comments.iter().collect(),
            _ => vec![],
        }
    }
}
