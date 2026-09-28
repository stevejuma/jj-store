use jj_store_jj::{objects::NativeObjects, policy::Protection};
use jj_store_model as m;
use jj_store_object::memory::MemoryStore;
use jj_store_service::{
    auth::{Permission, SystemClock},
    collaboration::{
        AgentRelationKind, AgentRelationRequest, AgentRelationSubject, AgentSessionPager, Error,
    },
    native::{NativeSession, SessionLimits, create_repository_local_trusted},
    pagination,
};
use jj_store_wal::{
    self as wal,
    publication::{
        AgentCatalogEditV1, AgentCatalogKey, AgentCatalogKind, AgentDeltaRequest, AgentDeltaV1,
        AgentObjectV1, AgentRelationPageV1, AgentRelationSubjectV1, AgentSessionId,
        AgentSessionSnapshotV1, AssociationGranularity, AssociationId, AssociationRelation,
        CodeAnchorV1, CodeAssociationV1, CompletenessV1, Coordinator, CoverageV1, EvidenceMethod,
        IntegrationId, Limits, PublicationGuard, PublishOutcome, SessionState, SourceTrust,
    },
};
use std::{collections::BTreeMap, collections::BTreeSet, sync::Arc};

const REPOSITORY: m::RepositoryId = m::RepositoryId::new([51; 16]);
const EPOCH: m::EpochId = m::EpochId::new([52; 16]);
const PRINCIPAL: m::PrincipalId = m::PrincipalId::new([53; 16]);

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

fn association(seed: u8, session_id: AgentSessionId) -> CodeAssociationV1 {
    CodeAssociationV1 {
        schema_version: 1,
        association_id: AssociationId::new([seed; 16]),
        checkpoint_id: None,
        session_id,
        turn_id: None,
        tool_call_ids: BTreeSet::new(),
        scope_id: None,
        relation: AssociationRelation::ObservedDuringInterval,
        before: CodeAnchorV1 {
            operations: BTreeSet::from([m::OperationId::new([0; 64])]),
            commits: BTreeSet::new(),
        },
        after: CodeAnchorV1::default(),
        changes: BTreeSet::new(),
        paths: vec![],
        granularity: AssociationGranularity::Operation,
        evidence_method: EvidenceMethod::HookBoundary,
        source_trust: SourceTrust::LocalUserHook,
        exclusive_write_scope: false,
        evidence: vec![],
        coverage: coverage(),
        created_at_millis: i64::from(seed),
        supersedes: None,
        correction_reason: None,
    }
}

async fn publish_relations(fixture: &Fixture) -> AgentSessionId {
    let session_id = AgentSessionId::new([61; 16]);
    let first = fixture
        .staging
        .stage_agent_object(&AgentObjectV1::Association(association(62, session_id)))
        .await
        .unwrap();
    let second = fixture
        .staging
        .stage_agent_object(&AgentObjectV1::Association(association(63, session_id)))
        .await
        .unwrap();
    let subject = AgentRelationSubjectV1::Session { session_id };
    let tail = fixture
        .staging
        .stage_agent_object(&AgentObjectV1::RelationPage(AgentRelationPageV1 {
            schema_version: 1,
            subject: subject.clone(),
            selections: vec![second],
            next: None,
        }))
        .await
        .unwrap();
    let head = fixture
        .staging
        .stage_agent_object(&AgentObjectV1::RelationPage(AgentRelationPageV1 {
            schema_version: 1,
            subject: subject.clone(),
            selections: vec![first],
            next: Some(tail),
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
            transcripts: vec![],
            tools: BTreeMap::new(),
            coverage: coverage(),
        }))
        .await
        .unwrap();
    let request = AgentDeltaRequest::new(
        PRINCIPAL,
        m::RequestId::new([64; 16]),
        m::PublicationId::new([65; 16]),
        AgentDeltaV1 {
            schema_version: 1,
            edits: vec![
                AgentCatalogEditV1 {
                    catalog: AgentCatalogKind::Sessions,
                    key: AgentCatalogKey::derive("session", session_id.as_bytes()).unwrap(),
                    expected: None,
                    replacement: Some(session),
                },
                AgentCatalogEditV1 {
                    catalog: AgentCatalogKind::Relations,
                    key: subject.catalog_key().unwrap(),
                    expected: None,
                    replacement: Some(head),
                },
            ],
        },
        b"published relation fixture".to_vec(),
    )
    .unwrap();
    assert!(matches!(
        fixture.session.publish_agent_delta(&request).await.unwrap(),
        PublishOutcome::Committed(_)
    ));
    session_id
}

async fn publish_unrelated(fixture: &Fixture) {
    let session_id = AgentSessionId::new([71; 16]);
    let session = fixture
        .staging
        .stage_agent_object(&AgentObjectV1::SessionSnapshot(AgentSessionSnapshotV1 {
            schema_version: 1,
            session_id,
            integration: IntegrationId::Codex,
            state: SessionState::Active,
            parent_session_id: None,
            evidence: vec![],
            event_batches: vec![],
            transcripts: vec![],
            tools: BTreeMap::new(),
            coverage: coverage(),
        }))
        .await
        .unwrap();
    let request = AgentDeltaRequest::new(
        PRINCIPAL,
        m::RequestId::new([72; 16]),
        m::PublicationId::new([73; 16]),
        AgentDeltaV1 {
            schema_version: 1,
            edits: vec![AgentCatalogEditV1 {
                catalog: AgentCatalogKind::Sessions,
                key: AgentCatalogKey::derive("session", session_id.as_bytes()).unwrap(),
                expected: None,
                replacement: Some(session),
            }],
        },
        b"published unrelated session".to_vec(),
    )
    .unwrap();
    assert!(matches!(
        fixture.session.publish_agent_delta(&request).await.unwrap(),
        PublishOutcome::Committed(_)
    ));
}

#[tokio::test]
async fn relation_pages_retain_root_and_reject_subject_cursor_reuse() {
    let fixture = fixture().await;
    let session_id = publish_relations(&fixture).await;
    let pager =
        AgentSessionPager::new(pagination::Limits::default(), Arc::new(SystemClock)).unwrap();
    let request = AgentRelationRequest {
        subject: AgentRelationSubject::Session { session_id },
        limit: 1,
    };
    let first = pager
        .begin_relations(fixture.session.clone(), request)
        .await
        .unwrap();
    assert_eq!(first.relations.len(), 1);
    assert_eq!(first.relations[0].kind, AgentRelationKind::Association);
    let cursor = first.next_cursor.clone().unwrap();

    publish_unrelated(&fixture).await;
    let second = pager
        .resume_relations(fixture.session.clone(), &cursor, request)
        .await
        .unwrap();
    assert_eq!(second.snapshot, first.snapshot);
    assert_eq!(second.relations.len(), 1);
    assert!(second.next_cursor.is_none());
    assert_ne!(first.relations[0].object_id, second.relations[0].object_id);

    assert_eq!(
        pager
            .resume_relations(
                fixture.session,
                &cursor,
                AgentRelationRequest {
                    subject: AgentRelationSubject::Session {
                        session_id: AgentSessionId::new([99; 16]),
                    },
                    limit: 1,
                },
            )
            .await
            .unwrap_err(),
        Error::InvalidCursor
    );
}
