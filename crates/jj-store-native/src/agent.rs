//! Native preparation is retained verbatim across unknown publication outcomes.
use super::*;

pub fn from_agent_delta(request: &p::AgentDeltaRequest) -> c::Result<StoreRequest<AgentCommand>> {
    request.validate()?;
    Ok(StoreRequest::new(
        request.request_id,
        AgentCommand::Prepared {
            delta: Box::new(PreparedAgentDelta {
                actor: request.principal,
                delta: request.delta.clone(),
                result: request.result.clone(),
                thread_binding: request.thread_binding.as_ref().map(|value| {
                    PreparedThreadBinding {
                        binding: value.binding.clone(),
                        accepted_at_millis: value.accepted_at_millis,
                    }
                }),
            }),
        },
    )
    .with_publication_id(request.publication_id))
}
pub fn to_agent_delta(
    actor: m::PrincipalId,
    request: &StoreRequest<AgentCommand>,
) -> c::Result<p::AgentDeltaRequest> {
    let AgentCommand::Prepared { delta } = &request.command else {
        return Err(c::Error::Unsupported(
            "retain native preparation before publishing",
        ));
    };
    if actor != delta.actor {
        return Err(c::Error::Denied);
    }
    let value = p::AgentDeltaRequest::new(
        actor,
        request.request_id,
        request
            .publication_id
            .ok_or(c::Error::Invalid("prepared native publication identity"))?,
        delta.delta.clone(),
        delta.result.clone(),
    )?;
    Ok(match &delta.thread_binding {
        Some(binding) => {
            value.with_thread_binding(binding.binding.clone(), binding.accepted_at_millis)?
        }
        None => value,
    })
}
impl NativeStore {
    async fn prepare_agent_inner(
        &self,
        request: &StoreRequest<AgentCommand>,
    ) -> c::Result<StoreRequest<AgentCommand>> {
        self.context.admit(StoreKind::Agent, Access::Write).await?;
        let actor = self.actor(true)?;
        if matches!(request.command, AgentCommand::Prepared { .. }) {
            to_agent_delta(actor, request)?;
            return Ok(request.clone());
        }
        let current = self.coordinator.read().await?;
        if self
            .coordinator
            .receipt_at(&current, actor, request.request_id)
            .await?
            .is_some()
        {
            return Err(c::Error::Unsupported(
                "request already committed; use its original status",
            ));
        }
        let root = current.manifest().agent_root.ok_or(c::Error::Unsupported(
            "enable native collaboration explicitly",
        ))?;
        let pinned = self.coordinator.agent_store(root).await?;
        let at = self.context.host.now_millis()?;
        if at < 0 {
            return Err(c::Error::Invalid("capture acceptance time"));
        }
        let delta = match &request.command {
            AgentCommand::Capture { profile, batch, .. } => {
                self.coordinator
                    .stage_agent_capture(&pinned, profile, batch, at)
                    .await?
            }
            AgentCommand::Provenance {
                expected,
                object,
                evidence,
            } => {
                if !evidence.is_empty() {
                    return Err(c::Error::Unsupported("native portable capsule carrier"));
                }
                c::engine::AgentProjector::new(&self.coordinator)
                    .stage_agent_provenance(&pinned, *expected, object)
                    .await?
            }
            AgentCommand::Initialize => {
                return Err(c::Error::Unsupported(
                    "native collaboration enablement belongs to the host",
                ));
            }
            AgentCommand::Prepared { .. } => unreachable!(),
        };
        let mut native = p::AgentDeltaRequest::new(
            actor,
            request.request_id,
            self.publication(request),
            delta,
            vec![],
        )?;
        if let AgentCommand::Capture {
            binding: Some(binding),
            ..
        } = &request.command
        {
            native = native.with_thread_binding(binding.clone(), at)?;
        }
        self.context.admit(StoreKind::Agent, Access::Write).await?;
        if self.actor(false)? != actor {
            return Err(c::Error::Denied);
        }
        from_agent_delta(&native)
    }
    async fn execute_agent_inner(
        &self,
        request: &StoreRequest<AgentCommand>,
    ) -> c::Result<StoreOutcome<AgentResult>> {
        self.context.admit(StoreKind::Agent, Access::Write).await?;
        let request = to_agent_delta(self.actor(true)?, request)?;
        let outcome = self.coordinator.publish_agent_delta(&request).await?;
        map_outcome(self.outcome(StoreKind::Agent, outcome).await?, agent_result)
    }
}
fn agent_result(receipt: &StoreReceipt<Vec<u8>>) -> c::Result<AgentResult> {
    let selected: wal::records::RecordRef = serde_json::from_slice(&receipt.selection)?;
    Ok(AgentResult {
        root: selected.digest,
        batch: None,
    })
}
#[async_trait::async_trait]
impl AgentStore for NativeStore {
    async fn replay(
        &self,
        request: &StoreRequest<AgentCommand>,
    ) -> c::Result<Option<StoreOutcome<AgentResult>>> {
        self.context.admit(StoreKind::Agent, Access::Write).await?;
        let native = to_agent_delta(self.actor(false)?, request)?;
        let current = self.coordinator.read().await?;
        let Some(receipt) = self
            .coordinator
            .receipt_at(&current, native.principal, native.request_id)
            .await?
        else {
            return Ok(None);
        };
        if receipt.intent != native.intent_digest(self.coordinator.repository_id())?
            || receipt.publication_id != native.publication_id
        {
            return Err(c::Error::IntentConflict);
        }
        Ok(Some(map_outcome(
            self.committed(StoreKind::Agent, receipt).await?,
            agent_result,
        )?))
    }

