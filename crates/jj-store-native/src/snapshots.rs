use super::*;
#[async_trait::async_trait]
impl WorkStore for NativeStore {
    async fn snapshot(&self) -> c::Result<Pinned<Arc<jj_work::WorkStateV1>>> {
        let pin = self.pin(StoreKind::Work).await?;
        let data = match pin.manifest().work_root {
            Some(root) => self.coordinator.work_store(root).await?.state().clone(),
            None => jj_work::WorkStateV1::default(),
        };
        self.pinned(StoreKind::Work, &pin, Arc::new(data)).await
    }
    async fn execute(
        &self,
        request: &StoreRequest<jj_work::WorkCommandV1>,
    ) -> c::Result<StoreOutcome<jj_work::WorkCommandResultV1>> {
        self.context.admit(StoreKind::Work, Access::Write).await?;
        let native = p::WorkCommandRequest::new(
            self.actor(true)?,
            request.request_id,
            self.publication(request),
            request.command.clone(),
        )
        .map_err(work_error)?;
        let current = self.coordinator.read().await?;
        if let Some(receipt) = self
            .coordinator
            .receipt_at(&current, native.principal, native.request_id)
            .await?
        {
            if receipt.intent
                != native
                    .intent_digest(self.coordinator.repository_id())
                    .map_err(work_error)?
                || receipt.publication_id != native.publication_id
            {
                return Err(c::Error::IntentConflict);
            }
            return map_outcome(self.committed(StoreKind::Work, receipt).await?, work_result);
        }
        let value = self
            .coordinator
            .publish_work_command_at(&native, self.context.host.now_millis()?)
            .await
            .map_err(work_error)?;
        map_outcome(self.outcome(StoreKind::Work, value).await?, work_result)
    }
    async fn status(
        &self,
        request: RequestId,
    ) -> c::Result<StoreOutcome<jj_work::WorkCommandResultV1>> {
        map_outcome(
            self.status_raw(StoreKind::Work, request).await?,
            work_result,
        )
    }
}
fn work_result(value: &StoreReceipt<Vec<u8>>) -> c::Result<jj_work::WorkCommandResultV1> {
    let result: jj_work::WorkCommandResultV1 = serde_json::from_slice(&value.result)?;
    if serde_json::to_vec(&result)? != value.result {
        return Err(c::Error::Corrupt("noncanonical work receipt"));
    }
    Ok(result)
}
fn work_error(error: p::WorkCommandError) -> c::Error {
    match error {
        p::WorkCommandError::Publication(e) => e.into(),
        p::WorkCommandError::ReplicationUnsupported => c::Error::ReplicationUnsupported,
        p::WorkCommandError::Model(e) => match e {
            jj_work::Error::Stale => c::Error::Stale,
            jj_work::Error::NotFound(_) => c::Error::NotFound,
            jj_work::Error::Limit => c::Error::Limit("work command"),
            jj_work::Error::Invalid(v) => c::Error::Invalid(v),
        },
    }
}
#[async_trait::async_trait]
impl ReviewStore for NativeStore {
    async fn snapshot(&self) -> c::Result<Pinned<Arc<dyn c::engine::ReviewSnapshotRead>>> {
        let pin = self.pin(StoreKind::Review).await?;
        let root = pin.manifest().review_root.ok_or(c::Error::Unsupported(
            "enable native collaboration explicitly",
        ))?;
        let data = Arc::new(self.coordinator.review_store(root).await?)
            as Arc<dyn c::engine::ReviewSnapshotRead>;
        self.pinned(StoreKind::Review, &pin, data).await
    }
    async fn execute(
        &self,
        request: &StoreRequest<ReviewCommand>,
    ) -> c::Result<StoreOutcome<Option<jj_review::ReviewCommandResultV1>>> {
        self.context.admit(StoreKind::Review, Access::Write).await?;
        let command = match &request.command {
            ReviewCommand::Initialize => {
                return Err(c::Error::Unsupported(
                    "native collaboration enablement belongs to the host",
                ));
            }
            ReviewCommand::RepositoryThread { body } => jj_review::ReviewCommandV1::CreateThread {
                anchor: Box::new(jj_review::ArtifactAnchorV1 {
                    schema_version: 1,
                    repository_id: self.coordinator.repository_id(),
                    target: jj_review::ArtifactTargetV1::Repository,
                    created_context: jj_review::CreatedContextV1 {
                        publication_position: None,
                        code_state_version: None,
                        agent_root: None,
                    },
                    sensitivity: jj_review::SensitivityV1::Repository,
                }),
                body: body.clone(),
            },
            ReviewCommand::Apply { command, evidence } => {
                if !evidence.is_empty() {
                    return Err(c::Error::Unsupported("native portable capsule carrier"));
                }
                command.as_ref().clone()
            }
        };
        let native = p::ReviewCommandRequest::new(
            self.actor(true)?,
            request.request_id,
            self.publication(request),
            command,
        )
        .map_err(review_error)?;
        // Replay is independent of live source substitutions and mutable targets.
        let current = self.coordinator.read().await?;
        if let Some(receipt) = self
            .coordinator
            .receipt_at(&current, native.principal, native.request_id)
            .await?
        {
            if receipt.intent
                != native
                    .intent_digest(self.coordinator.repository_id())
                    .map_err(review_error)?
                || receipt.publication_id != native.publication_id
            {
                return Err(c::Error::IntentConflict);
            }
            return map_outcome(
                self.committed(StoreKind::Review, receipt).await?,
                review_result,
            );
        }
        let sources = review_source_stores(&native.command)?;
        self.context
            .admit_sources(&self.group, sources.clone(), Access::Read)
            .await?;
        let value = self
            .coordinator
            .publish_review_command_at(&native, self.context.host.now_millis()?)
            .await
            .map_err(review_error)?;
        self.context
            .admit_sources(&self.group, sources, Access::Read)
            .await?;
        map_outcome(self.outcome(StoreKind::Review, value).await?, review_result)
    }
    async fn status(
        &self,
        request: RequestId,
    ) -> c::Result<StoreOutcome<Option<jj_review::ReviewCommandResultV1>>> {
        map_outcome(
            self.status_raw(StoreKind::Review, request).await?,
            review_result,
        )
    }
}
fn review_result(
    value: &StoreReceipt<Vec<u8>>,
) -> c::Result<Option<jj_review::ReviewCommandResultV1>> {
    jj_review::decode_command_result(&value.result)
        .map(Some)
        .map_err(|_| c::Error::Corrupt("review receipt"))
}
fn review_error(error: c::engine::ReviewCommandError) -> c::Error {
    use c::engine::ReviewCommandError as E;
    match error {
        E::Publication(e) => e,
        E::Stale => c::Error::Stale,
        E::ThreadResolved => c::Error::ThreadResolved,
        E::ThreadOpen => c::Error::ThreadOpen,
        E::NotFound(_) => c::Error::NotFound,
        E::Invalid(v) => c::Error::Invalid(v),
    }
}
