//! Fault matrix for atomic visibility, idempotency, concurrency and recovery.
use jj_agent::{
    AgentCatalogEditV1, AgentCatalogKey, AgentCatalogKind, AgentCheckpointId, AgentDeltaV1,
    AgentIndexPageV1, AgentObjectId, AgentObjectKind, AgentObjectRef, AgentObjectV1, AgentRootV1,
    AgentSessionId, AgentSessionSnapshotV1, AssociationGranularity, AssociationId,
    AssociationRelation, CodeAnchorV1, CodeAssociationV1, CompletenessV1, CoverageV1,
    EvidenceMethod, IntegrationId, SessionState, SourceTrust,
};
use jj_store_model::*;
use jj_store_object::{memory::MemoryStore, *};
use jj_store_wal::{
    self as wal,
    catalog::Catalog,
    inventory::{DependencySource, Representation, SourceScope, VerifiedNode},
    publication::*,
    records::{RecordKind, RecordRef, Records},
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
    async fn authorize_domain(&self, _: &ManifestSnapshot, _: &DomainRequest) -> wal::Result<()> {
        Ok(())
    }
    async fn authorize_collaboration(
        &self,
        _: &ManifestSnapshot,
        _: &CollaborationRequest,
    ) -> wal::Result<()> {
        Ok(())
    }
}
const REPO: RepositoryId = RepositoryId::new([1; 16]);
fn op(n: u8) -> OperationId {
    OperationId::new([n; 64])
}
fn commit(n: u8) -> CommitId {
    CommitId::new([n; 64])
}
struct Source {
    scope: SourceScope,
    nodes: BTreeMap<ObjectKey, VerifiedNode>,
}
impl Source {
    fn new() -> Self {
        let mut nodes: BTreeMap<_, _> = (0..=128)
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
            .collect();
        nodes.extend((1..=128).map(|n| {
            let key = ObjectKey::Commit(commit(n));
            (
                key,
                VerifiedNode {
                    key,
                    representation: Representation::Stored {
                        envelope_digest: EnvelopeDigest::new([n; 32]),
                        bytes: 100,
                    },
                    dependencies: BTreeSet::new(),
                },
            )
        }));
        Self {
            scope: SourceScope::new(REPO),
            nodes,
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

async fn replication_fixture() -> (Arc<FaultStore>, Coordinator) {
    let store = Arc::new(FaultStore::default());
    let c = coordinator(store.clone());
    c.initialize_replica(
        EpochId::new([2; 16]),
        b"initial".to_vec(),
        ReplicaId::new([7; 16]),
        WriterEpoch::new([8; 16]),
    )
    .await
    .unwrap();
    (store, c)
}

#[tokio::test]
async fn authored_publications_atomically_advance_one_origin_stream() {
    let (_, c) = replication_fixture().await;
    let first = request(40, &[0], 1);
    let first_receipt = committed(c.publish(&first).await.unwrap());
    let first_snapshot = c.read().await.unwrap();
    let first_root = c
        .read_replication_root(first_snapshot.manifest().replication_root.unwrap())
        .await
        .unwrap();
    assert_eq!(first_root.local_sequence, 1);
    let first_key = OriginKeyV1 {
        stream: first_root.local_stream,
        sequence: 1,
    };
    let first_origin_receipt = c
        .origin_receipt(&first_snapshot, first_key)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        first_origin_receipt.accepted.position,
        first_receipt.accepted.position
    );
    let first_origin = c.origin_publication(first_origin_receipt).await.unwrap();
    assert_eq!(first_origin.origin_key, first_key);
    assert_eq!(first_origin.previous_origin_digest, None);
    assert!(first_origin.causal_frontier.is_empty());
    assert_eq!(first_origin.published_operations, BTreeSet::from([op(1)]));
    assert_eq!(
        first_origin.changed_domains,
        BTreeSet::from([OriginDomainV1::Code])
    );

    assert_eq!(committed(c.publish(&first).await.unwrap()), first_receipt);
    assert_eq!(c.read().await.unwrap().manifest().published, WalPosition(1));

    let second = request(41, &[1], 2);
    committed(c.publish(&second).await.unwrap());
    let second_snapshot = c.recover(100).await.unwrap();
    let second_root = c
        .read_replication_root(second_snapshot.manifest().replication_root.unwrap())
        .await
        .unwrap();
    assert_eq!(second_root.local_sequence, 2);
    let second_origin = c
        .origin_publication(
            c.origin_receipt(
                &second_snapshot,
                OriginKeyV1 {
                    stream: second_root.local_stream,
                    sequence: 2,
                },
            )
            .await
            .unwrap()
            .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        second_origin.previous_origin_digest,
        first_root.local_terminal_digest
    );
    assert_eq!(
        second_origin.causal_frontier,
        vec![OriginPrefixV1 {
            stream: first_root.local_stream,
            sequence: 1,
            terminal_digest: first_root.local_terminal_digest.unwrap(),
        }]
    );
}

#[tokio::test]
async fn failed_manifest_candidate_does_not_consume_origin_sequence() {
    let (store, c) = replication_fixture().await;
    store.arm("manifest.json", Fault::Before);
    assert!(matches!(
        c.publish(&request(42, &[0], 1)).await.unwrap(),
        PublishOutcome::OutcomeUnknown { .. }
    ));
    let unchanged = c.read().await.unwrap();
    let root = c
        .read_replication_root(unchanged.manifest().replication_root.unwrap())
        .await
        .unwrap();
    assert_eq!(root.local_sequence, 0);
    assert!(root.accepted_origins.is_none());

    committed(c.publish(&request(43, &[0], 2)).await.unwrap());
    let accepted = c.read().await.unwrap();
    let root = c
        .read_replication_root(accepted.manifest().replication_root.unwrap())
        .await
        .unwrap();
    assert_eq!(root.local_sequence, 1);
    assert!(
        c.origin_receipt(
            &accepted,
            OriginKeyV1 {
                stream: root.local_stream,
                sequence: 1,
            },
        )
        .await
        .unwrap()
        .is_some()
    );
}

#[derive(serde::Serialize)]
struct LegacyManifest {
    format: u32,
    repository: RepositoryId,
    epoch: EpochId,
    revision: u64,
    published: WalPosition,
    heads: BTreeSet<OperationId>,
    tail: Option<RecordRef>,
    inventory: Option<RecordRef>,
    receipts: Option<RecordRef>,
    history: Option<RecordRef>,
    checkpoint: Option<RecordRef>,
    initial_settings: RecordRef,
    settings: RecordRef,
}

async fn legacy_fixture() -> (Arc<FaultStore>, Coordinator) {
    let (store, coordinator) = fixture().await;
    let current = coordinator.read().await.unwrap();
    let manifest = current.manifest();
    let legacy = LegacyManifest {
        format: 1,
        repository: manifest.repository,
        epoch: manifest.epoch,
        revision: manifest.revision,
        published: manifest.published,
        heads: manifest.heads.clone(),
        tail: manifest.tail,
        inventory: manifest.inventory,
        receipts: manifest.receipts,
        history: manifest.history,
        checkpoint: manifest.checkpoint,
        initial_settings: manifest.initial_settings,
        settings: manifest.settings,
    };
    let bytes = serde_json::to_vec(&legacy).unwrap();
    let key = coordinator.manifest_key();
    let version = store.head(&key).await.unwrap().version;
    let upload = StagedUpload::from_bytes(&bytes, 128 * 1024).await.unwrap();
    assert!(matches!(
        store
            .put(&key, Condition::Matches(version), &upload)
            .await
            .unwrap(),
        WriteOutcome::Applied(_)
    ));
    assert_eq!(coordinator.read().await.unwrap().manifest().format, 1);
    (store, coordinator)
}

#[tokio::test]
async fn domain_publications_preserve_roots_without_staling_code() {
    let (_, c) = fixture().await;
    let initial = c.read().await.unwrap();
    let code_expected = initial.manifest().snapshot();
    let review_initial = initial.manifest().domain_state(Domain::Review).unwrap();
    let agent_initial = initial.manifest().domain_state(Domain::Agent).unwrap();
    assert_eq!(initial.manifest().format, 2);
    assert_eq!(initial.manifest().code_state_version, 0);

    let agent_page = c
        .stage_agent_object(&AgentObjectV1::IndexPage(AgentIndexPageV1::Leaf {
            prefix: String::new(),
            entries: BTreeMap::new(),
        }))
        .await
        .unwrap();
    let agent_root = c
        .stage_agent_root(&AgentRootV1 {
            sessions: Some(agent_page),
            ..AgentRootV1::empty()
        })
        .await
        .unwrap();
    let agent = DomainRequest::new(
        PrincipalId::new([3; 16]),
        RequestId::new([90; 16]),
        PublicationId::new([90; 16]),
        Domain::Agent,
        agent_initial,
        agent_root,
        b"agent accepted".to_vec(),
    )
    .unwrap();
    committed(c.publish_domain(&agent).await.unwrap());

    let after_agent = c.read().await.unwrap();
    assert_eq!(after_agent.manifest().published, WalPosition(1));
    assert_eq!(after_agent.manifest().code_state_version, 0);
    assert_eq!(
        after_agent.manifest().domain_state(Domain::Review).unwrap(),
        review_initial
    );

    let code = Request {
        transition: Some(Transition::Strict {
            expected: code_expected,
            heads: BTreeSet::from([op(1)]),
        }),
        ..request(91, &[0], 1)
    };
    committed(c.publish(&code).await.unwrap());
    let after_code = c.read().await.unwrap();
    assert_eq!(after_code.manifest().published, WalPosition(2));
    assert_eq!(after_code.manifest().code_state_version, 1);
    assert_eq!(
        after_code
            .manifest()
            .domain_state(Domain::Agent)
            .unwrap()
            .root,
        agent_root
    );

    let review_root = c
        .stage_review_root(&jj_review::ReviewRootV1::empty())
        .await
        .unwrap();
    let review = DomainRequest::new(
        PrincipalId::new([3; 16]),
        RequestId::new([92; 16]),
        PublicationId::new([92; 16]),
        Domain::Review,
        review_initial,
        review_root,
        b"review accepted".to_vec(),
    )
    .unwrap();
    committed(c.publish_domain(&review).await.unwrap());
    let recovered = c.recover(100).await.unwrap();
    assert_eq!(recovered.manifest().published, WalPosition(3));
    assert_eq!(recovered.manifest().code_state_version, 1);
    assert_eq!(
        recovered
            .manifest()
            .domain_state(Domain::Agent)
            .unwrap()
            .root,
        agent_root
    );
    assert_eq!(
        recovered
            .manifest()
            .domain_state(Domain::Review)
            .unwrap()
            .root,
        review_root
    );
}

#[tokio::test]
async fn agent_roots_require_complete_typed_dependency_closures() {
    let (_, c) = fixture().await;
    let missing = AgentObjectRef::new(AgentObjectKind::IndexPage, AgentObjectId::new([251; 32]));
    let root = AgentRootV1 {
        sessions: Some(missing),
        ..AgentRootV1::empty()
    };
    assert!(matches!(
        c.stage_agent_root(&root).await,
        Err(wal::Error::Storage(jj_store_object::Error::NotFound))
    ));

    let stored = c
        .stage_agent_object(&AgentObjectV1::IndexPage(AgentIndexPageV1::Leaf {
            prefix: String::new(),
            entries: BTreeMap::new(),
        }))
        .await
        .unwrap();
    let wrong_kind = AgentObjectRef::new(AgentObjectKind::SessionSnapshot, stored.id);
    let key = jj_agent::AgentCatalogKey::derive("session", &[7; 16]).unwrap();
    let parent = AgentObjectV1::IndexPage(AgentIndexPageV1::Leaf {
        prefix: String::new(),
        entries: BTreeMap::from([(key, wrong_kind)]),
    });
    assert!(matches!(
        c.stage_agent_object(&parent).await,
        Err(wal::Error::Corrupt("agent object kind"))
    ));
}

#[tokio::test]
async fn agent_publication_retains_exact_native_association_dependencies() {
    let (_, c) = fixture().await;
    let initial = c.read().await.unwrap();
    let association = CodeAssociationV1 {
        schema_version: 1,
        association_id: AssociationId::new([80; 16]),
        checkpoint_id: Some(AgentCheckpointId::new([81; 16])),
        session_id: AgentSessionId::new([82; 16]),
        turn_id: None,
        tool_call_ids: BTreeSet::new(),
        scope_id: None,
        relation: AssociationRelation::ObservedDuringInterval,
        before: CodeAnchorV1 {
            operations: BTreeSet::from([op(125)]),
            commits: BTreeSet::from([commit(126)]),
        },
        after: CodeAnchorV1 {
            operations: BTreeSet::from([op(127)]),
            commits: BTreeSet::from([commit(128)]),
        },
        changes: BTreeSet::new(),
        paths: Vec::new(),
        granularity: AssociationGranularity::SnapshotInterval,
        evidence_method: EvidenceMethod::HookBoundary,
        source_trust: SourceTrust::LocalUserHook,
        exclusive_write_scope: false,
        evidence: Vec::new(),
        coverage: CoverageV1 {
            completeness: CompletenessV1::Complete,
            gaps: 0,
            redacted_fields: 0,
            truncated: false,
        },
        created_at_millis: 1_790_000_000_000,
        supersedes: None,
        correction_reason: None,
    };
    let mut missing = association.clone();
    missing.association_id = AssociationId::new([84; 16]);
    missing.before = CodeAnchorV1 {
        operations: BTreeSet::from([op(250)]),
        commits: BTreeSet::new(),
    };
    missing.after = CodeAnchorV1::default();
    let association = c
        .stage_agent_object(&AgentObjectV1::Association(association))
        .await
        .unwrap();
    let page = c
        .stage_agent_object(&AgentObjectV1::IndexPage(AgentIndexPageV1::Leaf {
            prefix: String::new(),
            entries: BTreeMap::from([(
                AgentCatalogKey::derive("association", &[80; 16]).unwrap(),
                association,
            )]),
        }))
        .await
        .unwrap();
    let root = c
        .stage_agent_root(&AgentRootV1 {
            associations: Some(page),
            ..AgentRootV1::empty()
        })
        .await
        .unwrap();
    let request = DomainRequest::new(
        PrincipalId::new([3; 16]),
        RequestId::new([83; 16]),
        PublicationId::new([83; 16]),
        Domain::Agent,
        initial.manifest().domain_state(Domain::Agent).unwrap(),
        root,
        Vec::new(),
    )
    .unwrap();
    committed(c.publish_domain(&request).await.unwrap());

    let selected = c.read().await.unwrap();
    let retained: BTreeSet<_> = c
        .inventory(&selected, 128)
        .await
        .unwrap()
        .into_iter()
        .map(|node| node.key)
        .collect();
    for key in [
        ObjectKey::Operation(op(125)),
        ObjectKey::Commit(commit(126)),
        ObjectKey::Operation(op(127)),
        ObjectKey::Commit(commit(128)),
    ] {
        assert!(retained.contains(&key));
    }
    assert_eq!(c.recover(128).await.unwrap().manifest().published.0, 1);

    assert!(matches!(
        c.stage_agent_object(&AgentObjectV1::Association(missing))
            .await,
        Err(wal::Error::Inventory(
            wal::inventory::InventoryError::Source(_)
        ))
    ));
}

#[tokio::test]
async fn agent_delta_publications_rebase_disjoint_keys_and_pin_old_roots() {
    let (_, c) = fixture().await;
    let initial = c.read().await.unwrap();
    let initial_agent = initial.manifest().domain_state(Domain::Agent).unwrap();
    let stage_session = |byte| {
        AgentObjectV1::SessionSnapshot(AgentSessionSnapshotV1 {
            schema_version: 1,
            session_id: AgentSessionId::new([byte; 16]),
            integration: IntegrationId::Codex,
            state: SessionState::Active,
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
        })
    };
    let first = c.stage_agent_object(&stage_session(90)).await.unwrap();
    let second = c.stage_agent_object(&stage_session(91)).await.unwrap();
    let conflicting = c.stage_agent_object(&stage_session(92)).await.unwrap();
    let first_key = AgentCatalogKey::derive("session", &[90; 16]).unwrap();
    let second_key = AgentCatalogKey::derive("session", &[91; 16]).unwrap();
    let delta = |key, replacement| AgentDeltaV1 {
        schema_version: 1,
        edits: vec![AgentCatalogEditV1 {
            catalog: AgentCatalogKind::Sessions,
            key,
            expected: None,
            replacement: Some(replacement),
        }],
    };
    let first_request = AgentDeltaRequest::new(
        PrincipalId::new([3; 16]),
        RequestId::new([93; 16]),
        PublicationId::new([93; 16]),
        delta(first_key, first),
        b"first session".to_vec(),
    )
    .unwrap();
    let first_receipt = committed(c.publish_agent_delta(&first_request).await.unwrap());
    let first_root = c
        .read()
        .await
        .unwrap()
        .manifest()
        .domain_state(Domain::Agent)
        .unwrap()
        .root;

    let second_request = AgentDeltaRequest::new(
        PrincipalId::new([3; 16]),
        RequestId::new([94; 16]),
        PublicationId::new([94; 16]),
        delta(second_key, second),
        b"second session".to_vec(),
    )
    .unwrap();
    committed(c.publish_agent_delta(&second_request).await.unwrap());
    let current = c.read().await.unwrap();
    let current_agent = current.manifest().domain_state(Domain::Agent).unwrap();
    assert_eq!(current_agent.version, initial_agent.version + 2);

    let pinned_first = c.agent_store(first_root).await.unwrap();
    assert_eq!(
        pinned_first
            .lookup(AgentCatalogKind::Sessions, first_key)
            .await
            .unwrap(),
        Some(first)
    );
    assert_eq!(
        pinned_first
            .lookup(AgentCatalogKind::Sessions, second_key)
            .await
            .unwrap(),
        None
    );
    let pinned_current = c.agent_store(current_agent.root).await.unwrap();
    assert_eq!(
        pinned_current
            .lookup(AgentCatalogKind::Sessions, first_key)
            .await
            .unwrap(),
        Some(first)
    );
    assert_eq!(
        pinned_current
            .lookup(AgentCatalogKind::Sessions, second_key)
            .await
            .unwrap(),
        Some(second)
    );
    assert_eq!(
        pinned_current.read_object(first).await.unwrap(),
        stage_session(90)
    );
    assert!(matches!(
        pinned_current.read_object(conflicting).await,
        Err(wal::Error::Stale)
    ));
    assert_eq!(
        committed(c.publish_agent_delta(&first_request).await.unwrap()),
        first_receipt
    );

    let conflict = AgentDeltaRequest::new(
        PrincipalId::new([3; 16]),
        RequestId::new([95; 16]),
        PublicationId::new([95; 16]),
        delta(first_key, conflicting),
        Vec::new(),
    )
    .unwrap();
    assert!(matches!(
        c.publish_agent_delta(&conflict).await,
        Err(wal::Error::Stale)
    ));
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

fn enabled(outcome: CollaborationOutcome) -> Receipt {
    match outcome {
        CollaborationOutcome::Enabled(receipt) => receipt,
        other => panic!("expected enabled: {other:?}"),
    }
}

#[tokio::test]
async fn legacy_repository_enables_collaboration_without_rewriting_code_history() {
    let (store, c) = legacy_fixture().await;
    committed(c.publish(&request(70, &[0], 1)).await.unwrap());
    let legacy = c.read().await.unwrap();
    let code_expected = legacy.manifest().snapshot();
    assert_eq!(legacy.manifest().format, 1);
    assert_eq!(legacy.manifest().published, WalPosition(1));
    assert_eq!(legacy.manifest().code_state_version, 0);

    let enable = CollaborationRequest::new(
        PrincipalId::new([3; 16]),
        RequestId::new([71; 16]),
        PublicationId::new([71; 16]),
        b"collaboration enabled".to_vec(),
    )
    .unwrap();
    let receipt = enabled(c.enable_collaboration(&enable).await.unwrap());
    assert_eq!(receipt.accepted.position, WalPosition(2));
    assert_eq!(receipt.accepted.code_state_version, 0);
    assert_eq!(
        enabled(c.enable_collaboration(&enable).await.unwrap()),
        receipt
    );

    let enabled_snapshot = c.read().await.unwrap();
    let manifest = enabled_snapshot.manifest();
    assert_eq!(manifest.format, 2);
    assert_eq!(manifest.domain_history_start, 2);
    assert_eq!(manifest.heads, BTreeSet::from([op(1)]));
    assert_eq!(manifest.code_state_version, 0);
    let domains = manifest.domains().unwrap();
    assert_eq!(domains.agent.version, 0);
    assert_eq!(domains.review.version, 0);
    let another = CollaborationRequest::new(
        PrincipalId::new([3; 16]),
        RequestId::new([72; 16]),
        PublicationId::new([72; 16]),
        Vec::new(),
    )
    .unwrap();
    assert_eq!(
        c.enable_collaboration(&another).await.unwrap(),
        CollaborationOutcome::AlreadyEnabled(domains)
    );

    let code = Request {
        transition: Some(Transition::Strict {
            expected: code_expected,
            heads: BTreeSet::from([op(2)]),
        }),
        ..request(73, &[1], 2)
    };
    committed(c.publish(&code).await.unwrap());
    let recovered = c.recover(100).await.unwrap();
    assert_eq!(recovered.manifest().published, WalPosition(3));
    assert_eq!(recovered.manifest().code_state_version, 1);
    let selected = recovered
        .selected_records(store, WalPosition(0), 10)
        .await
        .unwrap();
    assert_eq!(selected.len(), 3);
    assert!(selected[0].transaction().before_domains.is_none());
    assert_eq!(
        selected[1].transaction().kind,
        SemanticKind::CollaborationEnabled
    );
    assert!(selected[1].transaction().before_domains.is_none());
    assert_eq!(selected[1].transaction().after_domains, Some(domains));
    assert_eq!(selected[2].transaction().before_domains, Some(domains));
}

#[tokio::test]
async fn collaboration_enablement_resolves_unknown_manifest_outcomes() {
    for fault in [Fault::Before, Fault::After] {
        let (store, c) = legacy_fixture().await;
        let request = CollaborationRequest::new(
            PrincipalId::new([3; 16]),
            RequestId::new([74; 16]),
            PublicationId::new([74; 16]),
            Vec::new(),
        )
        .unwrap();
        store.arm("/manifest.json", fault);
        let first = c.enable_collaboration(&request).await.unwrap();
        if matches!(fault, Fault::Before) {
            assert!(matches!(first, CollaborationOutcome::OutcomeUnknown { .. }));
        } else {
            assert!(matches!(first, CollaborationOutcome::Enabled(_)));
        }
        let retry = c.enable_collaboration(&request).await.unwrap();
        assert!(matches!(retry, CollaborationOutcome::Enabled(_)));
        assert_eq!(c.recover(100).await.unwrap().manifest().format, 2);
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

#[path = "publication/application.rs"]
mod application_acceptance;

#[tokio::test]
async fn selected_reader_keeps_exact_committed_refs_and_inventory_across_checkpoint() {
    let (store, c) = fixture().await;
    store.arm("manifest.json", Fault::Before);
    assert!(matches!(
        c.publish(&request(1, &[0], 1)).await.unwrap(),
        PublishOutcome::OutcomeUnknown { .. }
    ));
    let orphan = store
        .writes
        .lock()
        .unwrap()
        .iter()
        .find(|key| key.contains("/wal/"))
        .unwrap()
        .clone();
    committed(c.publish(&request(1, &[0], 1)).await.unwrap());
    let pinned = c.read().await.unwrap();
    let first = pinned
        .selected_records(store.clone(), WalPosition(0), 1)
        .await
        .unwrap();
    assert_eq!(first.len(), 1);
    assert_eq!(Some(first[0].reference()), pinned.manifest().tail);
    assert_ne!(first[0].reference().key(REPO).unwrap().as_str(), orphan);
    assert_eq!(first[0].transaction().after.position, WalPosition(1));
    committed(c.publish(&request(2, &[1], 2)).await.unwrap());
    let checkpoint = c.checkpoint().await.unwrap();
    let earlier = pinned
        .selected_records(store.clone(), WalPosition(0), 32)
        .await
        .unwrap();
    assert_eq!(earlier.len(), 1);
    assert_eq!(earlier[0].reference(), first[0].reference());
    let later = checkpoint
        .selected_records(store.clone(), WalPosition(1), 1)
        .await
        .unwrap();
    assert_eq!(later[0].transaction().previous, Some(first[0].reference()));
    assert_eq!(Some(later[0].reference()), checkpoint.manifest().tail);
    let inventory = first[0].inventory(store.clone(), 100).await.unwrap();
    assert!(
        inventory
            .iter()
            .any(|n| n.key == ObjectKey::Operation(op(1)))
    );
    assert!(
        !inventory
            .iter()
            .any(|n| n.key == ObjectKey::Operation(op(2)))
    );
    assert!(
        checkpoint
            .selected_records(store.clone(), WalPosition(3), 1)
            .await
            .is_err()
    );
    assert!(
        checkpoint
            .selected_records(store.clone(), WalPosition(0), 1025)
            .await
            .is_err()
    );
    assert!(first[0].inventory(store, 1).await.is_err());
}
