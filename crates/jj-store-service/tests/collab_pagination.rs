use jj_collab::{
    CollabCommandV1, ReactionCommandV1, ReactionTargetV1, ReactionValueV1, ThreadCommandV1,
    ThreadId,
};
use jj_store_jj::{objects::NativeObjects, policy::Protection};
use jj_store_model as m;
use jj_store_object::memory::MemoryStore;
use jj_store_service::{
    auth::{self, Clock, Permission, SystemClock},
    collab_mutations::{CollabCommandService, Request},
    collab_pages::{CollabPager, Error, Items, Limits, Query},
    collab_reads::{ReactionActorVisibility, RepositoryReactionActors},
    native::{NativeSession, SessionLimits, create_repository_local_trusted},
};
use std::{
    collections::BTreeSet,
    sync::{
        Arc, Condvar, Mutex,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
    time::Duration,
};
const REPO: m::RepositoryId = m::RepositoryId::new([1; 16]);
const ACTOR: m::PrincipalId = m::PrincipalId::new([3; 16]);
struct Time(AtomicU64);
impl Time {
    fn set(&self, n: u64) {
        self.0.store(n, Ordering::SeqCst);
    }
}
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
        use jj_store_service::work::*;
        WorkCommandService
            .apply(
                f.session(ACTOR),
                &Request {
                    version: 1,
                    request_id: m::RequestId::new([99; 16]),
                    command: WorkCommandV1::CreateProject {
                        id: ProjectId::new([99; 16]),
                        title: "Home".into(),
                        body: String::new(),
                    },
                },
            )
            .await
            .unwrap();
        f
    }
    fn session(&self, actor: m::PrincipalId) -> Arc<NativeSession> {
        Arc::new(
            NativeSession::new_local_trusted(
                self.store.clone(),
                self.objects.clone(),
                actor,
                Permission::Admin,
                Arc::new(SystemClock),
                Protection::default(),
                SessionLimits::default(),
            )
            .unwrap(),
        )
    }
    fn pager(&self, limits: Limits) -> CollabPager {
        CollabPager::new(
            limits,
            self.clock.clone(),
            Arc::new(RepositoryReactionActors(REPO)),
        )
        .unwrap()
    }
    async fn create(&self, n: u8) {
        self.apply(
            ACTOR,
            n,
            CollabCommandV1::Thread {
                intent: ThreadCommandV1::Create {
                    thread_id: ThreadId::new([n; 16]),
                    title: format!("Thread {n}"),
                    baseline: None,
                },
            },
        )
        .await;
    }
    async fn apply(&self, actor: m::PrincipalId, n: u8, command: CollabCommandV1) {
        CollabCommandService
            .apply(
                self.session(actor),
                &Request {
                    version: 1,
                    request_id: m::RequestId::new([n; 16]),
                    command,
                },
            )
            .await
            .unwrap();
    }
    async fn react(&self, n: u8, target: ReactionTargetV1) {
        self.apply(
            m::PrincipalId::new([n; 16]),
            n,
            CollabCommandV1::Reaction {
                intent: ReactionCommandV1::Set {
                    target,
                    value: ReactionValueV1::Like,
                    observed: BTreeSet::new(),
                },
            },
        )
        .await;
    }
}
fn ids(items: Items) -> Vec<ThreadId> {
    let Items::Threads { threads } = items else {
        panic!("Thread page")
    };
    threads.into_iter().map(|t| t.value.thread_id).collect()
}

