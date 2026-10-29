//! Memory-backed provider example. Real RustFS qualification is recorded separately.
use super::*;
use jj_store_native::NativeProvider;
use jj_store_object::memory::MemoryStore;
use jj_store_wal::{
    self as wal,
    inventory::{DependencySource, Representation, SourceScope, VerifiedNode},
    publication as p,
};
const REPO: m::RepositoryId = m::RepositoryId::new([61; 16]);
struct Source;
#[async_trait::async_trait]
impl DependencySource for Source {
    fn scope(&self) -> SourceScope {
        SourceScope::new(REPO)
    }
    async fn load_verified(
        &self,
        key: m::ObjectKey,
        _: u64,
    ) -> wal::inventory::Result<VerifiedNode> {
        Ok(VerifiedNode {
            key,
            representation: Representation::Virtual,
            dependencies: Default::default(),
        })
    }
}
struct Guard;
#[async_trait::async_trait]
impl p::PublicationGuard for Guard {
    async fn authorize(&self, _: &p::ManifestSnapshot, _: &p::Request) -> wal::Result<()> {
        Ok(())
    }
    async fn authorize_domain(
        &self,
        _: &p::ManifestSnapshot,
        _: &p::DomainRequest,
    ) -> wal::Result<()> {
        Ok(())
    }
    async fn validate_transition(
        &self,
        _: &p::ManifestSnapshot,
        _: &p::Request,
        _: &p::Snapshot,
    ) -> wal::Result<()> {
        Ok(())
    }
}
fn coordinator(store: Arc<MemoryStore>) -> p::Coordinator {
    p::Coordinator::new(
        store,
        REPO,
        Arc::new(Source),
        Arc::new(Guard),
        p::Limits::default(),
    )
    .unwrap()
}
fn open(coordinator: p::Coordinator, host: Arc<Host>) -> RepositoryStores {
    let provider = NativeProvider::new(coordinator, "independent-example").unwrap();
    let mut factories = StoreFactories::empty();
    provider.register(&mut factories, "native").unwrap();
    factories
        .loader()
        .open(
            &StoreConfig::all("native"),
            StoreContext::new(provider.scope(), host),
        )
        .unwrap()
}
pub async fn run() {
    let storage = Arc::new(MemoryStore::default());
    let c = coordinator(storage.clone());
    c.initialize(m::EpochId::new([62; 16]), vec![])
        .await
        .unwrap();
    let host = Arc::new(Host::default());
    let stores = open(c.clone(), host.clone());
    let before = c.read().await.unwrap();
    assert_eq!(stores.agent().unwrap().snapshot().await.unwrap().version, 0);
    assert_eq!(
        stores.review().unwrap().snapshot().await.unwrap().version,
        0
    );
    assert_eq!(stores.work().unwrap().snapshot().await.unwrap().version, 0);
    assert_eq!(
        stores.thread().unwrap().snapshot().await.unwrap().version,
        0
    );
    assert_eq!(
        stores.reaction().unwrap().snapshot().await.unwrap().version,
        0
    );
    assert_eq!(host.writes.load(Ordering::SeqCst), 0);
    assert_eq!(c.read().await.unwrap().manifest(), before.manifest());
    let work = request(
        63,
        jj_work::WorkCommandV1::CreateProject {
            id: jj_work::ProjectId::new([63; 16]),
            title: "Native library home".into(),
            body: String::new(),
        },
    )
    .with_publication_id(m::PublicationId::new([163; 16]));
    let w = committed(stores.work().unwrap().execute(&work).await.unwrap());
    assert_eq!(
        w.result.project.as_ref().unwrap().actor.created_at_millis,
        12345
    );
    let t = committed(stores.thread().unwrap().execute(&thread(64)).await.unwrap());
    let jj_collab::CollabCommandResultV1::Thread { result: tdata } = &t.result else {
        panic!("thread")
    };
    let raw = request(
        65,
        capture(Some(jj_collab::ThreadBindingV1 {
            thread_id: tdata.thread_id,
            observed_thread: tdata.version,
            observed_assignment: None,
        })),
    )
    .with_publication_id(m::PublicationId::new([165; 16]));
    let prepared = stores.prepare_agent(&raw).await.unwrap();
    let original = jj_store_native::to_agent_delta(ACTOR, &prepared).unwrap();
    let retained: p::AgentDeltaRequest =
        serde_json::from_slice(&serde_json::to_vec(&original).unwrap()).unwrap();
    assert_eq!(retained, original);
    let prepared = jj_store_native::from_agent_delta(&retained).unwrap();
    let a = committed(stores.execute_agent(&prepared).await.unwrap());
    let r = committed(stores.review().unwrap().execute(&review(66)).await.unwrap());
    let x = committed(
        stores
            .reaction()
            .unwrap()
            .execute(&reaction(67, tdata.thread_id))
            .await
            .unwrap(),
    );
    let selected = c.read().await.unwrap();
    assert_eq!(selected.manifest().agent_state_version, 1);
    assert_eq!(selected.manifest().thread_state_version, 2);
    c.checkpoint().await.unwrap();
    c.recover(100).await.unwrap();
    let reopened = open(coordinator(storage), host);
    assert_eq!(
        committed(reopened.work().unwrap().status(w.request_id).await.unwrap()).provider_receipt,
        w.provider_receipt
    );
    assert_eq!(
        committed(
            reopened
                .thread()
                .unwrap()
                .status(t.request_id)
                .await
                .unwrap()
        )
        .provider_receipt,
        t.provider_receipt
    );
    assert_eq!(
        committed(reopened.execute_agent(&prepared).await.unwrap()).provider_receipt,
        a.provider_receipt
    );
    assert_eq!(
        committed(
            reopened
                .review()
                .unwrap()
                .status(r.request_id)
                .await
                .unwrap()
        )
        .provider_receipt,
        r.provider_receipt
    );
    assert_eq!(
        committed(
            reopened
                .reaction()
                .unwrap()
                .status(x.request_id)
                .await
                .unwrap()
        )
        .provider_receipt,
        x.provider_receipt
    );
    assert_eq!(
        committed(reopened.work().unwrap().execute(&work).await.unwrap()).selection,
        w.selection
    );
    assert_eq!(
        c.read().await.unwrap().manifest().published,
        selected.manifest().published
    );
}
