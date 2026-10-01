use jj_store_jj::{objects::NativeObjects, policy::Protection};
use jj_store_model as m;
use jj_store_object::memory::MemoryStore;
use jj_store_service::{
    auth::{Permission, SystemClock},
    native::{NativeSession, SessionLimits, create_repository_local_trusted},
    work::*,
};
use std::sync::Arc;

const REPO: m::RepositoryId = m::RepositoryId::new([1; 16]);
const ACTOR: m::PrincipalId = m::PrincipalId::new([2; 16]);
async fn fixture() -> (Arc<MemoryStore>, Arc<NativeSession>) {
    let store = Arc::new(MemoryStore::default());
    let objects = Arc::new(NativeObjects::new(store.clone(), REPO, 2 * 1024 * 1024).unwrap());
    create_repository_local_trusted(
        store.clone(),
        objects.clone(),
        Permission::Admin,
        m::EpochId::new([3; 16]),
        Default::default(),
    )
    .await
    .unwrap();
    let session = session(store.clone(), Permission::Write);
    (store, session)
}
fn session(store: Arc<MemoryStore>, permission: Permission) -> Arc<NativeSession> {
    let objects = Arc::new(NativeObjects::new(store.clone(), REPO, 2 * 1024 * 1024).unwrap());
    Arc::new(
        NativeSession::new_local_trusted(
            store,
            objects,
            ACTOR,
            permission,
            Arc::new(SystemClock),
            Protection::default(),
            SessionLimits::default(),
        )
        .unwrap(),
    )
}
fn request(byte: u8, command: WorkCommandV1) -> Request {
    Request {
        version: 1,
        request_id: m::RequestId::new([byte; 16]),
        command,
    }
}
async fn create(session: Arc<NativeSession>, byte: u8) {
    WorkCommandService
        .apply(
            session,
            &request(
                byte,
                WorkCommandV1::CreateProject {
                    id: ProjectId::new([byte; 16]),
                    title: format!("Project {byte}"),
                    body: "Metadata".into(),
                },
            ),
        )
        .await
        .unwrap();
}
#[tokio::test]
async fn git_acceptance_reauthorizes_write_and_preserves_home_origins() {
    let (store, writer) = fixture().await;
    create(writer.clone(), 40).await;
    let before = writer.snapshot().await.unwrap();
    let id = m::RequestId::new([41; 16]);
    let request = jj_store_service::git_bridge::Request {
        version: 1,
        request_id: id,
        transition: None,
        records: vec![jj_store_wal::publication::GitRecordV1::Remote(
            jj_store_wal::publication::GitRemoteV1 {
                remote: id,
                event: id,
                predecessors: Default::default(),
                name: "origin".into(),
                active: true,
            },
        )],
    };
    let reader = session(store.clone(), Permission::Read);
    assert!(matches!(
        jj_store_service::git_bridge::accept(reader.clone(), &request).await,
        Err(jj_store_wal::Error::Denied)
    ));
    assert_eq!(before.digest(), writer.snapshot().await.unwrap().digest());
    let outcome = jj_store_service::git_bridge::accept(writer.clone(), &request)
        .await
        .unwrap();
    assert_eq!(
        jj_store_service::git_bridge::accept(writer.clone(), &request)
            .await
            .unwrap(),
        outcome
    );
    let after = writer.snapshot().await.unwrap();
    assert_eq!(after.manifest().heads, before.manifest().heads);
    assert_eq!(
        after.manifest().code_state_version,
        before.manifest().code_state_version
    );
    assert_eq!(
        after.manifest().replication_root,
        before.manifest().replication_root
    );
    assert_eq!(after.manifest().work_root, before.manifest().work_root);
    let state = jj_store_service::git_bridge::read(reader.clone())
        .await
        .unwrap();
    assert_eq!(
        state.git.unwrap().remotes().unwrap().get("origin"),
        Some(&id)
    );
    assert!(matches!(
        jj_store_service::git_bridge::status(reader, id).await,
        Err(jj_store_wal::Error::Denied)
    ));
    let cold = session(store, Permission::Write);
    assert_eq!(
        jj_store_service::git_bridge::accept(cold.clone(), &request)
            .await
            .unwrap(),
        outcome
    );
    assert!(
        jj_store_service::git_bridge::status(cold, id)
            .await
            .unwrap()
            .is_some()
    );
}