#[tokio::test]
async fn resumed_pages_keep_all_source_roots_after_publication_and_bind_viewer_query_and_size() {
    let f = Fixture::new().await;
    f.create(10).await;
    f.create(11).await;
    let pager = f.pager(Limits::default());
    let session = f.session(ACTOR);
    let query = Query::Threads { limit: 1 };
    let first = pager
        .page(session.clone(), query.clone(), None)
        .await
        .unwrap();
    let token = first.next_cursor.as_deref().unwrap();
    assert_eq!(ids(first.items.clone()), vec![ThreadId::new([10; 16])]);
    f.create(12).await;
    let second = pager
        .page(session.clone(), query.clone(), Some(token))
        .await
        .unwrap();
    assert_eq!(first.snapshot, second.snapshot);
    assert_eq!(ids(second.items), vec![ThreadId::new([11; 16])]);
    assert!(second.next_cursor.is_none());
    assert_ne!(
        pager
            .page(session.clone(), query.clone(), None)
            .await
            .unwrap()
            .snapshot,
        first.snapshot
    );
    for changed in [
        Query::Threads { limit: 2 },
        Query::Emojis { limit: 1 },
        Query::Events {
            thread_id: ThreadId::new([10; 16]),
            limit: 1,
        },
    ] {
        assert_eq!(
            pager
                .page(session.clone(), changed, Some(token))
                .await
                .unwrap_err(),
            Error::InvalidCursor
        );
    }
    assert_eq!(
        pager
            .page(
                f.session(m::PrincipalId::new([4; 16])),
                query.clone(),
                Some(token)
            )
            .await
            .unwrap_err(),
        Error::InvalidCursor
    );
    let mut parts = token.split('.').map(str::to_owned).collect::<Vec<_>>();
    let last = parts[2].pop().unwrap();
    parts[2].push(if last == 'a' { 'b' } else { 'a' });
    let forged = parts.join(".");
    assert_eq!(
        pager
            .page(session.clone(), query.clone(), Some(&forged))
            .await
            .unwrap_err(),
        Error::InvalidCursor
    );
    assert_eq!(
        pager
            .page(session, query, Some(&"a".repeat(4097)))
            .await
            .unwrap_err(),
        Error::InvalidCursor
    );
    let encoded = serde_json::to_value(&first).unwrap();
    assert!(encoded.get("next_id").is_none());
    assert!(encoded["items"].get("next_id").is_none());
}

#[tokio::test]
async fn fixed_expiry_restart_and_capacity_require_explicit_reset_without_fallback() {
    let f = Fixture::new().await;
    f.create(10).await;
    f.create(11).await;
    let limits = Limits {
        snapshots: 1,
        ttl: Duration::from_secs(10),
        ..Limits::default()
    };
    let pager = f.pager(limits);
    let session = f.session(ACTOR);
    let query = Query::Threads { limit: 1 };
    let first = pager
        .page(session.clone(), query.clone(), None)
        .await
        .unwrap();
    let token = first.next_cursor.as_deref().unwrap();
    assert_eq!(
        pager
            .page(session.clone(), query.clone(), None)
            .await
            .unwrap_err(),
        Error::Busy
    );
    f.clock.set(1009);
    let second = pager
        .page(session.clone(), query.clone(), Some(token))
        .await
        .unwrap();
    assert_eq!(second.snapshot, first.snapshot);
    f.clock.set(1010);
    assert_eq!(
        pager
            .page(session.clone(), query.clone(), Some(token))
            .await
            .unwrap_err(),
        Error::CursorReset
    );
    let fresh = pager
        .page(session.clone(), query.clone(), None)
        .await
        .unwrap();
    assert!(fresh.next_cursor.is_some());
    let restarted = f.pager(limits);
    assert_eq!(
        restarted
            .page(session.clone(), query.clone(), fresh.next_cursor.as_deref())
            .await
            .unwrap_err(),
        Error::CursorReset
    );
    f.clock.set(999);
    assert_eq!(
        pager
            .page(session, query, fresh.next_cursor.as_deref())
            .await
            .unwrap_err(),
        Error::CursorReset
    );
}

