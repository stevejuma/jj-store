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
async fn work_pages_pin_roots_reauthorize_and_bind_query_without_publication() {
    let (store, writer) = fixture().await;
    let pager = WorkPager::new(Arc::new(SystemClock));
    let empty = pager
        .page(writer.clone(), WorkQuery::default())
        .await
        .unwrap();
    assert!(empty.snapshot.work_root.is_none());
    assert_eq!(
        writer.snapshot().await.unwrap().manifest().published,
        m::WalPosition(0)
    );
    create(writer.clone(), 10).await;
    create(writer.clone(), 11).await;
    let query = WorkQuery {
        limit: 1,
        ..Default::default()
    };
    let first = pager.page(writer.clone(), query.clone()).await.unwrap();
    let mut next = query.clone();
    next.cursor = first.next_cursor.clone();
    create(writer.clone(), 12).await;
    let second = pager.page(writer.clone(), next.clone()).await.unwrap();
    assert_eq!(second.snapshot, first.snapshot);
    assert_eq!(second.projects.len(), 1);
    assert!(second.next_cursor.is_none());
    next.limit = 2;
    assert_eq!(
        pager.page(writer.clone(), next.clone()).await.unwrap_err(),
        Error::InvalidCursor
    );
    next.limit = 1;
    next.project = Some(ProjectId::new([10; 16]));
    assert_eq!(
        pager.page(writer.clone(), next.clone()).await.unwrap_err(),
        Error::InvalidCursor
    );
    next.project = None;
    next.cursor.as_mut().unwrap().replace_range(0..1, "z");
    assert_eq!(
        pager.page(writer.clone(), next).await.unwrap_err(),
        Error::InvalidCursor
    );
    let fresh = WorkPager::new(Arc::new(SystemClock));
    let mut next = query;
    next.cursor = first.next_cursor;
    assert_eq!(
        fresh.page(writer.clone(), next).await.unwrap_err(),
        Error::CursorReset
    );
    let reader = session(store, Permission::Read);
    let before = writer.snapshot().await.unwrap().digest();
    assert!(
        pager
            .page(reader.clone(), WorkQuery::default())
            .await
            .is_ok()
    );
    assert_eq!(
        WorkCommandService
            .apply(
                reader,
                &request(
                    13,
                    WorkCommandV1::CreateProject {
                        id: ProjectId::new([13; 16]),
                        title: "Denied".into(),
                        body: "".into()
                    }
                )
            )
            .await
            .unwrap_err(),
        Error::Denied
    );
    assert_eq!(writer.snapshot().await.unwrap().digest(), before);
}
#[tokio::test]
async fn work_commands_derive_actor_replay_receipts_and_wake_idle_events() {
    let (store, writer) = fixture().await;
    let request = request(
        10,
        WorkCommandV1::CreateProject {
            id: ProjectId::new([10; 16]),
            title: "Home".into(),
            body: "".into(),
        },
    );
    use jj_store_service::events::{EventKind, Limits as EventLimits, Subscription};
    use std::time::Duration;
    let mut subscription = Subscription::open(
        writer.clone(),
        None,
        EventLimits {
            poll_interval: Duration::from_secs(60),
            ..Default::default()
        },
    )
    .await
    .unwrap();
    assert!(matches!(
        subscription.next().await.unwrap().unwrap().kind,
        EventKind::Ready { .. }
    ));
    let outcome = WorkCommandService
        .apply(writer.clone(), &request)
        .await
        .unwrap();
    let event = tokio::time::timeout(Duration::from_secs(2), subscription.next())
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    assert!(matches!(event.kind, EventKind::Changed));
    let Outcome::Committed { receipt } = &outcome else {
        panic!("commit required")
    };
    assert_eq!(
        receipt.result.project.as_ref().unwrap().actor.created_by,
        ACTOR
    );
    assert_eq!(
        WorkCommandService
            .apply(writer.clone(), &request)
            .await
            .unwrap(),
        outcome
    );
    let fresh = session(store, Permission::Write);
    assert_eq!(
        WorkCommandService
            .status(fresh, request.request_id)
            .await
            .unwrap(),
        Status::Committed {
            receipt: receipt.clone()
        }
    );
    let mut changed = request.clone();
    if let WorkCommandV1::CreateProject { title, .. } = &mut changed.command {
        *title = "Different intent".into();
    }
    assert_eq!(
        WorkCommandService
            .apply(writer.clone(), &changed)
            .await
            .unwrap_err(),
        Error::IntentConflict
    );
    assert_eq!(
        WorkCommandService
            .status(writer, m::RequestId::new([11; 16]))
            .await
            .unwrap(),
        Status::NotKnown {
            request_id: m::RequestId::new([11; 16])
        }
    );
}

