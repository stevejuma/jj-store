//! Real owned-ref publication of production capture envelopes; no scanner claim.
use jj_agent::*;
use jj_collab::*;
use jj_store_conversations::{capture::*, privacy::Sanitizer};
use jj_store_core::{engine::AgentReader, *};
use jj_store_git::{
    GitProvider, LocalRepository,
    metadata::{Limits, MetadataStore},
};
use jj_store_model::{PrincipalId, RequestId};
use serde_json::{Value, json};
use std::sync::{Arc, Mutex};

struct Host;
#[async_trait::async_trait]
impl StoreHost for Host {
    async fn authorize(&self, _: StoreKind, _: Access) -> jj_store_core::Result<()> {
        Ok(())
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
        _: &mut String,
        chunk: &str,
        _: bool,
    ) -> jj_store_conversations::Result<String> {
        Ok(chunk.into())
    }
}
struct Observed(Arc<Mutex<Vec<Vec<u8>>>>);
#[async_trait::async_trait]
impl DurableCapture for Observed {
    async fn persist(
        &mut self,
        batch: &CaptureBatch,
    ) -> std::result::Result<Receipt, CaptureError> {
        self.0.lock().unwrap().push(batch.bytes().into());
        Ok(batch.receipt())
    }
}
fn open(repository: LocalRepository) -> RepositoryStores {
    let provider = GitProvider::new(repository);
    let mut factories = StoreFactories::empty();
    provider.register(&mut factories, "git").unwrap();
    factories
        .loader()
        .open(
            &StoreConfig::all("git"),
            StoreContext::new(provider.scope(), Arc::new(Host)),
        )
        .unwrap()
}
fn committed<T>(outcome: StoreOutcome<T>) -> StoreReceipt<T> {
    let StoreOutcome::Committed(value) = outcome else {
        panic!("committed receipt required")
    };
    value
}
fn observation(bytes: &[u8], index: u8) -> SanitizedObservationBatchV1 {
    let envelope = NativeCaptureEnvelopeV1::decode(bytes).unwrap();
    SanitizedObservationBatchV1 {
        schema_version: 1,
        batch_id: CaptureBatchId::new([index; 16]),
        integration: IntegrationId::OwnedController,
        source_instance_id: SourceInstanceId::new([5; 16]),
        sanitizer_version: envelope.policy,
        representation: SanitizedRepresentation::SanitizedNative,
        observations: vec![SanitizedObservationV1 {
            observation_id: ObservationId::new([index; 16]),
            source_sequence: envelope.ordinal,
            occurred_at_millis: None,
            source_kind: SourceKind::NativeApi,
            source_trust: SourceTrust::ApprovedHarnessApi,
            payload: SanitizedPayloadV1::new(SanitizedMediaType::Json, bytes).unwrap(),
        }],
    }
}

