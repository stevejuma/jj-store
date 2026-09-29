#[path = "support/review.rs"]
mod review_support;

use jj_agent::{
    AgentCatalogEditV1, AgentCatalogKey, AgentCatalogKind, AgentDeltaV1, AgentObjectV1,
    AgentSessionId, AgentSessionSnapshotV1, CompletenessV1, CoverageV1, IntegrationId,
    SessionState,
};
use jj_review::{ReviewCatalogKey, ReviewCatalogKind, ReviewDeltaV1};
use jj_store_model::{
    EnvelopeDigest, EpochId, ObjectKey, OperationId, PrincipalId, PublicationId, RepositoryId,
    RequestId,
};
use jj_store_object::{
    ByteRange, Condition, Download, Key, Metadata, ObjectStore, StagedUpload, WriteOutcome,
    memory::MemoryStore,
};
use jj_store_wal::{
    self as wal,
    inventory::{DependencySource, InventoryError, Representation, SourceScope, VerifiedNode},
    publication::{
        AgentDeltaRequest, Coordinator, Domain, DomainRequest, Limits, ManifestSnapshot,
        PublicationGuard, PublishOutcome, Request, SemanticKind, Snapshot, Transition,
    },
};
use sha2::{Digest as _, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{Arc, Mutex},
};

const REPOSITORY: RepositoryId = RepositoryId::new([1; 16]);
type NativeFixture = (
    Arc<BTreeMap<ObjectKey, VerifiedNode>>,
    Vec<(ObjectKey, Vec<u8>)>,
);

struct Permit;

#[async_trait::async_trait]
impl PublicationGuard for Permit {
    async fn authorize(&self, _: &ManifestSnapshot, _: &Request) -> wal::Result<()> {
        Ok(())
    }

    async fn authorize_domain(&self, _: &ManifestSnapshot, _: &DomainRequest) -> wal::Result<()> {
        Ok(())
    }

    async fn validate_transition(
        &self,
        _: &ManifestSnapshot,
        _: &Request,
        _: &Snapshot,
    ) -> wal::Result<()> {
        Ok(())
    }
}

#[derive(Clone)]
struct StoredSource {
    scope: SourceScope,
    store: Arc<dyn ObjectStore>,
    nodes: Arc<BTreeMap<ObjectKey, VerifiedNode>>,
}

#[async_trait::async_trait]
impl DependencySource for StoredSource {
    fn scope(&self) -> SourceScope {
        self.scope
    }

    async fn load_verified(
        &self,
        key: ObjectKey,
        max_bytes: u64,
    ) -> wal::inventory::Result<VerifiedNode> {
        let expected = self
            .nodes
            .get(&key)
            .cloned()
            .ok_or_else(|| InventoryError::Source("missing native fixture object".into()))?;
        if matches!(expected.representation, Representation::Virtual) {
            return Ok(expected);
        }
        let object_key = Key::new(key.storage_path(REPOSITORY))
            .map_err(|error| InventoryError::Source(error.to_string()))?;
        let bytes = self
            .store
            .get(&object_key, None)
            .await
            .map_err(|error| InventoryError::Source(error.to_string()))?
            .read_bounded(max_bytes)
            .await
            .map_err(|error| InventoryError::Source(error.to_string()))?;
        let actual = VerifiedNode {
            key,
            representation: Representation::Stored {
                envelope_digest: EnvelopeDigest::new(Sha256::digest(&bytes).into()),
                bytes: bytes.len() as u64,
            },
            dependencies: expected.dependencies.clone(),
        };
        if actual != expected {
            return Err(InventoryError::Source(
                "native fixture integrity mismatch".into(),
            ));
        }
        Ok(actual)
    }
}

struct DestinationStore {
    inner: MemoryStore,
    writes: Mutex<Vec<String>>,
    unknown_manifest_ack: bool,
}

impl DestinationStore {
    fn new(unknown_manifest_ack: bool) -> Self {
        Self {
            inner: MemoryStore::default(),
            writes: Mutex::new(Vec::new()),
            unknown_manifest_ack,
        }
    }
}

#[async_trait::async_trait]
impl ObjectStore for DestinationStore {
    async fn head(&self, key: &Key) -> jj_store_object::Result<Metadata> {
        self.inner.head(key).await
    }

    async fn get(&self, key: &Key, range: Option<ByteRange>) -> jj_store_object::Result<Download> {
        self.inner.get(key, range).await
    }