    async fn snapshot(&self) -> c::Result<Pinned<Arc<dyn c::engine::AgentSnapshotRead>>> {
        let pin = self.pin(StoreKind::Agent).await?;
        let root = pin.manifest().agent_root.ok_or(c::Error::Unsupported(
            "enable native collaboration explicitly",
        ))?;
        let data = Arc::new(self.coordinator.agent_store(root).await?)
            as Arc<dyn c::engine::AgentSnapshotRead>;
        self.pinned(StoreKind::Agent, &pin, data).await
    }
    async fn prepare(
        &self,
        request: &StoreRequest<AgentCommand>,
    ) -> c::Result<StoreRequest<AgentCommand>> {
        if request.command.has_thread_binding() {
            return Err(c::Error::Unsupported("select the ThreadStore participant"));
        }
        self.prepare_agent_inner(request).await
    }
    async fn prepare_with_thread(
        &self,
        request: &StoreRequest<AgentCommand>,
        thread: &dyn ThreadStore,
    ) -> c::Result<StoreRequest<AgentCommand>> {
        require_atomic_participants(self, thread)?;
        self.context
            .admit_sources(&self.group, [StoreKind::Thread], Access::Write)
            .await?;
        self.prepare_agent_inner(request).await
    }
    async fn execute(
        &self,
        request: &StoreRequest<AgentCommand>,
    ) -> c::Result<StoreOutcome<AgentResult>> {
        if let Some(outcome) = self.replay(request).await? {
            return Ok(outcome);
        }
        if request.command.has_thread_binding() {
            return Err(c::Error::Unsupported("select the ThreadStore participant"));
        }
        self.execute_agent_inner(request).await
    }
    async fn execute_with_thread(
        &self,
        request: &StoreRequest<AgentCommand>,
        thread: &dyn ThreadStore,
    ) -> c::Result<StoreOutcome<AgentResult>> {
        require_atomic_participants(self, thread)?;
        self.context
            .admit_sources(&self.group, [StoreKind::Thread], Access::Write)
            .await?;
        let result = self.execute_agent_inner(request).await?;
        self.context
            .admit_sources(&self.group, [StoreKind::Thread], Access::Write)
            .await?;
        Ok(result)
    }
    async fn status(&self, request: RequestId) -> c::Result<StoreOutcome<AgentResult>> {
        map_outcome(
            self.status_raw(StoreKind::Agent, request).await?,
            agent_result,
        )
    }
}
