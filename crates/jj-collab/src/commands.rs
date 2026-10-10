use crate::*;
use jj_agent::AgentSessionId;
use jj_store_model::{PrincipalId, RequestId};
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum ThreadCommandV1 {
    CreateIntentional {
        thread_id: ThreadId,
        title: String,
        repositories: Vec<jj_store_model::RepositoryId>,
    },
    CreateConversation {
        thread_id: ThreadId,
        title: String,
        target: ConversationMetadataTargetV1,
    },
    CreateAnnotation {
        thread_id: ThreadId,
        title: String,
    },
    SetCodebase {
        thread_id: ThreadId,
        expected: CollabObjectRef,
        repositories: Vec<jj_store_model::RepositoryId>,
    },
    SetMetadata {
        thread_id: ThreadId,
        expected_thread: CollabObjectRef,
        expected_metadata: Option<CollabObjectRef>,
        target: ConversationMetadataTargetV1,
        value: Box<ConversationMetadataValueV1>,
    },
    Submit {
        submission: Box<ConversationSubmissionV1>,
    },
    CreateCommentDiscussion {
        thread_id: ThreadId,
        title: String,
        source: ConversationSourceV1,
    },
    CreateAgentDiscussion {
        thread_id: ThreadId,
        title: String,
        target: ReactionTargetV1,
    },
    Create {
        thread_id: ThreadId,
        title: String,
        baseline: Option<ThreadCodeAnchorV1>,
    },
    Rename {
        thread_id: ThreadId,
        expected: CollabObjectRef,
        title: String,
    },
    Lifecycle {
        thread_id: ThreadId,
        expected: CollabObjectRef,
        action: ThreadActionV1,
    },
    Message {
        thread_id: ThreadId,
        expected_thread: CollabObjectRef,
        message_id: HumanMessageId,
        expected_revision: Option<CollabObjectRef>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        reply_to: Option<CollabObjectRef>,
        body: Option<String>,
    },
    Link {
        thread_id: ThreadId,
        expected_thread: CollabObjectRef,
        link_id: ThreadLinkId,
        target: ThreadLinkTargetV1,
        observed_primary: Option<CollabObjectRef>,
    },
    Detach {
        thread_id: ThreadId,
        expected_thread: CollabObjectRef,
        expected_link: CollabObjectRef,
    },
}
impl ThreadCommandV1 {
    pub fn thread_id(&self) -> ThreadId {
        match self {
            Self::SetMetadata { thread_id, .. } => *thread_id,
            Self::Submit { submission } => submission.thread_id,
            Self::Create { thread_id, .. }
            | Self::CreateAgentDiscussion { thread_id, .. }
            | Self::CreateCommentDiscussion { thread_id, .. }
            | Self::Rename { thread_id, .. }
            | Self::Lifecycle { thread_id, .. }
            | Self::Message { thread_id, .. }
            | Self::Link { thread_id, .. }
            | Self::Detach { thread_id, .. } => *thread_id,
            Self::CreateIntentional { thread_id, .. }
            | Self::CreateConversation { thread_id, .. }
            | Self::CreateAnnotation { thread_id, .. }
            | Self::SetCodebase { thread_id, .. } => *thread_id,
        }
    }
    pub fn validate(&self) -> Result<()> {
        match self {
            Self::CreateIntentional {
                title,
                repositories,
                ..
            } => {
                if !label(title, 1024) {
                    return Err(Error::Invalid("Thread title"));
                }
                validate_codebase(repositories)?;
            }
            Self::CreateConversation { title, target, .. } => {
                if !label(title, 1024) {
                    return Err(Error::Invalid("conversation title"));
                }
                target.key()?;
            }
            Self::CreateAnnotation { title, .. } => {
                if !label(title, 1024) {
                    return Err(Error::Invalid("annotation title"));
                }
            }
            Self::SetCodebase {
                expected,
                repositories,
                ..
            } => {
                expect(*expected, CollabObjectKind::ThreadVersion)?;
                validate_codebase(repositories)?;
            }
            Self::SetMetadata {
                expected_thread,
                expected_metadata,
                target,
                value,
                ..
            } => {
                expect(*expected_thread, CollabObjectKind::ThreadVersion)?;
                if let Some(p) = expected_metadata {
                    expect(*p, CollabObjectKind::ConversationMetadata)?;
                }
                target.key()?;
                value.validate()?;
            }
            Self::Submit { submission } => submission.validate()?,
            Self::CreateCommentDiscussion { title, source, .. } => {
                if !label(title, 1024) {
                    return Err(Error::Invalid("discussion title"));
                }
                source.validate()?;
            }
            Self::CreateAgentDiscussion { title, target, .. } => {
                if !label(title, 1024) {
                    return Err(Error::Invalid("Thread title"));
                }
                crate::threads::validate_agent_discussion(target)?;
            }
            Self::Create {
                title, baseline, ..
            } => {
                if !label(title, 1024) {
                    return Err(Error::Invalid("Thread title"));
                }
                if let Some(anchor) = baseline {
                    anchor
                        .validate()
                        .map_err(|_| Error::Invalid("Thread baseline"))?;
                }
            }
            Self::Rename {
                expected, title, ..
            } => {
                expect(*expected, CollabObjectKind::ThreadVersion)?;
                if !label(title, 1024) {
                    return Err(Error::Invalid("Thread title"));
                }
            }
            Self::Lifecycle { expected, .. } => expect(*expected, CollabObjectKind::ThreadVersion)?,
            Self::Message {
                expected_thread,
                expected_revision,
                reply_to,
                body,
                ..
            } => {
                expect(*expected_thread, CollabObjectKind::ThreadVersion)?;
                if let Some(reference) = expected_revision {
                    expect(*reference, CollabObjectKind::HumanMessageRevision)?;
                }
                if let Some(parent) = reply_to {
                    expect(*parent, CollabObjectKind::HumanMessageRevision)?;
                }
                if body.as_ref().is_some_and(|b| {
                    b.trim().is_empty() || b.len() > 256 * 1024 || b.contains('\0')
                }) {
                    return Err(Error::Invalid("human message body"));
                }
                if body.is_none() && expected_revision.is_none() {
                    return Err(Error::Invalid("new tombstoned message"));
                }
            }
            Self::Link {
                expected_thread,
                observed_primary,
                ..
            } => {
                expect(*expected_thread, CollabObjectKind::ThreadVersion)?;
                if let Some(reference) = observed_primary {
                    expect(*reference, CollabObjectKind::ThreadLink)?;
                }
            }
            Self::Detach {
                expected_thread,
                expected_link,
                ..
            } => {
                expect(*expected_thread, CollabObjectKind::ThreadVersion)?;
                expect(*expected_link, CollabObjectKind::ThreadLink)?;
            }
        }
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ThreadCommandResultV1 {
    pub thread_id: ThreadId,
    pub version: CollabObjectRef,
    pub event: CollabObjectRef,
    pub message: Option<CollabObjectRef>,
    pub link: Option<CollabObjectRef>,
    pub reassigned_from: Option<ThreadId>,
}
#[derive(Clone, Debug)]
pub struct ThreadStateV1 {
    pub root: ThreadRootV1,
    pub threads: BTreeMap<ThreadId, CollabObjectRef>,
    pub messages: BTreeMap<HumanMessageId, CollabObjectRef>,
    pub links: BTreeMap<ThreadLinkId, CollabObjectRef>,
    pub assignments: BTreeMap<AgentSessionId, CollabObjectRef>,
    pub metadata: BTreeMap<CollabCatalogKey, CollabObjectRef>,
}
impl ThreadStateV1 {
    pub fn read(root: ThreadRootV1, graph: &CollabGraphV1) -> Result<Self> {
        root.validate()?;
        graph.closure(root.dependencies())?;
        let mut result = Self {
            root: root.clone(),
            threads: BTreeMap::new(),
            messages: BTreeMap::new(),
            links: BTreeMap::new(),
            assignments: BTreeMap::new(),
            metadata: BTreeMap::new(),
        };
        for (key, reference) in graph.catalog(root.threads)? {
            let CollabObjectV1::ThreadVersion(v) = graph.get(reference)? else {
                return Err(Error::Invalid("Thread catalog kind"));
            };
            if key != entity_key("thread", v.thread_id.as_bytes())? {
                return Err(Error::Invalid("Thread catalog key"));
            }
            result.threads.insert(v.thread_id, reference);
        }
        for (key, reference) in graph.catalog(root.messages)? {
            let CollabObjectV1::HumanMessageRevision(v) = graph.get(reference)? else {
                return Err(Error::Invalid("message catalog kind"));
            };
            if key != entity_key("message", v.message_id.as_bytes())?
                || !result.threads.contains_key(&v.thread_id)
            {
                return Err(Error::Invalid("message catalog membership"));
            }
            result.messages.insert(v.message_id, reference);
        }
        for (key, reference) in graph.catalog(root.links)? {
            let CollabObjectV1::ThreadLink(v) = graph.get(reference)? else {
                return Err(Error::Invalid("link catalog kind"));
            };
            if key != entity_key("link", v.id.as_bytes())?
                || !result.threads.contains_key(&v.thread_id)
            {
                return Err(Error::Invalid("link catalog membership"));
            }
            result.links.insert(v.id, reference);
        }
        for (key, reference) in graph.catalog(root.assignments)? {
            let CollabObjectV1::ThreadLink(v) = graph.get(reference)? else {
                return Err(Error::Invalid("assignment catalog kind"));
            };
            let ThreadLinkTargetV1::Session {
                session_id,
                primary: true,
                ..
            } = v.target
            else {
                return Err(Error::Invalid("assignment target"));
            };
            if key != entity_key("assignment", session_id.as_bytes())?
                || result.links.get(&v.id) != Some(&reference)
            {
                return Err(Error::Invalid("assignment catalog membership"));
            }
            if v.active {
                result.assignments.insert(session_id, reference);
            }
        }
        for (key, reference) in graph.catalog(root.metadata)? {
            let CollabObjectV1::ConversationMetadata(v) = graph.get(reference)? else {
                return Err(Error::Invalid("metadata catalog kind"));
            };
            if key != v.target.key()? || !result.threads.contains_key(&v.thread_id) {
                return Err(Error::Invalid("metadata catalog membership"));
            }
            result.metadata.insert(key, reference);
        }
        for reference in result.links.values() {
            let CollabObjectV1::ThreadLink(v) = graph.get(*reference)? else {
                unreachable!()
            };
            if let ThreadLinkTargetV1::Session {
                session_id,
                primary: true,
                ..
            } = v.target
                && v.active
                && result.assignments.get(&session_id) != Some(reference)
            {
                return Err(Error::Invalid("multiple primary Thread assignments"));
            }
        }
        Ok(result)
    }
    pub fn empty() -> Self {
        Self {
            root: ThreadRootV1::empty(),
            threads: BTreeMap::new(),
            messages: BTreeMap::new(),
            links: BTreeMap::new(),
            assignments: BTreeMap::new(),
            metadata: BTreeMap::new(),
        }
    }
    /// Pure staging. A provider must authorize all referenced targets and publish the selected root atomically.
    pub fn apply(
        &self,
        command: &ThreadCommandV1,
        actor: PrincipalId,
        request: RequestId,
        at: i64,
        graph: &mut CollabGraphV1,
    ) -> Result<(ThreadRootV1, ThreadCommandResultV1)> {
        command.validate()?;
        if matches!(command, ThreadCommandV1::SetMetadata { .. }) {
            return self.set_metadata(command, actor, request, at, graph);
        }
        if let ThreadCommandV1::Submit { submission } = command {
            return self.submit_conversation(submission, actor, request, at, graph);
        }
        let thread_id = command.thread_id();
        let mut root = self.root.clone();
        let old = self.threads.get(&thread_id).copied();
        let expected = match command {
            ThreadCommandV1::Submit { .. } | ThreadCommandV1::SetMetadata { .. } => unreachable!(),
            ThreadCommandV1::Create { .. }
            | ThreadCommandV1::CreateAgentDiscussion { .. }
            | ThreadCommandV1::CreateCommentDiscussion { .. } => None,
            ThreadCommandV1::CreateIntentional { .. }
            | ThreadCommandV1::CreateConversation { .. }
            | ThreadCommandV1::CreateAnnotation { .. } => None,
            ThreadCommandV1::Rename { expected, .. }
            | ThreadCommandV1::SetCodebase { expected, .. }
            | ThreadCommandV1::Lifecycle { expected, .. } => Some(*expected),
            ThreadCommandV1::Message {
                expected_thread, ..
            }
            | ThreadCommandV1::Link {
                expected_thread, ..
            }
            | ThreadCommandV1::Detach {
                expected_thread, ..
            } => Some(*expected_thread),
        };
        if old != expected {
            return Err(Error::Stale);
        }
        let mut message = None;
        let mut link = None;
        let mut reassigned_from = None;
        let prior = old.and_then(|r| match graph.get(r).ok()? {
            CollabObjectV1::ThreadVersion(v) => Some(v),
            _ => None,
        });
        let purpose = match command {
            ThreadCommandV1::CreateConversation { target, .. } => {
                Some(ThreadPurposeV1::Conversation {
                    target: target.clone(),
                })
            }
            ThreadCommandV1::CreateCommentDiscussion { .. }
            | ThreadCommandV1::CreateAgentDiscussion { .. }
            | ThreadCommandV1::CreateAnnotation { .. } => Some(ThreadPurposeV1::Annotation),
            _ => prior.and_then(|v| v.purpose.clone()),
        };
        let codebase = match command {
            ThreadCommandV1::CreateIntentional { repositories, .. }
            | ThreadCommandV1::SetCodebase { repositories, .. } => Some(repositories.clone()),
            _ => prior.and_then(|v| v.codebase.clone()),
        };
        if matches!(command, ThreadCommandV1::SetCodebase { .. })
            && !prior.is_some_and(ThreadVersionV1::is_user_thread)
        {
            return Err(Error::Invalid("internal conversation Codebase"));
        }
        let comment_source = match command {
            ThreadCommandV1::CreateCommentDiscussion { source, .. } => Some(source.clone()),
            _ => old.and_then(|r| match graph.get(r).ok()? {
                CollabObjectV1::ThreadVersion(v) => v.comment_source.clone(),
                _ => None,
            }),
        };
        let agent_message = match command {
            ThreadCommandV1::CreateCommentDiscussion { source, .. }
                if validate_agent_discussion(&source.target).is_ok() =>
            {
                Some(source.target.clone())
            }
            ThreadCommandV1::CreateAgentDiscussion { target, .. } => Some(target.clone()),
            _ => old.and_then(|r| match graph.get(r).ok()? {
                CollabObjectV1::ThreadVersion(v) => v.agent_message.clone(),
                _ => None,
            }),
        };
        let (title, status, baseline, event_kind) = match command {
            ThreadCommandV1::CreateAgentDiscussion { title, .. }
            | ThreadCommandV1::CreateIntentional { title, .. }
            | ThreadCommandV1::CreateConversation { title, .. }
            | ThreadCommandV1::CreateAnnotation { title, .. }
            | ThreadCommandV1::CreateCommentDiscussion { title, .. } => (
                title.clone(),
                ThreadStatusV1::Created,
                None,
                ThreadEventKindV1::Created,
            ),
            ThreadCommandV1::Create {
                title, baseline, ..
            } => (
                title.clone(),
                ThreadStatusV1::Created,
                baseline.clone(),
                ThreadEventKindV1::Created,
            ),
            _ => {
                let CollabObjectV1::ThreadVersion(previous) =
                    graph.get(old.ok_or(Error::Stale)?)?.clone()
                else {
                    return Err(Error::Invalid("Thread selection"));
                };
                let mut title = previous.title.clone();
                let mut status = previous.status;
                let event_kind = match command {
                    ThreadCommandV1::SetCodebase { .. } => ThreadEventKindV1::CodebaseChanged,
                    ThreadCommandV1::Rename { title: new, .. } => {
                        title = new.clone();
                        ThreadEventKindV1::Renamed {
                            previous_title: previous.title,
                        }
                    }
                    ThreadCommandV1::Lifecycle { action, .. } => {
                        status = status.transition(*action)?;
                        ThreadEventKindV1::Lifecycle {
                            previous_status: previous.status,
                            action: *action,
                        }
                    }
                    ThreadCommandV1::Message {
                        message_id,
                        expected_revision,
                        body,
                        reply_to,
                        ..
                    } => {
                        if matches!(
                            status,
                            ThreadStatusV1::Archived | ThreadStatusV1::Tombstoned
                        ) {
                            return Err(Error::Invalid("Thread is closed for human messages"));
                        }
                        if self.messages.get(message_id).copied() != *expected_revision {
                            return Err(Error::Stale);
                        }
                        if let Some(reference) = expected_revision {
                            let CollabObjectV1::HumanMessageRevision(old) =
                                graph.get(*reference)?
                            else {
                                return Err(Error::Invalid("message selection"));
                            };
                            if old.thread_id != thread_id
                                || old.actor != actor
                                || old.reply_to != *reply_to
                            {
                                return Err(Error::Invalid("message author or membership"));
                            }
                        }
                        if let Some(parent) = reply_to {
                            let CollabObjectV1::HumanMessageRevision(parent) =
                                graph.get(*parent)?
                            else {
                                return Err(Error::Invalid("reply parent"));
                            };
                            if parent.thread_id != thread_id
                                || parent.message_id == *message_id
                                || parent.body.is_none()
                                || !self.messages.contains_key(&parent.message_id)
                            {
                                return Err(Error::Invalid("reply parent membership"));
                            }
                        }
                        let reference = graph.stage(CollabObjectV1::HumanMessageRevision(
                            HumanMessageRevisionV1 {
                                reply_to: *reply_to,
                                schema_version: 1,
                                message_id: *message_id,
                                thread_id,
                                actor,
                                accepted_at_millis: at,
                                previous: *expected_revision,
                                body: body.clone(),
                            },
                        ))?;
                        root.messages = graph.catalog_update(
                            root.messages,
                            BTreeMap::from([(
                                entity_key("message", message_id.as_bytes())?,
                                reference,
                            )]),
                        )?;
                        message = Some(reference);
                        ThreadEventKindV1::Message {
                            revision: reference,
                        }
                    }
                    ThreadCommandV1::Link {
                        link_id,
                        target,
                        observed_primary,
                        ..
                    } => {
                        if self.links.contains_key(link_id) {
                            return Err(Error::Stale);
                        }
                        if let ThreadLinkTargetV1::Session {
                            session_id,
                            primary: true,
                            ..
                        } = target
                        {
                            if self.assignments.get(session_id).copied() != *observed_primary {
                                return Err(Error::Stale);
                            }
                            if let Some(reference) = observed_primary {
                                let CollabObjectV1::ThreadLink(mut old_link) =
                                    graph.get(*reference)?.clone()
                                else {
                                    return Err(Error::Invalid("primary assignment"));
                                };
                                old_link.active = false;
                                old_link.previous = Some(*reference);
                                old_link.actor = actor;
                                old_link.accepted_at_millis = at;
                                let prior_thread = old_link.thread_id;
                                let old_id = old_link.id;
                                let inactive = graph.stage(CollabObjectV1::ThreadLink(old_link))?;
                                root.links = graph.catalog_update(
                                    root.links,
                                    BTreeMap::from([(
                                        entity_key("link", old_id.as_bytes())?,
                                        inactive,
                                    )]),
                                )?;
                                if prior_thread != thread_id {
                                    let prior = self
                                        .threads
                                        .get(&prior_thread)
                                        .copied()
                                        .ok_or(Error::Invalid("assignment Thread"))?;
                                    let CollabObjectV1::ThreadVersion(mut other) =
                                        graph.get(prior)?.clone()
                                    else {
                                        return Err(Error::Invalid("assignment Thread selection"));
                                    };
                                    let event = stage_event(
                                        graph,
                                        prior_thread,
                                        actor,
                                        request,
                                        at,
                                        Some(other.event),
                                        ThreadEventKindV1::Link { version: inactive },
                                    )?;
                                    other.previous = Some(prior);
                                    other.event = event;
                                    other.actor = actor;
                                    other.accepted_at_millis = at;
                                    let other =
                                        graph.stage(CollabObjectV1::ThreadVersion(other))?;
                                    root.threads = graph.catalog_update(
                                        root.threads,
                                        BTreeMap::from([(
                                            entity_key("thread", prior_thread.as_bytes())?,
                                            other,
                                        )]),
                                    )?;
                                    reassigned_from = Some(prior_thread);
                                }
                            }
                        } else if let ThreadLinkTargetV1::CaptureGap { binding, .. } = target {
                            if binding.thread_id != thread_id
                                || Some(binding.observed_thread) != old
                                || binding.observed_assignment != *observed_primary
                            {
                                return Err(Error::Invalid("capture gap binding intent"));
                            }
                            if let Some(reference) = observed_primary {
                                let CollabObjectV1::ThreadLink(old) = graph.get(*reference)? else {
                                    return Err(Error::Invalid("capture gap assignment"));
                                };
                                let ThreadLinkTargetV1::Session {
                                    session_id,
                                    primary: true,
                                    ..
                                } = old.target
                                else {
                                    return Err(Error::Invalid("capture gap assignment"));
                                };
                                if !old.active
                                    || self.assignments.get(&session_id) != Some(reference)
                                    || self.links.get(&old.id) != Some(reference)
                                {
                                    return Err(Error::Stale);
                                }
                            }
                        } else if observed_primary.is_some() {
                            return Err(Error::Invalid("non-primary link assignment precondition"));
                        }
                        let reference = graph.stage(CollabObjectV1::ThreadLink(ThreadLinkV1 {
                            schema_version: 1,
                            id: *link_id,
                            thread_id,
                            actor,
                            accepted_at_millis: at,
                            previous: None,
                            active: true,
                            target: target.clone(),
                        }))?;
                        root.links = graph.catalog_update(
                            root.links,
                            BTreeMap::from([(entity_key("link", link_id.as_bytes())?, reference)]),
                        )?;
                        if let ThreadLinkTargetV1::Session {
                            session_id,
                            primary: true,
                            ..
                        } = target
                        {
                            root.assignments = graph.catalog_update(
                                root.assignments,
                                BTreeMap::from([(
                                    entity_key("assignment", session_id.as_bytes())?,
                                    reference,
                                )]),
                            )?;
                        }
                        link = Some(reference);
                        ThreadEventKindV1::Link { version: reference }
                    }
                    ThreadCommandV1::Detach { expected_link, .. } => {
                        let CollabObjectV1::ThreadLink(mut old_link) =
                            graph.get(*expected_link)?.clone()
                        else {
                            return Err(Error::Invalid("detach selection"));
                        };
                        if old_link.thread_id != thread_id
                            || !old_link.active
                            || self.links.get(&old_link.id) != Some(expected_link)
                        {
                            return Err(Error::Stale);
                        }
                        old_link.active = false;
                        old_link.previous = Some(*expected_link);
                        old_link.actor = actor;
                        old_link.accepted_at_millis = at;
                        let id = old_link.id;
                        let target = old_link.target.clone();
                        let reference = graph.stage(CollabObjectV1::ThreadLink(old_link))?;
                        root.links = graph.catalog_update(
                            root.links,
                            BTreeMap::from([(entity_key("link", id.as_bytes())?, reference)]),
                        )?;
                        if let ThreadLinkTargetV1::Session {
                            session_id,
                            primary: true,
                            ..
                        } = target
                        {
                            root.assignments = graph.catalog_update(
                                root.assignments,
                                BTreeMap::from([(
                                    entity_key("assignment", session_id.as_bytes())?,
                                    reference,
                                )]),
                            )?;
                        }
                        link = Some(reference);
                        ThreadEventKindV1::Link { version: reference }
                    }
                    _ => unreachable!(),
                };
                (title, status, previous.baseline, event_kind)
            }
        };
        let prior_event = old
            .map(|reference| match graph.get(reference) {
                Ok(CollabObjectV1::ThreadVersion(v)) => Ok(v.event),
                _ => Err(Error::Invalid("Thread selection")),
            })
            .transpose()?;
        let event = stage_event(
            graph,
            thread_id,
            actor,
            request,
            at,
            prior_event,
            event_kind,
        )?;
        let version = graph.stage(CollabObjectV1::ThreadVersion(ThreadVersionV1 {
            schema_version: 1,
            thread_id,
            title,
            status,
            actor,
            accepted_at_millis: at,
            previous: old,
            event,
            baseline,
            agent_message,
            comment_source,
            purpose,
            codebase,
        }))?;
        root.threads = graph.catalog_update(
            root.threads,
            BTreeMap::from([(entity_key("thread", thread_id.as_bytes())?, version)]),
        )?;
        Self::read(root.clone(), graph)?;
        Ok((
            root,
            ThreadCommandResultV1 {
                thread_id,
                version,
                event,
                message,
                link,
                reassigned_from,
            },
        ))
    }
}
pub fn entity_key(namespace: &str, id: &[u8; 16]) -> Result<CollabCatalogKey> {
    CollabCatalogKey::derive(namespace, id)
}
fn stage_event(
    graph: &mut CollabGraphV1,
    thread_id: ThreadId,
    actor: PrincipalId,
    request: RequestId,
    at: i64,
    previous: Option<CollabObjectRef>,
    event: ThreadEventKindV1,
) -> Result<CollabObjectRef> {
    let mut hash = Sha256::new_with_prefix(b"jj-store/thread-event/v1\0");
    hash.update(actor.as_bytes());
    hash.update(request.as_bytes());
    hash.update(thread_id.as_bytes());
    let digest: [u8; 32] = hash.finalize().into();
    let mut id = [0; 16];
    id.copy_from_slice(&digest[..16]);
    graph.stage(CollabObjectV1::ThreadEvent(ThreadEventV1 {
        schema_version: 1,
        id: ThreadEventId::new(id),
        thread_id,
        actor,
        accepted_at_millis: at,
        previous,
        event,
    }))
}
/// A typed reaction command never accepts an actor from an API caller.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum ReactionCommandV1 {
    Set {
        target: ReactionTargetV1,
        value: ReactionValueV1,
        observed: std::collections::BTreeSet<CollabObjectRef>,
    },
    Clear {
        target: ReactionTargetV1,
        observed: std::collections::BTreeSet<CollabObjectRef>,
    },
    ResolveConflict {
        target: ReactionTargetV1,
        value: Option<ReactionValueV1>,
        observed: std::collections::BTreeSet<CollabObjectRef>,
    },
}
impl ReactionCommandV1 {
    pub fn target(&self) -> &ReactionTargetV1 {
        match self {
            Self::Set { target, .. }
            | Self::Clear { target, .. }
            | Self::ResolveConflict { target, .. } => target,
        }
    }
    pub fn validate(&self) -> Result<()> {
        self.target().validate()?;
        let observed = match self {
            Self::Set {
                value, observed, ..
            } => {
                value.validate()?;
                observed
            }
            Self::Clear { observed, .. } => observed,
            Self::ResolveConflict {
                value, observed, ..
            } => {
                if let Some(value) = value {
                    value.validate()?;
                }
                observed
            }
        };
        if observed.len() > MAX_SLOT_RECORDS {
            return Err(Error::Limit);
        }
        for reference in observed {
            expect(*reference, CollabObjectKind::Reaction)?;
        }
        Ok(())
    }
    pub fn apply(
        &self,
        slot: &ReactionSlotV1,
        reaction: ReactionId,
        retraction: RetractionId,
        at: i64,
        graph: &CollabGraphV1,
    ) -> Result<ReactionTransitionV1> {
        self.validate()?;
        if self.target().exact_key()? != slot.target.exact_key()? {
            return Err(Error::Invalid("reaction command target"));
        }
        match self {
            Self::Set {
                value, observed, ..
            } => slot.apply(
                observed,
                Some(value.clone()),
                false,
                (reaction, retraction),
                at,
                &graph.objects,
            ),
            Self::Clear { observed, .. } => slot.apply(
                observed,
                None,
                false,
                (reaction, retraction),
                at,
                &graph.objects,
            ),
            Self::ResolveConflict {
                value, observed, ..
            } => slot.apply(
                observed,
                value.clone(),
                true,
                (reaction, retraction),
                at,
                &graph.objects,
            ),
        }
    }
}
