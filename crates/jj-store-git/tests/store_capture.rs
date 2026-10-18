//! Production capture sink against a real Git carrier and fsynced test-host journal.
#[path = "support/capture_journal.rs"]
mod capture_journal;
use capture_journal::FileJournal;
use jj_agent::{AgentObjectV1, NativeCaptureBindingV1, SessionState};
use jj_collab::{ThreadCommandV1, ThreadId, ThreadStateV1};
use jj_store_conversations::{capture::*, privacy::Sanitizer, store_capture::*};
use jj_store_core::{
    engine::{AgentReader, AgentSnapshotRead},
    *,
};
use jj_store_git::{
    GitProvider, LocalRepository,
    metadata::{Limits, MetadataStore},
};
use jj_store_model::{PrincipalId, RepositoryId, RequestId};
use serde_json::{Value, json};
use std::{
    future::Future,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicU8, Ordering},
    },
    task::{Context, Poll, Waker},
};

#[derive(Default)]
struct Host(AtomicBool);
#[async_trait::async_trait]
impl StoreHost for Host {
    async fn authorize(&self, _: StoreKind, _: Access) -> jj_store_core::Result<()> {
        if self.0.load(Ordering::SeqCst) {
            Err(jj_store_core::Error::Denied)
        } else {
            Ok(())
        }
    }
    fn actor(&self, _: bool) -> jj_store_core::Result<Option<PrincipalId>> {
        Ok(Some(PrincipalId::new([11; 16])))
    }
    fn now_millis(&self) -> jj_store_core::Result<i64> {
        Ok(12345)
    }
}
struct Privacy;
impl Sanitizer for Privacy {
    fn sanitize(&self, value: Value) -> jj_store_conversations::Result<Value> {
        Ok(value)
    }
    fn replace_stream_text(
        &self,
        pending: &mut String,
        chunk: &str,
        complete: bool,
    ) -> jj_store_conversations::Result<String> {
        pending.push_str(chunk);
        Ok(if complete {
            std::mem::take(pending).replace("/private/fixture", "$WORKSPACE")
        } else {
            String::new()
        })
    }
}
#[derive(Default)]
struct Probe {
    events: Arc<Mutex<Vec<&'static str>>>,
    execute: AtomicU8,
    replay: AtomicU8,
}
struct Agent {
    inner: Arc<dyn AgentStore>,
    probe: Arc<Probe>,
}
impl StoreBinding for Agent {
    fn name(&self) -> &str {
        self.inner.name()
    }
    fn scope(&self) -> &PublicationScope {
        self.inner.scope()
    }
    fn publication_group(&self) -> Option<Arc<PublicationGroup>> {
        self.inner.publication_group()
    }
}
#[async_trait::async_trait]
impl AgentStore for Agent {
    async fn snapshot(&self) -> jj_store_core::Result<Pinned<Arc<dyn AgentSnapshotRead>>> {
        self.inner.snapshot().await
    }
    async fn replay(
        &self,
        request: &StoreRequest<AgentCommand>,
    ) -> jj_store_core::Result<Option<StoreOutcome<AgentResult>>> {
        self.probe.events.lock().unwrap().push("replay");
        if self.probe.replay.load(Ordering::SeqCst) == 1 {
            return Ok(None);
        }
        let mut outcome = self.inner.replay(request).await?;
        if self.probe.replay.load(Ordering::SeqCst) == 2
            && let Some(StoreOutcome::Committed(receipt)) = &mut outcome
        {
            receipt.version += 1;
        }
        Ok(outcome)
    }
    async fn status(&self, request: RequestId) -> jj_store_core::Result<StoreOutcome<AgentResult>> {
        self.probe.events.lock().unwrap().push("status");
        self.inner.status(request).await
    }
    async fn prepare_with_thread(
        &self,
        request: &StoreRequest<AgentCommand>,
        thread: &dyn ThreadStore,
    ) -> jj_store_core::Result<StoreRequest<AgentCommand>> {
        self.probe.events.lock().unwrap().push("prepare");
        self.inner.prepare_with_thread(request, thread).await
    }
    async fn execute(
        &self,
        request: &StoreRequest<AgentCommand>,
    ) -> jj_store_core::Result<StoreOutcome<AgentResult>> {
        self.inner.execute(request).await
    }
    async fn execute_with_thread(
        &self,
        request: &StoreRequest<AgentCommand>,
        thread: &dyn ThreadStore,
    ) -> jj_store_core::Result<StoreOutcome<AgentResult>> {
        self.probe.events.lock().unwrap().push("execute");
        if self.probe.execute.load(Ordering::SeqCst) == 2 {
            return Ok(StoreOutcome::Unknown {
                request_id: request.request_id,
            });
        }
        let result = self.inner.execute_with_thread(request, thread).await?;
        if self.probe.execute.load(Ordering::SeqCst) == 1 {
            return Err(jj_store_core::Error::OutcomeUnknown);
        }
        Ok(result)
    }
}
fn open(repository: LocalRepository, host: Arc<Host>, probe: Arc<Probe>) -> Arc<RepositoryStores> {
    let provider = GitProvider::new(repository);
    let mut factories = StoreFactories::empty();
    provider.register(&mut factories, "git").unwrap();
    let base = factories
        .loader()
        .open(
            &StoreConfig::all("git"),
            StoreContext::new(provider.scope(), host.clone()),
        )
        .unwrap();
    let mut wrapped = StoreFactories::empty();
    let inner = base.agent().unwrap().clone();
    wrapped
        .add_agent_store("git", move |_| {
            Ok(Arc::new(Agent {
                inner: inner.clone(),
                probe: probe.clone(),
            }))
        })
        .unwrap();
    let thread = base.thread().unwrap().clone();
    wrapped
        .add_thread_store("git", move |_| Ok(thread.clone()))
        .unwrap();
    let mut config = StoreConfig::default();
    config.enable(StoreKind::Agent, "git");
    config.enable(StoreKind::Thread, "git");
    Arc::new(
        wrapped
            .loader()
            .open(&config, StoreContext::new(provider.scope(), host))
            .unwrap(),
    )
}
fn committed<T>(outcome: StoreOutcome<T>) -> StoreReceipt<T> {
    match outcome {
        StoreOutcome::Committed(r) => r,
        _ => panic!("committed"),
    }
}
struct Fixture {
    directory: tempfile::TempDir,
    metadata: MetadataStore,
    repository: LocalRepository,
    stores: Arc<RepositoryStores>,
    journal: Arc<FileJournal>,
    context: ContextV1,
    probe: Arc<Probe>,
    host: Arc<Host>,
}
impl Fixture {
    async fn new() -> Self {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("repo.git");
        gix::init_bare(&path).unwrap();
        let metadata = MetadataStore::open(&path, Limits::default()).unwrap();
        let repository = LocalRepository::new(&path, metadata.clone());
        let host = Arc::new(Host::default());
        let probe = Arc::new(Probe::default());
        let stores = open(repository.clone(), host.clone(), probe.clone());
        let thread = ThreadId::new([7; 16]);
        let receipt = committed(
            stores
                .thread()
                .unwrap()
                .execute(&StoreRequest::new(
                    RequestId::new([7; 16]),
                    ThreadCommandV1::Create {
                        thread_id: thread,
                        title: "Durable native capture".into(),
                        baseline: None,
                    },
                ))
                .await
                .unwrap(),
        );
        let repository = repository.bound_to(receipt.repository);
        let stores = open(repository.clone(), host.clone(), probe.clone());
        let context = ContextV1 {
            repository: receipt.repository,
            thread,
            binding: NativeCaptureBindingV1 {
                scope: "vault".into(),
                worker: "worker".into(),
                request: "open".into(),
                target: "target".into(),
                window: "main".into(),
                admission: "generation".into(),
            },
            policy: "fixture/v1".into(),
        };
        let journal = Arc::new(FileJournal::open(
            directory.path().join("journal"),
            probe.events.clone(),
        ));
        probe.events.lock().unwrap().clear();
        Self {
            directory,
            metadata,
            repository,
            stores,
            journal,
            context,
            probe,
            host,
        }
    }
    fn sink(&self) -> StoreCapture {
        StoreCapture::new(
            self.context.clone(),
            self.stores.clone(),
            self.journal.clone(),
        )
        .unwrap()
    }
    fn owner(&self) -> NativeCapture {
        self.owner_with_sink(Box::new(self.sink()))
    }
    fn owner_with_sink(&self, sink: Box<dyn DurableCapture>) -> NativeCapture {
        let b = &self.context.binding;
        NativeCapture::new(
            Binding {
                scope: b.scope.clone(),
                worker: b.worker.clone(),
                request: b.request.clone(),
                target: b.target.clone(),
                window: b.window.clone(),
                admission: b.admission.clone(),
            },
            self.context.policy.clone(),
            Arc::new(Privacy),
            sink,
        )
        .unwrap()
    }
    async fn entry(&self, ordinal: u64) -> EntryV1 {
        self.journal
            .load(self.context.request(ordinal))
            .await
            .unwrap()
            .unwrap()
    }
    fn events(&self) -> Vec<&'static str> {
        self.probe.events.lock().unwrap().clone()
    }
    fn selected(&self) -> jj_store_git::repository::MetadataCommit {
        self.metadata.read().unwrap().unwrap().commit()
    }
}
fn opened() -> Value {
    json!({"version":3,"type":"opened","sessionId":"worker","requestId":"open","adapterSessionId":"provider","sequence":2})
}
fn input(frame: &Value, ordinal: u64) -> Input<'_> {
    Input {
        worker: "worker",
        ordinal,
        publication: if ordinal == 1 {
            Publication::Facts
        } else {
            Publication::Event {
                sequence: ordinal - 1,
            }
        },
        frame,
    }
}

