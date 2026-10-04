use jj_store_jj::{objects::NativeObjects, policy::Protection};
use jj_store_model as m;
use jj_store_object::memory::MemoryStore;
use jj_store_service::{
    auth::{Permission, SystemClock},
    collaboration::{AgentSessionPager, Error},
    native::{NativeSession, SessionLimits, create_repository_local_trusted},
    pagination,
};
use jj_store_wal::{
    self as wal,
    publication::{
        AgentCatalogEditV1, AgentCatalogKey, AgentCatalogKind, AgentCheckpointId,
        AgentCheckpointVersionV1, AgentDeltaRequest, AgentDeltaV1, AgentObjectRef, AgentObjectV1,
        AgentSessionId, AgentSessionSnapshotV1, CaptureStatusV1, CodeAnchorV1, CodeRelation,
        CompletenessV1, Coordinator, CoverageV1, EvidenceMethod, ExecutionOutcomeV1, IntegrationId,
        Limits, PublicationGuard, PublishOutcome, SessionState, SourceTrust, TranscriptManifestV1,
    },
};
use std::{collections::BTreeMap, collections::BTreeSet, sync::Arc};

const REPOSITORY: m::RepositoryId = m::RepositoryId::new([81; 16]);
const EPOCH: m::EpochId = m::EpochId::new([82; 16]);
const PRINCIPAL: m::PrincipalId = m::PrincipalId::new([83; 16]);

struct NoPublication;
#[async_trait::async_trait]
impl PublicationGuard for NoPublication {
    async fn authorize(
        &self,
        _: &wal::publication::ManifestSnapshot,
        _: &wal::publication::Request,
    ) -> wal::Result<()> {
        Err(wal::Error::Denied)
    }

    async fn validate_transition(
        &self,
        _: &wal::publication::ManifestSnapshot,
        _: &wal::publication::Request,
        _: &wal::publication::Snapshot,
    ) -> wal::Result<()> {
        Err(wal::Error::Denied)
    }
}

struct Fixture {
    session: Arc<NativeSession>,
    staging: Coordinator,
}

async fn fixture() -> Fixture {
    let store = Arc::new(MemoryStore::default());
    let objects = Arc::new(NativeObjects::new(store.clone(), REPOSITORY, 1024 * 1024).unwrap());
    create_repository_local_trusted(
        store.clone(),
        objects.clone(),
        Permission::Admin,
        EPOCH,
        Limits::default(),
    )
    .await
    .unwrap();
    let staging = Coordinator::new(
        store.clone(),
        REPOSITORY,
        objects.clone(),
        Arc::new(NoPublication),
        Limits::default(),
    )
    .unwrap();
    let session = Arc::new(
        NativeSession::new_local_trusted(
            store,
            objects,
            PRINCIPAL,
            Permission::Admin,
            Arc::new(SystemClock),
            Protection::default(),
            SessionLimits::default(),
        )
        .unwrap(),
    );
    Fixture { session, staging }
}

fn coverage() -> CoverageV1 {
    CoverageV1 {
        completeness: CompletenessV1::Complete,
        gaps: 0,
        redacted_fields: 0,
        truncated: false,
    }
}

async fn support(fixture: &Fixture, seed: u8) -> (AgentSessionId, AgentObjectRef, AgentObjectRef) {
    let session_id = AgentSessionId::new([seed; 16]);
    let transcript = fixture
        .staging
        .stage_agent_object(&AgentObjectV1::TranscriptManifest(TranscriptManifestV1 {
            schema_version: 1,
            session_id,
            streams: vec![],
            coverage: coverage(),
        }))
        .await
        .unwrap();
    let session = fixture
        .staging
        .stage_agent_object(&AgentObjectV1::SessionSnapshot(AgentSessionSnapshotV1 {
            schema_version: 1,
            session_id,
            integration: IntegrationId::Codex,
            state: SessionState::Ended,
            parent_session_id: None,
            evidence: vec![],
            event_batches: vec![],
            transcripts: vec![transcript],
            tools: BTreeMap::new(),
            coverage: coverage(),
        }))
        .await
        .unwrap();
    (session_id, session, transcript)
}

async fn version(
    fixture: &Fixture,
    checkpoint_id: AgentCheckpointId,
    session_id: AgentSessionId,
    session_snapshot: AgentObjectRef,
    transcript_manifest: AgentObjectRef,
    outcome: ExecutionOutcomeV1,
    supersedes: Option<AgentObjectRef>,
) -> AgentObjectRef {
    let anchor = CodeAnchorV1 {
        operations: BTreeSet::from([m::OperationId::new([0; 64])]),
        commits: BTreeSet::new(),
    };
    fixture
        .staging
        .stage_agent_object(&AgentObjectV1::CheckpointVersion(
            AgentCheckpointVersionV1 {
                schema_version: 1,
                checkpoint_id,
                session_id,
                turn_id: None,
                scope_id: None,
                session_snapshot,
                transcript_manifest,
                event_batches: vec![],
                tools: BTreeMap::new(),
                before: anchor.clone(),
                after: anchor,
                code_relation: CodeRelation::NoChange,
                associations: vec![],
                evidence_method: EvidenceMethod::ControlledScope,
                source_trust: SourceTrust::AuthenticatedController,
                exclusive_write_scope: true,
                coverage: coverage(),
                capture_status: CaptureStatusV1::Complete,
                execution_outcome: outcome,
                supersedes,
            },
        ))
        .await
        .unwrap()
}

