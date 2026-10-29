use super::*;
use crate::engine::{AgentSnapshotRead, ReviewSnapshotRead};

pub trait StoreBinding: Send + Sync {
    /// Sharing this capability guarantees compatible domain encodings, source
    /// selections and atomic publication. Wrappers may delegate their provider's
    /// capability; unrelated implementations must use their own group or None.
    fn publication_group(&self) -> Option<Arc<PublicationGroup>> {
        None
    }
    fn name(&self) -> &str;
    fn scope(&self) -> &PublicationScope;
}
#[derive(Clone)]
pub struct ThreadData {
    pub root: jj_collab::ThreadRootV1,
    pub graph: Arc<jj_collab::CollabGraphV1>,
}
#[derive(Clone)]
pub struct ReactionData {
    pub root: jj_collab::ReactionRootV1,
    pub graph: Arc<jj_collab::CollabGraphV1>,
}
macro_rules! store {
    ($name:ident, $snapshot:ty, $command:ty, $result:ty $(, $extra:item)*) => {
        #[async_trait::async_trait]
        pub trait $name: StoreBinding {
            async fn snapshot(&self) -> Result<Pinned<$snapshot>>;
            async fn execute(
                &self,
                request: &StoreRequest<$command>,
            ) -> Result<StoreOutcome<$result>>;
            async fn status(&self, request: RequestId) -> Result<StoreOutcome<$result>>;
            $($extra)*
        }
    };
}
#[async_trait::async_trait]
pub trait AgentStore: StoreBinding {
    /// Request-aware receipt replay with no preparation or source-state reads.
    async fn replay(
        &self,
        _request: &StoreRequest<AgentCommand>,
    ) -> Result<Option<StoreOutcome<AgentResult>>> {
        Ok(None)
    }
    /// Identity preparation is sufficient for providers whose command is already
    /// durable. Native providers return stable typed deltas for caller retention.
    async fn prepare(
        &self,
        request: &StoreRequest<AgentCommand>,
    ) -> Result<StoreRequest<AgentCommand>> {
        if request.command.has_thread_binding() {
            return Err(Error::Unsupported(
                "use selected ThreadStore for preparation",
            ));
        }
        Ok(request.clone())
    }
    async fn prepare_with_thread(
        &self,
        _request: &StoreRequest<AgentCommand>,
        _thread: &dyn ThreadStore,
    ) -> Result<StoreRequest<AgentCommand>> {
        Err(Error::Unsupported("atomic Agent/Thread preparation"))
    }

    async fn snapshot(&self) -> Result<Pinned<Arc<dyn AgentSnapshotRead>>>;
    async fn execute(
        &self,
        request: &StoreRequest<AgentCommand>,
    ) -> Result<StoreOutcome<AgentResult>>;
    async fn status(&self, request: RequestId) -> Result<StoreOutcome<AgentResult>>;
    /// Bound capture requires the selected Thread implementation's real transaction participant.
    async fn execute_with_thread(
        &self,
        _request: &StoreRequest<AgentCommand>,
        _thread: &dyn ThreadStore,
    ) -> Result<StoreOutcome<AgentResult>> {
        Err(Error::Unsupported("atomic Agent/Thread publication"))
    }
}
store!(
    WorkStore,
    Arc<jj_work::WorkStateV1>,
    jj_work::WorkCommandV1,
    jj_work::WorkCommandResultV1
);
store!(
    ReviewStore,
    Arc<dyn ReviewSnapshotRead>,
    ReviewCommand,
    Option<jj_review::ReviewCommandResultV1>
);
store!(
    ThreadStore,
    ThreadData,
    jj_collab::ThreadCommandV1,
    jj_collab::CollabCommandResultV1,
    async fn execute_with_reviews(
        &self,
        request: &StoreRequest<jj_collab::ThreadCommandV1>,
        qualified: Option<&dyn jj_collab::QualifiedReviewRead>,
    ) -> Result<StoreOutcome<jj_collab::CollabCommandResultV1>> {
        if qualified.is_some() {
            return Err(Error::Unsupported("qualified review source"));
        }
        self.execute(request).await
    }
);
store!(
    ReactionStore,
    ReactionData,
    ReactionCommand,
    jj_collab::CollabCommandResultV1,
    async fn execute_with_reviews(
        &self,
        request: &StoreRequest<ReactionCommand>,
        qualified: Option<&dyn jj_collab::QualifiedReviewRead>,
    ) -> Result<StoreOutcome<jj_collab::CollabCommandResultV1>> {
        if qualified.is_some() {
            return Err(Error::Unsupported("qualified review source"));
        }
        self.execute(request).await
    }
);
