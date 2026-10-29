use super::{LocalError, LocalRepository, LocalResult, collab_graph, collab_types::*};
use crate::domain::{
    AgentObjectIo, AgentObjectV1, AgentSnapshotRead, CollabTargetReaderV1, ReviewObjectV1,
    ReviewSnapshotRead,
};
use crate::metadata::{Domain, Preparation, PublishOutcome, Update};
use jj_collab::*;
use jj_store_core::{StoreKind, reaction_source_store, thread_source_stores};
use jj_store_model as m;

impl LocalRepository {
    pub async fn apply_collab_command(
        &self,
        request: &LocalCollabRequest,
    ) -> LocalResult<LocalCollabOutcome> {
        self.apply_collab_command_with_reviews(request, None).await
    }
    pub async fn apply_collab_command_with_reviews(
        &self,
        request: &LocalCollabRequest,
        qualified: Option<&dyn QualifiedReviewRead>,
    ) -> LocalResult<LocalCollabOutcome> {
        request.validate()?;
        let sources = match &request.command {
            LocalCollabCommand::Thread { intent } => thread_source_stores(intent),
            LocalCollabCommand::Reaction { intent } => {
                reaction_source_store(intent.target()).into_iter().collect()
            }
            LocalCollabCommand::Emoji { .. } => Default::default(),
        };
        self.check_identity(self.metadata.read()?.as_ref())?;
        let actor = self.ensure_actor()?;
        let proposed = Self::proposed_repository()?;
        let intent = request.intent_for(actor)?;
        let at = self.now_millis()?;
        for _ in 0..8 {
            let pin = self.metadata.read()?;
            self.check_identity(pin.as_ref())?;
            if let Some(value) = pin
                .as_ref()
                .and_then(|s| s.receipt(actor, request.request_id))
            {
                if value.intent != intent {
                    return Err(LocalError::IntentConflict);
                }
                return receipt(value);
            }
            let repository = pin.as_ref().map_or(proposed, |s| s.repository());
            let mut threads = if matches!(request.command, LocalCollabCommand::Thread { .. })
                || sources.contains(&StoreKind::Thread)
            {
                Some(self.thread_store_at(pin.as_ref()).await?)
            } else {
                None
            };
            let mut reactions = if !matches!(request.command, LocalCollabCommand::Thread { .. }) {
                Some(self.reaction_store_at(pin.as_ref(), repository).await?)
            } else {
                None
            };
            let agents = if sources.contains(&StoreKind::Agent) {
                Some(self.agent_store_at(pin.as_ref(), repository).await?)
            } else {
                None
            };
            let reviews = if sources.contains(&StoreKind::Review) {
                Some(self.review_store_at(pin.as_ref(), repository).await?)
            } else {
                None
            };
            let (domain, bytes, retained, result) = match &request.command {
                LocalCollabCommand::Thread { intent: command } => {
                    let threads = threads.as_mut().ok_or(LocalError::Corrupt)?;
                    if let ThreadCommandV1::SetMetadata { value, .. } = command
                        && !value.source_refs().is_empty()
                    {
                        for source in value.source_refs() {
                            let AgentObjectV1::SessionSnapshot(snapshot) = agents
                                .as_ref()
                                .ok_or(LocalError::Unsupported)?
                                .io
                                .read_agent_object(source.snapshot)
                                .await?
                            else {
                                return Err(LocalError::NotFound);
                            };
                            if snapshot.session_id != source.session {
                                return Err(LocalError::Corrupt);
                            }
                        }
                    }
                    if let ThreadCommandV1::CreateAgentDiscussion { target, .. } = command {
                        let reader = CollabTargetReaderV1::new(
                            repository,
                            None,
                            agents.as_ref().map(|v| v as &dyn AgentSnapshotRead),
                            None,
                        )?;
                        resolve_reaction_target(&reader, target).await?;
                    }
                    if !command.conversation_sources().is_empty()
                        || !command.comment_references().is_empty()
                    {
                        let reader = CollabTargetReaderV1::new(
                            repository,
                            Some((threads.state.root.clone(), &threads.graph)),
                            agents.as_ref().map(|v| v as &dyn AgentSnapshotRead),
                            reviews.as_ref().map(|v| v as &dyn ReviewSnapshotRead),
                        )?
                        .with_qualified_reviews(qualified);
                        for source in command.conversation_sources() {
                            resolve_conversation_source(&reader, source).await?;
                        }
                        if let ThreadCommandV1::Link {
                            target: ThreadLinkTargetV1::CommentReply { reply },
                            ..
                        } = command
                        {
                            jj_collab::resolve_comment_reply(&reader, reply).await?;
                        }
                        for target in command.comment_references() {
                            resolve_reaction_target(&reader, target).await?;
                        }
                    }
                    validate_link(command, agents.as_ref(), reviews.as_ref()).await?;
                    let (root, result) = threads.state.apply(
                        command,
                        actor,
                        request.request_id,
                        at,
                        &mut threads.graph,
                    )?;
                    let (bytes, retained) = collab_graph::stage(
                        self,
                        pin.as_ref(),
                        repository,
                        root,
                        &threads.graph,
                        &threads.external,
                    )
                    .await?;
                    (
                        Domain::Thread,
                        bytes,
                        retained,
                        LocalCollabResultV1::Thread { result },
                    )
                }
                LocalCollabCommand::Reaction { intent: command } => {
                    let reactions = reactions.as_mut().ok_or(LocalError::Corrupt)?;
                    let work = if sources.contains(&StoreKind::Work) {
                        Some(self.work_state(pin.as_ref())?)
                    } else {
                        None
                    };
                    let mut reader = CollabTargetReaderV1::new(
                        repository,
                        threads.as_ref().map(|v| (v.state.root.clone(), &v.graph)),
                        agents.as_ref().map(|v| v as &dyn AgentSnapshotRead),
                        reviews.as_ref().map(|v| v as &dyn ReviewSnapshotRead),
                    )?
                    .with_qualified_reviews(qualified);
                    if let Some(work) = &work {
                        reader = reader.with_work(repository, work)?;
                    }
                    resolve_reaction_target(&reader, command.target()).await?;
                    let dependencies = command.target().collab_dependencies();
                    if !dependencies.is_empty() {
                        let threads = threads.as_ref().ok_or(LocalError::Unsupported)?;
                        for reference in threads.graph.closure(dependencies)? {
                            let object = threads.graph.get(reference)?.clone();
                            if reactions.graph.stage(object)? != reference {
                                return Err(LocalError::Corrupt);
                            }
                        }
                    }
                    let (root, result) = reactions.state.apply(
                        command,
                        repository,
                        actor,
                        request.request_id,
                        at,
                        &mut reactions.graph,
                    )?;
                    let (bytes, retained) = collab_graph::stage(
                        self,
                        pin.as_ref(),
                        repository,
                        root,
                        &reactions.graph,
                        &reactions.external,
                    )
                    .await?;
                    (
                        Domain::Reaction,
                        bytes,
                        retained,
                        LocalCollabResultV1::Reaction { result },
                    )
                }
                LocalCollabCommand::Emoji { intent: command } => {
                    let reactions = reactions.as_mut().ok_or(LocalError::Corrupt)?;
                    let (root, version) =
                        reactions
                            .state
                            .apply_emoji(command, actor, at, &mut reactions.graph)?;
                    let (bytes, retained) = collab_graph::stage(
                        self,
                        pin.as_ref(),
                        repository,
                        root,
                        &reactions.graph,
                        &reactions.external,
                    )
                    .await?;
                    (
                        Domain::Reaction,
                        bytes,
                        retained,
                        LocalCollabResultV1::Emoji { version },
                    )
                }
            };
            let result = serde_json::to_vec(&CollabResult {
                accepted_at_millis: at,
                result,
            })
            .map_err(|_| LocalError::Corrupt)?;
            let update = Update {
                repository,
                actor,
                request: request.request_id,
                intent,
                domain,
                expected: pin.as_ref().and_then(|s| s.selection(domain)).cloned(),
                bytes,
                result,
                retained,
            };
            let candidate = match self.metadata.prepare(pin.as_ref(), &update)? {
                Preparation::Committed(value) => return receipt(&value),
                Preparation::Ready(candidate) => candidate,
            };
            match self.metadata.publish(*candidate)? {
                PublishOutcome::Committed(value) => return receipt(&value),
                PublishOutcome::Conflict => continue,
                PublishOutcome::Unknown => {
                    return Ok(LocalCollabOutcome::OutcomeUnknown {
                        request_id: request.request_id,
                    });
                }
            }
        }
        Err(LocalError::Busy)
    }
    pub fn collab_command_status(&self, request: m::RequestId) -> LocalResult<LocalCollabOutcome> {
        self.check_identity(self.metadata.read()?.as_ref())?;
        if let Some(actor) = self.actor()?
            && let Some(pin) = self.metadata.read()?
            && let Some(value) = pin.receipt(actor, request)
        {
            return receipt(value);
        }
        Ok(LocalCollabOutcome::NotKnown {
            request_id: request,
        })
    }
}
async fn validate_link(
    command: &ThreadCommandV1,
    agents: Option<&super::LocalAgentStore>,
    reviews: Option<&super::LocalReviewStore>,
) -> LocalResult<()> {
    let ThreadCommandV1::Link { target, .. } = command else {
        return Ok(());
    };
    match target {
        ThreadLinkTargetV1::CommentBatch { .. } | ThreadLinkTargetV1::CommentReply { .. } => {}
        ThreadLinkTargetV1::CaptureGap {
            source_state,
            event_batch,
            ..
        } => {
            agents
                .ok_or(LocalError::Unsupported)?
                .reads()
                .validate_capture_gap(*source_state, *event_batch)
                .await?;
        }

        ThreadLinkTargetV1::Session {
            session_id,
            snapshot,
            ..
        } => {
            let agents = agents.ok_or(LocalError::Unsupported)?;
            let selected = agents
                .lookup(
                    crate::domain::AgentCatalogKind::Sessions,
                    crate::domain::AgentCatalogKey::derive("session", session_id.as_bytes())
                        .map_err(|_| LocalError::Invalid)?,
                )
                .await?;
            if selected != Some(*snapshot) {
                return Err(LocalError::Stale);
            }
            let AgentObjectV1::SessionSnapshot(session) = agents.read_object(*snapshot).await?
            else {
                return Err(LocalError::NotFound);
            };
            if session.session_id != *session_id {
                return Err(LocalError::Corrupt);
            }
        }
        ThreadLinkTargetV1::Review {
            review_id,
            snapshot,
        } => {
            let reviews = reviews.ok_or(LocalError::Unsupported)?;
            let selected = reviews
                .lookup(
                    crate::domain::ReviewCatalogKind::Threads,
                    crate::domain::ReviewCatalogKey::derive("thread", review_id.as_bytes())
                        .map_err(|_| LocalError::Invalid)?,
                )
                .await?;
            if selected != Some(*snapshot) {
                return Err(LocalError::Stale);
            }
            let id = match reviews.read_object(*snapshot).await? {
                ReviewObjectV1::ThreadSnapshot(t) => t.thread_id,
                ReviewObjectV1::ThreadMerge(t) => t.thread_id,
                _ => return Err(LocalError::Corrupt),
            };
            if id != *review_id {
                return Err(LocalError::Corrupt);
            }
        }
        ThreadLinkTargetV1::Code {
            evidence_method,
            evidence,
            ..
        } => {
            if *evidence_method != crate::domain::EvidenceMethod::ManualAssertion {
                return Err(LocalError::Unsupported);
            }
            if let Some(reference) = evidence {
                agents
                    .ok_or(LocalError::Unsupported)?
                    .read_object(*reference)
                    .await?;
            }
        }
    }
    Ok(())
}