#[tokio::test]
async fn git_push_executor_cannot_be_spoofed_or_changed_by_another_writer() {
    use jj_store_wal::publication::*;
    use std::collections::{BTreeMap, BTreeSet};
    let (store, writer) = fixture().await;
    create(writer.clone(), 50).await;
    let before = writer.snapshot().await.unwrap();
    let remote = m::RequestId::new([51; 16]);
    let observation = m::RequestId::new([52; 16]);
    let binding = m::RecordDigest::new([53; 32]);
    let request = jj_store_service::git_bridge::Request {
        version: 1,
        request_id: remote,
        transition: None,
        records: vec![
            GitRecordV1::Remote(GitRemoteV1 {
                remote,
                event: remote,
                predecessors: BTreeSet::new(),
                name: "origin".into(),
                active: true,
            }),
            GitRecordV1::Observation(GitObservationV1 {
                id: observation,
                remote,
                binding,
                predecessors: BTreeSet::new(),
                operation: *before.manifest().heads.first().unwrap(),
                complete_heads: true,
                refs: BTreeMap::new(),
            }),
        ],
    };
    jj_store_service::git_bridge::accept(writer.clone(), &request)
        .await
        .unwrap();
    let attempt = m::RequestId::new([54; 16]);
    let mut intent = GitPushIntentV1 {
        id: attempt,
        executor: m::PrincipalId::new([55; 16]),
        remote,
        binding,
        operation: *before.manifest().heads.first().unwrap(),
        observations: BTreeSet::from([observation]),
        refs: BTreeMap::from([(
            "refs/heads/missing".into(),
            GitPushRefV1 {
                expected: None,
                desired: GitRefV1 {
                    oid: None,
                    commit_oid: None,
                    native: None,
                },
            },
        )]),
    };
    let mut request = jj_store_service::git_bridge::Request {
        version: 1,
        request_id: attempt,
        transition: None,
        records: vec![GitRecordV1::PushIntent(intent.clone())],
    };
    let before = writer.snapshot().await.unwrap();
    assert!(matches!(
        jj_store_service::git_bridge::accept(writer.clone(), &request).await,
        Err(jj_store_wal::Error::Denied)
    ));
    assert_eq!(writer.snapshot().await.unwrap().digest(), before.digest());
    intent.executor = ACTOR;
    request.records = vec![GitRecordV1::PushIntent(intent)];
    jj_store_service::git_bridge::accept(writer.clone(), &request)
        .await
        .unwrap();
    let event = m::RequestId::new([56; 16]);
    let mut unselected = jj_store_jj::roots::operation();
    unselected.metadata.description = "unselected source".into();
    let objects = NativeObjects::new(store.clone(), REPO, 2 * 1024 * 1024).unwrap();
    let (m::ObjectKey::Operation(unselected), _) = objects.put(&unselected).await.unwrap() else {
        panic!()
    };
    let mut unselected_request = request.clone();
    unselected_request.request_id = m::RequestId::new([57; 16]);
    let GitRecordV1::PushIntent(unselected_intent) = &mut unselected_request.records[0] else {
        panic!()
    };
    unselected_intent.id = unselected_request.request_id;
    unselected_intent.operation = unselected;
    let before = writer.snapshot().await.unwrap();
    assert!(
        jj_store_service::git_bridge::accept(writer.clone(), &unselected_request)
            .await
            .is_err(),
        "unselected source operation cannot authorize Git publication"
    );
    assert_eq!(writer.snapshot().await.unwrap().digest(), before.digest());
    let other = Arc::new(
        NativeSession::new_local_trusted(
            store.clone(),
            Arc::new(NativeObjects::new(store, REPO, 2 * 1024 * 1024).unwrap()),
            m::PrincipalId::new([55; 16]),
            Permission::Write,
            Arc::new(SystemClock),
            Protection::default(),
            SessionLimits::default(),
        )
        .unwrap(),
    );
    let before = writer.snapshot().await.unwrap();
    request.request_id = event;
    request.records = vec![GitRecordV1::PushEvent(GitPushEventV1 {
        id: event,
        attempt,
        predecessors: BTreeSet::new(),
        kind: GitPushEventKindV1::TransportStarted,
    })];
    assert!(matches!(
        jj_store_service::git_bridge::accept(other, &request).await,
        Err(jj_store_wal::Error::Denied)
    ));
    assert_eq!(writer.snapshot().await.unwrap().digest(), before.digest());
    let forged = GitPushEventV1 {
        id: event,
        attempt,
        predecessors: BTreeSet::new(),
        kind: GitPushEventKindV1::ResultRecorded {
            refs: BTreeMap::from([(
                "refs/heads/missing".into(),
                GitPushRefOutcomeV1::Acknowledged,
            )]),
        },
    };
    request.request_id = event;
    request.records = vec![GitRecordV1::PushEvent(forged)];
    assert!(
        jj_store_service::git_bridge::accept(writer, &request)
            .await
            .is_err()
    );
}