#[tokio::test]
async fn native_capture_evidence_and_transcript_publish_with_thread_and_reopen_exactly() {
    let directory = tempfile::tempdir().unwrap();
    gix::init_bare(directory.path()).unwrap();
    let metadata = MetadataStore::open(directory.path(), Limits::default()).unwrap();
    let repository = LocalRepository::new(directory.path(), metadata.clone());
    let stores = open(repository.clone());
    let before = stores.agent().unwrap().snapshot().await.unwrap();
    assert!(metadata.read().unwrap().is_none());
    let thread = ThreadId::new([7; 16]);
    committed(
        stores
            .thread()
            .unwrap()
            .execute(&StoreRequest::new(
                RequestId::new([7; 16]),
                ThreadCommandV1::Create {
                    thread_id: thread,
                    title: "Native capture".into(),
                    baseline: None,
                },
            ))
            .await
            .unwrap(),
    );
    let observed = Arc::new(Mutex::new(Vec::new()));
    let mut capture = NativeCapture::new(
        Binding {
            scope: "vault".into(),
            worker: "worker".into(),
            request: "open".into(),
            target: "target".into(),
            window: "main".into(),
            admission: "generation".into(),
        },
        "fixture-policy/v1".into(),
        Arc::new(Privacy),
        Box::new(Observed(observed.clone())),
    )
    .unwrap();
    let frames = [
        (
            Publication::Facts,
            json!({"version":3,"type":"opened","sessionId":"worker","requestId":"native-open","adapterSessionId":"provider","sequence":2}),
        ),
        (
            Publication::Event { sequence: 1 },
            json!({"version":3,"type":"activity","sessionId":"worker","requestId":"turn","sequence":81,"update":{"sessionUpdate":"agent_message_chunk","content":{"type":"text","text":"safe answer"}}}),
        ),
        (
            Publication::Event { sequence: 2 },
            json!({"version":3,"type":"record","sessionId":"worker","requestId":"turn","sequence":82,"record":{"status":"completed"}}),
        ),
    ];
    for (index, (publication, frame)) in frames.iter().enumerate() {
        capture
            .capture(Input {
                worker: "worker",
                ordinal: index as u64 + 1,
                publication: *publication,
                frame,
            })
            .await
            .unwrap();
    }
    let captured = observed.lock().unwrap().clone();
    let mut requests = Vec::new();
    let mut receipts = Vec::new();
    let mut session_id = None;
    for (index, bytes) in captured.iter().enumerate() {
        let source = observation(bytes, index as u8 + 20);
        let normalized =
            normalize_batch(&NormalizerProfileV1::native_capture_v1(), &source).unwrap();
        let session = normalized
            .aliases
            .iter()
            .find_map(|alias| match alias.target {
                AgentEntityRefV1::Session { session_id } => Some(session_id),
                _ => None,
            })
            .unwrap();
        assert!(session_id.is_none_or(|previous| previous == session));
        session_id = Some(session);
        let threads = stores.thread().unwrap().snapshot().await.unwrap();
        let state =
            ThreadStateV1::read(threads.data.root.clone(), threads.data.graph.as_ref()).unwrap();
        let request = StoreRequest::new(
            RequestId::new([index as u8 + 20; 16]),
            AgentCommand::Capture {
                profile: NormalizerProfileV1::native_capture_v1(),
                batch: Box::new(source),
                binding: Some(ThreadBindingV1 {
                    thread_id: thread,
                    observed_thread: state.threads[&thread],
                    observed_assignment: state.assignments.get(&session).copied(),
                }),
            },
        );
        receipts.push(committed(stores.execute_agent(&request).await.unwrap()));
        let agent = stores.agent().unwrap().snapshot().await.unwrap();
        let thread_pin = stores.thread().unwrap().snapshot().await.unwrap();
        assert_eq!(
            agent.selection, thread_pin.selection,
            "both roots select one committed ref"
        );
        requests.push(request);
    }
    assert!(
        AgentReader::new(before.data.as_ref())
            .list_sessions(None, 16)
            .await
            .unwrap()
            .selections
            .is_empty()
    );
    drop(stores);
    let reopened_metadata = MetadataStore::open(directory.path(), Limits::default()).unwrap();
    let reopened = open(LocalRepository::new(
        directory.path(),
        reopened_metadata.clone(),
    ));
    let agent = reopened.agent().unwrap().snapshot().await.unwrap();
    let reader = AgentReader::new(agent.data.as_ref());
    let rows = reader.list_sessions(None, 16).await.unwrap();
    assert_eq!(rows.selections.len(), 1);
    let AgentObjectV1::SessionSnapshot(session) =
        agent.data.read_object(rows.selections[0].1).await.unwrap()
    else {
        panic!("session")
    };
    assert_eq!(session.session_id, session_id.unwrap());
    assert_eq!(session.state, SessionState::Idle);
    assert_eq!(session.evidence.len(), 3);
    for (reference, bytes) in session.evidence.iter().zip(&captured) {
        let AgentObjectV1::ObservationBatch(batch) =
            agent.data.read_object(*reference).await.unwrap()
        else {
            panic!("source")
        };
        assert_eq!(batch.observations[0].payload.bytes(), bytes);
    }
    let transcript = reader
        .transcript_range(
            session.transcripts[0],
            SourceInstanceId::new([5; 16]),
            1,
            None,
            16,
            4096,
        )
        .await
        .unwrap();
    assert_eq!(transcript.chunks.len(), 1);
    assert_eq!(transcript.chunks[0].sequence, 3);
    assert_eq!(transcript.chunks[0].payload.bytes(), b"safe answer");
    let threads = reopened.thread().unwrap().snapshot().await.unwrap();
    let state = ThreadStateV1::read(threads.data.root, threads.data.graph.as_ref()).unwrap();
    assert!(state.assignments.contains_key(&session.session_id));
    let selected = metadata.read().unwrap().unwrap().commit();
    for (request, receipt) in requests.iter().zip(receipts) {
        assert_eq!(
            committed(reopened.execute_agent(request).await.unwrap()).provider_receipt,
            receipt.provider_receipt
        );
    }
    assert_eq!(metadata.read().unwrap().unwrap().commit(), selected);
    let mut changed = requests.last().unwrap().clone();
    let AgentCommand::Capture { batch, .. } = &mut changed.command else {
        unreachable!()
    };
    let mut envelope: Value =
        serde_json::from_slice(batch.observations[0].payload.bytes()).unwrap();
    envelope["frames"][0]["assistantText"] = json!("different answer");
    batch.observations[0].payload = SanitizedPayloadV1::new(
        SanitizedMediaType::Json,
        &serde_json::to_vec(&envelope).unwrap(),
    )
    .unwrap();
    assert!(matches!(
        reopened.execute_agent(&changed).await,
        Err(jj_store_core::Error::IntentConflict)
    ));
    assert_eq!(
        reopened_metadata.read().unwrap().unwrap().commit(),
        selected
    );
}
