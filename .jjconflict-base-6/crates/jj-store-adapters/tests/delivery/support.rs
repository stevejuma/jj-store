use jj_store_adapters::*;
use jj_store_model as m;
use jj_store_object::{
    self as object, ByteRange, Condition, Download, Key, Metadata, ObjectStore, StagedUpload,
    WriteOutcome,
};
use jj_store_wal::{
    self as wal,
    inventory::*,
    publication::{
        Coordinator, ManifestSnapshot, PublicationGuard, PublishOutcome, Request, SemanticKind,
        Transition,
    },
};
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::PathBuf,
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
        name: "test".into(),
        implementation: "1.0".into(),
        schema: 1,
        config: m::RecordDigest::new([3; 32]),
    }
}
pub struct Access {
    pub allowed: AtomicBool,
}
#[async_trait::async_trait]
impl Authorizer for Access {
    fn repository(&self) -> m::RepositoryId {
        REPO
    }
    async fn authorize(&self) -> jj_store_adapters::Result<m::EpochId> {
        if self.allowed.load(Ordering::SeqCst) {
            Ok(EPOCH)
        } else {
            Err(Error::Denied)
        }
    }
}
#[derive(Clone, Copy)]
pub enum Fault {
    Before,
    After,
    Conflict,
}
#[derive(Default)]
pub struct Store {
    pub inner: object::memory::MemoryStore,
    pub fault: Mutex<Option<(&'static str, Fault)>>,
    pub writes: Mutex<Vec<String>>,
}
#[async_trait::async_trait]
impl ObjectStore for Store {
    async fn head(&self, key: &Key) -> object::Result<Metadata> {
        self.inner.head(key).await
    }
    async fn get(&self, key: &Key, range: Option<ByteRange>) -> object::Result<Download> {
        self.inner.get(key, range).await
    }
    async fn put(
        &self,
        key: &Key,
        condition: Condition,
        data: &StagedUpload,
    ) -> object::Result<WriteOutcome> {
        self.writes.lock().unwrap().push(key.as_str().into());
        let fault = {
            let mut f = self.fault.lock().unwrap();
            if f.as_ref().is_some_and(|(s, _)| key.as_str().contains(s)) {
                f.take().map(|(_, f)| f)
            } else {
                None
            }
        };
        if matches!(fault, Some(Fault::Before)) {
            return Ok(WriteOutcome::Unknown);
        }
        if matches!(fault, Some(Fault::Conflict)) {
            return Ok(WriteOutcome::PreconditionFailed);
        }
        let outcome = self.inner.put(key, condition, data).await?;
        if matches!(fault, Some(Fault::After)) {
            Ok(WriteOutcome::Unknown)
        } else {
            Ok(outcome)
        }
    }
}
struct Dependencies {
    scope: SourceScope,
    nodes: BTreeMap<m::ObjectKey, VerifiedNode>,
}
#[async_trait::async_trait]
impl DependencySource for Dependencies {
    fn scope(&self) -> SourceScope {
        self.scope
    }
    async fn load_verified(
        &self,
        key: m::ObjectKey,
        _: u64,
    ) -> wal::inventory::Result<VerifiedNode> {
        Ok(self.nodes[&key].clone())
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
    pub store: Arc<Store>,
    pub coordinator: Coordinator,
    pub access: Arc<Access>,
}
impl Fixture {
    pub async fn new() -> Self {
        let store = Arc::new(Store::default());
        let mut nodes = BTreeMap::new();
        for n in 0..=16 {
            let key = m::ObjectKey::Operation(op(n));
            let representation = if n == 0 {
                Representation::Virtual
            } else {
                let bytes =
                    m::envelope::encode(key, format!("operation {n}").as_bytes(), 1024).unwrap();
                let header =
                    m::envelope::Header::decode(&bytes[..m::envelope::HEADER_LEN], key, 1024)
                        .unwrap();
                store
                    .put(
                        &Key::new(key.storage_path(REPO)).unwrap(),
                        Condition::Absent,
                        &StagedUpload::from_bytes(&bytes, 2048).await.unwrap(),
                    )
                    .await
                    .unwrap();
                Representation::Stored {
                    envelope_digest: header.digest,
                    bytes: bytes.len() as u64,
                }
            };
            nodes.insert(
                key,
                VerifiedNode {
                    key,
                    representation,
                    dependencies: if n == 0 {
                        BTreeSet::new()
                    } else {
                        BTreeSet::from([m::ObjectKey::Operation(op(0))])
                    },
                },
            );
        }
        let source = Arc::new(Dependencies {
            scope: SourceScope::new(REPO),
            nodes,
        });
        let coordinator = Coordinator::new(
            store.clone(),
            REPO,
            source,
            Arc::new(Permit),
            wal::publication::Limits::default(),
        )
        .unwrap();
        coordinator
            .initialize(EPOCH, b"fixture".to_vec())
            .await
            .unwrap();
        Self {
            store,
            coordinator,
            access: Arc::new(Access {
                allowed: AtomicBool::new(true),
            }),
        }
    }
    pub fn source(&self) -> Source {
        Source::new(self.store.clone(), self.access.clone())
    }
    pub async fn publish(&self, n: u8) -> PublishOutcome {
        self.coordinator
            .publish(&Request {
                principal: m::PrincipalId::new([3; 16]),
                request_id: m::RequestId::new([n; 16]),
                publication_id: m::PublicationId::new([n; 16]),
                transition: Some(Transition::Native {
                    remove: BTreeSet::from([op(n - 1)]),
                    add: op(n),
                }),
                settings: None,
                result: vec![n],
                kind: SemanticKind::OperationPublished,
            })
            .await
            .unwrap()
    }
    pub fn arm(&self, path: &'static str, fault: Fault) {
        *self.store.fault.lock().unwrap() = Some((path, fault));
    }
    pub async fn remote(&self) -> Option<Checkpoint> {
        match self
            .store
            .get(
                &Key::new(format!("repos/{REPO}/adapters/v1/test/cursor.json")).unwrap(),
                None,
            )
            .await
        {
            Ok(d) => Some(serde_json::from_slice(&d.read_bounded(16384).await.unwrap()).unwrap()),
            Err(object::Error::NotFound) => None,
            _ => panic!("cursor"),
        }
    }
}
#[derive(Clone, Copy)]
pub enum EffectFault {
    Before,
    After,
    Lie,
}
#[derive(Default)]
pub struct Control {
    pub fault: Option<EffectFault>,
    pub poison: Option<u64>,
    pub attempts: Vec<Vec<u64>>,
    pub bootstraps: usize,
    pub deny_after_effect: Option<Arc<Access>>,
}
#[derive(Serialize, Deserialize)]
struct Artifact {
    checkpoint: Checkpoint,
    positions: Vec<u64>,
    keys: Vec<m::ObjectKey>,
}
pub struct FileProjection {
    pub path: PathBuf,
    pub control: Arc<Mutex<Control>>,
}
impl FileProjection {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            control: Arc::new(Mutex::new(Control::default())),
        }
    }
    fn digest(
        identity: &Identity,
        source: &Position,
        positions: &[u64],
        keys: &[m::ObjectKey],
    ) -> m::RecordDigest {
        m::RecordDigest::new(
            Sha256::digest(serde_json::to_vec(&(identity, source, positions, keys)).unwrap())
                .into(),
        )
    }
    async fn persist(
        &self,
        snapshot: jj_store_adapters::Snapshot,
        identity: &Identity,
        positions: Vec<u64>,
        context: Context,
    ) -> jj_store_adapters::Result<Checkpoint> {
        let mut keys = vec![];
        for &key in snapshot.objects.entries().keys() {
            context.check()?;
            snapshot.objects.read(key).await?;
            keys.push(key);
        }
        // A staged future object is not in the selected snapshot's inventory.
        assert!(matches!(
            snapshot.objects.read(m::ObjectKey::Operation(op(16))).await,
            Err(Error::Denied)
        ));
        let artifact = Self::digest(identity, &snapshot.position, &positions, &keys);
        let checkpoint = Checkpoint {
            version: 1,
            identity: identity.clone(),
            source: snapshot.position,
            artifact,
        };
        let fault = self.control.lock().unwrap().fault.take();
        if matches!(fault, Some(EffectFault::Before)) {
            return Err(Error::Projection);
        }
        if matches!(fault, Some(EffectFault::Lie)) {
            return Ok(checkpoint);
        }
        let state = Artifact {
            checkpoint: checkpoint.clone(),
            positions,
            keys,
        };
        let bytes = serde_json::to_vec(&state).unwrap();
        let mut temp = tempfile::NamedTempFile::new_in(self.path.parent().unwrap()).unwrap();
        use std::io::Write as _;
        temp.write_all(&bytes).unwrap();
        temp.as_file().sync_all().unwrap();
        temp.persist(&self.path).unwrap();
        std::fs::File::open(self.path.parent().unwrap())
            .unwrap()
            .sync_all()
            .unwrap();
        if let Some(access) = self.control.lock().unwrap().deny_after_effect.take() {
            access.allowed.store(false, Ordering::SeqCst);
        }
        if matches!(fault, Some(EffectFault::After)) {
            return Err(Error::Projection);
        }
        Ok(checkpoint)
    }
    pub fn positions(&self) -> Vec<u64> {
        serde_json::from_slice::<Artifact>(&std::fs::read(&self.path).unwrap())
            .unwrap()
            .positions
    }
}
#[async_trait::async_trait]
impl Projection for FileProjection {
    async fn recover(&mut self, context: Context) -> jj_store_adapters::Result<Option<Checkpoint>> {
        context.check()?;
        let bytes = match std::fs::read(&self.path) {
            Ok(b) => b,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(_) => return Err(Error::Unavailable),
        };
        let state: Artifact = serde_json::from_slice(&bytes).map_err(|_| Error::Corrupt)?;
        if Self::digest(
            &state.checkpoint.identity,
            &state.checkpoint.source,
            &state.positions,
            &state.keys,
        ) != state.checkpoint.artifact
        {
            return Err(Error::Corrupt);
        }
        Ok(Some(state.checkpoint))
    }
    async fn bootstrap(
        &mut self,
        snapshot: jj_store_adapters::Snapshot,
        identity: &Identity,
        context: Context,
    ) -> jj_store_adapters::Result<Checkpoint> {
        self.control.lock().unwrap().bootstraps += 1;
        let position = snapshot.position.position.0;
        self.persist(snapshot, identity, vec![position], context)
            .await
    }
    async fn apply(
        &mut self,
        batch: Batch,
        identity: &Identity,
        context: Context,
    ) -> jj_store_adapters::Result<Checkpoint> {
        let positions: Vec<_> = batch
            .records
            .iter()
            .map(|r| r.transaction().after.position.0)
            .collect();
        {
            let mut control = self.control.lock().unwrap();
            control.attempts.push(positions.clone());
            if control.poison.is_some_and(|p| positions.contains(&p)) {
                return Err(Error::Projection);
            }
        }
        let mut all = self.positions();
        assert_eq!(all.last().copied(), Some(batch.before.position.0));
        all.extend(positions);
        self.persist(batch.snapshot, identity, all, context).await
    }
}
