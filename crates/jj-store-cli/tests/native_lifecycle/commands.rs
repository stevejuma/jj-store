use super::*;
use jj_store_service::commands::{Command, Error, Executor, Limits};

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn typed_describe_prepares_native_descendants_without_publication() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let cache = tempfile::tempdir().unwrap();
            let settings = settings();
            let loader = f
                .stores
                .initialize_cache(&settings, cache.path())
                .await
                .unwrap();
            let repo = f
                .stores
                .load_at(&loader, roots::ROOT_OPERATION)
                .await
                .unwrap();
            let mut tx = repo.start_transaction();
            let first = tx
                .repo_mut()
                .new_commit(
                    vec![repo.store().root_commit_id().clone()],
                    repo.store().empty_merged_tree(),
                )
                .set_description("before")
                .set_author(b::Signature {
                    name: "Existing author".into(),
                    email: "author@example.invalid".into(),
                    timestamp: b::Timestamp {
                        timestamp: b::MillisSinceEpoch(-1234),
                        tz_offset: 345,
                    },
                })
                .write()
                .await
                .unwrap();
            let child = tx
                .repo_mut()
                .new_commit(vec![first.id().clone()], first.tree())
                .set_description("child")
                .set_author(first.author().clone())
                .write()
                .await
                .unwrap();
            tx.repo_mut()
                .set_wc_commit("browser".into(), child.id().clone())
                .unwrap();
            tx.repo_mut().set_local_bookmark_target(
                "topic".as_ref(),
                o::RefTarget::normal(first.id().clone()),
            );
            let original = tx.commit("command seed").await.unwrap();
            let expected = f.session.snapshot().await.unwrap().manifest().snapshot();
            let executor = Executor::new(Limits::default(), 2).unwrap();
            let prepared = f
                .session
                .prepare_command(
                    &executor,
                    &expected,
                    Command::Describe {
                        commit: cid(first.id()),
                        description: "after".into(),
                    },
                )
                .await
                .unwrap();
            assert_eq!(
                f.session.snapshot().await.unwrap().manifest().snapshot(),
                expected
            );
            assert_eq!(prepared.rewrites.len(), 2);
            let candidate = f.stores.load_at(&loader, prepared.operation).await.unwrap();
            let rewritten = prepared
                .rewrites
                .iter()
                .find(|rewrite| rewrite.predecessors == [cid(first.id())])
                .unwrap();
            let new_first = candidate
                .store()
                .get_commit(&b::CommitId::from_bytes(rewritten.commit.as_bytes()))
                .unwrap();
            assert_eq!(new_first.description(), "after");
            assert_eq!(new_first.change_id(), first.change_id());
            assert_eq!(new_first.author(), first.author());
            assert_eq!(new_first.tree_ids(), first.tree_ids());
            assert_eq!(new_first.committer().name, ACTOR.to_string());
            assert_eq!(
                candidate
                    .view()
                    .get_local_bookmark("topic".as_ref())
                    .as_normal(),
                Some(new_first.id())
            );
            let new_child = candidate
                .store()
                .get_commit(
                    candidate
                        .view()
                        .get_wc_commit_id("browser".as_ref())
                        .unwrap(),
                )
                .unwrap();
            assert_eq!(new_child.parent_ids(), &[new_first.id().clone()]);
            assert_eq!(new_child.change_id(), child.change_id());
            assert_eq!(new_child.author(), child.author());
            assert_eq!(new_child.tree_ids(), child.tree_ids());
            assert_eq!(prepared.workspaces.len(), 1);
            assert_eq!(prepared.bookmarks.len(), 1);
            assert_eq!(
                candidate.operation().parent_ids(),
                &[original.op_id().clone()]
            );
            let request = Request {
                principal: ACTOR,
                request_id: m::RequestId::new([71; 16]),
                publication_id: m::PublicationId::new([72; 16]),
                transition: Some(Transition::Strict {
                    expected,
                    heads: BTreeSet::from([prepared.operation]),
                }),
                settings: None,
                result: vec![],
                kind: SemanticKind::OperationPublished,
            };
            assert!(matches!(
                f.session.publish_request(&request).await.unwrap(),
                PublishOutcome::Committed(_)
            ));
        })
        .await;
}

