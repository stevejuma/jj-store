//! Atomic selection of explicit capture assignments and durable replay.
use jj_agent as a;
use jj_collab::*;
use jj_store_model as m;
use jj_store_object::{
    ByteRange, Condition, Download, Key, Metadata, ObjectStore, StagedUpload, WriteOutcome,
    memory::MemoryStore,
};
use jj_store_wal::{
    self as wal,
    inventory::{DependencySource, Representation, SourceScope, VerifiedNode},
    publication::*,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};
const REPO: m::RepositoryId = m::RepositoryId::new([1; 16]);
const ACTOR: m::PrincipalId = m::PrincipalId::new([2; 16]);
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
            dependencies: BTreeSet::new(),
        })
    }
}
#[derive(Default)]
struct Guard {
    deny_thread: AtomicBool,
}
#[async_trait::async_trait]
impl PublicationGuard for Guard {
    async fn authorize(&self, _: &ManifestSnapshot, _: &Request) -> wal::Result<()> {
        Ok(())
    }
    async fn authorize_domain(&self, _: &ManifestSnapshot, r: &DomainRequest) -> wal::Result<()> {
        if r.domain == Domain::Thread && self.deny_thread.load(Ordering::SeqCst) {
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
#[derive(Default)]
struct Store {
    memory: MemoryStore,
    unknown: AtomicBool,
    block: AtomicBool,
    blocked: tokio::sync::Notify,
    release: tokio::sync::Notify,
}
#[async_trait::async_trait]
impl ObjectStore for Store {
    async fn head(&self, k: &Key) -> jj_store_object::Result<Metadata> {
        self.memory.head(k).await
    }
    async fn get(&self, k: &Key, r: Option<ByteRange>) -> jj_store_object::Result<Download> {
        self.memory.get(k, r).await
    }
    async fn put(
        &self,
        k: &Key,
        c: Condition,
        u: &StagedUpload,
    ) -> jj_store_object::Result<WriteOutcome> {
        if k.as_str().ends_with("/manifest.json") && self.block.swap(false, Ordering::SeqCst) {
            self.blocked.notify_one();
            self.release.notified().await;
        }
        if k.as_str().ends_with("/manifest.json") && self.unknown.swap(false, Ordering::SeqCst) {
            return Ok(WriteOutcome::Unknown);
        }
        self.memory.put(k, c, u).await
    }
}

#[tokio::test]
async fn capture_cas_race_reapplies_disjoint_changes_but_rejects_changed_assignment() {
    let (store, _, c) = fixture().await;
    let c = Arc::new(c);
    let first = thread(&c, 51).await;
    let second = thread(&c, 52).await;
    let req = bind(capture(&c, 53).await, &first, None);
    store.block.store(true, Ordering::SeqCst);
    let task = tokio::spawn({
        let c = c.clone();
        let req = req.clone();
        async move { c.publish_agent_delta(&req).await }
    });
    tokio::time::timeout(std::time::Duration::from_secs(5), store.blocked.notified())
        .await
        .unwrap();
    let disjoint = thread(&c, 54).await;
    store.release.notify_one();
    assert!(matches!(
        task.await.unwrap().unwrap(),
        PublishOutcome::Committed(_)
    ));
    let selected = c.read().await.unwrap();
    let threads = c
        .native_thread_store(selected.manifest().thread_root.unwrap())
        .await
        .unwrap();
    assert_eq!(threads.state.threads[&disjoint.thread_id], disjoint.version);
    let session_id = a::AgentSessionId::new([53; 16]);
    let primary = threads.state.assignments[&session_id];
    let snapshot = req.delta.edits[0].replacement.unwrap();
    let a::AgentObjectV1::SessionSnapshot(mut session) =
        c.read_agent_object(snapshot).await.unwrap()
    else {
        panic!()
    };
    session.state = a::SessionState::Idle;
    let replacement = c
        .stage_agent_object(&a::AgentObjectV1::SessionSnapshot(session))
        .await
        .unwrap();
    let next = AgentDeltaRequest::new(
        ACTOR,
        m::RequestId::new([55; 16]),
        m::PublicationId::new([55; 16]),
        a::AgentDeltaV1 {
            schema_version: 1,
            edits: vec![a::AgentCatalogEditV1 {
                expected: Some(snapshot),
                replacement: Some(replacement),
                ..req.delta.edits[0].clone()
            }],
        },
        vec![55],
    )
    .unwrap()
    .with_thread_binding(
        ThreadBindingV1 {
            thread_id: second.thread_id,
            observed_thread: second.version,
            observed_assignment: Some(primary),
        },
        124,
    )
    .unwrap();
    store.block.store(true, Ordering::SeqCst);
    let task = tokio::spawn({
        let c = c.clone();
        async move { c.publish_agent_delta(&next).await }
    });
    tokio::time::timeout(std::time::Duration::from_secs(5), store.blocked.notified())
        .await
        .unwrap();
    // Change only the assignment's source Thread. The destination version stays
    // unchanged, so a stale Thread check alone cannot catch this race.
    command(
        &c,
        56,
        ThreadCommandV1::Link {
            thread_id: first.thread_id,
            expected_thread: threads.state.threads[&first.thread_id],
            observed_primary: Some(primary),
            link_id: ThreadLinkId::new([56; 16]),
            target: ThreadLinkTargetV1::Session {
                session_id,
                snapshot,
                primary: true,
            },
        },
    )
    .await;
    let changed = c.read().await.unwrap();
    store.release.notify_one();
    assert!(matches!(task.await.unwrap(), Err(wal::Error::Stale)));
    assert_eq!(c.read().await.unwrap().digest(), changed.digest());
    let agents = c
        .agent_store(changed.manifest().agent_root.unwrap())
        .await
        .unwrap();
    assert_eq!(
        agents
            .lookup(a::AgentCatalogKind::Sessions, req.delta.edits[0].key)
            .await
            .unwrap(),
        Some(snapshot)
    );
    assert_eq!(
        c.native_thread_store(changed.manifest().thread_root.unwrap())
            .await
            .unwrap()
            .state
            .threads[&second.thread_id],
        second.version
    );
}
fn coordinator(store: Arc<Store>, guard: Arc<Guard>) -> Coordinator {
    Coordinator::new(store, REPO, Arc::new(Source), guard, Limits::default()).unwrap()
}
async fn diagnostic_capture(c: &Coordinator, n: u8) -> AgentDeltaRequest {
    let root = match c.read().await.unwrap().manifest().agent_root {
        Some(root) => root,
        None => c.stage_agent_root(&a::AgentRootV1::empty()).await.unwrap(),
    };
    let base = c.agent_store(root).await.unwrap();
    let batch = a::SanitizedObservationBatchV1 {
        schema_version: 1,
        batch_id: a::CaptureBatchId::new([n; 16]),
        integration: a::IntegrationId::Codex,
        source_instance_id: a::SourceInstanceId::new([n; 16]),
        sanitizer_version: "agent-redaction@0.1.0".into(),
        representation: a::SanitizedRepresentation::SanitizedNative,
        observations: vec![a::SanitizedObservationV1 {
            observation_id: a::ObservationId::new([n; 16]),
            source_sequence: 1,
            occurred_at_millis: None,
            source_kind: a::SourceKind::Hook,
            source_trust: a::SourceTrust::LocalUserHook,
            payload: a::SanitizedPayloadV1::new(
                a::SanitizedMediaType::Json,
                br#"{"future_record":true}"#,
            )
            .unwrap(),
        }],
    };
    let delta = c
        .stage_agent_capture(&base, &a::NormalizerProfileV1::codex_0_156_1(), &batch, 123)
        .await
        .unwrap();
    AgentDeltaRequest::new(
        ACTOR,
        m::RequestId::new([n; 16]),
        m::PublicationId::new([n; 16]),
        delta,
        vec![n],
    )
    .unwrap()
}
#[tokio::test]
async fn bound_diagnostic_gap_selects_atomic_evidence_and_preserves_unknown_restart_receipt() {
    let (store, guard, c) = fixture().await;
    let t = thread(&c, 61).await;
    let request = bind(diagnostic_capture(&c, 62).await, &t, None);
    let before = c.read().await.unwrap();
    store.unknown.store(true, Ordering::SeqCst);
    assert!(matches!(
        c.publish_agent_delta(&request).await.unwrap(),
        PublishOutcome::OutcomeUnknown { .. }
    ));
    assert_eq!(c.read().await.unwrap().digest(), before.digest());
    let accepted = c.publish_agent_delta(&request).await.unwrap();
    let selected = c.read().await.unwrap();
    assert_eq!(
        selected.manifest().published.0,
        before.manifest().published.0 + 1
    );
    assert_eq!(selected.manifest().heads, before.manifest().heads);
    let agents = c
        .agent_store(selected.manifest().agent_root.unwrap())
        .await
        .unwrap();
    assert!(
        agents
            .list_sessions(None, 10)
            .await
            .unwrap()
            .selections
            .is_empty()
    );
    let threads = c
        .native_thread_store(selected.manifest().thread_root.unwrap())
        .await
        .unwrap();
    assert!(threads.state.assignments.is_empty());
    assert_eq!(threads.state.links.len(), 1);
    let CollabObjectV1::ThreadLink(link) = threads
        .graph
        .get(*threads.state.links.values().next().unwrap())
        .unwrap()
    else {
        panic!()
    };
    let ThreadLinkTargetV1::CaptureGap {
        source_state,
        event_batch,
        binding,
    } = &link.target
    else {
        panic!()
    };
    assert_eq!(binding, &request.thread_binding.as_ref().unwrap().binding);
    AgentReader::new(&agents)
        .validate_capture_gap(*source_state, *event_batch)
        .await
        .unwrap();
    assert_eq!(
        CollabObjectV1::ThreadLink(link.clone()).agent_dependencies(),
        BTreeSet::from([*source_state, *event_batch])
    );
    c.checkpoint().await.unwrap();
    let checkpointed = c.read().await.unwrap();
    let fresh = coordinator(store, guard);
    fresh.recover(100).await.unwrap();
    assert_eq!(fresh.publish_agent_delta(&request).await.unwrap(), accepted);
    assert_eq!(fresh.read().await.unwrap().digest(), checkpointed.digest());
}
#[tokio::test]
async fn stale_denied_or_misleading_bound_gap_selects_neither_root() {
    let (_, guard, c) = fixture().await;
    let t = thread(&c, 61).await;
    let request = bind(diagnostic_capture(&c, 62).await, &t, None);
    let before = c.read().await.unwrap();
    let mut misleading = request.clone();
    misleading.request_id = m::RequestId::new([63; 16]);
    misleading.publication_id = m::PublicationId::new([63; 16]);
    let source_edit = misleading
        .delta
        .edits
        .iter_mut()
        .find(|e| e.catalog == a::AgentCatalogKind::Sources)
        .unwrap();
    let old_source = source_edit.replacement.unwrap();
    let a::AgentObjectV1::SourceState(mut state) = c.read_agent_object(old_source).await.unwrap()
    else {
        panic!()
    };
    state.receipts[0].first_sequence = 2;
    state.receipts[0].last_sequence = 2;
    state.position = a::SourcePositionV1::NativeSequence { sequence: 2 };
    let new_source = c
        .stage_agent_object(&a::AgentObjectV1::SourceState(state))
        .await
        .unwrap();
    source_edit.replacement = Some(new_source);
    let projection_edit = misleading
        .delta
        .edits
        .iter_mut()
        .find(|e| e.catalog == a::AgentCatalogKind::Projections)
        .unwrap();
    let a::AgentObjectV1::ProjectionCatalog(mut view) = c
        .read_agent_object(projection_edit.replacement.unwrap())
        .await
        .unwrap()
    else {
        panic!()
    };
    view.source_states = vec![new_source];
    projection_edit.replacement = Some(
        c.stage_agent_object(&a::AgentObjectV1::ProjectionCatalog(view))
            .await
            .unwrap(),
    );
    assert!(matches!(
        c.publish_agent_delta(&misleading).await,
        Err(wal::Error::Corrupt(_))
    ));
    assert_eq!(c.read().await.unwrap().digest(), before.digest());
    guard.deny_thread.store(true, Ordering::SeqCst);
    assert!(matches!(
        c.publish_agent_delta(&request).await,
        Err(wal::Error::Denied)
    ));
    assert_eq!(c.read().await.unwrap().digest(), before.digest());
    guard.deny_thread.store(false, Ordering::SeqCst);
    command(
        &c,
        64,
        ThreadCommandV1::Rename {
            thread_id: t.thread_id,
            expected: t.version,
            title: "Changed".into(),
        },
    )
    .await;
    let renamed = c.read().await.unwrap();
    assert!(matches!(
        c.publish_agent_delta(&request).await,
        Err(wal::Error::Stale)
    ));
    assert_eq!(c.read().await.unwrap().digest(), renamed.digest());
}
async fn fixture() -> (Arc<Store>, Arc<Guard>, Coordinator) {
    let store = Arc::new(Store::default());
    let guard = Arc::new(Guard::default());
    let c = coordinator(store.clone(), guard.clone());
    c.initialize(m::EpochId::new([3; 16]), vec![])
        .await
        .unwrap();
    (store, guard, c)
}
async fn thread(c: &Coordinator, n: u8) -> ThreadCommandResultV1 {
    command(
        c,
        n,
        ThreadCommandV1::Create {
            thread_id: ThreadId::new([n; 16]),
            title: format!("Thread {n}"),
            baseline: None,
        },
    )
    .await
}
async fn command(c: &Coordinator, n: u8, cmd: ThreadCommandV1) -> ThreadCommandResultV1 {
    let req = CollabCommandRequestV1 {
        schema_version: 1,
        principal: ACTOR,
        request_id: m::RequestId::new([n; 16]),
        publication_id: m::PublicationId::new([n; 16]),
        command: CollabCommandV1::Thread { intent: cmd },
    };
    let PublishOutcome::Committed(r) = c.publish_collab_command(&req).await.unwrap() else {
        panic!()
    };
    let CollabCommandResultV1::Thread { result } =
        CollabCommandReceiptV1::decode(&r.result).unwrap().result
    else {
        panic!()
    };
    result
}
async fn capture(c: &Coordinator, n: u8) -> AgentDeltaRequest {
    let session_id = a::AgentSessionId::new([n; 16]);
    let value = a::AgentSessionSnapshotV1 {
        schema_version: 1,
        session_id,
        integration: a::IntegrationId::Codex,
        state: a::SessionState::Active,
        parent_session_id: None,
        evidence: vec![],
        event_batches: vec![],
        transcripts: vec![],
        tools: BTreeMap::new(),
        coverage: a::CoverageV1 {
            completeness: a::CompletenessV1::Unknown,
            gaps: 0,
            redacted_fields: 0,
            truncated: false,
        },
    };
    let replacement = c
        .stage_agent_object(&a::AgentObjectV1::SessionSnapshot(value))
        .await
        .unwrap();
    AgentDeltaRequest::new(
        ACTOR,
        m::RequestId::new([n; 16]),
        m::PublicationId::new([n; 16]),
        a::AgentDeltaV1 {
            schema_version: 1,
            edits: vec![a::AgentCatalogEditV1 {
                catalog: a::AgentCatalogKind::Sessions,
                key: a::AgentCatalogKey::derive("session", session_id.as_bytes()).unwrap(),
                expected: None,
                replacement: Some(replacement),
            }],
        },
        vec![n],
    )
    .unwrap()
}
fn bind(
    r: AgentDeltaRequest,
    t: &ThreadCommandResultV1,
    observed: Option<CollabObjectRef>,
) -> AgentDeltaRequest {
    r.with_thread_binding(
        ThreadBindingV1 {
            thread_id: t.thread_id,
            observed_thread: t.version,
            observed_assignment: observed,
        },
        123,
    )
    .unwrap()
}
#[tokio::test]
async fn bound_capture_selects_both_roots_once_preserves_lifecycle_and_replays_after_recovery() {
    let (store, guard, c) = fixture().await;
    let first = thread(&c, 11).await;
    let archived = command(
        &c,
        12,
        ThreadCommandV1::Lifecycle {
            thread_id: first.thread_id,
            expected: first.version,
            action: ThreadActionV1::Activate,
        },
    )
    .await;
    let archived = command(
        &c,
        13,
        ThreadCommandV1::Lifecycle {
            thread_id: first.thread_id,
            expected: archived.version,
            action: ThreadActionV1::Archive,
        },
    )
    .await;
    let req = bind(capture(&c, 14).await, &archived, None);
    let before = c.read().await.unwrap();
    let outcome = c.publish_agent_delta(&req).await.unwrap();
    let PublishOutcome::Committed(receipt) = &outcome else {
        panic!()
    };
    assert_eq!(receipt.result, vec![14]);
    let after = c.read().await.unwrap();
    assert_eq!(
        after.manifest().published.0,
        before.manifest().published.0 + 1
    );
    assert_eq!(
        after.manifest().agent_state_version,
        before.manifest().agent_state_version + 1
    );
    assert_eq!(
        after.manifest().thread_state_version,
        before.manifest().thread_state_version + 1
    );
    assert_eq!(after.manifest().heads, before.manifest().heads);
    assert_eq!(after.manifest().review_root, before.manifest().review_root);
    let threads = c
        .native_thread_store(after.manifest().thread_root.unwrap())
        .await
        .unwrap();
    let session = a::AgentSessionId::new([14; 16]);
    let assignment = threads.state.assignments[&session];
    let CollabObjectV1::ThreadLink(link) = threads.graph.get(assignment).unwrap() else {
        panic!()
    };
    assert!(link.active);
    assert_eq!(link.thread_id, first.thread_id);
    let CollabObjectV1::ThreadVersion(value) = threads
        .graph
        .get(threads.state.threads[&first.thread_id])
        .unwrap()
    else {
        panic!()
    };
    assert_eq!(value.status, jj_collab::ThreadStatusV1::Archived);
    let target_snapshot = match link.target {
        ThreadLinkTargetV1::Session {
            snapshot,
            primary: true,
            ..
        } => snapshot,
        _ => panic!(),
    };
    let agents = c
        .agent_store(after.manifest().agent_root.unwrap())
        .await
        .unwrap();
    assert_eq!(
        agents
            .lookup(a::AgentCatalogKind::Sessions, req.delta.edits[0].key)
            .await
            .unwrap(),
        Some(target_snapshot)
    );
    let later = thread(&c, 15).await;
    assert_ne!(later.version, archived.version);
    c.checkpoint().await.unwrap();
    let fresh = coordinator(store, guard);
    fresh.recover(100).await.unwrap();
    assert_eq!(fresh.publish_agent_delta(&req).await.unwrap(), outcome);
    assert_eq!(
        fresh.read().await.unwrap().manifest().published.0,
        after.manifest().published.0 + 1
    );
}
#[tokio::test]
async fn stale_or_denied_binding_cannot_select_any_agent_edits() {
    let (_, guard, c) = fixture().await;
    let t = thread(&c, 21).await;
    let req = bind(capture(&c, 22).await, &t, None);
    command(
        &c,
        23,
        ThreadCommandV1::Rename {
            thread_id: t.thread_id,
            expected: t.version,
            title: "Renamed".into(),
        },
    )
    .await;
    let before = c.read().await.unwrap();
    assert!(matches!(
        c.publish_agent_delta(&req).await,
        Err(wal::Error::Stale)
    ));
    assert_eq!(c.read().await.unwrap().digest(), before.digest());
    let current = c
        .native_thread_store(before.manifest().thread_root.unwrap())
        .await
        .unwrap()
        .state
        .threads[&t.thread_id];
    let mut req = req;
    req.thread_binding.as_mut().unwrap().binding.observed_thread = current;
    guard.deny_thread.store(true, Ordering::SeqCst);
    assert!(matches!(
        c.publish_agent_delta(&req).await,
        Err(wal::Error::Denied)
    ));
    assert_eq!(c.read().await.unwrap().digest(), before.digest());
    let agents = c
        .agent_store(before.manifest().agent_root.unwrap())
        .await
        .unwrap();
    assert!(
        agents
            .lookup(a::AgentCatalogKind::Sessions, req.delta.edits[0].key)
            .await
            .unwrap()
            .is_none()
    );
}
#[tokio::test]
async fn unknown_binding_retains_identity_and_changed_retry_is_rejected() {
    let (store, _, c) = fixture().await;
    let t = thread(&c, 31).await;
    let req = bind(capture(&c, 32).await, &t, None);
    let before = c.read().await.unwrap();
    store.unknown.store(true, Ordering::SeqCst);
    assert_eq!(
        c.publish_agent_delta(&req).await.unwrap(),
        PublishOutcome::OutcomeUnknown {
            request_id: req.request_id,
            publication_id: req.publication_id
        }
    );
    assert_eq!(c.read().await.unwrap().digest(), before.digest());
    let accepted = c.publish_agent_delta(&req).await.unwrap();
    assert!(matches!(accepted, PublishOutcome::Committed(_)));
    let after = c.read().await.unwrap();
    let mut changed = req.clone();
    changed.thread_binding.as_mut().unwrap().accepted_at_millis += 1;
    assert!(matches!(
        c.publish_agent_delta(&changed).await,
        Err(wal::Error::IntentConflict)
    ));
    assert_eq!(c.read().await.unwrap().digest(), after.digest());
    assert_eq!(c.publish_agent_delta(&req).await.unwrap(), accepted);
}
#[tokio::test]
async fn unlinked_encodings_are_unchanged_and_bindings_require_one_exact_session() {
    let (_, _, c) = fixture().await;
    let req = capture(&c, 42).await;
    #[derive(serde::Serialize)]
    struct Legacy<'a> {
        principal: m::PrincipalId,
        request_id: m::RequestId,
        publication_id: m::PublicationId,
        delta: &'a a::AgentDeltaV1,
        result: &'a Vec<u8>,
    }
    let legacy = Legacy {
        principal: req.principal,
        request_id: req.request_id,
        publication_id: req.publication_id,
        delta: &req.delta,
        result: &req.result,
    };
    assert_eq!(
        serde_json::to_vec(&req).unwrap(),
        serde_json::to_vec(&legacy).unwrap()
    );
    let t = thread(&c, 41).await;
    let mut two = req.clone();
    two.delta.edits.extend(capture(&c, 43).await.delta.edits);
    assert!(
        two.with_thread_binding(
            ThreadBindingV1 {
                thread_id: t.thread_id,
                observed_thread: t.version,
                observed_assignment: None
            },
            123
        )
        .is_err()
    );
    let mut wrong = bind(req.clone(), &t, None);
    wrong.delta.edits[0].key = a::AgentCatalogKey::derive("session", b"wrong").unwrap();
    let before = c.read().await.unwrap();
    assert!(c.publish_agent_delta(&wrong).await.is_err());
    assert_eq!(c.read().await.unwrap().digest(), before.digest());
    c.publish_agent_delta(&req).await.unwrap();
    let after = c.read().await.unwrap();
    assert_eq!(after.manifest().thread_root, before.manifest().thread_root);
    let threads = c
        .native_thread_store(after.manifest().thread_root.unwrap())
        .await
        .unwrap();
    assert!(threads.state.assignments.is_empty());
}