#[tokio::test]
async fn production_sink_publishes_atomic_roots_and_durable_receipts_before_output() {
    let f = Fixture::new().await;
    let before = f.stores.agent().unwrap().snapshot().await.unwrap();
    let mut owner = f.owner();
    let frames = [
        opened(),
        json!({"version":3,"type":"activity","sessionId":"worker","requestId":"turn","sequence":81,"update":{"sessionUpdate":"agent_message_chunk","content":{"type":"text","text":"/private/fixture safe answer"}}}),
        json!({"version":3,"type":"record","sessionId":"worker","requestId":"turn","sequence":82,"record":{"status":"completed"}}),
    ];
    let mut bytes = Vec::new();
    for (i, frame) in frames.iter().enumerate() {
        let ordinal = i as u64 + 1;
        let batch = owner.capture(input(frame, ordinal)).await.unwrap();
        let entry = f.entry(ordinal).await;
        assert_eq!(entry.candidate.capture, batch.bytes());
        assert_eq!(entry.candidate.receipt(), batch.receipt());
        assert!(entry.confirmation.is_some());
        assert!(!String::from_utf8_lossy(batch.bytes()).contains("/private/fixture"));
        let agent = f.stores.agent().unwrap().snapshot().await.unwrap();
        let thread = f.stores.thread().unwrap().snapshot().await.unwrap();
        assert_eq!(agent.selection, thread.selection);
        bytes.push(batch.bytes().to_vec());
    }
    let events = f.events();
    assert_eq!(
        &events[..7],
        &[
            "begin", "status", "prepare", "prepared", "replay", "execute", "confirm"
        ]
    );
    assert!(
        AgentReader::new(before.data.as_ref())
            .list_sessions(None, 16)
            .await
            .unwrap()
            .selections
            .is_empty()
    );
    let reopened = open(f.repository.clone(), f.host.clone(), f.probe.clone());
    let pin = reopened.agent().unwrap().snapshot().await.unwrap();
    let rows = AgentReader::new(pin.data.as_ref())
        .list_sessions(None, 16)
        .await
        .unwrap();
    assert_eq!(rows.selections.len(), 1);
    let AgentObjectV1::SessionSnapshot(session) =
        pin.data.read_object(rows.selections[0].1).await.unwrap()
    else {
        panic!("session")
    };
    assert_eq!(session.state, SessionState::Idle);
    for (reference, bytes) in session.evidence.iter().zip(&bytes) {
        let AgentObjectV1::ObservationBatch(batch) =
            pin.data.read_object(*reference).await.unwrap()
        else {
            panic!("source")
        };
        assert_eq!(batch.observations[0].payload.bytes(), bytes);
    }
    let transcript = AgentReader::new(pin.data.as_ref())
        .transcript_range(
            session.transcripts[0],
            f.context.source(),
            1,
            None,
            16,
            4096,
        )
        .await
        .unwrap();
    assert_eq!(
        transcript.chunks[0].payload.bytes(),
        b"$WORKSPACE safe answer"
    );
    let thread = reopened.thread().unwrap().snapshot().await.unwrap();
    assert!(
        ThreadStateV1::read(thread.data.root.clone(), &thread.data.graph)
            .unwrap()
            .assignments
            .contains_key(&session.session_id)
    );
    let journal = Arc::new(FileJournal::open(
        f.directory.path().join("journal"),
        f.probe.events.clone(),
    ));
    let recovered = StoreCapture::new(f.context.clone(), reopened, journal).unwrap();
    let selected = f.selected();
    f.probe.events.lock().unwrap().clear();
    for ordinal in 1..=3 {
        assert_eq!(
            Some(recovered.recover(ordinal).await.unwrap()),
            f.entry(ordinal).await.confirmation
        );
    }
    assert_eq!(f.selected(), selected);
    assert!(
        !f.events()
            .iter()
            .any(|e| ["prepare", "execute"].contains(e))
    );
}