    async fn put(
        &self,
        key: &Key,
        condition: Condition,
        upload: &StagedUpload,
    ) -> jj_store_object::Result<WriteOutcome> {
        self.writes.lock().unwrap().push(key.as_str().to_owned());
        let outcome = self.inner.put(key, condition, upload).await?;
        if self.unknown_manifest_ack
            && key.as_str().ends_with("/manifest.json")
            && matches!(outcome, WriteOutcome::Applied(_))
        {
            Ok(WriteOutcome::Unknown)
        } else {
            Ok(outcome)
        }
    }
}

fn operation(byte: u8) -> OperationId {
    OperationId::new([byte; 64])
}

fn nodes() -> NativeFixture {
    let root = ObjectKey::Operation(operation(0));
    let selected = ObjectKey::Operation(operation(1));
    let bytes = b"verified native operation one".to_vec();
    let nodes = BTreeMap::from([
        (
            root,
            VerifiedNode {
                key: root,
                representation: Representation::Virtual,
                dependencies: BTreeSet::new(),
            },
        ),
        (
            selected,
            VerifiedNode {
                key: selected,
                representation: Representation::Stored {
                    envelope_digest: EnvelopeDigest::new(Sha256::digest(&bytes).into()),
                    bytes: bytes.len() as u64,
                },
                dependencies: BTreeSet::from([root]),
            },
        ),
    ]);
    (Arc::new(nodes), vec![(selected, bytes)])
}

async fn put_native(store: &Arc<dyn ObjectStore>, objects: &[(ObjectKey, Vec<u8>)]) {
    for (key, bytes) in objects {
        let upload = StagedUpload::from_bytes(bytes, 1024).await.unwrap();
        assert!(matches!(
            store
                .put(
                    &Key::new(key.storage_path(REPOSITORY)).unwrap(),
                    Condition::Absent,
                    &upload,
                )
                .await
                .unwrap(),
            WriteOutcome::Applied(_)
        ));
    }
}

fn coordinator(
    store: Arc<dyn ObjectStore>,
    nodes: Arc<BTreeMap<ObjectKey, VerifiedNode>>,
) -> Coordinator {
    Coordinator::new(
        store.clone(),
        REPOSITORY,
        Arc::new(StoredSource {
            scope: SourceScope::new(REPOSITORY),
            store,
            nodes,
        }),
        Arc::new(Permit),
        Limits::default(),
    )
    .unwrap()
}

fn committed(outcome: PublishOutcome) {
    assert!(matches!(outcome, PublishOutcome::Committed(_)));
}

async fn three_root_fixture() -> (
    Coordinator,
    Arc<BTreeMap<ObjectKey, VerifiedNode>>,
    AgentCatalogKey,
    jj_agent::AgentObjectRef,
    ReviewCatalogKey,
    jj_review::ReviewObjectRef,
) {
    let source_store: Arc<dyn ObjectStore> = Arc::new(MemoryStore::default());
    let (nodes, native) = nodes();
    put_native(&source_store, &native).await;
    let coordinator = coordinator(source_store, nodes.clone());
    coordinator
        .initialize(EpochId::new([2; 16]), b"native-copy".to_vec())
        .await
        .unwrap();
    committed(
        coordinator
            .publish(&Request {
                principal: PrincipalId::new([3; 16]),
                request_id: RequestId::new([10; 16]),
                publication_id: PublicationId::new([10; 16]),
                transition: Some(Transition::Native {
                    remove: BTreeSet::from([operation(0)]),
                    add: operation(1),
                }),
                settings: None,
                result: Vec::new(),
                kind: SemanticKind::OperationPublished,
            })
            .await
            .unwrap(),
    );

    let session_id = AgentSessionId::new([20; 16]);
    let session_key = AgentCatalogKey::derive("session", session_id.as_bytes()).unwrap();
    let session = coordinator
        .stage_agent_object(&AgentObjectV1::SessionSnapshot(AgentSessionSnapshotV1 {
            schema_version: 1,
            session_id,
            integration: IntegrationId::Codex,
            state: SessionState::Ended,
            parent_session_id: None,
            evidence: Vec::new(),
            event_batches: Vec::new(),
            transcripts: Vec::new(),
            tools: BTreeMap::new(),
            coverage: CoverageV1 {
                completeness: CompletenessV1::Complete,
                gaps: 0,
                redacted_fields: 0,
                truncated: false,
            },
        }))
        .await
        .unwrap();
    committed(
        coordinator
            .publish_agent_delta(
                &AgentDeltaRequest::new(
                    PrincipalId::new([3; 16]),
                    RequestId::new([21; 16]),
                    PublicationId::new([21; 16]),
                    AgentDeltaV1 {
                        schema_version: 1,
                        edits: vec![AgentCatalogEditV1 {
                            catalog: AgentCatalogKind::Sessions,
                            key: session_key,
                            expected: None,
                            replacement: Some(session),
                        }],
                    },
                    Vec::new(),
                )
                .unwrap(),
            )
            .await
            .unwrap(),
    );

    let selected = coordinator.read().await.unwrap();
    let review_state = selected.manifest().domain_state(Domain::Review).unwrap();
    let review = coordinator.review_store(review_state.root).await.unwrap();
    let thread = review_support::stage_thread(&coordinator, 30, None).await;
    let review_root = coordinator
        .stage_review_delta(
            &review,
            &ReviewDeltaV1 {
                schema_version: 1,
                edits: review_support::edits(&thread),
            },
        )
        .await
        .unwrap();
    committed(
        coordinator
            .publish_domain(
                &DomainRequest::new(
                    PrincipalId::new([3; 16]),
                    RequestId::new([31; 16]),
                    PublicationId::new([31; 16]),
                    Domain::Review,
                    review_state,
                    review_root,
                    Vec::new(),
                )
                .unwrap(),
            )
            .await
            .unwrap(),
    );
    coordinator.checkpoint().await.unwrap();
    let thread_key = ReviewCatalogKey::derive("thread", thread.thread_id.as_bytes()).unwrap();
    (
        coordinator,
        nodes,
        session_key,
        session,
        thread_key,
        thread.snapshot,
    )
}

