use super::*;
use jj_store_jj::query as q;
use jj_store_service::{
    pagination as p,
    reads::{OpenRead, ReadLimits},
};
use std::{
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};

struct TestClock(AtomicU64);
impl Clock for TestClock {
    fn unix_seconds(&self) -> jj_store_service::auth::Result<u64> {
        Ok(self.0.load(Ordering::SeqCst))
    }
}
fn pager(path: &std::path::Path, limits: p::Limits, clock: Arc<dyn Clock>) -> p::GraphPager {
    p::GraphPager::new(
        q::GraphExecutor::new(path.to_owned(), q::Limits::default(), 4).unwrap(),
        limits,
        clock,
    )
    .unwrap()
}
fn page(start: p::Start) -> p::Page {
    let p::Start::Page(page) = start else {
        panic!("expected graph page");
    };
    *page
}
async fn history(
    f: &Fixture,
    cache: &std::path::Path,
    work: &std::path::Path,
) -> (Arc<ReadonlyRepo>, jj_lib::commit::Commit) {
    let repo = init_working(f, cache, work).await;
    let id = repo
        .view()
        .get_wc_commit_id(jj_lib::ref_name::WorkspaceName::new("fixture"))
        .unwrap();
    let commit = repo.store().get_commit_async(id).await.unwrap();
    describe(&repo, &commit, "cursor history").await
}
struct NoPublications;
#[async_trait::async_trait]
impl PublicationGuard for NoPublications {
    async fn authorize(&self, _: &ManifestSnapshot, _: &Request) -> wal::Result<()> {
        Err(wal::Error::Denied)
    }
    async fn validate_transition(
        &self,
        _: &ManifestSnapshot,
        _: &Request,
        _: &Snapshot,
    ) -> wal::Result<()> {
        Err(wal::Error::Denied)
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn graph_cursor_retains_exact_snapshot_through_publication_checkpoint_and_replay() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let cache = tempfile::tempdir().unwrap();
            let work = tempfile::tempdir().unwrap();
            let graphs = tempfile::tempdir().unwrap();
            let (repo, commit) = history(&f, cache.path(), work.path()).await;
            let pager = pager(graphs.path(), p::Limits::default(), Arc::new(SystemClock));
            let before = f.session.snapshot().await.unwrap().manifest().clone();
            let first = page(
                pager
                    .begin(f.session.clone(), None, q::Request::Operations, 1)
                    .await
                    .unwrap(),
            );
            assert_eq!(first.snapshot.operation_id, oid(repo.op_id()));
            assert!(first.graph.next.is_none());
            let cursor = first.next_cursor.unwrap();
            let (later, _) = describe(&repo, &commit, "after first page").await;
            let checkpoint = Coordinator::new(
                f.store.clone(),
                REPO,
                f.objects.clone(),
                Arc::new(NoPublications),
                Limits::default(),
            )
            .unwrap()
            .checkpoint()
            .await
            .unwrap();
            assert!(checkpoint.manifest().revision > before.revision);
            let current = checkpoint.manifest().clone();
            let continued = pager
                .resume(
                    f.session(f.reader.clone(), Protection::default()),
                    &cursor,
                    Some(oid(repo.op_id())),
                    q::Request::Operations,
                    1,
                )
                .await
                .unwrap();
            assert_eq!(continued.snapshot, first.snapshot);
            assert!(continued.graph.next.is_none());
            let again = pager
                .resume(f.session.clone(), &cursor, None, q::Request::Operations, 1)
                .await
                .unwrap();
            assert_eq!(again.snapshot, first.snapshot);
            assert_eq!(again.graph.data, continued.graph.data);
            assert_eq!(again.next_cursor, continued.next_cursor);
            let mut collected = vec![];
            let mut next = Some(cursor);
            while let Some(cursor) = next {
                let result = pager
                    .resume(f.session.clone(), &cursor, None, q::Request::Operations, 1)
                    .await
                    .unwrap();
                let q::Nodes::Operations(nodes) = result.graph.data else {
                    unreachable!()
                };
                collected.extend(nodes.into_iter().map(|n| n.id));
                next = result.next_cursor;
            }
            assert!(!collected.contains(&oid(later.op_id())));
            assert_eq!(collected.last(), Some(&roots::ROOT_OPERATION));
            assert_eq!(*f.session.snapshot().await.unwrap().manifest(), current);
            // Cursor APIs neither use nor expose a deserializable read capability.
            let OpenRead::Pinned(pin) = f
                .session
                .open_read(Some(oid(repo.op_id())), ReadLimits::default())
                .await
                .unwrap()
            else {
                unreachable!()
            };
            let exec =
                q::GraphExecutor::new(graphs.path().to_owned(), q::Limits::default(), 1).unwrap();
            let expected = Arc::new(pin)
                .graph(
                    &exec,
                    q::Request::Operations,
                    q::Page {
                        limit: 100,
                        after: Some(m::ObjectKey::Operation(oid(repo.op_id()))),
                    },
                )
                .await
                .unwrap();
            let q::Nodes::Operations(nodes) = expected.data else {
                unreachable!()
            };
            assert_eq!(
                collected,
                nodes.into_iter().map(|n| n.id).collect::<Vec<_>>()
            );
        })
        .await;
}
fn log() -> q::Request {
    q::Request::Log {
        revset: "all()".into(),
        workspace: Some("fixture".into()),
        user_email: "cursor@example.invalid".into(),
        at_time_millis: 1_790_000_000_000,
    }
}
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn graph_cursor_rejects_tampering_changed_context_and_current_revocation() {
    use jj_store_service::administration::{TokenChange, TokenRequest};
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let cache = tempfile::tempdir().unwrap();
            let work = tempfile::tempdir().unwrap();
            let graphs = tempfile::tempdir().unwrap();
            let (repo, _) = history(&f, cache.path(), work.path()).await;
            let pager = pager(graphs.path(), p::Limits::default(), Arc::new(SystemClock));
            let reader = f.session(f.reader.clone(), Protection::default());
            let first = page(pager.begin(reader.clone(), None, log(), 1).await.unwrap());
            let token = first.next_cursor.unwrap();
            let other_id = m::RepositoryId::new([232; 16]);
            let other_objects =
                Arc::new(NativeObjects::new(f.store.clone(), other_id, 4 * 1024 * 1024).unwrap());
            let mut policy = TokenPolicy::new(Scope::Repository(other_id));
            let other_token = policy.issue(ACTOR, Permission::Admin, None).unwrap();
            let mut operator = TokenPolicy::new(Scope::Operator);
            let operator_token = operator
                .issue(ACTOR, Permission::CreateRepository, None)
                .unwrap();
            create_repository(
                f.store.clone(),
                other_objects.clone(),
                &operator,
                &operator_token,
                &policy,
                m::EpochId::new([233; 16]),
                &SystemClock,
                Limits::default(),
            )
            .await
            .unwrap();
            let other_session = Arc::new(
                NativeSession::new(
                    f.store.clone(),
                    other_objects,
                    other_token,
                    Arc::new(SystemClock),
                    Protection::default(),
                    SessionLimits::default(),
                )
                .unwrap(),
            );
            assert_eq!(
                pager
                    .resume(other_session, &token, None, log(), 1)
                    .await
                    .unwrap_err(),
                p::Error::InvalidCursor
            );
            let mut altered = vec![q::Request::Operations];
            for field in 0..4 {
                let mut request = log();
                let q::Request::Log {
                    revset,
                    workspace,
                    user_email,
                    at_time_millis,
                } = &mut request
                else {
                    unreachable!()
                };
                match field {
                    0 => *revset = "root()".into(),
                    1 => *workspace = None,
                    2 => *user_email = "another@example.invalid".into(),
                    _ => *at_time_millis += 1,
                }
                altered.push(request);
            }
            for request in altered {
                assert_eq!(
                    pager
                        .resume(reader.clone(), &token, None, request, 1)
                        .await
                        .unwrap_err(),
                    p::Error::InvalidCursor
                );
            }
            assert_eq!(
                pager
                    .resume(reader.clone(), &token, None, log(), 2)
                    .await
                    .unwrap_err(),
                p::Error::InvalidCursor
            );
            assert_eq!(
                pager
                    .resume(
                        reader.clone(),
                        &token,
                        Some(roots::ROOT_OPERATION),
                        log(),
                        1
                    )
                    .await
                    .unwrap_err(),
                p::Error::InvalidCursor
            );
            let mut tampered = token.clone().into_bytes();
            let index = token.rfind('.').unwrap() + 1;
            tampered[index] = if tampered[index] == b'A' { b'B' } else { b'A' };
            for invalid in [
                String::from_utf8(tampered).unwrap(),
                token[..token.len() - 1].into(),
                "x".repeat(4097),
            ] {
                assert_eq!(
                    pager
                        .resume(reader.clone(), &invalid, None, log(), 1)
                        .await
                        .unwrap_err(),
                    p::Error::InvalidCursor
                );
            }
            assert!(
                pager
                    .resume(reader.clone(), &token, Some(oid(repo.op_id())), log(), 1)
                    .await
                    .is_ok()
            );
            f.session
                .change_token(&TokenRequest {
                    request_id: m::RequestId::new([230; 16]),
                    expected: f.session.snapshot().await.unwrap().manifest().snapshot(),
                    change: TokenChange::Revoke {
                        token: f.reader.id(),
                    },
                })
                .await
                .unwrap();
            for cursor in [&token, "invalid"] {
                assert_eq!(
                    pager
                        .resume(reader.clone(), cursor, None, log(), 1)
                        .await
                        .unwrap_err(),
                    p::Error::Denied
                );
            }
            assert!(format!("{pager:?}").contains("[REDACTED]"));
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn graph_cursor_expiry_restart_capacity_and_epoch_changes_require_reselection() {
    use jj_store_object::{Condition, Key, ObjectStore as _, StagedUpload, WriteOutcome};
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let cache = tempfile::tempdir().unwrap();
            let work = tempfile::tempdir().unwrap();
            let graphs = tempfile::tempdir().unwrap();
            let _ = history(&f, cache.path(), work.path()).await;
            let clock = Arc::new(TestClock(AtomicU64::new(1000)));
            let limits = p::Limits {
                snapshots: 1,
                ttl: Duration::from_secs(2),
                ..p::Limits::default()
            };
            let p = pager(graphs.path(), limits, clock.clone());
            let first = page(
                p.begin(f.session.clone(), None, q::Request::Operations, 1)
                    .await
                    .unwrap(),
            );
            let token = first.next_cursor.unwrap();
            assert_eq!(
                p.begin(f.session.clone(), None, q::Request::Operations, 1)
                    .await
                    .unwrap_err(),
                p::Error::Busy
            );
            let tiny = pager(
                graphs.path(),
                p::Limits {
                    inventory_nodes: 1,
                    ..limits
                },
                clock.clone(),
            );
            assert_eq!(
                tiny.begin(f.session.clone(), None, q::Request::Operations, 1)
                    .await
                    .unwrap_err(),
                p::Error::Busy
            );
            let restarted = pager(graphs.path(), limits, clock.clone());
            assert_eq!(
                restarted
                    .resume(f.session.clone(), &token, None, q::Request::Operations, 1)
                    .await
                    .unwrap_err(),
                p::Error::CursorReset
            );
            clock.0.store(1002, Ordering::SeqCst);
            assert_eq!(
                p.resume(f.session.clone(), &token, None, q::Request::Operations, 1)
                    .await
                    .unwrap_err(),
                p::Error::CursorReset
            );
            let next = page(
                p.begin(f.session.clone(), None, q::Request::Operations, 1)
                    .await
                    .unwrap(),
            );
            let key = Key::new(format!("repos/{REPO}/manifest.json")).unwrap();
            let download = f.store.get(&key, None).await.unwrap();
            let version = download.metadata.version.clone();
            let mut manifest: Manifest =
                serde_json::from_slice(&download.read_bounded(128 * 1024).await.unwrap()).unwrap();
            manifest.epoch = m::EpochId::new([231; 16]);
            let upload =
                StagedUpload::from_bytes(&serde_json::to_vec(&manifest).unwrap(), 128 * 1024)
                    .await
                    .unwrap();
            assert!(matches!(
                f.store
                    .put(&key, Condition::Matches(version), &upload)
                    .await
                    .unwrap(),
                WriteOutcome::Applied(_)
            ));
            assert_eq!(
                p.resume(
                    f.session.clone(),
                    next.next_cursor.as_ref().unwrap(),
                    None,
                    q::Request::Operations,
                    1
                )
                .await
                .unwrap_err(),
                p::Error::Read(jj_store_service::reads::Error::EpochChanged)
            );
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn graph_cursor_renews_request_deadline_keeps_head_selection_explicit_and_pages_evolution() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let cache = tempfile::tempdir().unwrap();
            let work = tempfile::tempdir().unwrap();
            let graphs = tempfile::tempdir().unwrap();
            let (base, original) = history(&f, cache.path(), work.path()).await;
            let (left, tip) = describe(&base, &original, "left").await;
            let (right, _) = describe(&base, &original, "right").await;
            let before = f.session.snapshot().await.unwrap().manifest().clone();
            assert_eq!(
                before.heads,
                BTreeSet::from([oid(left.op_id()), oid(right.op_id())])
            );
            let pager = pager(
                graphs.path(),
                p::Limits {
                    snapshots: 1,
                    read: ReadLimits {
                        timeout: Duration::from_secs(1),
                        ..ReadLimits::default()
                    },
                    ..p::Limits::default()
                },
                Arc::new(SystemClock),
            );
            for _ in 0..2 {
                let p::Start::SelectHead { snapshot } = pager
                    .begin(f.session.clone(), None, q::Request::Operations, 1)
                    .await
                    .unwrap()
                else {
                    panic!("must select an operation");
                };
                assert_eq!(snapshot.heads, before.heads);
            }
            // Fully delivered pages consume no retained cursor slot.
            for _ in 0..2 {
                let complete = page(
                    pager
                        .begin(
                            f.session.clone(),
                            Some(roots::ROOT_OPERATION),
                            q::Request::Operations,
                            1,
                        )
                        .await
                        .unwrap(),
                );
                assert!(complete.next_cursor.is_none());
            }
            let request = q::Request::Evolution {
                commit: cid(tip.id()),
            };
            let start = page(
                pager
                    .begin(
                        f.session.clone(),
                        Some(oid(left.op_id())),
                        request.clone(),
                        1,
                    )
                    .await
                    .unwrap(),
            );
            let cursor = start.next_cursor.unwrap();
            tokio::time::sleep(Duration::from_millis(1100)).await;
            let resumed = pager
                .resume(f.session.clone(), &cursor, None, request, 1)
                .await
                .unwrap();
            assert_eq!(resumed.snapshot, start.snapshot);
            let q::Nodes::Evolution(nodes) = resumed.graph.data else {
                unreachable!()
            };
            assert_eq!(nodes[0].commit.id, cid(original.id()));
            assert_eq!(*f.session.snapshot().await.unwrap().manifest(), before);
        })
        .await;
}