#[tokio::test]
async fn lost_commit_ack_and_journal_confirmation_recover_without_another_write() {
    for fault in [1, 3] {
        let f = Fixture::new().await;
        if fault == 1 {
            f.probe.execute.store(1, Ordering::SeqCst);
        } else {
            f.journal.failure.store(3, Ordering::SeqCst);
        }
        let before = f.selected();
        let mut owner = f.owner();
        assert_eq!(
            owner.capture(input(&opened(), 1)).await.err(),
            Some(CaptureError::Unknown)
        );
        assert_ne!(f.selected(), before);
        assert!(f.entry(1).await.prepared.is_some());
        assert!(f.entry(1).await.confirmation.is_none());
        assert_eq!(
            owner.capture(input(&opened(), 1)).await.err(),
            Some(CaptureError::Stopped)
        );
        f.probe.execute.store(0, Ordering::SeqCst);
        f.journal.failure.store(0, Ordering::SeqCst);
        f.probe.events.lock().unwrap().clear();
        let selected = f.selected();
        let proof = f.sink().recover(1).await.unwrap();
        assert_eq!(f.entry(1).await.confirmation, Some(proof));
        assert_eq!(f.selected(), selected);
        assert!(!f.events().contains(&"execute"));
        assert!(!f.events().contains(&"prepare"));
    }
}

