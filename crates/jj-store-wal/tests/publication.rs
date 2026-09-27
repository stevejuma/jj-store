//! Fault matrix for atomic visibility, idempotency, concurrency and recovery.
use jj_store_model::*;
use jj_store_object::{memory::MemoryStore, *};
use jj_store_wal::{
    self as wal,
    catalog::Catalog,
    inventory::{DependencySource, Representation, SourceScope, VerifiedNode},
    publication::*,
    records::{RecordKind, Records},
};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{Arc, Mutex},
    time::Duration,
};

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
        _: &Snapshot,
    ) -> wal::Result<()> {
        Ok(())
    }
}
const REPO: RepositoryId = RepositoryId::new([1; 16]);
fn op(n: u8) -> OperationId {
    OperationId::new([n; 64])
}
struct Source {
    scope: SourceScope,
    nodes: BTreeMap<ObjectKey, VerifiedNode>,
}
impl Source {
    fn new() -> Self {
        Self {
            scope: SourceScope::new(REPO),
            nodes: (0..=128)
                .map(|n| {
                    let key = ObjectKey::Operation(op(n));
                    (
                        key,
                        VerifiedNode {
                            key,
                            representation: if n == 0 {
                                Representation::Virtual
                            } else {
                                Representation::Stored {
                                    envelope_digest: EnvelopeDigest::new([n; 32]),
                                    bytes: 100,
                                }
                            },
                            dependencies: if n == 0 {
                                BTreeSet::new()
                            } else {
                                BTreeSet::from([ObjectKey::Operation(op(0))])
                            },
                        },
                    )
                })
                .collect(),
        }
    }
}
#[async_trait::async_trait]
impl DependencySource for Source {
    fn scope(&self) -> SourceScope {
        self.scope
    }
    async fn load_verified(&self, key: ObjectKey, _: u64) -> wal::inventory::Result<VerifiedNode> {
        self.nodes.get(&key).cloned().ok_or_else(|| {
            wal::inventory::InventoryError::Source("missing fixture operation".into())
        })
    }
}
#[derive(Clone, Copy)]
enum Fault {
    Before,
    After,
    HoldBefore,
    HoldAfter,
}
struct FaultStore {
    inner: MemoryStore,
    armed: Mutex<Option<(&'static str, Fault)>>,
    writes: Mutex<Vec<String>>,
    entered: tokio::sync::Notify,
    release: tokio::sync::Notify,
}
impl Default for FaultStore {
    fn default() -> Self {
        Self {
            inner: MemoryStore::default(),
            armed: Mutex::new(None),
            writes: Mutex::new(Vec::new()),
            entered: tokio::sync::Notify::new(),
            release: tokio::sync::Notify::new(),
        }
    }
}
impl FaultStore {
    fn arm(&self, path: &'static str, fault: Fault) {
        *self.armed.lock().unwrap() = Some((path, fault));
    }
}
#[async_trait::async_trait]
impl ObjectStore for FaultStore {
    async fn head(&self, key: &Key) -> Result<Metadata> {
        self.inner.head(key).await
    }
    async fn get(&self, key: &Key, range: Option<ByteRange>) -> Result<Download> {
        self.inner.get(key, range).await
    }
    async fn put(
        &self,
        key: &Key,
        condition: Condition,
        upload: &StagedUpload,
    ) -> Result<WriteOutcome> {
        self.writes.lock().unwrap().push(key.as_str().to_owned());
        let fault = {
            let mut armed = self.armed.lock().unwrap();
            if armed
                .as_ref()
                .is_some_and(|(path, _)| key.as_str().contains(path))
            {
                armed.take().map(|(_, fault)| fault)
            } else {
                None
            }
        };
        if matches!(fault, Some(Fault::Before)) {
            return Ok(WriteOutcome::Unknown);
        }
        if matches!(fault, Some(Fault::HoldBefore)) {
            self.entered.notify_one();
            self.release.notified().await;
        }
        let result = self.inner.put(key, condition, upload).await?;
        if matches!(fault, Some(Fault::HoldAfter)) {
            self.entered.notify_one();
            self.release.notified().await;
            return Ok(WriteOutcome::Unknown);
        }
        if matches!(fault, Some(Fault::After)) {
            return Ok(WriteOutcome::Unknown);
        }
        Ok(result)
    }
}
fn coordinator(store: Arc<FaultStore>) -> Coordinator {
    Coordinator::new(
        store,
        REPO,
        Arc::new(Source::new()),
        Arc::new(Permit),
        Limits {
            timeout: Duration::from_secs(15),
            ..Limits::default()
        },
    )
    .unwrap()
}
async fn fixture() -> (Arc<FaultStore>, Coordinator) {
    let store = Arc::new(FaultStore::default());
    let c = coordinator(store.clone());
    c.initialize(EpochId::new([2; 16]), b"initial".to_vec())
        .await
        .unwrap();
    (store, c)
}
fn request(n: u8, remove: &[u8], add: u8) -> Request {
    Request {
        principal: PrincipalId::new([3; 16]),
        request_id: RequestId::new([n; 16]),
        publication_id: PublicationId::new([n; 16]),
        transition: Some(Transition::Native {
            remove: remove.iter().map(|n| op(*n)).collect(),
            add: op(add),
        }),
        settings: None,
        result: vec![n],
        kind: SemanticKind::OperationPublished,
    }
}
fn committed(outcome: PublishOutcome) -> Receipt {
    match outcome {
        PublishOutcome::Committed(receipt) => receipt,
        other => panic!("expected committed: {other:?}"),
    }
}

#[tokio::test]
async fn atomic_publication_retains_receipts_through_restart_and_checkpoint() {
    let (store, c) = fixture().await;
    let first = request(1, &[0], 1);
    let receipt = committed(c.publish(&first).await.unwrap());
    assert_eq!(receipt.accepted.position, WalPosition(1));
    let second = request(2, &[1], 2);
    committed(c.publish(&second).await.unwrap());
    let before = c.read().await.unwrap();
    let after = c.checkpoint().await.unwrap();
    assert_eq!(after.manifest().published, WalPosition(2));
    assert_eq!(after.manifest().revision, before.manifest().revision + 1);
    let restarted = coordinator(store);
    assert_eq!(
        restarted.status(&first).await.unwrap(),
        Some(receipt.clone())
    );
    assert_eq!(committed(restarted.publish(&first).await.unwrap()), receipt);
    let current = restarted.recover(100).await.unwrap();
    assert_eq!(current.manifest().heads, BTreeSet::from([op(2)]));
    assert_eq!(current.manifest().published, WalPosition(2));
    let log = restarted
        .committed(&current, WalPosition(0), 10)
        .await
        .unwrap();
    assert_eq!(log.len(), 2);
    assert_eq!(log[0].after.position, WalPosition(1));
    assert_eq!(log[1].before, log[0].after);
    let inventory = restarted.inventory(&current, 100).await.unwrap();
    assert_eq!(inventory.len(), 3);
    assert!(
        inventory
            .iter()
            .any(|n| n.key == ObjectKey::Operation(op(1)))
    );
}
#[tokio::test]
async fn every_staging_or_manifest_fault_preserves_atomic_visibility_and_retry_identity() {
    for path in [
        "/inventories/",
        "/catalogs/",
        "/receipts/",
        "/wal/",
        "/manifest.json",
    ] {
        for fault in [Fault::Before, Fault::After] {
            let (store, c2) = fixture().await;
            store.arm(path, fault);
            let req = request(1, &[0], 1);
            let result = c2.publish(&req).await.unwrap();
            let snapshot = c2.read().await.unwrap();
            match result {
                PublishOutcome::Committed(_) => {
                    assert_eq!(snapshot.manifest().published, WalPosition(1))
                }
                PublishOutcome::OutcomeUnknown {
                    request_id,
                    publication_id,
                } => {
                    assert_eq!(request_id, req.request_id);
                    assert_eq!(publication_id, req.publication_id);
                    assert_eq!(snapshot.manifest().published, WalPosition(0));
                    assert!(c2.status(&req).await.unwrap().is_none());
                }
            }
            let receipt = committed(c2.publish(&req).await.unwrap());
            assert_eq!(receipt.accepted.position, WalPosition(1));
            let snapshot = c2.read().await.unwrap();
            assert_eq!(snapshot.manifest().published, WalPosition(1));
            assert_eq!(
                c2.committed(&snapshot, WalPosition(0), 10)
                    .await
                    .unwrap()
                    .len(),
                1
            );
        }
    }
}
#[tokio::test]
async fn competing_candidates_preserve_siblings_and_only_selected_records_are_committed() {
    let (store, c) = fixture().await;
    store.arm("/manifest.json", Fault::HoldBefore);
    let c1 = c.clone();
    let first = tokio::spawn(async move { c1.publish(&request(1, &[0], 1)).await.unwrap() });
    store.entered.notified().await;
    committed(c.publish(&request(2, &[0], 2)).await.unwrap());
    store.release.notify_one();
    committed(first.await.unwrap());
    let snapshot = c.read().await.unwrap();
    assert_eq!(snapshot.manifest().heads, BTreeSet::from([op(1), op(2)]));
    assert_eq!(snapshot.manifest().published, WalPosition(2));
    let log = c.committed(&snapshot, WalPosition(0), 10).await.unwrap();
    assert_eq!(log.len(), 2);
    let candidates = store
        .writes
        .lock()
        .unwrap()
        .iter()
        .filter(|k| k.contains("/wal/"))
        .cloned()
        .collect::<BTreeSet<_>>();
    assert_eq!(
        candidates.len(),
        3,
        "failed CAS leaves one harmless unselected candidate"
    );
}
#[tokio::test]
async fn delayed_duplicate_cannot_resurrect_superseded_heads() {
    let (store, c) = fixture().await;
    store.arm("/manifest.json", Fault::HoldBefore);
    let first = request(1, &[0], 1);
    let c1 = c.clone();
    let q = first.clone();
    let delayed = tokio::spawn(async move { c1.publish(&q).await.unwrap() });
    store.entered.notified().await;
    let accepted = committed(c.publish(&first).await.unwrap());
    committed(c.publish(&request(2, &[1], 2)).await.unwrap());
    c.checkpoint().await.unwrap();
    store.release.notify_one();
    assert_eq!(committed(delayed.await.unwrap()), accepted);
    assert_eq!(
        c.read().await.unwrap().manifest().heads,
        BTreeSet::from([op(2)])
    );
}
#[tokio::test]
async fn lost_acknowledgement_resolves_from_history_after_another_publication() {
    let (store, c) = fixture().await;
    store.arm("/manifest.json", Fault::HoldAfter);
    let c1 = c.clone();
    let first = tokio::spawn(async move { c1.publish(&request(1, &[0], 1)).await.unwrap() });
    store.entered.notified().await;
    committed(c.publish(&request(2, &[1], 2)).await.unwrap());
    c.checkpoint().await.unwrap();
    store.release.notify_one();
    assert_eq!(
        committed(first.await.unwrap()).accepted.position,
        WalPosition(1)
    );
    assert_eq!(c.read().await.unwrap().manifest().published, WalPosition(2));
}
#[tokio::test]
async fn strict_stale_intent_reuse_and_missing_dependencies_publish_nothing() {
    let (_, c) = fixture().await;
    let snapshot = c.read().await.unwrap().manifest().snapshot();
    let first = request(1, &[0], 1);
    committed(c.publish(&first).await.unwrap());
    let mut strict = request(2, &[0], 2);
    strict.transition = Some(Transition::Strict {
        expected: snapshot,
        heads: BTreeSet::from([op(2)]),
    });
    assert!(matches!(c.publish(&strict).await, Err(wal::Error::Stale)));
    let mut different = first.clone();
    different.result.push(99);
    assert!(matches!(
        c.publish(&different).await,
        Err(wal::Error::IntentConflict)
    ));
    let mut same_pub = request(3, &[1], 3);
    same_pub.publication_id = first.publication_id;
    assert!(matches!(
        c.publish(&same_pub).await,
        Err(wal::Error::IntentConflict)
    ));
    assert!(c.publish(&request(4, &[1], 250)).await.is_err());
    assert_eq!(c.read().await.unwrap().manifest().published, WalPosition(1));
    // Checkpoint revisions do not invalidate a logically unchanged strict preview.
    let current = c.read().await.unwrap().manifest().snapshot();
    c.checkpoint().await.unwrap();
    strict.transition = Some(Transition::Strict {
        expected: current,
        heads: BTreeSet::from([op(2)]),
    });
    committed(c.publish(&strict).await.unwrap());
}
#[tokio::test]
async fn checkpoint_race_preserves_newer_tail_receipts_and_inventory() {
    let (store, c) = fixture().await;
    let first = request(1, &[0], 1);
    committed(c.publish(&first).await.unwrap());
    store.arm("/checkpoints/", Fault::HoldBefore);
    let c1 = c.clone();
    let checkpoint = tokio::spawn(async move { c1.checkpoint().await.unwrap() });
    store.entered.notified().await;
    committed(c.publish(&request(2, &[1], 2)).await.unwrap());
    store.release.notify_one();
    let snapshot = checkpoint.await.unwrap();
    assert_eq!(snapshot.manifest().published, WalPosition(2));
    assert_eq!(snapshot.manifest().heads, BTreeSet::from([op(2)]));
    assert_eq!(
        c.committed(&snapshot, WalPosition(0), 10)
            .await
            .unwrap()
            .len(),
        2
    );
    assert_eq!(c.inventory(&snapshot, 100).await.unwrap().len(), 3);
    let records = Records::new(store, REPO);
    let cp: Checkpoint = records
        .get(
            snapshot.manifest().checkpoint.unwrap(),
            RecordKind::Checkpoint,
        )
        .await
        .unwrap();
    assert_eq!(cp.captured.published, WalPosition(1));
    assert!(c.status(&first).await.unwrap().is_some());
}
#[tokio::test]
async fn settings_are_selected_atomically_and_initialization_is_idempotent() {
    let (_, c) = fixture().await;
    let before = c.read().await.unwrap();
    let mut req = request(1, &[], 0);
    req.transition = None;
    req.settings = Some(b"new policy".to_vec());
    req.kind = SemanticKind::SettingsChanged;
    committed(c.publish(&req).await.unwrap());
    let after = c.read().await.unwrap();
    assert_eq!(c.settings(&before).await.unwrap(), b"initial");
    assert_eq!(c.settings(&after).await.unwrap(), b"new policy");
    assert_eq!(
        c.initialize(EpochId::new([2; 16]), b"initial".to_vec())
            .await
            .unwrap()
            .manifest()
            .published,
        WalPosition(1)
    );
    assert!(matches!(
        c.initialize(EpochId::new([2; 16]), b"different".to_vec())
            .await,
        Err(wal::Error::IntentConflict)
    ));
    assert!(matches!(
        c.initialize(EpochId::new([9; 16]), b"initial".to_vec())
            .await,
        Err(wal::Error::AlreadyExists)
    ));
}
#[tokio::test]
async fn persistent_catalog_splits_are_bounded_and_old_roots_remain_readable() {
    let store = Arc::new(MemoryStore::default());
    let records = Records::new(store, REPO);
    let catalog = Catalog::new(records.clone());
    let value = records
        .put(RecordKind::Settings, None, &"original")
        .await
        .unwrap();
    let keys = (0..200u16)
        .map(|n| {
            let mut key = [0; 32];
            key[..2].copy_from_slice(&n.to_be_bytes());
            RecordDigest::new(key)
        })
        .collect::<Vec<_>>();
    let old = catalog
        .update(None, keys.iter().map(|key| (*key, value)).collect())
        .await
        .unwrap();
    assert_eq!(catalog.entries(old, 200).await.unwrap().len(), 200);
    assert!(catalog.entries(old, 199).await.is_err());
    let replacement = records
        .put(RecordKind::Settings, None, &"replacement")
        .await
        .unwrap();
    let new = catalog
        .update(old, BTreeMap::from([(keys[77], replacement)]))
        .await
        .unwrap();
    assert_eq!(catalog.get(old, keys[77]).await.unwrap(), Some(value));
    assert_eq!(catalog.get(new, keys[77]).await.unwrap(), Some(replacement));
    for key in keys {
        assert!(catalog.get(new, key).await.unwrap().is_some());
    }
}

#[tokio::test]
async fn cold_recovery_rejects_missing_native_data_and_control_corruption() {
    let (store, c) = fixture().await;
    committed(c.publish(&request(1, &[0], 1)).await.unwrap());
    let mut source = Source::new();
    source.nodes.remove(&ObjectKey::Operation(op(1)));
    let broken = Coordinator::new(
        store.clone(),
        REPO,
        Arc::new(source),
        Arc::new(Permit),
        Limits::default(),
    )
    .unwrap();
    assert!(broken.recover(100).await.is_err());
    assert!(c.recover(1).await.is_err());
    store
        .inner
        .inject(jj_store_object::memory::Fault::ReadCorrupt);
    assert!(c.recover(100).await.is_err());
    assert_eq!(
        c.recover(100).await.unwrap().manifest().published,
        WalPosition(1)
    );
}

#[tokio::test]
async fn deterministic_publication_sequences_replay_with_original_results() {
    let (store, c) = fixture().await;
    let mut expected = BTreeSet::from([op(0)]);
    let mut receipts = Vec::new();
    let mut random = 0x21acfe93848u64;
    for n in 1..=64u8 {
        random = random.wrapping_mul(6364136223846793005).wrapping_add(1);
        let remove = if random & 1 == 0 {
            expected.clone()
        } else {
            BTreeSet::new()
        };
        let mut req = request(n, &[], n);
        req.transition = Some(Transition::Native {
            remove: remove.clone(),
            add: op(n),
        });
        expected = expected.difference(&remove).copied().collect();
        expected.insert(op(n));
        let receipt = committed(c.publish(&req).await.unwrap());
        assert_eq!(receipt.accepted.heads, expected);
        receipts.push((req, receipt));
        if n % 13 == 0 {
            c.checkpoint().await.unwrap();
        }
    }
    let restarted = coordinator(store);
    let snapshot = restarted.recover(256).await.unwrap();
    assert_eq!(snapshot.manifest().heads, expected);
    for (request, receipt) in receipts.iter().rev() {
        assert_eq!(
            committed(restarted.publish(request).await.unwrap()),
            *receipt
        );
    }
    assert_eq!(
        restarted.read().await.unwrap().manifest().published,
        WalPosition(64)
    );
    let mut after = WalPosition(0);
    let mut seen = 0;
    loop {
        let page = restarted.committed(&snapshot, after, 7).await.unwrap();
        if page.is_empty() {
            break;
        }
        for tx in page {
            seen += 1;
            after = tx.after.position;
        }
    }
    assert_eq!(seen, 64);
}

#[tokio::test]
async fn authorization_is_rechecked_after_cas_conflict_and_before_receipt_replay() {
    struct RevokeAfterFirst;
    #[async_trait::async_trait]
    impl PublicationGuard for RevokeAfterFirst {
        async fn authorize(&self, base: &ManifestSnapshot, _: &Request) -> wal::Result<()> {
            if base.manifest().published.0 > 0 {
                Err(wal::Error::Denied)
            } else {
                Ok(())
            }
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
    let (store, permitted) = fixture().await;
    let guarded = Coordinator::new(
        store.clone(),
        REPO,
        Arc::new(Source::new()),
        Arc::new(RevokeAfterFirst),
        Limits::default(),
    )
    .unwrap();
    store.arm("/manifest.json", Fault::HoldBefore);
    let delayed = guarded.clone();
    let task = tokio::spawn(async move { delayed.publish(&request(1, &[0], 1)).await });
    store.entered.notified().await;
    let other = request(2, &[0], 2);
    committed(permitted.publish(&other).await.unwrap());
    store.release.notify_one();
    assert!(matches!(task.await.unwrap(), Err(wal::Error::Denied)));
    assert_eq!(
        permitted.read().await.unwrap().manifest().heads,
        BTreeSet::from([op(2)])
    );
    assert!(matches!(
        guarded.status(&other).await,
        Err(wal::Error::Denied)
    ));
    assert!(matches!(
        guarded.publish(&other).await,
        Err(wal::Error::Denied)
    ));
    assert_eq!(
        permitted.read().await.unwrap().manifest().published,
        WalPosition(1)
    );
}
#[tokio::test]
async fn semantic_denial_stages_no_publication_records() {
    struct Reject;
    #[async_trait::async_trait]
    impl PublicationGuard for Reject {
        async fn authorize(&self, _: &ManifestSnapshot, _: &Request) -> wal::Result<()> {
            Ok(())
        }
        async fn validate_transition(
            &self,
            _: &ManifestSnapshot,
            _: &Request,
            _: &Snapshot,
        ) -> wal::Result<()> {
            Err(wal::Error::Denied)
        }
    }
    let (store, permitted) = fixture().await;
    let before = store.writes.lock().unwrap().len();
    let guarded = Coordinator::new(
        store.clone(),
        REPO,
        Arc::new(Source::new()),
        Arc::new(Reject),
        Limits::default(),
    )
    .unwrap();
    assert!(matches!(
        guarded.publish(&request(1, &[0], 1)).await,
        Err(wal::Error::Denied)
    ));
    assert_eq!(store.writes.lock().unwrap().len(), before);
    assert_eq!(
        permitted.read().await.unwrap().manifest().published,
        WalPosition(0)
    );
}

#[tokio::test]
async fn cold_recovery_verifies_retained_historical_roots_and_settings() {
    for target in ["inventory", "settings", "checkpoint_receipts"] {
        let (store, c) = fixture().await;
        let mut first = request(1, &[0], 1);
        first.settings = Some(b"historical settings".to_vec());
        first.kind = SemanticKind::SettingsChanged;
        committed(c.publish(&first).await.unwrap());
        let captured = c.checkpoint().await.unwrap();
        let mut second = request(2, &[1], 2);
        second.settings = Some(b"current settings".to_vec());
        second.kind = SemanticKind::SettingsChanged;
        committed(c.publish(&second).await.unwrap());
        let reference = match target {
            "inventory" => captured.manifest().inventory.unwrap(),
            "settings" => captured.manifest().settings,
            _ => captured.manifest().receipts.unwrap(),
        };
        c.recover(100).await.unwrap();
        let key = reference.key(REPO).unwrap();
        let original = store.inner.get(&key, None).await.unwrap();
        let version = original.metadata.version.clone();
        let bytes = original.read_bounded(1024 * 1024).await.unwrap();
        let bad = StagedUpload::from_bytes(b"corrupted retained record", 100)
            .await
            .unwrap();
        store
            .inner
            .put(&key, Condition::Matches(version), &bad)
            .await
            .unwrap();
        assert!(c.recover(100).await.is_err(), "accepted missing {target}");
        let version = store.inner.head(&key).await.unwrap().version;
        let restored = StagedUpload::from_bytes(&bytes, 1024 * 1024).await.unwrap();
        store
            .inner
            .put(&key, Condition::Matches(version), &restored)
            .await
            .unwrap();
        c.recover(100).await.unwrap();
    }
}

#[tokio::test]
async fn cancellation_preserves_unknown_identity_before_and_after_manifest_application() {
    for fault in [Fault::HoldBefore, Fault::HoldAfter] {
        let (store, original) = fixture().await;
        let c = Coordinator::new(
            store.clone(),
            REPO,
            Arc::new(Source::new()),
            Arc::new(Permit),
            Limits {
                timeout: Duration::from_secs(60),
                ..Limits::default()
            },
        )
        .unwrap();
        store.arm("/manifest.json", fault);
        let request = request(1, &[0], 1);
        let input = request.clone();
        let pending = tokio::spawn(async move { c.publish(&input).await });
        // Do not start the simulated timeout until the exact before/after
        // manifest boundary is reached. Disk scheduling must not choose which
        // failure point this test exercises.
        tokio::time::timeout(Duration::from_secs(15), store.entered.notified())
            .await
            .unwrap();
        tokio::time::pause();
        tokio::time::advance(Duration::from_secs(61)).await;
        let outcome = pending.await.unwrap().unwrap();
        tokio::time::resume();
        assert_eq!(
            outcome,
            PublishOutcome::OutcomeUnknown {
                request_id: request.request_id,
                publication_id: request.publication_id
            }
        );
        let expected = if matches!(fault, Fault::HoldAfter) {
            1
        } else {
            0
        };
        assert_eq!(
            original.read().await.unwrap().manifest().published,
            WalPosition(expected)
        );
        let receipt = committed(original.publish(&request).await.unwrap());
        assert_eq!(receipt.accepted.position, WalPosition(1));
        assert_eq!(
            original.recover(100).await.unwrap().manifest().published,
            WalPosition(1)
        );
    }
}

#[tokio::test]
async fn historical_settings_bind_selected_snapshot_and_survive_checkpoint() {
    let (store, c) = fixture().await;
    let initial = c.read().await.unwrap();
    let initial_settings = c.settings(&initial).await.unwrap();
    let mut update = request(91, &[0], 1);
    update.kind = SemanticKind::SettingsChanged;
    update.settings = Some(b"policy-one".to_vec());
    let first = committed(c.publish(&update).await.unwrap());
    let mut next = request(92, &[1], 2);
    next.kind = SemanticKind::SettingsChanged;
    next.settings = Some(b"policy-two".to_vec());
    let second = committed(c.publish(&next).await.unwrap());
    c.checkpoint().await.unwrap();
    let restarted = coordinator(store.clone());
    let current = restarted.read().await.unwrap();
    assert_eq!(
        restarted
            .settings_at(&current, &initial.manifest().snapshot())
            .await
            .unwrap(),
        initial_settings
    );
    assert_eq!(
        restarted
            .settings_at(&current, &first.accepted)
            .await
            .unwrap(),
        b"policy-one"
    );
    assert_eq!(
        restarted
            .settings_at(&current, &second.accepted)
            .await
            .unwrap(),
        b"policy-two"
    );
    for bad in [
        Snapshot {
            epoch: EpochId::new([77; 16]),
            ..first.accepted.clone()
        },
        Snapshot {
            heads: BTreeSet::from([op(9)]),
            ..first.accepted.clone()
        },
        Snapshot {
            position: WalPosition(999),
            ..first.accepted.clone()
        },
        Snapshot {
            heads: BTreeSet::from([op(9)]),
            ..initial.manifest().snapshot()
        },
    ] {
        assert!(matches!(
            restarted.settings_at(&current, &bad).await,
            Err(wal::Error::Stale)
        ));
    }
    // A selected settings record's corruption is an error, never an empty policy.
    let transaction = restarted
        .committed(&current, WalPosition(0), 1)
        .await
        .unwrap()
        .pop()
        .unwrap();
    let key = transaction.settings.key(REPO).unwrap();
    let version = store.head(&key).await.unwrap().version;
    let corrupt = StagedUpload::from_bytes(b"corrupt", 1024).await.unwrap();
    store
        .put(&key, Condition::Matches(version), &corrupt)
        .await
        .unwrap();
    assert!(matches!(
        restarted.settings_at(&current, &first.accepted).await,
        Err(wal::Error::Corrupt(_))
    ));
}