#[tokio::test]
async fn actor_aggregation_and_exact_target_remain_bound_across_reaction_pages() {
    let f = Fixture::new().await;
    f.create(10).await;
    f.create(11).await;
    let target = ReactionTargetV1::Thread {
        thread_id: ThreadId::new([10; 16]),
    };
    f.react(20, target.clone()).await;
    f.react(21, target.clone()).await;
    let pager = f.pager(Limits::default());
    let session = f.session(ACTOR);
    let query = Query::Reactions {
        target: target.clone(),
        limit: 1,
    };
    let first = pager
        .page(session.clone(), query.clone(), None)
        .await
        .unwrap();
    let Items::Reactions {
        reactions, actors, ..
    } = &first.items
    else {
        panic!()
    };
    assert_eq!(reactions[0].count, 2);
    assert_eq!(actors.len(), 1);
    f.react(22, target.clone()).await;
    let next = pager
        .page(session.clone(), query.clone(), first.next_cursor.as_deref())
        .await
        .unwrap();
    assert_eq!(first.snapshot, next.snapshot);
    let Items::Reactions {
        reactions, actors, ..
    } = next.items
    else {
        panic!()
    };
    assert_eq!(reactions[0].count, 2);
    assert_eq!(actors[0].actor, m::PrincipalId::new([21; 16]));
    assert_eq!(
        pager
            .page(
                session,
                Query::Reactions {
                    target: ReactionTargetV1::Thread {
                        thread_id: ThreadId::new([11; 16])
                    },
                    limit: 1
                },
                first.next_cursor.as_deref()
            )
            .await
            .unwrap_err(),
        Error::InvalidCursor
    );
}

#[tokio::test]
async fn history_and_emoji_pages_keep_pinned_membership_and_query_kind() {
    let f = Fixture::new().await;
    f.create(10).await;
    let session = f.session(ACTOR);
    let pin = jj_store_service::collab_reads::PinnedCollabRead::open(
        session.clone(),
        Arc::new(RepositoryReactionActors(REPO)),
    )
    .await
    .unwrap();
    let selected = pin
        .thread(session.clone(), ThreadId::new([10; 16]))
        .await
        .unwrap();
    f.apply(
        ACTOR,
        11,
        CollabCommandV1::Thread {
            intent: ThreadCommandV1::Rename {
                thread_id: ThreadId::new([10; 16]),
                expected: selected.reference,
                title: "Renamed".into(),
            },
        },
    )
    .await;
    let pager = f.pager(Limits::default());
    let query = Query::Events {
        thread_id: ThreadId::new([10; 16]),
        limit: 1,
    };
    let first = pager
        .page(session.clone(), query.clone(), None)
        .await
        .unwrap();
    let token = first.next_cursor.as_deref().unwrap();
    let next = pager
        .page(session.clone(), query, Some(token))
        .await
        .unwrap();
    assert_eq!(next.snapshot, first.snapshot);
    assert!(next.next_cursor.is_none());
    let Items::Events { events, .. } = next.items else {
        panic!()
    };
    assert!(matches!(
        events[0].value.event,
        jj_collab::ThreadEventKindV1::Created
    ));
    assert_eq!(
        pager
            .page(
                session.clone(),
                Query::Events {
                    thread_id: ThreadId::new([12; 16]),
                    limit: 1
                },
                Some(token)
            )
            .await
            .unwrap_err(),
        Error::InvalidCursor
    );
    for n in [20, 21] {
        let mut bytes = std::io::Cursor::new(Vec::new());
        image::DynamicImage::new_rgba8(1, 1)
            .write_to(&mut bytes, image::ImageFormat::Png)
            .unwrap();
        f.apply(
            ACTOR,
            n,
            CollabCommandV1::Emoji {
                intent: jj_collab::CustomEmojiCommandV1::Define {
                    emoji_id: jj_collab::CustomEmojiId::new([n; 16]),
                    expected: None,
                    shortcode: format!("emoji_{n}"),
                    label: "Custom".into(),
                    bytes: bytes.into_inner(),
                },
            },
        )
        .await;
    }
    let first = pager
        .page(session.clone(), Query::Emojis { limit: 1 }, None)
        .await
        .unwrap();
    let next = pager
        .page(
            session.clone(),
            Query::Emojis { limit: 1 },
            first.next_cursor.as_deref(),
        )
        .await
        .unwrap();
    assert_eq!(first.snapshot, next.snapshot);
    assert!(next.next_cursor.is_none());
    let Items::Emojis {
        definitions,
        versions,
    } = next.items
    else {
        panic!()
    };
    assert_eq!(
        definitions[0].emoji_id,
        jj_collab::CustomEmojiId::new([21; 16])
    );
    assert_eq!(versions.len(), 1);
    assert_eq!(
        versions.get(&definitions[0].emoji_id),
        Some(
            &jj_collab::CollabObjectV1::CustomEmojiVersion(definitions[0].clone())
                .reference()
                .unwrap()
        )
    );
    assert_eq!(
        pager
            .page(
                session,
                Query::Threads { limit: 1 },
                first.next_cursor.as_deref()
            )
            .await
            .unwrap_err(),
        Error::InvalidCursor
    );
}

