use super::{LocalError, LocalRepository, LocalResult, collab_graph, collab_types::*};
use jj_collab::*;
use jj_store_jj::local_metadata::{Domain, Preparation, PublishOutcome, Update};
use jj_store_model as m;
use jj_store_wal::publication::{
    AgentObjectV1, AgentSnapshotRead, CollabTargetReaderV1, ReviewObjectV1, ReviewSnapshotRead,
};

impl LocalRepository {
    pub async fn apply_collab_command(
        &self,
        request: &LocalCollabRequest,
    ) -> LocalResult<LocalCollabOutcome> {
        request.validate()?;
        let actor = self.ensure_actor()?;
        let proposed = Self::proposed_repository()?;
        let intent = request.intent_for(actor)?;
        let at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|_| LocalError::Unavailable)?
            .as_millis()
            .try_into()
            .map_err(|_| LocalError::Limit)?;
        for _ in 0..8 {
            let pin = self.metadata.read()?;
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
            let mut threads = self.thread_store_at(pin.as_ref()).await?;
            let mut reactions = self.reaction_store_at(pin.as_ref(), repository).await?;
            let agents = self.agent_store_at(pin.as_ref(), repository).await?;
            let reviews = self.review_store_at(pin.as_ref(), repository).await?;
            let (domain, bytes, retained, result) = match &request.command {
                LocalCollabCommand::Thread { intent: command } => {
                    if let ThreadCommandV1::CreateAgentDiscussion { target, .. } = command {
                        let reader =
                            CollabTargetReaderV1::new(repository, None, Some(&agents), None)?;
                        resolve_reaction_target(&reader, target).await?;
                    }
                    validate_link(command, &agents, &reviews).await?;
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
                    let work = self.work_state(pin.as_ref())?;
                    let reader = CollabTargetReaderV1::new(
                        repository,
                        Some((threads.state.root.clone(), &threads.graph)),
                        Some(&agents),
                        Some(&reviews),
                    )?
                    .with_work(repository, &work)?;
                    resolve_reaction_target(&reader, command.target()).await?;
                    for reference in threads
                        .graph
                        .closure(command.target().collab_dependencies())?
                    {
                        let object = threads.graph.get(reference)?.clone();
                        if reactions.graph.stage(object)? != reference {
                            return Err(LocalError::Corrupt);
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
    agents: &super::LocalAgentStore,
    reviews: &super::LocalReviewStore,
) -> LocalResult<()> {
    let ThreadCommandV1::Link { target, .. } = command else {
        return Ok(());
    };
    match target {
        ThreadLinkTargetV1::CaptureGap {
            source_state,
            event_batch,
            ..
        } => {
            agents
                .reads()
                .validate_capture_gap(*source_state, *event_batch)
                .await?;
        }

        ThreadLinkTargetV1::Session {
            session_id,
            snapshot,
            ..
        } => {
            let selected = agents
                .lookup(
                    jj_store_wal::publication::AgentCatalogKind::Sessions,
                    jj_store_wal::publication::AgentCatalogKey::derive(
                        "session",
                        session_id.as_bytes(),
                    )
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
            let selected = reviews
                .lookup(
                    jj_store_wal::publication::ReviewCatalogKind::Threads,
                    jj_store_wal::publication::ReviewCatalogKey::derive(
                        "thread",
                        review_id.as_bytes(),
                    )
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
            if *evidence_method != jj_store_wal::publication::EvidenceMethod::ManualAssertion {
                return Err(LocalError::Unsupported);
            }
            if let Some(reference) = evidence {
                agents.read_object(*reference).await?;
            }
        }
    }
    Ok(())
}
