//! Named per-domain factories for the Git companion publication provider.
use crate::{
    metadata::{Domain, Snapshot},
    repository::*,
    work_api,
};
use jj_store_core::*;
use jj_store_model::{RepositoryId, RequestId};
use std::sync::Arc;

#[derive(Clone)]
pub struct GitProvider {
    repository: LocalRepository,
    group: Arc<PublicationGroup>,
}
impl GitProvider {
    pub fn new(repository: LocalRepository) -> Self {
        Self {
            repository,
            group: PublicationGroup::new(),
        }
    }
    pub fn scope(&self) -> PublicationScope {
        PublicationScope {
            repository: self.repository.expected_repository,
            authority: format!(
                "git-companion:{}",
                self.repository.metadata_store().git_dir().display()
            ),
        }
    }
    fn bind(&self, name: &str, context: &StoreContext) -> Result<Arc<GitStore>> {
        let expected = self.scope();
        if expected.authority != context.scope.authority
            || (expected.repository.is_some() && expected.repository != context.scope.repository)
        {
            return Err(Error::Invalid("Git provider repository binding"));
        }
        let mut repository = self.repository.clone().with_host(context.host.clone());
        if let Some(id) = context.scope.repository {
            repository = repository.bound_to(id);
        }
        Ok(Arc::new(GitStore {
            repository,
            context: context.clone(),
            name: name.into(),
            group: self.group.clone(),
        }))
    }
    pub fn add_agent_store(&self, factories: &mut StoreFactories, name: &str) -> Result<()> {
        let provider = self.clone();
        let implementation = name.to_owned();
        factories.add_agent_store(name, move |ctx| Ok(provider.bind(&implementation, ctx)?))
    }
    pub fn add_work_store(&self, factories: &mut StoreFactories, name: &str) -> Result<()> {
        let provider = self.clone();
        let implementation = name.to_owned();
        factories.add_work_store(name, move |ctx| Ok(provider.bind(&implementation, ctx)?))
    }
    pub fn add_review_store(&self, factories: &mut StoreFactories, name: &str) -> Result<()> {
        let provider = self.clone();
        let implementation = name.to_owned();
        factories.add_review_store(name, move |ctx| Ok(provider.bind(&implementation, ctx)?))
    }
    pub fn add_thread_store(&self, factories: &mut StoreFactories, name: &str) -> Result<()> {
        let provider = self.clone();
        let implementation = name.to_owned();
        factories.add_thread_store(name, move |ctx| Ok(provider.bind(&implementation, ctx)?))
    }
    pub fn add_reaction_store(&self, factories: &mut StoreFactories, name: &str) -> Result<()> {
        let provider = self.clone();
        let implementation = name.to_owned();
        factories.add_reaction_store(name, move |ctx| Ok(provider.bind(&implementation, ctx)?))
    }
    pub fn register(&self, factories: &mut StoreFactories, name: &str) -> Result<()> {
        self.add_agent_store(factories, name)?;
        self.add_work_store(factories, name)?;
        self.add_review_store(factories, name)?;
        self.add_thread_store(factories, name)?;
        self.add_reaction_store(factories, name)
    }
}
struct GitStore {
    repository: LocalRepository,
    context: StoreContext,
    name: String,
    group: Arc<PublicationGroup>,
}
impl StoreBinding for GitStore {
    fn name(&self) -> &str {
        &self.name
    }
    fn scope(&self) -> &PublicationScope {
        &self.context.scope
    }
    fn publication_group(&self) -> Option<Arc<PublicationGroup>> {
        Some(self.group.clone())
    }
}
impl GitStore {
    async fn pin(&self, kind: StoreKind) -> Result<Option<Snapshot>> {
        self.context.admit(kind, Access::Read).await?;
        let pin = self
            .repository
            .metadata_store()
            .read()
            .map_err(LocalError::from)?;
        self.repository.check_identity(pin.as_ref())?;
        Ok(pin)
    }
    fn pinned<T>(&self, pin: Option<&Snapshot>, domain: Domain, data: T) -> Result<Pinned<T>> {
        let mut scope = self.scope().clone();
        scope.repository = pin.map(|p| p.repository());
        Ok(Pinned {
            scope,
            version: pin
                .and_then(|p| p.selection(domain))
                .map_or(0, |s| s.version),
            selection: serde_json::to_vec(&pin.map(|p| p.commit()))?,
            data,
        })
    }
    async fn agent(
        &self,
        request: &StoreRequest<AgentCommand>,
    ) -> Result<StoreOutcome<AgentResult>> {
        self.context.admit(StoreKind::Agent, Access::Write).await?;
        reject_native_identity(request)?;
        let command = match &request.command {
            AgentCommand::Prepared { .. } => {
                return Err(Error::Unsupported(
                    "native prepared agent delta in Git provider",
                ));
            }
            AgentCommand::Initialize => LocalAgentCommand::Initialize,
            AgentCommand::Capture {
                profile,
                batch,
                binding: None,
            } => LocalAgentCommand::Capture {
                profile: profile.clone(),
                batch: batch.clone(),
            },
            AgentCommand::Capture {
                profile,
                batch,
                binding: Some(binding),
            } => LocalAgentCommand::BoundCapture {
                profile: profile.clone(),
                batch: batch.clone(),
                binding: binding.clone(),
            },
            AgentCommand::Provenance {
                expected,
                object,
                evidence,
            } => LocalAgentCommand::Provenance {
                expected: *expected,
                object: object.clone(),
                evidence: evidence
                    .iter()
                    .map(|e| LocalCodeEvidenceV1 {
                        reference: e.reference.clone(),
                        bytes: e.bytes.clone(),
                    })
                    .collect(),
            },
        };
        let outcome = self
            .repository
            .apply_agent_command(&LocalAgentRequest {
                version: 1,
                request_id: request.request_id,
                command,
            })
            .await?;
        self.context.admit(StoreKind::Agent, Access::Write).await?;
        agent_outcome(outcome)
    }
    async fn collab(
        &self,
        request: RequestId,
        command: LocalCollabCommand,
        kind: StoreKind,
    ) -> Result<StoreOutcome<jj_collab::CollabCommandResultV1>> {
        self.context.admit(kind, Access::Write).await?;
        let outcome = self
            .repository
            .apply_collab_command(&LocalCollabRequest {
                version: 1,
                request_id: request,
                command,
            })
            .await?;
        self.context.admit(kind, Access::Write).await?;
        collab_outcome(outcome, kind)
    }
}
fn reject_native_identity<C>(request: &StoreRequest<C>) -> Result<()> {
    if request.publication_id.is_some() {
        return Err(Error::Invalid(
            "native publication identity supplied to Git provider",
        ));
    }
    Ok(())
}
#[async_trait::async_trait]
impl AgentStore for GitStore {
    async fn prepare_with_thread(
        &self,
        request: &StoreRequest<AgentCommand>,
        thread: &dyn ThreadStore,
    ) -> Result<StoreRequest<AgentCommand>> {
        require_atomic_participants(self, thread)?;
        Ok(request.clone())
    }