async fn publish(
    fixture: &Fixture,
    seed: u8,
    edits: Vec<(AgentCheckpointId, Option<AgentObjectRef>, AgentObjectRef)>,
) {
    let request = AgentDeltaRequest::new(
        PRINCIPAL,
        m::RequestId::new([seed; 16]),
        m::PublicationId::new([seed + 1; 16]),
        AgentDeltaV1 {
            schema_version: 1,
            edits: edits
                .into_iter()
                .map(
                    |(checkpoint_id, expected, replacement)| AgentCatalogEditV1 {
                        catalog: AgentCatalogKind::Checkpoints,
                        key: AgentCatalogKey::derive("checkpoint", checkpoint_id.as_bytes())
                            .unwrap(),
                        expected,
                        replacement: Some(replacement),
                    },
                )
                .collect(),
        },
        b"published checkpoint fixture".to_vec(),
    )
    .unwrap();
    assert!(matches!(
        fixture.session.publish_agent_delta(&request).await.unwrap(),
        PublishOutcome::Committed(_)
    ));
}

#[tokio::test]
async fn checkpoint_pages_and_versions_retain_exact_root() {
    let fixture = fixture().await;
    let (first_session, first_snapshot, first_transcript) = support(&fixture, 91).await;
    let (second_session, second_snapshot, second_transcript) = support(&fixture, 92).await;
    let first_id = AgentCheckpointId::new([93; 16]);
    let second_id = AgentCheckpointId::new([94; 16]);
    let first_v1 = version(
        &fixture,
        first_id,
        first_session,
        first_snapshot,
        first_transcript,
        ExecutionOutcomeV1::Succeeded,
        None,
    )
    .await;
    let second_v1 = version(
        &fixture,
        second_id,
        second_session,
        second_snapshot,
        second_transcript,
        ExecutionOutcomeV1::Succeeded,
        None,
    )
    .await;
    publish(
        &fixture,
        95,
        vec![(first_id, None, first_v1), (second_id, None, second_v1)],
    )
    .await;

    let pager =
        AgentSessionPager::new(pagination::Limits::default(), Arc::new(SystemClock)).unwrap();
    let first = pager
        .begin_checkpoints(fixture.session.clone(), 1)
        .await
        .unwrap();
    let cursor = first.next_cursor.clone().unwrap();
    let first_v2 = version(
        &fixture,
        first_id,
        first_session,
        first_snapshot,
        first_transcript,
        ExecutionOutcomeV1::Failed,
        Some(first_v1),
    )
    .await;
    let second_v2 = version(
        &fixture,
        second_id,
        second_session,
        second_snapshot,
        second_transcript,
        ExecutionOutcomeV1::Interrupted,
        Some(second_v1),
    )
    .await;
    publish(
        &fixture,
        96,
        vec![
            (first_id, Some(first_v1), first_v2),
            (second_id, Some(second_v1), second_v2),
        ],
    )
    .await;
    let second = pager
        .resume_checkpoints(fixture.session.clone(), &cursor, 1)
        .await
        .unwrap();
    assert_eq!(second.snapshot, first.snapshot);
    let pinned: BTreeSet<_> = first
        .checkpoints
        .iter()
        .chain(&second.checkpoints)
        .map(|item| item.version_id)
        .collect();
    assert_eq!(pinned, BTreeSet::from([first_v1.id, second_v1.id]));

    let current = pager
        .begin_checkpoints(fixture.session.clone(), 10)
        .await
        .unwrap();
    assert_eq!(
        current
            .checkpoints
            .iter()
            .map(|item| item.version_id)
            .collect::<BTreeSet<_>>(),
        BTreeSet::from([first_v2.id, second_v2.id])
    );
    let detail = pager
        .checkpoint(fixture.session.clone(), first_id)
        .await
        .unwrap();
    assert_eq!(detail.checkpoint.version_id, first_v2.id);
    assert_eq!(detail.checkpoint.supersedes, Some(first_v1.id));
    let historical = pager
        .checkpoint_version(fixture.session.clone(), first_id, first_v1.id)
        .await
        .unwrap();
    assert_eq!(historical.checkpoint.version_id, first_v1.id);
    assert_eq!(historical.snapshot, detail.snapshot);
    assert_eq!(
        pager
            .checkpoint_version(fixture.session.clone(), first_id, second_v1.id)
            .await
            .unwrap_err(),
        Error::NotFound
    );
    assert_eq!(
        pager
            .checkpoint(fixture.session, AgentCheckpointId::new([99; 16]))
            .await
            .unwrap_err(),
        Error::NotFound
    );
}