#[tokio::test]
async fn object_byte_deadline_and_configuration_budgets_fail_closed() {
    let f = Fixture::new().await;
    f.create(10).await;
    f.create(11).await;
    for limits in [
        Limits {
            objects: 1,
            ..Limits::default()
        },
        Limits {
            encoded_collab_bytes: 1,
            ..Limits::default()
        },
        Limits {
            timeout: Duration::from_nanos(1),
            ..Limits::default()
        },
    ] {
        assert_eq!(
            f.pager(limits)
                .page(f.session(ACTOR), Query::Threads { limit: 1 }, None)
                .await
                .unwrap_err(),
            Error::Limit
        );
    }
    for limits in [
        Limits {
            snapshots: 0,
            ..Limits::default()
        },
        Limits {
            objects: 0,
            ..Limits::default()
        },
        Limits {
            encoded_collab_bytes: 0,
            ..Limits::default()
        },
        Limits {
            concurrency: 0,
            ..Limits::default()
        },
        Limits {
            ttl: Duration::from_millis(10),
            ..Limits::default()
        },
        Limits {
            timeout: Duration::ZERO,
            ..Limits::default()
        },
    ] {
        assert!(matches!(
            CollabPager::new(
                limits,
                f.clock.clone(),
                Arc::new(RepositoryReactionActors(REPO))
            ),
            Err(Error::Limit)
        ));
    }
    for limit in [0, 257] {
        assert_eq!(
            f.pager(Limits::default())
                .page(f.session(ACTOR), Query::Threads { limit }, None)
                .await
                .unwrap_err(),
            Error::Read(jj_store_service::collab_mutations::Error::Invalid)
        );
    }
}

