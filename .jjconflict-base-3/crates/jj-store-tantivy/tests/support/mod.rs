use jj_store_adapters::Checkpoint;
use jj_store_adapters::*;
use jj_store_model as m;
use jj_store_object::{Key, ObjectStore, memory::MemoryStore};
use jj_store_tantivy::{
    Document, DocumentKey, Extractor, Faults, IMPLEMENTATION, Limits as IndexLimits, Search,
    TantivyProjection, Update,
};
use jj_store_wal::{self as wal, inventory::*, publication::*};
use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
};
pub const REPO: m::RepositoryId = m::RepositoryId::new([1; 16]);
pub const EPOCH: m::EpochId = m::EpochId::new([2; 16]);
pub fn op(n: u8) -> m::OperationId {
    m::OperationId::new([n; 64])
}
pub fn identity() -> Identity {
    Identity {
        name: "tantivy".into(),
        implementation: IMPLEMENTATION.into(),
        schema: jj_store_tantivy::SCHEMA_VERSION,
        config: m::RecordDigest::new([3; 32]),
    }
}
#[derive(Default)]
pub struct Access {
    pub allowed: AtomicBool,
    pub foreign: AtomicBool,
    pub changed_epoch: AtomicBool,
}
#[async_trait::async_trait]
impl Authorizer for Access {
    fn repository(&self) -> m::RepositoryId {
        if self.foreign.load(Ordering::SeqCst) {
            m::RepositoryId::new([9; 16])
        } else {
            REPO
        }
    }
    async fn authorize(&self) -> jj_store_adapters::Result<m::EpochId> {
        if !self.allowed.load(Ordering::SeqCst) {
            return Err(Error::Denied);
        }
        Ok(if self.changed_epoch.load(Ordering::SeqCst) {
            m::EpochId::new([9; 16])
        } else {
            EPOCH
        })
    }
}
struct Dependencies;
#[async_trait::async_trait]
impl DependencySource for Dependencies {
    fn scope(&self) -> SourceScope {
        SourceScope::new(REPO)
    }
    async fn load_verified(
        &self,
        key: m::ObjectKey,
        _: u64,
    ) -> wal::inventory::Result<VerifiedNode> {
        let m::ObjectKey::Operation(id) = key else {
            panic!("fixture operation")
        };
        let n = id.as_bytes()[0];
        Ok(VerifiedNode {
            key,
            representation: Representation::Virtual,
            dependencies: if n == 0 {
                BTreeSet::new()
            } else {
                [m::ObjectKey::Operation(op(n - 1))].into()
            },
        })
    }
}
struct Permit;
#[async_trait::async_trait]
impl PublicationGuard for Permit {
    async fn authorize(&self, _: &ManifestSnapshot, _: &Request) -> wal::Result<()> {
        Ok(())
    }
    async fn validate_transition(
        &self,
        _: &ManifestSnapshot,
        _: &Request,
        _: &wal::publication::Snapshot,
    ) -> wal::Result<()> {
        Ok(())
    }
}
pub struct Fixture {
    pub store: Arc<MemoryStore>,
    pub coordinator: Coordinator,
    pub access: Arc<Access>,
}
impl Fixture {
    pub async fn new() -> Self {
        let store = Arc::new(MemoryStore::default());
        let coordinator = Coordinator::new(
            store.clone(),
            REPO,
            Arc::new(Dependencies),
            Arc::new(Permit),
            Default::default(),
        )
        .unwrap();
        coordinator.initialize(EPOCH, vec![]).await.unwrap();
        Self {
            store,
            coordinator,
            access: Arc::new(Access {
                allowed: AtomicBool::new(true),
                ..Default::default()
            }),
        }
    }
    pub fn source(&self) -> Source {
        Source::new(self.store.clone(), self.access.clone())
    }
    pub async fn publish(&self, n: u8) {
        self.coordinator
            .publish(&Request {
                principal: m::PrincipalId::new([1; 16]),
                request_id: m::RequestId::new([n; 16]),
                publication_id: m::PublicationId::new([n; 16]),
                transition: Some(Transition::Native {
                    remove: [op(n - 1)].into(),
                    add: op(n),
                }),
                settings: None,
                result: vec![],
                kind: SemanticKind::OperationPublished,
            })
            .await
            .unwrap();
    }
    pub async fn remote(&self) -> Option<Checkpoint> {
        let d = self
            .store
            .get(
                &Key::new(format!("repos/{REPO}/adapters/v1/tantivy/cursor.json")).unwrap(),
                None,
            )
            .await
            .ok()?;
        serde_json::from_slice(&d.read_bounded(16384).await.unwrap()).unwrap()
    }
    pub fn runner(
        &self,
        root: &Path,
        calls: Arc<Mutex<Vec<u64>>>,
        limits: IndexLimits,
    ) -> (
        Runner<TantivyProjection<FixtureExtractor>>,
        Search,
        Arc<Faults>,
    ) {
        let (projection, search) =
            TantivyProjection::open(root.join("index"), FixtureExtractor { calls }, limits)
                .unwrap();
        let faults = projection.faults();
        (
            Runner::new(
                self.source(),
                identity(),
                projection,
                root,
                Default::default(),
            )
            .unwrap(),
            search,
            faults,
        )
    }
}
pub struct FixtureExtractor {
    pub calls: Arc<Mutex<Vec<u64>>>,
}
#[async_trait::async_trait]
impl Extractor for FixtureExtractor {
    async fn bootstrap(
        &mut self,
        snapshot: &jj_store_adapters::Snapshot,
        _: &Context,
    ) -> jj_store_adapters::Result<Update> {
        self.calls
            .lock()
            .unwrap()
            .push(snapshot.position.position.0);
        Ok(Update {
            upsert: (0..=snapshot.position.position.0)
                .map(|p| document(p as u8))
                .collect(),
            ..Default::default()
        })
    }
    async fn update(&mut self, batch: &Batch, _: &Context) -> jj_store_adapters::Result<Update> {
        self.calls
            .lock()
            .unwrap()
            .push(batch.snapshot.position.position.0);
        Ok(Update {
            upsert: (0..=batch.snapshot.position.position.0)
                .map(|p| document(p as u8))
                .collect(),
            ..Default::default()
        })
    }
}
pub fn document(n: u8) -> Document {
    Document {
        key: DocumentKey::Operation { id: op(n) },
        title: format!("generation {n}"),
        body: format!("private body {n}"),
        actor: Some("authenticated-actor".into()),
        author: "native user".into(),
        workspace: None,
        operation: Some(op(n)),
        commit: None,
        change: None,
        file: None,
        parents: vec![],
        predecessors: vec![],
        attributes: Default::default(),
        language: None,
        has_conflict: false,
        timestamp_millis: i64::from(n),
    }
}
pub fn active_dir(root: &Path) -> PathBuf {
    let path = root.join("index");
    path.join(std::fs::read_to_string(path.join("active")).unwrap())
}
