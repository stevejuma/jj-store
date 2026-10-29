//! An embedding application can replace every store without implementing its siblings.
use jj_store_core::*;
use jj_store_model::{PrincipalId, RepositoryId, RequestId};
use std::{
    collections::BTreeSet,
    sync::{Arc, Mutex},
};
struct Host;
#[async_trait::async_trait]
impl StoreHost for Host {
    async fn authorize(&self, _: StoreKind, _: Access) -> Result<()> {
        Ok(())
    }
    fn actor(&self, _: bool) -> Result<Option<PrincipalId>> {
        Ok(None)
    }
    fn now_millis(&self) -> Result<i64> {
        Ok(0)
    }
}
struct Reader;
#[async_trait::async_trait]
impl engine::AgentSnapshotRead for Reader {
    fn repository(&self) -> RepositoryId {
        RepositoryId::new([1; 16])
    }
    fn root(&self) -> &jj_agent::AgentRootV1 {
        static ROOT: std::sync::OnceLock<jj_agent::AgentRootV1> = std::sync::OnceLock::new();
        ROOT.get_or_init(jj_agent::AgentRootV1::empty)
    }
    async fn lookup(
        &self,
        _: jj_agent::AgentCatalogKind,
        _: jj_agent::AgentCatalogKey,
    ) -> Result<Option<jj_agent::AgentObjectRef>> {
        Ok(None)
    }
    async fn read_object(&self, _: jj_agent::AgentObjectRef) -> Result<jj_agent::AgentObjectV1> {
        Err(Error::NotFound)
    }
}
#[async_trait::async_trait]
impl engine::ReviewSnapshotRead for Reader {
    fn repository(&self) -> RepositoryId {
        RepositoryId::new([1; 16])
    }
    fn root(&self) -> &jj_review::ReviewRootV1 {
        static ROOT: std::sync::OnceLock<jj_review::ReviewRootV1> = std::sync::OnceLock::new();
        ROOT.get_or_init(jj_review::ReviewRootV1::empty)
    }
    async fn lookup(
        &self,
        _: jj_review::ReviewCatalogKind,
        _: jj_review::ReviewCatalogKey,
    ) -> Result<Option<jj_review::ReviewObjectRef>> {
        Ok(None)
    }
    async fn read_object(
        &self,
        _: jj_review::ReviewObjectRef,
    ) -> Result<jj_review::ReviewObjectV1> {
        Err(Error::NotFound)
    }
}
struct External {
    name: String,
    context: StoreContext,
    calls: Arc<Mutex<BTreeSet<StoreKind>>>,
}
impl StoreBinding for External {
    fn name(&self) -> &str {
        &self.name
    }
    fn scope(&self) -> &PublicationScope {
        &self.context.scope
    }
}
impl External {
    async fn pin<T>(&self, kind: StoreKind, data: T) -> Result<Pinned<T>> {
        self.context.admit(kind, Access::Read).await?;
        self.calls.lock().unwrap().insert(kind);
        Ok(Pinned {
            scope: self.context.scope.clone(),
            version: 0,
            selection: Vec::new(),
            data,
        })
    }
}
macro_rules! external {
    ($trait:ident,$kind:ident,$data:ty,$value:expr,$command:ty,$result:ty) => {
        #[async_trait::async_trait]
        impl $trait for External {
            async fn snapshot(&self) -> Result<Pinned<$data>> {
                self.pin(StoreKind::$kind, $value).await
            }
            async fn execute(&self, _: &StoreRequest<$command>) -> Result<StoreOutcome<$result>> {
                Err(Error::Unsupported("read-only external example"))
            }
            async fn status(&self, request_id: RequestId) -> Result<StoreOutcome<$result>> {
                Ok(StoreOutcome::NotKnown { request_id })
            }
        }
    };
}
external!(
    AgentStore,
    Agent,
    Arc<dyn engine::AgentSnapshotRead>,
    Arc::new(Reader) as Arc<dyn engine::AgentSnapshotRead>,
    AgentCommand,
    AgentResult
);
external!(
    WorkStore,
    Work,
    Arc<jj_work::WorkStateV1>,
    Arc::new(jj_work::WorkStateV1::default()),
    jj_work::WorkCommandV1,
    jj_work::WorkCommandResultV1
);
external!(
    ReviewStore,
    Review,
    Arc<dyn engine::ReviewSnapshotRead>,
    Arc::new(Reader) as Arc<dyn engine::ReviewSnapshotRead>,
    ReviewCommand,
    Option<jj_review::ReviewCommandResultV1>
);
external!(
    ThreadStore,
    Thread,
    ThreadData,
    ThreadData {
        root: jj_collab::ThreadRootV1::empty(),
        graph: Arc::new(jj_collab::CollabGraphV1::default())
    },
    jj_collab::ThreadCommandV1,
    jj_collab::CollabCommandResultV1
);
external!(
    ReactionStore,
    Reaction,
    ReactionData,
    ReactionData {
        root: jj_collab::ReactionRootV1::empty(),
        graph: Arc::new(jj_collab::CollabGraphV1::default())
    },
    ReactionCommand,
    jj_collab::CollabCommandResultV1
);
fn register(
    f: &mut StoreFactories,
    name: &str,
    kind: StoreKind,
    calls: Arc<Mutex<BTreeSet<StoreKind>>>,
) {
    let implementation = name.to_owned();
    macro_rules! add {
        ($method:ident) => {
            f.$method(name, move |context| {
                Ok(Arc::new(External {
                    name: implementation.clone(),
                    context: context.clone(),
                    calls: calls.clone(),
                }))
            })
            .unwrap()
        };
    }
    match kind {
        StoreKind::Agent => add!(add_agent_store),
        StoreKind::Work => add!(add_work_store),
        StoreKind::Review => add!(add_review_store),
        StoreKind::Thread => add!(add_thread_store),
        StoreKind::Reaction => add!(add_reaction_store),
    }
}
#[tokio::test]
async fn each_store_is_independently_replaceable_by_an_external_implementation() {
    for selected in StoreKind::ALL {
        let mut factories = StoreFactories::empty();
        let calls = Arc::new(Mutex::new(BTreeSet::new()));
        for kind in StoreKind::ALL {
            register(
                &mut factories,
                "base",
                kind,
                Arc::new(Mutex::new(BTreeSet::new())),
            );
        }
        register(&mut factories, "external", selected, calls.clone());
        let mut config = StoreConfig::all("base");
        config.enable(selected, "external");
        let stores = factories
            .loader()
            .open(
                &config,
                StoreContext::new(
                    PublicationScope {
                        repository: Some(RepositoryId::new([1; 16])),
                        authority: "external-host".into(),
                    },
                    Arc::new(Host),
                ),
            )
            .unwrap();
        assert!(calls.lock().unwrap().is_empty());
        stores.agent().unwrap().snapshot().await.unwrap();
        stores.work().unwrap().snapshot().await.unwrap();
        stores.review().unwrap().snapshot().await.unwrap();
        stores.thread().unwrap().snapshot().await.unwrap();
        stores.reaction().unwrap().snapshot().await.unwrap();
        assert_eq!(*calls.lock().unwrap(), BTreeSet::from([selected]));
        assert!(matches!(
            require_atomic_participants(
                stores.agent().unwrap().as_ref(),
                stores.thread().unwrap().as_ref()
            ),
            Err(Error::Unsupported(_))
        ));
    }
}