#[tokio::test]
async fn incomplete_and_not_known_entries_stay_unknown_without_preparation_or_execute() {
    for fault in [1, 2, 5] {
        let f = Fixture::new().await;
        let before = f.selected();
        let mut owner = f.owner();
        if fault == 5 {
            f.probe.execute.store(2, Ordering::SeqCst);
        } else {
            f.journal.failure.store(fault, Ordering::SeqCst);
        }
        assert!(owner.capture(input(&opened(), 1)).await.is_err());
        assert_eq!(f.selected(), before);
        if fault == 1 {
            assert!(f.events().iter().all(|e| *e == "begin"));
        }
        f.journal.failure.store(0, Ordering::SeqCst);
        f.probe.execute.store(0, Ordering::SeqCst);
        f.probe.events.lock().unwrap().clear();
        assert_eq!(f.sink().recover(1).await.err(), Some(CaptureError::Unknown));
        assert!(!f.events().contains(&"execute"));
        assert!(!f.events().contains(&"prepare"));
        assert_eq!(f.selected(), before);
    }
}

#[tokio::test]
async fn dropped_wait_fences_owner_and_recovery_only_reads_retained_intent() {
    for fault in [4, 5] {
        let f = Fixture::new().await;
        f.journal.failure.store(fault, Ordering::SeqCst);
        let mut owner = f.owner();
        let frame = opened();
        let mut wait = Box::pin(owner.capture(input(&frame, 1)));
        assert!(matches!(
            wait.as_mut().poll(&mut Context::from_waker(Waker::noop())),
            Poll::Pending
        ));
        drop(wait);
        assert_eq!(
            owner.capture(input(&frame, 1)).await.err(),
            Some(CaptureError::Stopped)
        );
        f.journal.failure.store(0, Ordering::SeqCst);
        f.probe.events.lock().unwrap().clear();
        let result = f.sink().recover(1).await;
        if fault == 4 {
            assert!(result.is_ok());
        } else {
            assert_eq!(result.err(), Some(CaptureError::Unknown));
        }
        assert!(!f.events().contains(&"execute"));
        assert!(!f.events().contains(&"prepare"));
    }
}

