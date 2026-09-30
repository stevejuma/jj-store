use jj_store_model::*;
use jj_store_object::{
    ByteRange, Condition, Download, Key, Metadata, ObjectStore, StagedUpload, WriteOutcome,
    memory::MemoryStore,
};
use jj_store_replication::{TransferLimits, transfer_closures};
use jj_store_wal::{
    self as wal,
    inventory::{DependencySource, InventoryError, Representation, SourceScope, VerifiedNode},
    publication::*,
};
use sha2::{Digest as _, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{Arc, Mutex},
};

const REPOSITORY: RepositoryId = RepositoryId::new([1; 16]);

struct Permit;

#[async_trait::async_trait]
impl PublicationGuard for Permit {
    async fn authorize(&self, _: &ManifestSnapshot, _: &Request) -> wal::Result<()> {
        Ok(())
    }

    async fn authorize_replication(
        &self,
        _: &ManifestSnapshot,
        _: &ApplyOriginsRequestV1,
    ) -> wal::Result<()> {
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
            .ok_or_else(|| InventoryError::Source("missing test object".into()))?;
        if matches!(expected.representation, Representation::Virtual) {
            return Ok(expected);
        }
        let bytes = self
            .store
            .get(
                &Key::new(key.storage_path(REPOSITORY))
                    .map_err(|error| InventoryError::Source(error.to_string()))?,
                None,
            )
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
        (actual == expected)
            .then_some(actual)
            .ok_or_else(|| InventoryError::Source("test object changed".into()))
    }
}

struct FaultStore {
    inner: MemoryStore,
    unknown_manifest: Mutex<bool>,
}

impl FaultStore {
    fn new() -> Self {
        Self {
            inner: MemoryStore::default(),
            unknown_manifest: Mutex::new(false),
        }
    }

    fn lose_next_manifest_ack(&self) {
        *self.unknown_manifest.lock().unwrap() = true;
    }
}

#[async_trait::async_trait]
impl ObjectStore for FaultStore {
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
        let result = self.inner.put(key, condition, upload).await?;
        if key.as_str().ends_with("/manifest.json")
            && matches!(result, WriteOutcome::Applied(_))
            && std::mem::take(&mut *self.unknown_manifest.lock().unwrap())
        {
            Ok(WriteOutcome::Unknown)
        } else {
            Ok(result)
        }
    }
}

fn operation(value: u8) -> OperationId {
    OperationId::new([value; 64])
}