struct GateActors {
    armed: AtomicBool,
    entered: tokio::sync::Notify,
    released: Mutex<bool>,
    release: Condvar,
}
impl ReactionActorVisibility for GateActors {
    fn can_disclose(&self, _: m::RepositoryId, _: m::PrincipalId, _: m::PrincipalId) -> bool {
        if self.armed.swap(false, Ordering::SeqCst) {
            self.entered.notify_one();
            let mut released = self.released.lock().unwrap();
            while !*released {
                released = self.release.wait(released).unwrap();
            }
        }
        true
    }
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn concurrent_queries_fail_fast_without_waiting_for_pin_capacity() {
    let f = Fixture::new().await;
    f.create(10).await;
    let target = ReactionTargetV1::Thread {
        thread_id: ThreadId::new([10; 16]),
    };
    f.react(20, target.clone()).await;
    let gate = Arc::new(GateActors {
        armed: AtomicBool::new(true),
        entered: tokio::sync::Notify::new(),
        released: Mutex::new(false),
        release: Condvar::new(),
    });
    let pager = Arc::new(
        CollabPager::new(
            Limits {
                concurrency: 1,
                ..Limits::default()
            },
            f.clock.clone(),
            gate.clone(),
        )
        .unwrap(),
    );
    let task_pager = pager.clone();
    let session = f.session(ACTOR);
    let first = tokio::spawn(async move {
        task_pager
            .page(session, Query::Reactions { target, limit: 1 }, None)
            .await
    });
    tokio::time::timeout(Duration::from_secs(5), gate.entered.notified())
        .await
        .unwrap();
    let blocked = pager
        .page(f.session(ACTOR), Query::Threads { limit: 1 }, None)
        .await;
    *gate.released.lock().unwrap() = true;
    gate.release.notify_all();
    assert_eq!(blocked.unwrap_err(), Error::Busy);
    first.await.unwrap().unwrap();
    pager
        .page(f.session(ACTOR), Query::Threads { limit: 1 }, None)
        .await
        .unwrap();
}

#[tokio::test]
async fn resumed_pages_reauthorize_current_tokens_and_never_store_credentials() {
    use auth::{Scope, TokenPolicy};
    let store = Arc::new(MemoryStore::default());
    let objects = Arc::new(NativeObjects::new(store.clone(), REPO, 8 * 1024 * 1024).unwrap());
    let mut operator = TokenPolicy::new(Scope::Operator);
    let operator_token = operator
        .issue(ACTOR, Permission::CreateRepository, None)
        .unwrap();
    let mut policy = TokenPolicy::new(Scope::Repository(REPO));
    let admin = policy.issue(ACTOR, Permission::Admin, None).unwrap();
    let viewer = policy
        .issue(m::PrincipalId::new([4; 16]), Permission::Read, None)
        .unwrap();
    jj_store_service::native::create_repository(
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
    let admin = session(admin);
    let viewer_session = session(viewer.clone());
    use jj_store_service::work;
    work::WorkCommandService
        .apply(
            admin.clone(),
            &work::Request {
                version: 1,
                request_id: m::RequestId::new([99; 16]),
                command: work::WorkCommandV1::CreateProject {
                    id: work::ProjectId::new([99; 16]),
                    title: "Home".into(),
                    body: String::new(),
                },
            },
        )
        .await
        .unwrap();
    for n in [10u8, 11] {
        CollabCommandService
            .apply(
                admin.clone(),
                &Request {
                    version: 1,
                    request_id: m::RequestId::new([n; 16]),
                    command: CollabCommandV1::Thread {
                        intent: ThreadCommandV1::Create {
                            thread_id: ThreadId::new([n; 16]),
                            title: "Thread".into(),
                            baseline: None,
                        },
                    },
                },
            )
            .await
            .unwrap();
    }
    let pager = CollabPager::new(
        Limits::default(),
        Arc::new(SystemClock),
        Arc::new(RepositoryReactionActors(REPO)),
    )
    .unwrap();
    let query = Query::Threads { limit: 1 };
    let first = pager
        .page(viewer_session.clone(), query.clone(), None)
        .await
        .unwrap();
    let before = admin.snapshot().await.unwrap();
    admin
        .change_token(&jj_store_service::administration::TokenRequest {
            request_id: m::RequestId::new([50; 16]),
            expected: before.manifest().snapshot(),
            change: jj_store_service::administration::TokenChange::Revoke { token: viewer.id() },
        })
        .await
        .unwrap();
    assert_eq!(
        pager
            .page(
                viewer_session.clone(),
                query.clone(),
                first.next_cursor.as_deref()
            )
            .await
            .unwrap_err(),
        Error::Read(jj_store_service::collab_mutations::Error::Denied)
    );
    assert_eq!(
        pager.page(viewer_session, query, None).await.unwrap_err(),
        Error::Read(jj_store_service::collab_mutations::Error::Denied)
    );
}

#[tokio::test]
async fn absent_domains_stay_absent_and_final_pages_do_not_consume_pin_capacity() {
    let f = Fixture::new().await;
    let session = f.session(ACTOR);
    let before = session.snapshot().await.unwrap();
    let pager = f.pager(Limits {
        snapshots: 1,
        ..Limits::default()
    });
    for query in [
        Query::Threads { limit: 1 },
        Query::Emojis { limit: 1 },
        Query::Threads { limit: 1 },
    ] {
        let page = pager.page(session.clone(), query, None).await.unwrap();
        assert!(page.next_cursor.is_none());
        assert!(page.snapshot.thread_root.is_none());
        assert!(page.snapshot.reaction_root.is_none());
    }
    assert_eq!(session.snapshot().await.unwrap().digest(), before.digest());
}