#[tokio::test]
async fn linked_repository_metadata_does_not_grant_code_access_and_cursors_expire() {
    use jj_store_service::{
        auth::{BearerToken, Clock, Scope, TokenPolicy},
        native::create_repository,
    };
    use std::sync::atomic::{AtomicU64, Ordering};
    struct ManualClock(AtomicU64);
    impl Clock for ManualClock {
        fn unix_seconds(&self) -> jj_store_service::auth::Result<u64> {
            Ok(self.0.load(Ordering::SeqCst))
        }
    }
    let clock = Arc::new(ManualClock(AtomicU64::new(0)));
    let store = Arc::new(MemoryStore::default());
    let token = BearerToken::generate().unwrap();
    let mut operator = TokenPolicy::new(Scope::Operator);
    let operator_token = operator
        .issue(ACTOR, Permission::CreateRepository, None)
        .unwrap();
    let target = m::RepositoryId::new([9; 16]);
    for repo in [REPO, target] {
        let mut policy = TokenPolicy::new(Scope::Repository(repo));
        if repo == REPO {
            policy
                .grant(&token, ACTOR, Permission::Write, Some(1000))
                .unwrap();
        }
        create_repository(
            store.clone(),
            Arc::new(NativeObjects::new(store.clone(), repo, 2 * 1024 * 1024).unwrap()),
            &operator,
            &operator_token,
            &policy,
            m::EpochId::new([3; 16]),
            clock.as_ref(),
            Default::default(),
        )
        .await
        .unwrap();
    }
    let authenticated = |repo| {
        Arc::new(
            NativeSession::new(
                store.clone(),
                Arc::new(NativeObjects::new(store.clone(), repo, 2 * 1024 * 1024).unwrap()),
                token.clone(),
                clock.clone(),
                Protection::default(),
                SessionLimits::default(),
            )
            .unwrap(),
        )
    };
    let home = authenticated(REPO);
    create(home.clone(), 10).await;
    create(home.clone(), 11).await;
    WorkCommandService
        .apply(
            home.clone(),
            &request(
                12,
                WorkCommandV1::SetRepository {
                    project: ProjectId::new([10; 16]),
                    expected_version: 1,
                    repository: target,
                    active: true,
                },
            ),
        )
        .await
        .unwrap();
    let pager = WorkPager::new(clock.clone());
    let first = pager
        .page(
            home.clone(),
            WorkQuery {
                limit: 1,
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert!(first.projects[0].repositories.contains_key(&target));
    assert!(authenticated(target).snapshot().await.is_err());
    let query = WorkQuery {
        limit: 1,
        cursor: first.next_cursor,
        ..Default::default()
    };
    clock.0.store(300, Ordering::SeqCst);
    assert_eq!(
        pager.page(home.clone(), query.clone()).await.unwrap_err(),
        Error::CursorReset
    );
    let first = pager
        .page(
            home.clone(),
            WorkQuery {
                limit: 1,
                ..Default::default()
            },
        )
        .await
        .unwrap();
    clock.0.store(1000, Ordering::SeqCst);
    assert_eq!(
        pager
            .page(
                home,
                WorkQuery {
                    cursor: first.next_cursor,
                    ..query
                }
            )
            .await
            .unwrap_err(),
        Error::Denied
    );
}
