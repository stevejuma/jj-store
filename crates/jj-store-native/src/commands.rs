use super::*;
impl NativeStore {
    async fn collab(
        &self,
        request: &StoreRequest<jj_collab::CollabCommandV1>,
        kind: StoreKind,
        qualified: Option<&dyn jj_collab::QualifiedReviewRead>,
    ) -> c::Result<StoreOutcome<jj_collab::CollabCommandResultV1>> {
        self.context.admit(kind, Access::Write).await?;
        let native = p::CollabCommandRequestV1 {
            schema_version: 1,
            principal: self.actor(true)?,
            request_id: request.request_id,
            publication_id: self.publication(request),
            command: request.command.clone(),
        };
        native.validate().map_err(collab_error)?;
        let current = self.coordinator.read().await?;
        if let Some(receipt) = self
            .coordinator
            .receipt_at(&current, native.principal, native.request_id)
            .await?
        {
            if receipt.intent
                != native
                    .intent_digest(self.coordinator.repository_id())
                    .map_err(collab_error)?
                || receipt.publication_id != native.publication_id
            {
                return Err(c::Error::IntentConflict);
            }
            return map_outcome(self.committed(kind, receipt).await?, collab_result);
        }
        let sources = match &native.command {
            jj_collab::CollabCommandV1::Thread { intent } => thread_source_stores(intent),
            jj_collab::CollabCommandV1::Reaction { intent } => {
                reaction_source_store(intent.target()).into_iter().collect()
            }
            jj_collab::CollabCommandV1::Emoji { .. } => Default::default(),
        };
        self.context
            .admit_sources(&self.group, sources.clone(), Access::Read)
            .await?;
        let value = self
            .coordinator
            .publish_collab_command_at(&native, self.context.host.now_millis()?, qualified)
            .await
            .map_err(collab_error)?;
        self.context
            .admit_sources(&self.group, sources, Access::Read)
            .await?;
        map_outcome(self.outcome(kind, value).await?, collab_result)
    }
}
fn collab_result(value: &StoreReceipt<Vec<u8>>) -> c::Result<jj_collab::CollabCommandResultV1> {
    p::CollabCommandReceiptV1::decode(&value.result)
        .map(|v| v.result)
        .map_err(collab_error)
}
#[async_trait::async_trait]
impl ThreadStore for NativeStore {
    async fn snapshot(&self) -> c::Result<Pinned<ThreadData>> {
        let pin = self.pin(StoreKind::Thread).await?;
        let data = match pin.manifest().thread_root {
            Some(root) => {
                let value = self.coordinator.native_thread_store(root).await?;
                ThreadData {
                    root: value.state.root,
                    graph: Arc::new(value.graph),
                }
            }
            None => ThreadData {
                root: jj_collab::ThreadRootV1::empty(),
                graph: Arc::new(jj_collab::CollabGraphV1::default()),
            },
        };
        self.pinned(StoreKind::Thread, &pin, data).await
    }
    async fn execute(
        &self,
        request: &StoreRequest<jj_collab::ThreadCommandV1>,
    ) -> c::Result<StoreOutcome<jj_collab::CollabCommandResultV1>> {
        ThreadStore::execute_with_reviews(self, request, None).await
    }
    async fn execute_with_reviews(
        &self,
        request: &StoreRequest<jj_collab::ThreadCommandV1>,
        qualified: Option<&dyn jj_collab::QualifiedReviewRead>,
    ) -> c::Result<StoreOutcome<jj_collab::CollabCommandResultV1>> {
        let value = StoreRequest {
            publication_id: request.publication_id,
            request_id: request.request_id,
            command: jj_collab::CollabCommandV1::Thread {
                intent: request.command.clone(),
            },
        };
        self.collab(&value, StoreKind::Thread, qualified).await
    }
    async fn status(
        &self,
        request: RequestId,
    ) -> c::Result<StoreOutcome<jj_collab::CollabCommandResultV1>> {
        map_outcome(
            self.status_raw(StoreKind::Thread, request).await?,
            collab_result,
        )
    }
}
#[async_trait::async_trait]
impl ReactionStore for NativeStore {
    async fn snapshot(&self) -> c::Result<Pinned<ReactionData>> {
        let pin = self.pin(StoreKind::Reaction).await?;
        let data = match pin.manifest().reaction_root {
            Some(root) => {
                let value = self.coordinator.native_reaction_store(root).await?;
                ReactionData {
                    root: value.root,
                    graph: Arc::new(value.graph),
                }
            }
            None => ReactionData {
                root: jj_collab::ReactionRootV1::empty(),
                graph: Arc::new(jj_collab::CollabGraphV1::default()),
            },
        };
        self.pinned(StoreKind::Reaction, &pin, data).await
    }
    async fn execute(
        &self,
        request: &StoreRequest<ReactionCommand>,
    ) -> c::Result<StoreOutcome<jj_collab::CollabCommandResultV1>> {
        ReactionStore::execute_with_reviews(self, request, None).await
    }
    async fn execute_with_reviews(
        &self,
        request: &StoreRequest<ReactionCommand>,
        qualified: Option<&dyn jj_collab::QualifiedReviewRead>,
    ) -> c::Result<StoreOutcome<jj_collab::CollabCommandResultV1>> {
        let command = match &request.command {
            ReactionCommand::Reaction(intent) => jj_collab::CollabCommandV1::Reaction {
                intent: intent.clone(),
            },
            ReactionCommand::Emoji(intent) => jj_collab::CollabCommandV1::Emoji {
                intent: intent.clone(),
            },
        };
        let value = StoreRequest {
            publication_id: request.publication_id,
            request_id: request.request_id,
            command,
        };
        self.collab(&value, StoreKind::Reaction, qualified).await
    }
    async fn status(
        &self,
        request: RequestId,
    ) -> c::Result<StoreOutcome<jj_collab::CollabCommandResultV1>> {
        map_outcome(
            self.status_raw(StoreKind::Reaction, request).await?,
            collab_result,
        )
    }
}