#[tokio::test]
async fn recovery_reauthorizes_and_rejects_unavailable_or_changed_original_receipts() {
    let f = Fixture::new().await;
    f.owner().capture(input(&opened(), 1)).await.unwrap();
    for fault in [1, 2, 3] {
        f.probe.events.lock().unwrap().clear();
        f.probe
            .replay
            .store(if fault == 3 { 0 } else { fault }, Ordering::SeqCst);
        f.host.0.store(fault == 3, Ordering::SeqCst);
        let error = f.sink().recover(1).await.err().unwrap();
        assert_eq!(
            error,
            if fault == 2 {
                CaptureError::ReceiptMismatch
            } else {
                CaptureError::Unknown
            }
        );
        assert!(!f.events().contains(&"execute"));
        assert!(!f.events().contains(&"prepare"));
    }
}

#[tokio::test]
async fn changed_bytes_or_thread_under_same_ordinal_cannot_claim_another_request() {
    let f = Fixture::new().await;
    f.owner().capture(input(&opened(), 1)).await.unwrap();
    let selected = f.selected();
    let mut changed = opened();
    changed["adapterSessionId"] = json!("changed");
    assert_eq!(
        f.owner().capture(input(&changed, 1)).await.err(),
        Some(CaptureError::ReceiptMismatch)
    );
    let mut other = f.context.clone();
    other.thread = ThreadId::new([99; 16]);
    assert_eq!(other.request(1), f.context.request(1));
    assert_eq!(
        StoreCapture::new(other, f.stores.clone(), f.journal.clone())
            .unwrap()
            .recover(1)
            .await
            .err(),
        Some(CaptureError::ReceiptMismatch)
    );
    assert_eq!(f.selected(), selected);
}

#[tokio::test]
async fn exact_existing_batch_only_replays_and_incompatible_participants_fail_before_journal_io() {
    let f = Fixture::new().await;
    let first = f.owner().capture(input(&opened(), 1)).await.unwrap();
    let selected = f.selected();
    f.probe.events.lock().unwrap().clear();
    let repeated = f.owner().capture(input(&opened(), 1)).await.unwrap();
    assert_eq!(repeated.bytes(), first.bytes());
    assert_eq!(repeated.receipt(), first.receipt());
    assert_eq!(f.selected(), selected);
    assert_eq!(f.events(), ["begin", "replay", "confirm"]);
    f.probe.events.lock().unwrap().clear();
    let empty_journal = Arc::new(FileJournal::open(
        f.directory.path().join("lost-journal"),
        f.probe.events.clone(),
    ));
    let sink = StoreCapture::new(f.context.clone(), f.stores.clone(), empty_journal).unwrap();
    assert_eq!(
        f.owner_with_sink(Box::new(sink))
            .capture(input(&opened(), 1))
            .await
            .err(),
        Some(CaptureError::Unknown),
        "an existing provider receipt without original preparation cannot authorize output"
    );
    assert_eq!(f.events(), ["begin", "status"]);
    assert_eq!(f.selected(), selected);
    f.probe.events.lock().unwrap().clear();
    let other = open(f.repository.clone(), f.host.clone(), f.probe.clone());
    let mut factories = StoreFactories::empty();
    let agent = f.stores.agent().unwrap().clone();
    factories
        .add_agent_store("git", move |_| Ok(agent.clone()))
        .unwrap();
    let thread = other.thread().unwrap().clone();
    factories
        .add_thread_store("git", move |_| Ok(thread.clone()))
        .unwrap();
    let mut config = StoreConfig::default();
    config.enable(StoreKind::Agent, "git");
    config.enable(StoreKind::Thread, "git");
    let stores = factories
        .loader()
        .open(
            &config,
            StoreContext::new(f.stores.agent().unwrap().scope().clone(), f.host.clone()),
        )
        .unwrap();
    assert!(matches!(
        StoreCapture::new(f.context.clone(), Arc::new(stores), f.journal.clone()),
        Err(CaptureError::Rejected)
    ));
    assert!(matches!(
        StoreCapture::new(
            f.context.clone(),
            Arc::new(RepositoryStores::default()),
            f.journal.clone()
        ),
        Err(CaptureError::Rejected)
    ));
    assert!(f.events().is_empty());
}