struct Branches {
    f: Fixture,
    _cache: tempfile::TempDir,
    repo: Arc<ReadonlyRepo>,
    left: m::CommitId,
    right: m::CommitId,
}
impl Branches {
    async fn new() -> Self {
        use jj_lib::{merge::Merge, repo_path::RepoPathComponentBuf};
        let f = Fixture::new().await;
        let cache = tempfile::tempdir().unwrap();
        let settings = settings();
        let loader = f
            .stores
            .initialize_cache(&settings, cache.path())
            .await
            .unwrap();
        let root = f
            .stores
            .load_at(&loader, roots::ROOT_OPERATION)
            .await
            .unwrap();
        let mut ids = vec![];
        for n in [41u8, 42] {
            let file = f.objects.put_file([n, b'\n'].as_slice()).await.unwrap().0;
            let tree = b::Tree::from_sorted_entries(vec![(
                RepoPathComponentBuf::new("file").unwrap(),
                b::TreeValue::File {
                    id: b::FileId::from_bytes(file.as_bytes()),
                    executable: false,
                    copy_id: b::CopyId::placeholder(),
                },
            )]);
            let tree = f.objects.put(&tree).await.unwrap().0;
            let mut commit = roots::commit();
            commit.parents = vec![root.store().root_commit_id().clone()];
            commit.change_id = b::ChangeId::from_bytes(&[n; 16]);
            commit.root_tree = Merge::resolved(b::TreeId::from_bytes(tree.as_bytes()));
            commit.description = format!("branch {n}");
            let id = f.objects.put(&commit).await.unwrap().0;
            ids.push(m::CommitId::from_slice(id.as_bytes()).unwrap());
        }
        let native = |id: m::CommitId| b::CommitId::from_bytes(id.as_bytes());
        let mut view = root.view().store_view().clone();
        view.head_ids = ids.iter().map(|&id| native(id)).collect();
        view.wc_commit_ids.insert("browser".into(), native(ids[0]));
        view.local_bookmarks.insert(
            "topic".into(),
            o::RefTarget::from_merge(Merge::from_vec(vec![
                Some(native(ids[0])),
                None,
                Some(native(ids[1])),
            ])),
        );
        let operation = stage_view(&f, &root, view, "two native branches").await;
        f.session
            .publish(BTreeSet::from([roots::ROOT_OPERATION]), operation)
            .await
            .unwrap();
        let repo = f.stores.load_at(&loader, operation).await.unwrap();
        Self {
            f,
            _cache: cache,
            repo,
            left: ids[0],
            right: ids[1],
        }
    }
    async fn current(&self) -> Snapshot {
        self.f
            .session
            .snapshot()
            .await
            .unwrap()
            .manifest()
            .snapshot()
    }
    async fn prepare(
        &self,
        command: Command,
    ) -> std::result::Result<jj_store_service::commands::Prepared, Error> {
        self.f
            .session
            .prepare_command(
                &Executor::new(Limits::default(), 2).unwrap(),
                &self.current().await,
                command,
            )
            .await
    }
    async fn publish(
        &self,
        expected: Snapshot,
        prepared: &jj_store_service::commands::Prepared,
        n: u8,
    ) {
        let request = Request {
            principal: ACTOR,
            request_id: m::RequestId::new([n; 16]),
            publication_id: m::PublicationId::new([n; 16]),
            transition: Some(Transition::Strict {
                expected,
                heads: BTreeSet::from([prepared.operation]),
            }),
            settings: None,
            result: vec![],
            kind: SemanticKind::OperationPublished,
        };
        assert!(matches!(
            self.f.session.publish_request(&request).await.unwrap(),
            PublishOutcome::Committed(_)
        ));
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn typed_new_merges_ordered_parents_and_moves_only_logical_workspace() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let seed = Branches::new().await;
            let expected = seed.current().await;
            let parents: Vec<_> = [seed.right, seed.left]
                .iter()
                .map(|id| {
                    seed.repo
                        .store()
                        .get_commit(&b::CommitId::from_bytes(id.as_bytes()))
                        .unwrap()
                })
                .collect();
            let tree = jj_lib::rewrite::merge_commit_trees(seed.repo.as_ref(), &parents)
                .await
                .unwrap();
            let result = seed
                .prepare(Command::New {
                    parents: vec![seed.right, seed.left],
                    description: "merged new".into(),
                    workspace: Some("browser".into()),
                })
                .await
                .unwrap();
            assert_eq!(seed.current().await, expected);
            let native = seed
                .repo
                .store()
                .get_commit(&b::CommitId::from_bytes(result.created[0].as_bytes()))
                .unwrap();
            assert_eq!(
                native.parent_ids(),
                &parents.iter().map(|p| p.id().clone()).collect::<Vec<_>>()
            );
            assert_eq!(native.tree_ids(), tree.tree_ids());
            assert!(native.tree().has_conflict());
            assert_eq!(native.author().name, ACTOR.to_string());
            assert!(result.rewrites.is_empty());
            assert_eq!(result.workspaces["browser"].before, Some(seed.left));
            assert_eq!(result.workspaces["browser"].after, Some(result.created[0]));
            assert!(result.bookmarks.is_empty());
            seed.publish(expected, &result, 80).await;
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn typed_bookmarks_compare_complete_conflicted_targets_and_publish_once() {
    use jj_store_jj::native_dto::RefTarget;
    tokio::task::LocalSet::new()
        .run_until(async {
            let seed = Branches::new().await;
            let expected = seed.current().await;
            assert_eq!(
                seed.prepare(Command::BookmarkMove {
                    name: "topic".into(),
                    expected: RefTarget(vec![Some(seed.left)]),
                    target: seed.right
                })
                .await
                .unwrap_err(),
                Error::StaleBookmark
            );
            assert_eq!(
                seed.prepare(Command::BookmarkCreate {
                    name: "topic".into(),
                    target: seed.right
                })
                .await
                .unwrap_err(),
                Error::StaleBookmark
            );
            let moved = seed
                .prepare(Command::BookmarkMove {
                    name: "topic".into(),
                    expected: RefTarget(vec![Some(seed.left), None, Some(seed.right)]),
                    target: seed.right,
                })
                .await
                .unwrap();
            assert_eq!(
                moved.bookmarks["topic"].before,
                RefTarget(vec![Some(seed.left), None, Some(seed.right)])
            );
            assert_eq!(
                moved.bookmarks["topic"].after,
                RefTarget(vec![Some(seed.right)])
            );
            seed.publish(expected, &moved, 81).await;
            let expected = seed.current().await;
            let deleted = seed
                .prepare(Command::BookmarkDelete {
                    name: "topic".into(),
                    expected: RefTarget(vec![Some(seed.right)]),
                })
                .await
                .unwrap();
            assert_eq!(deleted.bookmarks["topic"].after, RefTarget(vec![None]));
            seed.publish(expected, &deleted, 82).await;
            let expected = seed.current().await;
            let created = seed
                .prepare(Command::BookmarkCreate {
                    name: "topic".into(),
                    target: seed.left,
                })
                .await
                .unwrap();
            assert_eq!(created.bookmarks["topic"].before, RefTarget(vec![None]));
            seed.publish(expected.clone(), &created, 83).await;
            assert_eq!(seed.current().await.position.0, expected.position.0 + 1);
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn typed_commands_reject_readers_stale_root_and_uncommitted_targets() {
    use jj_store_jj::query::Error as Q;
    tokio::task::LocalSet::new()
        .run_until(async {
            let seed = Branches::new().await;
            let expected = seed.current().await;
            let executor = Executor::new(Limits::default(), 2).unwrap();
            let command = Command::Describe {
                commit: seed.left,
                description: "changed".into(),
            };
            let reader = seed.f.session(seed.f.reader.clone(), Protection::default());
            assert_eq!(
                reader
                    .prepare_command(&executor, &expected, command.clone())
                    .await
                    .unwrap_err(),
                Error::Query(Q::Denied)
            );
            let mut stale = expected.clone();
            stale.position.0 -= 1;
            assert_eq!(
                seed.f
                    .session
                    .prepare_command(&executor, &stale, command)
                    .await
                    .unwrap_err(),
                Error::Stale
            );
            assert_eq!(
                seed.prepare(Command::Describe {
                    commit: roots::ROOT_COMMIT,
                    description: "root".into()
                })
                .await
                .unwrap_err(),
                Error::Root
            );
            let unpublished = seed
                .prepare(Command::New {
                    parents: vec![seed.left],
                    description: "unpublished".into(),
                    workspace: None,
                })
                .await
                .unwrap();
            assert_eq!(
                seed.prepare(Command::Describe {
                    commit: unpublished.created[0],
                    description: "not selected".into()
                })
                .await
                .unwrap_err(),
                Error::Query(Q::NotCommitted)
            );
            assert_eq!(
                seed.prepare(Command::New {
                    parents: vec![seed.left, seed.left],
                    description: "duplicate".into(),
                    workspace: None
                })
                .await
                .unwrap_err(),
                Error::Invalid
            );
            assert_eq!(
                seed.prepare(Command::New {
                    parents: vec![seed.left],
                    description: "unknown workspace".into(),
                    workspace: Some("missing".into())
                })
                .await
                .unwrap_err(),
                Error::Invalid
            );
            assert_eq!(seed.current().await, expected);
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn typed_preparation_limits_and_protected_publication_leave_authority_unchanged() {
    use jj_store_jj::query::Error as Q;
    tokio::task::LocalSet::new()
        .run_until(async {
            let seed = Branches::new().await;
            let expected = seed.current().await;
            let command = Command::Describe {
                commit: seed.left,
                description: "changed".into(),
            };
            let executor = Executor::new(
                Limits {
                    max_staged_objects: 1,
                    ..Default::default()
                },
                1,
            )
            .unwrap();
            assert_eq!(
                seed.f
                    .session
                    .prepare_command(&executor, &expected, command.clone())
                    .await
                    .unwrap_err(),
                Error::Query(Q::Limit)
            );
            assert_eq!(executor.workers().status().in_use, 0);
            let executor = Executor::new(
                Limits {
                    max_staged_bytes: 1,
                    ..Default::default()
                },
                1,
            )
            .unwrap();
            assert_eq!(
                seed.f
                    .session
                    .prepare_command(&executor, &expected, command.clone())
                    .await
                    .unwrap_err(),
                Error::Query(Q::Limit)
            );
            let protected = seed.f.session(
                seed.f.token.clone(),
                Protection {
                    commits: BTreeSet::from([seed.left]),
                    bookmarks: BTreeSet::new(),
                },
            );
            let prepared = protected
                .prepare_command(
                    &Executor::new(Limits::default(), 1).unwrap(),
                    &expected,
                    command,
                )
                .await
                .unwrap();
            let request = Request {
                principal: ACTOR,
                request_id: m::RequestId::new([84; 16]),
                publication_id: m::PublicationId::new([84; 16]),
                transition: Some(Transition::Strict {
                    expected: expected.clone(),
                    heads: BTreeSet::from([prepared.operation]),
                }),
                settings: None,
                result: vec![],
                kind: SemanticKind::OperationPublished,
            };
            assert!(matches!(
                protected.publish_request(&request).await,
                Err(wal::Error::Denied)
            ));
            assert_eq!(seed.current().await, expected);
        })
        .await;
}

use jj_store_object::{
    ByteRange, Condition, Download, Key, Metadata, ObjectStore, StagedUpload, WriteOutcome,
};
use std::{
    sync::{
        Condvar, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

/// A synchronous storage pause models native work which cannot be preempted by
/// dropping its async caller. Only the isolated command session uses this store.
struct PausedWrites {
    inner: Arc<MemoryStore>,
    armed: AtomicBool,
    entered: tokio::sync::Notify,
    gate: (Mutex<bool>, Condvar),
}
impl PausedWrites {
    fn new(f: &Fixture, token: BearerToken) -> (Arc<Self>, Arc<NativeSession>) {
        let store = Arc::new(Self {
            inner: f.store.clone(),
            armed: AtomicBool::new(true),
            entered: Default::default(),
            gate: Default::default(),
        });
        let objects = Arc::new(NativeObjects::new(store.clone(), REPO, 4 * 1024 * 1024).unwrap());
        let session = Arc::new(
            NativeSession::new(
                store.clone(),
                objects,
                token,
                Arc::new(SystemClock),
                Protection::default(),
                SessionLimits::default(),
            )
            .unwrap(),
        );
        (store, session)
    }
    fn release(&self) {
        *self.gate.0.lock().unwrap() = true;
        self.gate.1.notify_all();
    }
    async fn wait(&self) {
        tokio::time::timeout(Duration::from_secs(3), self.entered.notified())
            .await
            .unwrap();
    }
}
#[async_trait::async_trait]
impl ObjectStore for PausedWrites {
    async fn head(&self, key: &Key) -> jj_store_object::Result<Metadata> {
        self.inner.head(key).await
    }
    async fn get(&self, key: &Key, range: Option<ByteRange>) -> jj_store_object::Result<Download> {
        self.inner.get(key, range).await
    }
    async fn put(
        &self,
        key: &Key,
        condition: Condition,
        upload: &StagedUpload,
    ) -> jj_store_object::Result<WriteOutcome> {
        if key.as_str().contains("/objects/v1/") && self.armed.swap(false, Ordering::SeqCst) {
            self.entered.notify_one();
            let (_guard, timeout) = self
                .gate
                .1
                .wait_timeout_while(
                    self.gate.0.lock().unwrap(),
                    Duration::from_secs(10),
                    |released| !*released,
                )
                .unwrap();
            assert!(!timeout.timed_out(), "command storage pause not released");
        }
        self.inner.put(key, condition, upload).await
    }
}
fn prepare_task(
    session: Arc<NativeSession>,
    executor: Executor,
    expected: Snapshot,
    parent: m::CommitId,
) -> tokio::task::JoinHandle<std::result::Result<jj_store_service::commands::Prepared, Error>> {
    tokio::spawn(async move {
        session
            .prepare_command(
                &executor,
                &expected,
                Command::New {
                    parents: vec![parent],
                    description: "paused command".into(),
                    workspace: None,
                },
            )
            .await
    })
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn typed_cancellation_retains_shared_worker_until_native_exit() {
    use jj_store_jj::query as q;
    tokio::task::LocalSet::new()
        .run_until(async {
            let seed = Branches::new().await;
            let expected = seed.current().await;
            let cache = tempfile::tempdir().unwrap();
            let shared = q::workers::Executors::new(
                cache.path().to_owned(),
                q::Limits::default(),
                q::content::Limits::default(),
                q::cache::Policy::default(),
                1,
            )
            .unwrap();
            let executor =
                Executor::with_workers(Limits::default(), shared.workers().clone()).unwrap();
            let (store, session) = PausedWrites::new(&seed.f, seed.f.token.clone());
            let task = prepare_task(
                session.clone(),
                executor.clone(),
                expected.clone(),
                seed.left,
            );
            store.wait().await;
            task.abort();
            assert!(task.await.unwrap_err().is_cancelled());
            assert_eq!(shared.workers().status().in_use, 1);
            assert_eq!(
                prepare_task(
                    session.clone(),
                    executor.clone(),
                    expected.clone(),
                    seed.left
                )
                .await
                .unwrap()
                .unwrap_err(),
                Error::Query(q::Error::Busy)
            );
            store.release();
            tokio::time::timeout(Duration::from_secs(3), async {
                while shared.workers().status().in_use != 0 {
                    tokio::time::sleep(Duration::from_millis(5)).await;
                }
            })
            .await
            .unwrap();
            assert_eq!(seed.current().await, expected);
            assert!(
                prepare_task(session, executor, expected.clone(), seed.left)
                    .await
                    .unwrap()
                    .is_ok()
            );
            assert_eq!(seed.current().await, expected);
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn typed_preparation_rejects_snapshot_changes_during_staging() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let seed = Branches::new().await;
            let expected = seed.current().await;
            let (store, session) = PausedWrites::new(&seed.f, seed.f.token.clone());
            let task = prepare_task(
                session,
                Executor::new(Limits::default(), 1).unwrap(),
                expected.clone(),
                seed.left,
            );
            store.wait().await;
            let concurrent = seed
                .prepare(Command::BookmarkCreate {
                    name: "concurrent".into(),
                    target: seed.right,
                })
                .await
                .unwrap();
            seed.publish(expected, &concurrent, 85).await;
            let accepted = seed.current().await;
            store.release();
            assert_eq!(task.await.unwrap().unwrap_err(), Error::Stale);
            assert_eq!(seed.current().await, accepted);
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn typed_preparation_rechecks_revocation_during_staging() {
    use jj_store_service::administration::{TokenChange, TokenRequest};
    tokio::task::LocalSet::new()
        .run_until(async {
            let seed = Branches::new().await;
            let writer = BearerToken::generate().unwrap();
            seed.f
                .session
                .change_token(&TokenRequest {
                    request_id: m::RequestId::new([86; 16]),
                    expected: seed.current().await,
                    change: TokenChange::Grant {
                        credential: writer.clone(),
                        principal: m::PrincipalId::new([86; 16]),
                        permission: Permission::Write,
                        expires_at: None,
                    },
                })
                .await
                .unwrap();
            let expected = seed.current().await;
            let (store, session) = PausedWrites::new(&seed.f, writer.clone());
            let task = prepare_task(
                session,
                Executor::new(Limits::default(), 1).unwrap(),
                expected.clone(),
                seed.left,
            );
            store.wait().await;
            seed.f
                .session
                .change_token(&TokenRequest {
                    request_id: m::RequestId::new([87; 16]),
                    expected,
                    change: TokenChange::Revoke { token: writer.id() },
                })
                .await
                .unwrap();
            let accepted = seed.current().await;
            store.release();
            assert_eq!(
                task.await.unwrap().unwrap_err(),
                Error::Query(jj_store_jj::query::Error::Denied)
            );
            assert_eq!(seed.current().await, accepted);
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn typed_commands_distinguish_hidden_noop_and_ambiguous_heads() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let seed = Branches::new().await;
            let expected = seed.current().await;
            let noop = seed
                .prepare(Command::Describe {
                    commit: seed.left,
                    description: "branch 41".into(),
                })
                .await
                .unwrap();
            assert!(
                noop.created.is_empty()
                    && noop.rewrites.is_empty()
                    && noop.bookmarks.is_empty()
                    && noop.workspaces.is_empty()
            );
            assert_eq!(seed.current().await, expected);
            let changed = seed
                .prepare(Command::Describe {
                    commit: seed.left,
                    description: "changed".into(),
                })
                .await
                .unwrap();
            seed.publish(expected, &changed, 88).await;
            assert_eq!(
                seed.prepare(Command::Describe {
                    commit: seed.left,
                    description: "hidden".into()
                })
                .await
                .unwrap_err(),
                Error::Hidden
            );
            // Committed historical commits remain legal explicit parents for New.
            let historical = seed
                .prepare(Command::New {
                    parents: vec![seed.left],
                    description: "historical parent".into(),
                    workspace: None,
                })
                .await
                .unwrap();
            assert_eq!(historical.created.len(), 1);
            let first = stage_view(
                &seed.f,
                &seed.repo,
                seed.repo.view().store_view().clone(),
                "concurrent operation",
            )
            .await;
            seed.f
                .session
                .publish(BTreeSet::from([oid(seed.repo.op_id())]), first)
                .await
                .unwrap();
            let current = seed.current().await;
            assert_eq!(current.heads.len(), 2);
            assert_eq!(
                seed.prepare(Command::BookmarkCreate {
                    name: "ambiguous".into(),
                    target: seed.right
                })
                .await
                .unwrap_err(),
                Error::MultipleHeads
            );
            assert_eq!(seed.current().await, current);
        })
        .await;
}