#[tokio::test]
async fn native_copy_restores_code_agent_review_and_installs_manifest_last() {
    let (source, nodes, session_key, session, thread_key, thread) = three_root_fixture().await;

    let limited = Arc::new(DestinationStore::new(false));
    let limited_store: Arc<dyn ObjectStore> = limited.clone();
    let limited_source = Arc::new(StoredSource {
        scope: SourceScope::new(REPOSITORY),
        store: limited_store.clone(),
        nodes: nodes.clone(),
    });
    assert!(matches!(
        source
            .copy_native_to(limited_store, limited_source, 2)
            .await,
        Err(wal::Error::Limit("recovery history"))
    ));
    assert!(matches!(
        limited.head(&source.manifest_key()).await,
        Err(jj_store_object::Error::NotFound)
    ));

    let destination = Arc::new(DestinationStore::new(true));
    let destination_store: Arc<dyn ObjectStore> = destination.clone();
    let destination_source = Arc::new(StoredSource {
        scope: SourceScope::new(REPOSITORY),
        store: destination_store.clone(),
        nodes,
    });
    let outcome = source
        .copy_native_to(destination_store.clone(), destination_source.clone(), 64)
        .await
        .unwrap();
    assert_eq!(
        outcome.snapshot.digest(),
        source.read().await.unwrap().digest()
    );
    assert_eq!(outcome.native_objects, 1);
    assert!(outcome.control_records > 10);
    assert!(outcome.copied_bytes > 0);
    assert!(
        destination
            .writes
            .lock()
            .unwrap()
            .last()
            .unwrap()
            .ends_with("/manifest.json")
    );

    let restored = Coordinator::new(
        destination_store,
        REPOSITORY,
        destination_source,
        Arc::new(Permit),
        Limits::default(),
    )
    .unwrap();
    let snapshot = restored.recover(64).await.unwrap();
    assert_eq!(snapshot.manifest().published.0, 3);
    let agent_root = snapshot
        .manifest()
        .domain_state(Domain::Agent)
        .unwrap()
        .root;
    assert_eq!(
        restored
            .agent_store(agent_root)
            .await
            .unwrap()
            .lookup(AgentCatalogKind::Sessions, session_key)
            .await
            .unwrap(),
        Some(session)
    );
    let review_root = snapshot
        .manifest()
        .domain_state(Domain::Review)
        .unwrap()
        .root;
    assert_eq!(
        restored
            .review_store(review_root)
            .await
            .unwrap()
            .lookup(ReviewCatalogKind::Threads, thread_key)
            .await
            .unwrap(),
        Some(thread)
    );
}
