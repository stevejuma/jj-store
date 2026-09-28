use jj_store_jj::{objects::NativeObjects, policy::Protection};
use jj_store_model as m;
use jj_store_object::memory::MemoryStore;
use jj_store_service::{
    auth::{Permission, SystemClock},
    collaboration::{AgentSessionPager, Error},
    native::{NativeSession, SessionLimits, create_repository_local_trusted},
    pagination,
};
use jj_store_wal::publication::{
    CaptureBatchId, IntegrationId, NormalizerProfileV1, ObservationId, SanitizedMediaType,
    SanitizedObservationBatchV1, SanitizedObservationV1, SanitizedPayloadV1,
    SanitizedRepresentation, SourceInstanceId, SourceKind, SourceTrust,
};
use jj_store_wal::publication::{Limits, PublishOutcome};
use std::{collections::BTreeSet, sync::Arc};

const REPOSITORY: m::RepositoryId = m::RepositoryId::new([41; 16]);
const EPOCH: m::EpochId = m::EpochId::new([42; 16]);
const PRINCIPAL: m::PrincipalId = m::PrincipalId::new([43; 16]);

fn batch(seed: u8, native_session: &str) -> SanitizedObservationBatchV1 {
    let payload = serde_json::json!({
        "hook_event_name": "SessionStart",
        "session_id": native_session,
    });
    SanitizedObservationBatchV1 {
        schema_version: 1,
        batch_id: CaptureBatchId::new([seed; 16]),
        integration: IntegrationId::Codex,
        source_instance_id: SourceInstanceId::new([seed + 1; 16]),
        sanitizer_version: "mongodb-kingfisher@2.7.0/builtin-medium/v1".into(),
        representation: SanitizedRepresentation::SanitizedNative,
        observations: vec![SanitizedObservationV1 {
            observation_id: ObservationId::new([seed + 2; 16]),
            source_sequence: 1,
            occurred_at_millis: Some(1_725_000_000_000 + i64::from(seed)),
            source_kind: SourceKind::Hook,
            source_trust: SourceTrust::LocalUserHook,
            payload: SanitizedPayloadV1::new(
                SanitizedMediaType::Json,
                &serde_json::to_vec(&payload).unwrap(),
            )
            .unwrap(),
        }],
    }
}

async fn fixture() -> Arc<NativeSession> {
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
    Arc::new(
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
    )
}

async fn publish(session: &NativeSession, seed: u8, native_session: &str) {
    let request = session
        .prepare_agent_capture(
            m::RequestId::new([seed + 3; 16]),
            m::PublicationId::new([seed + 4; 16]),
            &NormalizerProfileV1::codex_0_156_1(),
            &batch(seed, native_session),
            format!("accepted {native_session}").into_bytes(),
        )
        .await
        .unwrap();
    assert!(matches!(
        session.publish_agent_delta(&request).await.unwrap(),
        PublishOutcome::Committed(_)
    ));
}

#[tokio::test]
async fn session_pages_retain_one_agent_root_across_later_publication() {
    let session = fixture().await;
    publish(&session, 11, "native-session-1").await;
    publish(&session, 21, "native-session-2").await;
    let pager =
        AgentSessionPager::new(pagination::Limits::default(), Arc::new(SystemClock)).unwrap();

    let first = pager.begin(session.clone(), 1).await.unwrap();
    assert_eq!(first.version, 1);
    assert_eq!(first.snapshot.repo_id, REPOSITORY);
    assert_eq!(first.snapshot.epoch, EPOCH);
    assert_eq!(first.sessions.len(), 1);
    let cursor = first.next_cursor.clone().unwrap();

    publish(&session, 31, "native-session-3").await;
    let second = pager.resume(session.clone(), &cursor, 1).await.unwrap();
    assert_eq!(second.snapshot, first.snapshot);
    assert_eq!(second.sessions.len(), 1);
    assert!(second.next_cursor.is_none());
    let pinned: BTreeSet<_> = first
        .sessions
        .iter()
        .chain(&second.sessions)
        .map(|row| row.session_id)
        .collect();
    assert_eq!(pinned.len(), 2);

    let current = pager.begin(session.clone(), 10).await.unwrap();
    assert!(current.snapshot.agent_state_version > first.snapshot.agent_state_version);
    assert_eq!(current.sessions.len(), 3);
    assert!(current.next_cursor.is_none());

    assert_eq!(
        pager.resume(session.clone(), &cursor, 2).await.unwrap_err(),
        Error::InvalidCursor
    );
    let mut tampered = cursor;
    let tail = tampered.pop().unwrap();
    tampered.push(if tail == 'A' { 'B' } else { 'A' });
    assert_eq!(
        pager.resume(session, &tampered, 1).await.unwrap_err(),
        Error::InvalidCursor
    );
}

#[tokio::test]
async fn session_detail_resolves_current_stable_identity_and_exact_links() {
    let session = fixture().await;
    publish(&session, 41, "native-detail-session").await;
    let pager =
        AgentSessionPager::new(pagination::Limits::default(), Arc::new(SystemClock)).unwrap();
    let page = pager.begin(session.clone(), 10).await.unwrap();
    let summary = page.sessions.first().unwrap();

    let detail = pager
        .detail(session.clone(), summary.session_id)
        .await
        .unwrap();
    assert_eq!(detail.version, 1);
    assert_eq!(detail.snapshot, page.snapshot);
    assert_eq!(&detail.session, summary);
    assert_eq!(detail.evidence.len(), summary.evidence_count);
    assert_eq!(detail.event_batches.len(), summary.event_batch_count);
    assert_eq!(detail.transcripts.len(), summary.transcript_count);
    assert_eq!(detail.tools.len(), summary.tool_count);

    assert_eq!(
        pager
            .detail(
                session,
                jj_store_wal::publication::AgentSessionId::new([99; 16])
            )
            .await
            .unwrap_err(),
        Error::NotFound
    );
}
