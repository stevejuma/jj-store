use jj_store_core::*;
use jj_store_model::*;
use jj_store_native::NativeProvider;
use jj_store_object::{
    ByteRange, Condition, Download, Key, Metadata, ObjectStore, StagedUpload, WriteOutcome,
    memory::MemoryStore,
};
use jj_store_wal::{
    self as wal,
    inventory::{DependencySource, Representation, SourceScope, VerifiedNode},
    publication as p,
};
use std::{
    collections::BTreeSet,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU8, AtomicUsize, Ordering},
    },
};
pub const REPO: RepositoryId = RepositoryId::new([1; 16]);
pub const ACTOR: PrincipalId = PrincipalId::new([2; 16]);
struct Source;
#[async_trait::async_trait]
impl DependencySource for Source {
    fn scope(&self) -> SourceScope {
        SourceScope::new(REPO)
    }
    async fn load_verified(&self, key: ObjectKey, _: u64) -> wal::inventory::Result<VerifiedNode> {
        Ok(VerifiedNode {
            key,
            representation: Representation::Virtual,
            dependencies: BTreeSet::new(),
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
#[derive(Default)]
pub struct Storage {
    pub memory: MemoryStore,
    pub mode: AtomicU8,
    pub puts: AtomicUsize,
    pub read_blocked: AtomicBool,
    pub read_missing: AtomicBool,
}
#[async_trait::async_trait]
impl ObjectStore for Storage {
    async fn head(&self, key: &Key) -> jj_store_object::Result<Metadata> {
        self.memory.head(key).await
    }
    async fn get(&self, key: &Key, range: Option<ByteRange>) -> jj_store_object::Result<Download> {
        if self.read_blocked.load(Ordering::SeqCst) {
            return Err(if self.read_missing.load(Ordering::SeqCst) {
                jj_store_object::Error::NotFound
            } else {
                jj_store_object::Error::Transport
            });
        }
        self.memory.get(key, range).await
    }
    async fn put(
        &self,
        key: &Key,
        condition: Condition,
        upload: &StagedUpload,
    ) -> jj_store_object::Result<WriteOutcome> {
        self.puts.fetch_add(1, Ordering::SeqCst);
        let mode = if key.as_str().ends_with("/manifest.json") {
            self.mode.swap(0, Ordering::SeqCst)
        } else {
            0
        };
        if mode == 1 {
            return Ok(WriteOutcome::Unknown);
        }
        let result = self.memory.put(key, condition, upload).await?;
        if (mode == 2 || mode == 3 || mode == 4) && matches!(result, WriteOutcome::Applied(_)) {
            self.read_blocked.store(true, Ordering::SeqCst);
            self.read_missing.store(mode == 4, Ordering::SeqCst);
        }
        if mode == 2 && matches!(result, WriteOutcome::Applied(_)) {
            Ok(WriteOutcome::Unknown)
        } else {
            Ok(result)
        }
    }
}
#[derive(Default)]
pub struct Host {
    pub denied: AtomicBool,
    pub denied_source: AtomicBool,
}
#[async_trait::async_trait]
impl StoreHost for Host {
    async fn authorize(&self, kind: StoreKind, _: Access) -> Result<()> {
        if self.denied.load(Ordering::SeqCst)
            || (kind == StoreKind::Thread && self.denied_source.load(Ordering::SeqCst))
        {
            Err(Error::Denied)
        } else {
            Ok(())
        }
    }
    fn actor(&self, _: bool) -> Result<Option<PrincipalId>> {
        Ok(Some(ACTOR))
    }
    fn now_millis(&self) -> Result<i64> {
        Ok(12345)
    }
}
pub fn coordinator(store: Arc<Storage>) -> p::Coordinator {
    p::Coordinator::new(
        store,
        REPO,
        Arc::new(Source),
        Arc::new(Guard),
        p::Limits::default(),
    )
    .unwrap()
}
pub fn open(
    c: p::Coordinator,
    host: Arc<Host>,
    configure: impl FnOnce(&NativeProvider, &mut StoreFactories, &mut StoreConfig),
) -> RepositoryStores {
    let provider = NativeProvider::new(c, "fixture").unwrap();
    let mut factories = StoreFactories::empty();
    provider.register(&mut factories, "native").unwrap();
    let mut config = StoreConfig::all("native");
    configure(&provider, &mut factories, &mut config);
    factories
        .loader()
        .open(&config, StoreContext::new(provider.scope(), host))
        .unwrap()
}
pub async fn fixture() -> (Arc<Storage>, p::Coordinator, Arc<Host>, RepositoryStores) {
    let store = Arc::new(Storage::default());
    let c = coordinator(store.clone());
    c.initialize(EpochId::new([3; 16]), vec![]).await.unwrap();
    let host = Arc::new(Host::default());
    let stores = open(c.clone(), host.clone(), |_, _, _| {});
    (store, c, host, stores)
}
pub fn request<C>(id: u8, command: C) -> StoreRequest<C> {
    StoreRequest::new(RequestId::new([id; 16]), command)
        .with_publication_id(PublicationId::new([id.wrapping_add(100); 16]))
}
pub fn committed<T>(value: StoreOutcome<T>) -> StoreReceipt<T> {
    let StoreOutcome::Committed(value) = value else {
        panic!("expected commit")
    };
    value
}
pub fn project(id: u8) -> StoreRequest<jj_work::WorkCommandV1> {
    request(
        id,
        jj_work::WorkCommandV1::CreateProject {
            id: jj_work::ProjectId::new([id; 16]),
            title: format!("Project {id}"),
            body: String::new(),
        },
    )
}
pub fn thread(id: u8) -> StoreRequest<jj_collab::ThreadCommandV1> {
    request(
        id,
        jj_collab::ThreadCommandV1::Create {
            thread_id: jj_collab::ThreadId::new([id; 16]),
            title: format!("Thread {id}"),
            baseline: None,
        },
    )
}
pub fn batch() -> jj_agent::SanitizedObservationBatchV1 {
    use jj_agent::*;
    let values: Vec<serde_json::Value> =
        serde_json::from_slice(include_bytes!("../fixtures/codex-0.156.1-hooks.json")).unwrap();
    SanitizedObservationBatchV1 {
        schema_version: 1,
        batch_id: CaptureBatchId::new([70; 16]),
        integration: IntegrationId::Codex,
        source_instance_id: SourceInstanceId::new([71; 16]),
        sanitizer_version: "agent-redaction@0.1.0".into(),
        representation: SanitizedRepresentation::SanitizedNative,
        observations: values
            .into_iter()
            .enumerate()
            .map(|(i, v)| SanitizedObservationV1 {
                observation_id: ObservationId::new([i as u8 + 1; 16]),
                source_sequence: i as u64 + 1,
                occurred_at_millis: Some(1725000000000 + i as i64),
                source_kind: SourceKind::Hook,
                source_trust: SourceTrust::LocalUserHook,
                payload: SanitizedPayloadV1::new(
                    SanitizedMediaType::Json,
                    &serde_json::to_vec(&v).unwrap(),
                )
                .unwrap(),
            })
            .collect(),
    }
}