    async fn snapshot(&self) -> Result<Pinned<Arc<dyn engine::AgentSnapshotRead>>> {
        let pin = self.pin(StoreKind::Agent).await?;
        let store = self
            .repository
            .agent_store_at(
                pin.as_ref(),
                pin.as_ref()
                    .map_or(RepositoryId::new([0; 16]), |p| p.repository()),
            )
            .await?;
        self.context.admit(StoreKind::Agent, Access::Read).await?;
        self.pinned(
            pin.as_ref(),
            Domain::Agent,
            Arc::new(store) as Arc<dyn engine::AgentSnapshotRead>,
        )
    }
    async fn execute(
        &self,
        request: &StoreRequest<AgentCommand>,
    ) -> Result<StoreOutcome<AgentResult>> {
        if request.command.has_thread_binding() {
            return Err(Error::Unsupported(
                "use the selected ThreadStore atomic participant",
            ));
        }
        self.agent(request).await
    }
    async fn execute_with_thread(
        &self,
        request: &StoreRequest<AgentCommand>,
        thread: &dyn ThreadStore,
    ) -> Result<StoreOutcome<AgentResult>> {
        require_atomic_participants(self, thread)?;
        self.context.admit(StoreKind::Thread, Access::Write).await?;
        let result = self.agent(request).await?;
        self.context.admit(StoreKind::Thread, Access::Write).await?;
        Ok(result)
    }
    async fn status(&self, request: RequestId) -> Result<StoreOutcome<AgentResult>> {
        self.context.admit(StoreKind::Agent, Access::Write).await?;
        let value = self.repository.agent_command_status(request)?;
        self.context.admit(StoreKind::Agent, Access::Write).await?;
        agent_outcome(value)
    }
}
#[async_trait::async_trait]
impl WorkStore for GitStore {
    async fn snapshot(&self) -> Result<Pinned<Arc<jj_work::WorkStateV1>>> {
        let pin = self.pin(StoreKind::Work).await?;
        let state = self.repository.work_state(pin.as_ref())?;
        self.context.admit(StoreKind::Work, Access::Read).await?;
        self.pinned(pin.as_ref(), Domain::Work, Arc::new(state))
    }
    async fn execute(
        &self,
        request: &StoreRequest<jj_work::WorkCommandV1>,
    ) -> Result<StoreOutcome<jj_work::WorkCommandResultV1>> {
        self.context.admit(StoreKind::Work, Access::Write).await?;
        reject_native_identity(request)?;
        let value = self.repository.apply_work_command(&work_api::Request {
            version: 1,
            request_id: request.request_id,
            command: request.command.clone(),
        })?;
        self.context.admit(StoreKind::Work, Access::Write).await?;
        work_outcome(value)
    }
    async fn status(
        &self,
        request: RequestId,
    ) -> Result<StoreOutcome<jj_work::WorkCommandResultV1>> {
        self.context.admit(StoreKind::Work, Access::Write).await?;
        let value = self.repository.work_command_status(request)?;
        self.context.admit(StoreKind::Work, Access::Write).await?;
        work_outcome(value)
    }
}
#[async_trait::async_trait]
impl ReviewStore for GitStore {
    async fn snapshot(&self) -> Result<Pinned<Arc<dyn engine::ReviewSnapshotRead>>> {
        let pin = self.pin(StoreKind::Review).await?;
        let store = self
            .repository
            .review_store_at(
                pin.as_ref(),
                pin.as_ref()
                    .map_or(RepositoryId::new([0; 16]), |p| p.repository()),
            )
            .await?;
        self.context.admit(StoreKind::Review, Access::Read).await?;
        self.pinned(
            pin.as_ref(),
            Domain::Review,
            Arc::new(store) as Arc<dyn engine::ReviewSnapshotRead>,
        )
    }
    async fn execute(
        &self,
        request: &StoreRequest<ReviewCommand>,
    ) -> Result<StoreOutcome<Option<jj_review::ReviewCommandResultV1>>> {
        self.context.admit(StoreKind::Review, Access::Write).await?;
        reject_native_identity(request)?;
        let command = match &request.command {
            ReviewCommand::Initialize => LocalReviewCommand::Initialize,
            ReviewCommand::RepositoryThread { body } => {
                LocalReviewCommand::RepositoryThread { body: body.clone() }
            }
            ReviewCommand::Apply { command, evidence } if evidence.is_empty() => {
                LocalReviewCommand::Apply {
                    intent: command.clone(),
                }
            }
            ReviewCommand::Apply { command, evidence } => LocalReviewCommand::WithEvidence {
                intent: command.clone(),
                evidence: evidence
                    .iter()
                    .map(|e| LocalCodeEvidenceV1 {
                        reference: e.reference.clone(),
                        bytes: e.bytes.clone(),
                    })
                    .collect(),
            },
        };
        let value = self
            .repository
            .apply_review_command(&LocalReviewRequest {
                version: 1,
                request_id: request.request_id,
                command,
            })
            .await?;
        self.context.admit(StoreKind::Review, Access::Write).await?;
        review_outcome(value)
    }
    async fn status(
        &self,
        request: RequestId,
    ) -> Result<StoreOutcome<Option<jj_review::ReviewCommandResultV1>>> {
        self.context.admit(StoreKind::Review, Access::Write).await?;
        let value = self.repository.review_command_status(request)?;
        self.context.admit(StoreKind::Review, Access::Write).await?;
        review_outcome(value)
    }
}
#[async_trait::async_trait]
impl ThreadStore for GitStore {
    async fn snapshot(&self) -> Result<Pinned<ThreadData>> {
        let pin = self.pin(StoreKind::Thread).await?;
        let store = self.repository.thread_store_at(pin.as_ref()).await?;
        self.context.admit(StoreKind::Thread, Access::Read).await?;
        self.pinned(
            pin.as_ref(),
            Domain::Thread,
            ThreadData {
                root: store.state.root.clone(),
                graph: Arc::new(store.graph().clone()),
            },
        )
    }
    async fn execute(
        &self,
        request: &StoreRequest<jj_collab::ThreadCommandV1>,
    ) -> Result<StoreOutcome<jj_collab::CollabCommandResultV1>> {
        reject_native_identity(request)?;
        self.collab(
            request.request_id,
            LocalCollabCommand::Thread {
                intent: request.command.clone(),
            },
            StoreKind::Thread,
        )
        .await
    }
    async fn status(
        &self,
        request: RequestId,
    ) -> Result<StoreOutcome<jj_collab::CollabCommandResultV1>> {
        self.context.admit(StoreKind::Thread, Access::Write).await?;
        let value = self.repository.collab_command_status(request)?;
        self.context.admit(StoreKind::Thread, Access::Write).await?;
        collab_outcome(value, StoreKind::Thread)
    }
}
#[async_trait::async_trait]
impl ReactionStore for GitStore {
    async fn snapshot(&self) -> Result<Pinned<ReactionData>> {
        let pin = self.pin(StoreKind::Reaction).await?;
        let store = self
            .repository
            .reaction_store_at(
                pin.as_ref(),
                pin.as_ref()
                    .map_or(RepositoryId::new([0; 16]), |p| p.repository()),
            )
            .await?;
        self.context
            .admit(StoreKind::Reaction, Access::Read)
            .await?;
        self.pinned(
            pin.as_ref(),
            Domain::Reaction,
            ReactionData {
                root: store.state().root.clone(),
                graph: Arc::new(store.graph().clone()),
            },
        )
    }
    async fn execute(
        &self,
        request: &StoreRequest<ReactionCommand>,
    ) -> Result<StoreOutcome<jj_collab::CollabCommandResultV1>> {
        reject_native_identity(request)?;
        let command = match &request.command {
            ReactionCommand::Reaction(intent) => LocalCollabCommand::Reaction {
                intent: intent.clone(),
            },
            ReactionCommand::Emoji(intent) => LocalCollabCommand::Emoji {
                intent: intent.clone(),
            },
        };
        self.collab(request.request_id, command, StoreKind::Reaction)
            .await
    }
    async fn status(
        &self,
        request: RequestId,
    ) -> Result<StoreOutcome<jj_collab::CollabCommandResultV1>> {
        self.context
            .admit(StoreKind::Reaction, Access::Write)
            .await?;
        let value = self.repository.collab_command_status(request)?;
        self.context
            .admit(StoreKind::Reaction, Access::Write)
            .await?;
        collab_outcome(value, StoreKind::Reaction)
    }
}
fn agent_outcome(value: LocalAgentOutcome) -> Result<StoreOutcome<AgentResult>> {
    Ok(match value {
        LocalAgentOutcome::OutcomeUnknown { request_id } => StoreOutcome::Unknown { request_id },
        LocalAgentOutcome::NotKnown { request_id } => StoreOutcome::NotKnown { request_id },
        LocalAgentOutcome::Committed { receipt: r } => StoreOutcome::Committed(StoreReceipt {
            request_id: r.request_id,
            actor: r.actor,
            repository: r.repository,
            version: r.agent_state_version,
            selection: r.agent_root.as_bytes().to_vec(),
            provider_receipt: serde_json::to_vec(&r)?,
            result: AgentResult {
                root: r.agent_root,
                batch: r.batch,
            },
        }),
    })
}
fn work_outcome(value: LocalWorkOutcome) -> Result<StoreOutcome<jj_work::WorkCommandResultV1>> {
    Ok(match value {
        LocalWorkOutcome::OutcomeUnknown { request_id } => StoreOutcome::Unknown { request_id },
        LocalWorkOutcome::NotKnown { request_id } => StoreOutcome::NotKnown { request_id },
        LocalWorkOutcome::Committed { receipt: r } => StoreOutcome::Committed(StoreReceipt {
            request_id: r.request_id,
            actor: r.actor,
            repository: r.repository,
            version: r.work_state_version,
            selection: r.work_object.to_string().into_bytes(),
            provider_receipt: serde_json::to_vec(&r)?,
            result: r.result,
        }),
    })
}
fn review_outcome(
    value: LocalReviewOutcome,
) -> Result<StoreOutcome<Option<jj_review::ReviewCommandResultV1>>> {
    Ok(match value {
        LocalReviewOutcome::OutcomeUnknown { request_id } => StoreOutcome::Unknown { request_id },
        LocalReviewOutcome::NotKnown { request_id } => StoreOutcome::NotKnown { request_id },
        LocalReviewOutcome::Committed { receipt: r } => StoreOutcome::Committed(StoreReceipt {
            request_id: r.request_id,
            actor: r.actor,
            repository: r.repository,
            version: r.review_state_version,
            selection: r.review_root.as_bytes().to_vec(),
            provider_receipt: serde_json::to_vec(&r)?,
            result: r.command.clone(),
        }),
    })
}
fn collab_outcome(
    value: LocalCollabOutcome,
    kind: StoreKind,
) -> Result<StoreOutcome<jj_collab::CollabCommandResultV1>> {
    Ok(match value {
        LocalCollabOutcome::OutcomeUnknown { request_id } => StoreOutcome::Unknown { request_id },
        LocalCollabOutcome::NotKnown { request_id } => StoreOutcome::NotKnown { request_id },
        LocalCollabOutcome::Committed { receipt: r } => {
            if matches!(r.result, jj_collab::CollabCommandResultV1::Thread { .. })
                != (kind == StoreKind::Thread)
            {
                return Err(Error::IntentConflict);
            }
            StoreOutcome::Committed(StoreReceipt {
                request_id: r.request_id,
                actor: r.actor,
                repository: r.repository,
                version: r.domain_state_version,
                selection: r.domain_object.to_string().into_bytes(),
                provider_receipt: serde_json::to_vec(&r)?,
                result: r.result.clone(),
            })
        }
    })
}