#[tokio::test]
async fn stale_thread_after_preparation_does_not_publish_capture_or_retry() {
    struct Race {
        journal: Arc<FileJournal>,
        stores: Arc<RepositoryStores>,
    }
    #[async_trait::async_trait]
    impl Journal for Race {
        async fn load(&self, r: RequestId) -> std::result::Result<Option<EntryV1>, CaptureError> {
            self.journal.load(r).await
        }
        async fn begin(&self, c: &CandidateV1) -> std::result::Result<Claim, CaptureError> {
            self.journal.begin(c).await
        }
        async fn prepared(
            &self,
            c: &CandidateV1,
            p: &PreparedV1,
        ) -> std::result::Result<(), CaptureError> {
            self.journal.prepared(c, p).await?;
            let PreparedCommandV1::Capture { binding } = &p.command else {
                unreachable!()
            };
            committed(
                self.stores
                    .thread()
                    .unwrap()
                    .execute(&StoreRequest::new(
                        RequestId::new([88; 16]),
                        ThreadCommandV1::Rename {
                            thread_id: binding.thread_id,
                            expected: binding.observed_thread,
                            title: "Changed after preparation".into(),
                        },
                    ))
                    .await
                    .unwrap(),
            );
            Ok(())
        }
        async fn confirm(
            &self,
            c: &CandidateV1,
            p: &ConfirmationV1,
        ) -> std::result::Result<(), CaptureError> {
            self.journal.confirm(c, p).await
        }
    }
    let f = Fixture::new().await;
    let sink = StoreCapture::new(
        f.context.clone(),
        f.stores.clone(),
        Arc::new(Race {
            journal: f.journal.clone(),
            stores: f.stores.clone(),
        }),
    )
    .unwrap();
    let mut owner = f.owner_with_sink(Box::new(sink));
    assert_eq!(
        owner.capture(input(&opened(), 1)).await.err(),
        Some(CaptureError::Unknown)
    );
    let selected = f.selected();
    let agent = f.stores.agent().unwrap().snapshot().await.unwrap();
    assert!(
        AgentReader::new(agent.data.as_ref())
            .list_sessions(None, 16)
            .await
            .unwrap()
            .selections
            .is_empty()
    );
    assert!(f.entry(1).await.prepared.is_some());
    assert!(f.entry(1).await.confirmation.is_none());
    f.probe.events.lock().unwrap().clear();
    assert_eq!(f.sink().recover(1).await.err(), Some(CaptureError::Unknown));
    assert_eq!(f.selected(), selected);
    assert!(!f.events().contains(&"prepare"));
    assert!(!f.events().contains(&"execute"));
}

#[tokio::test]
async fn foreign_primary_assignment_and_wrong_repository_are_rejected() {
    let mut f = Fixture::new().await;
    let mut owner = f.owner();
    owner.capture(input(&opened(), 1)).await.unwrap();
    let thread = ThreadId::new([99; 16]);
    committed(
        f.stores
            .thread()
            .unwrap()
            .execute(&StoreRequest::new(
                RequestId::new([99; 16]),
                ThreadCommandV1::Create {
                    thread_id: thread,
                    title: "Other".into(),
                    baseline: None,
                },
            ))
            .await
            .unwrap(),
    );
    let selected = f.selected();
    // A new ordinal on the same source may not move its existing primary link.
    struct Observe;
    #[async_trait::async_trait]
    impl DurableCapture for Observe {
        async fn persist(
            &mut self,
            b: &CaptureBatch,
        ) -> std::result::Result<Receipt, CaptureError> {
            Ok(b.receipt())
        }
    }
    // Obtain a sealed batch without publishing through the original writer.
    let mut observed = f.owner_with_sink(Box::new(Observe));
    observed.capture(input(&opened(), 1)).await.unwrap();
    let event = json!({"version":3,"type":"record","sessionId":"worker","requestId":"turn","sequence":82,"record":{"status":"completed"}});
    let batch = observed.capture(input(&event, 2)).await.unwrap();
    f.context.thread = thread;
    assert_eq!(
        f.sink().persist(&batch).await.err(),
        Some(CaptureError::Rejected)
    );
    assert_eq!(f.selected(), selected);
    f.context.repository = RepositoryId::new([99; 16]);
    assert!(matches!(
        StoreCapture::new(f.context.clone(), f.stores.clone(), f.journal.clone()),
        Err(CaptureError::Invalid)
    ));
}
