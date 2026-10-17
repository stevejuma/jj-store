use super::*;
use crate::engine::{AgentSnapshotRead, ReviewSnapshotRead};

pub trait StoreBinding: Send + Sync {
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
    ($name:ident, $snapshot:ty, $command:ty, $result:ty) => {
        #[async_trait::async_trait]
        pub trait $name: StoreBinding {
            async fn snapshot(&self) -> Result<Pinned<$snapshot>>;
            async fn execute(
                &self,
                request: &StoreRequest<$command>,
            ) -> Result<StoreOutcome<$result>>;
            async fn status(&self, request: RequestId) -> Result<StoreOutcome<$result>>;
        }
    };
}
store!(
    AgentStore,
    Arc<dyn AgentSnapshotRead>,
    AgentCommand,
    AgentResult
);
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
    jj_collab::CollabCommandResultV1
);
store!(
    ReactionStore,
    ReactionData,
    ReactionCommand,
    jj_collab::CollabCommandResultV1
);
