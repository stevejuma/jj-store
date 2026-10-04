use jj_collab::*;
#[path = "content_search/cache.rs"]
mod cache;
use jj_store_jj::{objects::NativeObjects, policy::Protection};
use jj_store_model as m;
use jj_store_object::memory::MemoryStore;
use jj_store_service::{
    auth::{self, Clock, Permission},
    collab_mutations::{CollabCommandService, Outcome, Request},
    content_search::*,
    native::{NativeSession, SessionLimits, create_repository_local_trusted},
    work,
};
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};
const REPO: m::RepositoryId = m::RepositoryId::new([1; 16]);
const ACTOR: m::PrincipalId = m::PrincipalId::new([3; 16]);
const THREAD: ThreadId = ThreadId::new([10; 16]);
const PROJECT: work::ProjectId = work::ProjectId::new([20; 16]);
const TASK: work::TaskId = work::TaskId::new([21; 16]);
struct Time(AtomicU64);
impl Clock for Time {
    fn unix_seconds(&self) -> auth::Result<u64> {
        Ok(self.0.load(Ordering::SeqCst))
    }
}
struct Fixture {
    store: Arc<MemoryStore>,
    objects: Arc<NativeObjects>,
    clock: Arc<Time>,
}
impl Fixture {
    async fn new() -> Self {
        let store = Arc::new(MemoryStore::default());
        let objects = Arc::new(NativeObjects::new(store.clone(), REPO, 8 * 1024 * 1024).unwrap());
        create_repository_local_trusted(
            store.clone(),
            objects.clone(),
            Permission::Admin,
            m::EpochId::new([2; 16]),
            Default::default(),
        )
        .await
        .unwrap();
        let f = Self {
            store,
            objects,
            clock: Arc::new(Time(AtomicU64::new(1000))),
        };
        f.work(
            99,
            work::WorkCommandV1::CreateProject {
                id: PROJECT,
                title: "Project".into(),
                body: "project description needle".into(),
            },
        )
        .await;
        f.work(
            98,
            work::WorkCommandV1::CreateTask {
                id: TASK,
                project: PROJECT,
                expected_project_version: 1,
                title: "Task".into(),
                body: "task description needle".into(),
                repository: None,
                parent: None,
            },
        )
        .await;
        f
    }
    fn session(&self, actor: m::PrincipalId, permission: Permission) -> Arc<NativeSession> {
        Arc::new(
            NativeSession::new_local_trusted(
                self.store.clone(),
                self.objects.clone(),
                actor,
                permission,
                self.clock.clone(),
                Protection::default(),
                SessionLimits::default(),
            )
            .unwrap(),
        )
    }
    fn writer(&self) -> Arc<NativeSession> {
        self.session(ACTOR, Permission::Admin)
    }
    async fn work(&self, n: u8, command: work::WorkCommandV1) {
        work::WorkCommandService
            .apply(
                self.writer(),
                &work::Request {
                    version: 1,
                    request_id: m::RequestId::new([n; 16]),
                    command,
                },
            )
            .await
            .unwrap();
    }
    async fn command(&self, n: u8, intent: ThreadCommandV1) -> ThreadCommandResultV1 {
        let Outcome::Committed { receipt } = CollabCommandService
            .apply(
                self.writer(),
                &Request {
                    version: 1,
                    request_id: m::RequestId::new([n; 16]),
                    command: CollabCommandV1::Thread { intent },
                },
            )
            .await
            .unwrap()
        else {
            panic!("committed")
        };
        let CollabCommandResultV1::Thread { result } = receipt.result else {
            panic!("thread")
        };
        result
    }
    async fn create(&self, n: u8) -> ThreadCommandResultV1 {
        self.command(
            n,
            ThreadCommandV1::Create {
                thread_id: ThreadId::new([n; 16]),
                title: format!("Thread {n}"),
                baseline: None,
            },
        )
        .await
    }
    async fn search(&self, pager: &ContentPager, q: &str) -> ContentPage {
        pager
            .page(
                self.writer(),
                ContentQuery {
                    q: q.into(),
                    ..Default::default()
                },
            )
            .await
            .unwrap()
    }
}
#[tokio::test]
async fn thread_directory_keeps_latest_message_preview_after_rename() {
    let f = Fixture::new().await;
    let thread = f.create(10).await;
    let message = f
        .command(
            11,
            ThreadCommandV1::Message {
                thread_id: THREAD,
                expected_thread: thread.version,
                message_id: HumanMessageId::new([11; 16]),
                expected_revision: None,
                reply_to: None,
                body: Some("Latest visible message".into()),
            },
        )
        .await;
    tokio::time::sleep(std::time::Duration::from_millis(2)).await;
    f.command(
        12,
        ThreadCommandV1::Rename {
            thread_id: THREAD,
            expected: message.version,
            title: "Renamed thread".into(),
        },
    )
    .await;
    let pin = jj_store_service::collab_reads::PinnedCollabRead::open(
        f.writer(),
        Arc::new(jj_store_service::collab_reads::RepositoryReactionActors(
            REPO,
        )),
    )
    .await
    .unwrap();
    let detail = pin.thread(f.writer(), THREAD).await.unwrap();
    assert_eq!(detail.value.title, "Renamed thread");
    let renamed_at = detail.value.accepted_at_millis;
    let directory = detail.directory.unwrap();
    assert_eq!(directory.excerpt, "Latest visible message");
    assert_eq!(directory.last_activity_at_millis, renamed_at);
    assert_eq!(directory.message_count, 1);
}
#[tokio::test]
async fn current_full_bodies_edits_tombstones_and_scopes_are_manifest_selected() {
    let f = Fixture::new().await;
    let pager = ContentPager::new(f.clock.clone());
    let thread = f.create(10).await;
    let first = f
        .command(
            11,
            ThreadCommandV1::Message {
                thread_id: THREAD,
                expected_thread: thread.version,
                message_id: HumanMessageId::new([11; 16]),
                expected_revision: None,
                reply_to: None,
                body: Some(format!("{} NEEDLE near the end", "prefix ".repeat(80))),
            },
        )
        .await;
    let reply = f
        .command(
            12,
            ThreadCommandV1::Message {
                thread_id: THREAD,
                expected_thread: first.version,
                message_id: HumanMessageId::new([12; 16]),
                expected_revision: None,
                reply_to: first.message,
                body: Some("reply needle".into()),
            },
        )
        .await;
    f.work(
        22,
        work::WorkCommandV1::AddComment {
            id: work::WorkCommentId::new([22; 16]),
            task: TASK,
            expected_task_version: 1,
            reply_to: None,
            body: "comment needle".into(),
        },
    )
    .await;
    let page = f.search(&pager, "needle").await;
    assert_eq!(page.results.len(), 5);
    assert!(
        page.results
            .iter()
            .find(|h| h.id == HumanMessageId::new([11; 16]).to_string())
            .unwrap()
            .excerpt
            .contains("NEEDLE")
    );
    assert!(page.snapshot.thread_root.is_some() && page.snapshot.work_root.is_some());
    let scoped = pager
        .page(
            f.writer(),
            ContentQuery {
                q: "needle".into(),
                section: ContentSection::Threads,
                thread: Some(THREAD),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(scoped.results.len(), 2);
    assert!(scoped.results.iter().all(|h| h.thread == Some(THREAD)));
    let current = f
        .command(
            13,
            ThreadCommandV1::Message {
                thread_id: THREAD,
                expected_thread: reply.version,
                message_id: HumanMessageId::new([11; 16]),
                expected_revision: first.message,
                reply_to: None,
                body: Some("replacement text".into()),
            },
        )
        .await;
    f.command(
        14,
        ThreadCommandV1::Message {
            thread_id: THREAD,
            expected_thread: current.version,
            message_id: HumanMessageId::new([12; 16]),
            expected_revision: reply.message,
            reply_to: first.message,
            body: None,
        },
    )
    .await;
    f.work(
        23,
        work::WorkCommandV1::EditComment {
            id: work::WorkCommentId::new([22; 16]),
            expected_version: 1,
            expected_task_version: 2,
            body: "replacement comment".into(),
        },
    )
    .await;
    let after = f.search(&pager, "needle").await;
    assert_eq!(after.results.len(), 2);
    assert!(
        after
            .results
            .iter()
            .all(|h| matches!(h.kind, ContentKind::Project | ContentKind::Task))
    );
    assert_eq!(f.search(&pager, "replacement").await.results.len(), 2);
    f.work(
        24,
        work::WorkCommandV1::TombstoneComment {
            id: work::WorkCommentId::new([22; 16]),
            expected_version: 2,
            expected_task_version: 3,
        },
    )
    .await;
    assert_eq!(f.search(&pager, "replacement").await.results.len(), 1);
}
#[tokio::test]
async fn recent_entities_and_cursor_binding_survive_writes_and_expire() {
    let f = Fixture::new().await;
    let pager = ContentPager::new(f.clock.clone());
    let thread = f.create(10).await;
    let inactive = f.create(15).await;
    f.command(
        16,
        ThreadCommandV1::Lifecycle {
            thread_id: ThreadId::new([15; 16]),
            expected: inactive.version,
            action: ThreadActionV1::Activate,
        },
    )
    .await;
    // A created Thread is recent until explicitly archived or removed.
    let message = f
        .command(
            11,
            ThreadCommandV1::Message {
                thread_id: THREAD,
                expected_thread: thread.version,
                message_id: HumanMessageId::new([11; 16]),
                expected_revision: None,
                reply_to: None,
                body: Some("latest activity".into()),
            },
        )
        .await;
    let query = ContentQuery {
        limit: 1,
        ..Default::default()
    };
    let first = pager.page(f.writer(), query.clone()).await.unwrap();
    assert!(first.next_cursor.is_some());
    assert!(matches!(
        first.results[0].kind,
        ContentKind::Thread | ContentKind::Project | ContentKind::Task
    ));
    f.clock.0.store(1299, Ordering::SeqCst);
    f.create(17).await;
    let next = ContentQuery {
        cursor: first.next_cursor.clone(),
        ..query.clone()
    };
    let second = pager.page(f.writer(), next.clone()).await.unwrap();
    assert_eq!(
        second.snapshot.manifest_digest,
        first.snapshot.manifest_digest
    );
    for changed in [
        ContentQuery {
            q: "changed".into(),
            ..next.clone()
        },
        ContentQuery {
            limit: 2,
            ..next.clone()
        },
        ContentQuery {
            section: ContentSection::Threads,
            ..next.clone()
        },
        ContentQuery {
            thread: Some(THREAD),
            ..next.clone()
        },
    ] {
        assert_eq!(
            pager.page(f.writer(), changed).await.unwrap_err(),
            work::Error::InvalidCursor
        );
    }
    assert_eq!(
        pager
            .page(
                f.session(m::PrincipalId::new([4; 16]), Permission::Admin),
                next.clone()
            )
            .await
            .unwrap_err(),
        work::Error::Denied
    );
    f.command(
        18,
        ThreadCommandV1::Lifecycle {
            thread_id: THREAD,
            expected: message.version,
            action: ThreadActionV1::Activate,
        },
    )
    .await;
    let pin = jj_store_service::collab_reads::PinnedCollabRead::open(
        f.writer(),
        Arc::new(jj_store_service::collab_reads::RepositoryReactionActors(
            REPO,
        )),
    )
    .await
    .unwrap();
    let t = pin.thread(f.writer(), THREAD).await.unwrap();
    f.command(
        19,
        ThreadCommandV1::Lifecycle {
            thread_id: THREAD,
            expected: t.reference,
            action: ThreadActionV1::Archive,
        },
    )
    .await;
    let recent = f.search(&pager, "").await;
    assert!(!recent.results.iter().any(|h| h.thread == Some(THREAD)));
    assert!(
        recent
            .results
            .iter()
            .all(|h| !matches!(h.kind, ContentKind::Message | ContentKind::Comment))
    );
    assert!(
        f.search(&pager, "Thread 10")
            .await
            .results
            .iter()
            .any(|h| h.thread == Some(THREAD))
    );
    f.clock.0.store(1300, Ordering::SeqCst);
    assert_eq!(
        pager
            .page(
                f.writer(),
                ContentQuery {
                    cursor: second.next_cursor,
                    ..query
                }
            )
            .await
            .unwrap_err(),
        work::Error::CursorReset
    );
    assert_eq!(
        pager.page(f.writer(), next).await.unwrap_err(),
        work::Error::CursorReset
    );
}
#[tokio::test]
async fn query_limits_and_foreign_repository_cursor_fail_closed() {
    let f = Fixture::new().await;
    f.create(10).await;
    let pager = ContentPager::new(f.clock.clone());
    let first = pager
        .page(
            f.writer(),
            ContentQuery {
                limit: 1,
                ..Default::default()
            },
        )
        .await
        .unwrap();
    for query in [
        ContentQuery {
            limit: 0,
            ..Default::default()
        },
        ContentQuery {
            limit: 257,
            ..Default::default()
        },
        ContentQuery {
            q: "x".repeat(513),
            ..Default::default()
        },
        ContentQuery {
            thread: Some(THREAD),
            task: Some(TASK),
            ..Default::default()
        },
    ] {
        assert_eq!(
            pager.page(f.writer(), query).await.unwrap_err(),
            work::Error::Invalid
        );
    }
    let other = m::RepositoryId::new([9; 16]);
    let objects = Arc::new(NativeObjects::new(f.store.clone(), other, 8 * 1024 * 1024).unwrap());
    create_repository_local_trusted(
        f.store.clone(),
        objects.clone(),
        Permission::Admin,
        m::EpochId::new([8; 16]),
        Default::default(),
    )
    .await
    .unwrap();
    let session = Arc::new(
        NativeSession::new_local_trusted(
            f.store.clone(),
            objects,
            ACTOR,
            Permission::Read,
            f.clock.clone(),
            Protection::default(),
            SessionLimits::default(),
        )
        .unwrap(),
    );
    assert_eq!(
        pager
            .page(
                session,
                ContentQuery {
                    limit: 1,
                    cursor: first.next_cursor,
                    ..Default::default()
                }
            )
            .await
            .unwrap_err(),
        work::Error::InvalidCursor
    );
}

#[tokio::test]
async fn fresh_and_resumed_content_never_disclose_after_token_revocation() {
    use auth::{Scope, SystemClock, TokenPolicy};
    use jj_store_service::native::create_repository;
    let store = Arc::new(MemoryStore::default());
    let objects = Arc::new(NativeObjects::new(store.clone(), REPO, 8 * 1024 * 1024).unwrap());
    let mut operator = TokenPolicy::new(Scope::Operator);
    let operator_token = operator
        .issue(ACTOR, Permission::CreateRepository, None)
        .unwrap();
    let mut policy = TokenPolicy::new(Scope::Repository(REPO));
    let admin_token = policy.issue(ACTOR, Permission::Admin, None).unwrap();
    let viewer_token = policy
        .issue(m::PrincipalId::new([4; 16]), Permission::Read, None)
        .unwrap();
    create_repository(
        store.clone(),
        objects.clone(),
        &operator,
        &operator_token,
        &policy,
        m::EpochId::new([2; 16]),
        &SystemClock,
        Default::default(),
    )
    .await
    .unwrap();
    let session = |token| {
        Arc::new(
            NativeSession::new(
                store.clone(),
                objects.clone(),
                token,
                Arc::new(SystemClock),
                Protection::default(),
                SessionLimits::default(),
            )
            .unwrap(),
        )
    };
    let admin = session(admin_token);
    let viewer = session(viewer_token.clone());
    for n in [20, 21] {
        work::WorkCommandService
            .apply(
                admin.clone(),
                &work::Request {
                    version: 1,
                    request_id: m::RequestId::new([n; 16]),
                    command: work::WorkCommandV1::CreateProject {
                        id: work::ProjectId::new([n; 16]),
                        title: "Private project".into(),
                        body: "private needle".into(),
                    },
                },
            )
            .await
            .unwrap();
    }
    let pager = ContentPager::new(Arc::new(SystemClock));
    let query = ContentQuery {
        q: "needle".into(),
        limit: 1,
        ..Default::default()
    };
    let first = pager.page(viewer.clone(), query.clone()).await.unwrap();
    let before = admin.snapshot().await.unwrap();
    admin
        .change_token(&jj_store_service::administration::TokenRequest {
            request_id: m::RequestId::new([50; 16]),
            expected: before.manifest().snapshot(),
            change: jj_store_service::administration::TokenChange::Revoke {
                token: viewer_token.id(),
            },
        })
        .await
        .unwrap();
    assert_eq!(
        pager
            .page(
                viewer.clone(),
                ContentQuery {
                    cursor: first.next_cursor,
                    ..query.clone()
                }
            )
            .await
            .unwrap_err(),
        work::Error::Denied
    );
    assert_eq!(
        pager.page(viewer, query).await.unwrap_err(),
        work::Error::Denied
    );
}

#[tokio::test]
async fn source_inventory_limits_apply_before_filtering_results() {
    let f = Fixture::new().await;
    f.create(10).await;
    let session = Arc::new(
        NativeSession::new_local_trusted(
            f.store.clone(),
            f.objects.clone(),
            ACTOR,
            Permission::Read,
            f.clock.clone(),
            Protection::default(),
            SessionLimits {
                inventory_nodes: 1,
                ..Default::default()
            },
        )
        .unwrap(),
    );
    let pager = ContentPager::new(f.clock.clone());
    assert_eq!(
        pager
            .page(
                session,
                ContentQuery {
                    q: "absent body".into(),
                    ..Default::default()
                }
            )
            .await
            .unwrap_err(),
        work::Error::Limit
    );
}
