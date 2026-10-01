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