fn native_fixture() -> (
    Arc<BTreeMap<ObjectKey, VerifiedNode>>,
    Vec<(ObjectKey, Vec<u8>)>,
) {
    let root = ObjectKey::Operation(operation(0));
    let first = ObjectKey::Operation(operation(1));
    let second = ObjectKey::Operation(operation(2));
    let first_bytes = b"replicated operation one".to_vec();
    let second_bytes = b"replicated operation two".to_vec();
    (
        Arc::new(BTreeMap::from([
            (
                root,
                VerifiedNode {
                    key: root,
                    representation: Representation::Virtual,
                    dependencies: BTreeSet::new(),
                },
            ),
            (
                first,
                VerifiedNode {
                    key: first,
                    representation: Representation::Stored {
                        envelope_digest: EnvelopeDigest::new(Sha256::digest(&first_bytes).into()),
                        bytes: first_bytes.len() as u64,
                    },
                    dependencies: BTreeSet::from([root]),
                },
            ),
            (
                second,
                VerifiedNode {
                    key: second,
                    representation: Representation::Stored {
                        envelope_digest: EnvelopeDigest::new(Sha256::digest(&second_bytes).into()),
                        bytes: second_bytes.len() as u64,
                    },
                    dependencies: BTreeSet::from([first]),
                },
            ),
        ])),
        vec![(first, first_bytes), (second, second_bytes)],
    )
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

fn request(value: u8, remove: u8, add: u8) -> Request {
    Request {
        principal: PrincipalId::new([3; 16]),
        request_id: RequestId::new([value; 16]),
        publication_id: PublicationId::new([value; 16]),
        transition: Some(Transition::Native {
            remove: BTreeSet::from([operation(remove)]),
            add: operation(add),
        }),
        settings: None,
        result: vec![value],
        kind: SemanticKind::OperationPublished,
    }
}

fn apply_request(value: u8, origins: Vec<OriginEnvelopeV1>) -> ApplyOriginsRequestV1 {
    ApplyOriginsRequestV1 {
        schema_version: 1,
        principal: PrincipalId::new([9; 16]),
        request_id: RequestId::new([value; 16]),
        publication_id: PublicationId::new([value; 16]),
        origins,
    }
}

#[tokio::test]
async fn one_way_import_is_atomic_idempotent_and_recovers_a_lost_response() {
    let (nodes, objects) = native_fixture();
    let source_store: Arc<dyn ObjectStore> = Arc::new(MemoryStore::default());
    put_native(&source_store, &objects).await;
    let source = coordinator(source_store.clone(), nodes.clone());
    source
        .initialize_replica(
            EpochId::new([2; 16]),
            b"source".to_vec(),
            ReplicaId::new([3; 16]),
            WriterEpoch::new([4; 16]),
        )
        .await
        .unwrap();
    assert!(matches!(
        source.publish(&request(1, 0, 1)).await.unwrap(),
        PublishOutcome::Committed(_)
    ));
    assert!(matches!(
        source.publish(&request(2, 1, 2)).await.unwrap(),
        PublishOutcome::Committed(_)
    ));
    let source_snapshot = source.read().await.unwrap();
    let feed = source
        .committed_origin_feed(&source_snapshot, WalPosition(0), 10)
        .await
        .unwrap();
    assert_eq!(feed.through, WalPosition(2));
    assert_eq!(feed.upper_bound, WalPosition(2));
    assert_eq!(feed.origins.len(), 2);

    let destination_store = Arc::new(FaultStore::new());
    let destination_object_store: Arc<dyn ObjectStore> = destination_store.clone();
    let destination = coordinator(destination_object_store.clone(), nodes);
    destination
        .initialize_replica(
            EpochId::new([5; 16]),
            b"destination".to_vec(),
            ReplicaId::new([6; 16]),
            WriterEpoch::new([7; 16]),
        )
        .await
        .unwrap();
    let offers = feed
        .origins
        .iter()
        .map(|item| OriginOfferV1 {
            origin_key: item.envelope.publication.origin_key,
            origin_digest: item.envelope.origin_digest,
        })
        .collect::<Vec<_>>();
    assert!(
        destination
            .offer_origins(&destination.read().await.unwrap(), &offers)
            .await
            .unwrap()
            .iter()
            .all(|result| matches!(result, OriginOfferResultV1::Missing { .. }))
    );
    let mut closures = Vec::new();
    for item in &feed.origins {
        closures.push(
            source
                .origin_closure(&source_snapshot, item.envelope.publication.origin_key, 100)
                .await
                .unwrap(),
        );
    }
    transfer_closures(
        source_store,
        destination_object_store,
        REPOSITORY,
        &closures,
        TransferLimits::default(),
    )
    .await
    .unwrap();

    let mut envelopes = feed
        .origins
        .iter()
        .map(|item| item.envelope.clone())
        .collect::<Vec<_>>();
    envelopes.reverse();
    let apply = apply_request(30, envelopes);
    destination_store.lose_next_manifest_ack();
    let ApplyOriginsOutcomeV1::Applied(first) = destination.apply_origins(&apply).await.unwrap()
    else {
        panic!("lost acknowledgement must resolve from the durable import receipt");
    };
    assert_eq!(first.publication_position, Some(WalPosition(1)));
    assert_eq!(first.receipts.len(), 2);
    assert!(first.receipts.iter().all(|receipt| {
        receipt.accepted.replica == ReplicaId::new([6; 16])
            && receipt.accepted.log_epoch == EpochId::new([5; 16])
            && receipt.accepted.position == WalPosition(1)
    }));
    let destination_snapshot = destination.recover(100).await.unwrap();
    assert_eq!(destination_snapshot.manifest().published, WalPosition(1));
    assert!(
        destination_snapshot
            .manifest()
            .heads
            .contains(&operation(2))
    );

    let ApplyOriginsOutcomeV1::Applied(replayed) = destination.apply_origins(&apply).await.unwrap()
    else {
        panic!("durable import receipt replay must resolve");
    };
    assert_eq!(replayed, first);
    assert_eq!(
        destination.read().await.unwrap().manifest().published,
        WalPosition(1)
    );

    let duplicate = apply_request(31, apply.origins.clone());
    let ApplyOriginsOutcomeV1::Applied(duplicate_result) =
        destination.apply_origins(&duplicate).await.unwrap()
    else {
        panic!("known origins must resolve without a content event");
    };
    assert_eq!(duplicate_result.publication_position, None);
    assert_eq!(
        destination.read().await.unwrap().manifest().published,
        WalPosition(1)
    );
}

#[tokio::test]
async fn holes_partial_closures_and_digest_reuse_never_publish() {
    let (nodes, objects) = native_fixture();
    let source_store: Arc<dyn ObjectStore> = Arc::new(MemoryStore::default());
    put_native(&source_store, &objects).await;
    let source = coordinator(source_store.clone(), nodes.clone());
    source
        .initialize_replica(
            EpochId::new([2; 16]),
            b"source".to_vec(),
            ReplicaId::new([3; 16]),
            WriterEpoch::new([4; 16]),
        )
        .await
        .unwrap();
    source.publish(&request(1, 0, 1)).await.unwrap();
    source.publish(&request(2, 1, 2)).await.unwrap();
    let source_snapshot = source.read().await.unwrap();
    let feed = source
        .committed_origin_feed(&source_snapshot, WalPosition(0), 10)
        .await
        .unwrap();

    let destination_store: Arc<dyn ObjectStore> = Arc::new(MemoryStore::default());
    let destination = coordinator(destination_store.clone(), nodes);
    destination
        .initialize_replica(
            EpochId::new([5; 16]),
            b"destination".to_vec(),
            ReplicaId::new([6; 16]),
            WriterEpoch::new([7; 16]),
        )
        .await
        .unwrap();
    assert!(
        destination
            .apply_origins(&apply_request(40, vec![feed.origins[0].envelope.clone()]))
            .await
            .is_err()
    );
    assert_eq!(
        destination.read().await.unwrap().manifest().published,
        WalPosition(0)
    );

    let second = source
        .origin_closure(
            &source_snapshot,
            feed.origins[1].envelope.publication.origin_key,
            100,
        )
        .await
        .unwrap();
    transfer_closures(
        source_store.clone(),
        destination_store.clone(),
        REPOSITORY,
        &[second],
        TransferLimits::default(),
    )
    .await
    .unwrap();
    assert!(
        destination
            .apply_origins(&apply_request(41, vec![feed.origins[1].envelope.clone()]))
            .await
            .is_err()
    );
    assert_eq!(
        destination.read().await.unwrap().manifest().published,
        WalPosition(0)
    );

    let first = source
        .origin_closure(
            &source_snapshot,
            feed.origins[0].envelope.publication.origin_key,
            100,
        )
        .await
        .unwrap();
    transfer_closures(
        source_store,
        destination_store,
        REPOSITORY,
        &[first],
        TransferLimits::default(),
    )
    .await
    .unwrap();
    let both = feed
        .origins
        .iter()
        .map(|item| item.envelope.clone())
        .collect::<Vec<_>>();
    assert!(matches!(
        destination
            .apply_origins(&apply_request(42, both))
            .await
            .unwrap(),
        ApplyOriginsOutcomeV1::Applied(_)
    ));
    let accepted = destination.read().await.unwrap();
    assert_eq!(accepted.manifest().published, WalPosition(1));

    let mut conflicting = feed.origins[0].envelope.clone();
    conflicting.publication.principal = PrincipalId::new([99; 16]);
    conflicting.origin_digest = conflicting.publication.digest().unwrap();
    assert!(
        destination
            .apply_origins(&apply_request(43, vec![conflicting]))
            .await
            .is_err()
    );
    assert_eq!(
        destination.read().await.unwrap().manifest().published,
        WalPosition(1)
    );
}
