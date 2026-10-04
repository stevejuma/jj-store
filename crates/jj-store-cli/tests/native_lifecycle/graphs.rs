use super::*;
use jj_store_jj::query as q;
use jj_store_service::reads::{CommittedRead, OpenRead, ReadLimits};
use std::time::Duration;

fn log(expression: &str) -> q::Request {
    q::Request::Log {
        revset: expression.into(),
        workspace: None,
        user_email: "native@example.invalid".into(),
        at_time_millis: 1_790_000_000_000,
    }
}
fn page(limit: usize) -> q::Page {
    q::Page { limit, after: None }
}
async fn pin(f: &Fixture, id: m::OperationId) -> Arc<CommittedRead> {
    let OpenRead::Pinned(read) = f
        .session
        .open_read(Some(id), ReadLimits::default())
        .await
        .unwrap()
    else {
        panic!("explicit pin")
    };
    Arc::new(read)
}
fn executor(path: &std::path::Path, limits: q::Limits) -> q::GraphExecutor {
    q::GraphExecutor::new(path.to_owned(), limits, 2).unwrap()
}
async fn initial(
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
    describe(&repo, &commit, "A").await
}
async fn child(
    repo: &Arc<ReadonlyRepo>,
    parent: &jj_lib::commit::Commit,
    description: &str,
) -> (Arc<ReadonlyRepo>, jj_lib::commit::Commit) {
    let mut tx = repo.start_transaction();
    let next = tx
        .repo_mut()
        .new_commit(vec![parent.id().clone()], parent.tree())
        .set_description(description)
        .write()
        .await
        .unwrap();
    tx.repo_mut()
        .set_wc_commit("fixture".into(), next.id().clone())
        .unwrap();
    tx.repo_mut().set_local_bookmark_target(
        jj_lib::ref_name::RefName::new("main"),
        o::RefTarget::normal(next.id().clone()),
    );
    (tx.commit(description).await.unwrap(), next)
}
fn commits(result: &q::GraphPage) -> &[q::CommitNode] {
    let q::Nodes::Log(nodes) = &result.data else {
        panic!("commit graph")
    };
    nodes
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn graph_query_three_histories_filtered_edges_and_snapshot_pagination() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let cache = tempfile::tempdir().unwrap();
            let work = tempfile::tempdir().unwrap();
            let graphs = tempfile::tempdir().unwrap();
            let (repo, a) = initial(&f, cache.path(), work.path()).await;
            let (repo, b) = child(&repo, &a, "B").await;
            let (repo, c) = child(&repo, &b, "C").await;
            let (repo, c_new) = describe(&repo, &c, "C rewritten").await;
            let mut tx = repo.start_transaction();
            tx.repo_mut().set_remote_bookmark(
                jj_lib::ref_name::RefName::new("main")
                    .to_remote_symbol(jj_lib::ref_name::RemoteName::new("origin")),
                o::RemoteRef {
                    target: o::RefTarget::normal(c_new.id().clone()),
                    state: o::RemoteRefState::Tracked,
                },
            );
            let repo = tx.commit("track remote fixture").await.unwrap();
            let read = pin(&f, oid(repo.op_id())).await;
            let exec = executor(graphs.path(), q::Limits::default());
            let before = f.session.snapshot().await.unwrap().manifest().clone();
            let full = read.graph(&exec, log("all()"), page(100)).await.unwrap();
            let expected = vec![
                cid(c_new.id()),
                cid(b.id()),
                cid(a.id()),
                roots::ROOT_COMMIT,
            ];
            assert_eq!(
                commits(&full)
                    .iter()
                    .map(|n| n.commit.id)
                    .collect::<Vec<_>>(),
                expected
            );
            let top = &commits(&full)[0];
            assert_eq!(top.commit.parents, vec![cid(b.id())]);
            assert_eq!(top.commit.workspaces, vec!["fixture"]);
            assert!(
                top.commit
                    .bookmarks
                    .iter()
                    .any(|r| r.name == "main" && r.remote.is_none() && r.added && !r.conflicted)
            );
            assert!(top.commit.bookmarks.iter().any(|r| r.name == "main"
                && r.remote.as_deref() == Some("origin")
                && r.added
                && !r.conflicted));
            assert!(commits(&full).iter().all(|n| n.commit.visible));
            let filtered = read
                .graph(
                    &exec,
                    log("subject(\"C rewritten\") | subject(\"A\")"),
                    page(100),
                )
                .await
                .unwrap();
            assert_eq!(commits(&filtered).len(), 2);
            assert_eq!(
                commits(&filtered)[0].edges,
                vec![q::Edge {
                    target: cid(a.id()),
                    kind: q::EdgeKind::Indirect
                }]
            );
            assert!(
                commits(&filtered)[1]
                    .edges
                    .iter()
                    .any(|e| e.kind == q::EdgeKind::Missing && e.target == roots::ROOT_COMMIT)
            );
            let first = read.graph(&exec, log("all()"), page(2)).await.unwrap();
            assert_eq!(first.next, Some(m::ObjectKey::Commit(cid(b.id()))));
            let (new_repo, _) = child(&repo, &c_new, "later publication").await;
            let second = read
                .graph(
                    &exec,
                    log("all()"),
                    q::Page {
                        limit: 2,
                        after: first.next,
                    },
                )
                .await
                .unwrap();
            assert_eq!(second.next, None);
            let joined: Vec<_> = commits(&first)
                .iter()
                .chain(commits(&second))
                .map(|n| n.commit.id)
                .collect();
            assert_eq!(joined, expected);
            let ops = read
                .graph(&exec, q::Request::Operations, page(100))
                .await
                .unwrap();
            let q::Nodes::Operations(ops) = ops.data else {
                unreachable!()
            };
            assert_eq!(ops[0].id, oid(repo.op_id()));
            assert_eq!(
                ops[0].parents,
                repo.operation()
                    .parent_ids()
                    .iter()
                    .map(oid)
                    .collect::<Vec<_>>()
            );
            assert_eq!(ops.last().unwrap().id, roots::ROOT_OPERATION);
            assert_eq!(
                ops[0].is_snapshot,
                repo.operation().store_operation().metadata.is_snapshot
            );
            assert_eq!(
                ops[0].workspace,
                repo.operation()
                    .store_operation()
                    .metadata
                    .workspace_name
                    .as_ref()
                    .map(|name| name.as_str().to_owned())
            );
            assert_eq!(
                ops[0].attributes,
                repo.operation().store_operation().metadata.attributes
            );
            let evolution = read
                .graph(
                    &exec,
                    q::Request::Evolution {
                        commit: cid(c_new.id()),
                    },
                    page(100),
                )
                .await
                .unwrap();
            let q::Nodes::Evolution(nodes) = evolution.data else {
                unreachable!()
            };
            assert_eq!(
                nodes.iter().map(|n| n.commit.id).collect::<Vec<_>>(),
                vec![cid(c_new.id()), cid(c.id())]
            );
            assert_eq!(nodes[0].predecessors, vec![cid(c.id())]);
            assert!(nodes[0].predecessors_known);
            assert!(nodes[1].predecessors_known && nodes[1].predecessors.is_empty());
            assert!(!nodes[1].commit.visible);
            assert_eq!(
                f.session.snapshot().await.unwrap().manifest().heads,
                BTreeSet::from([oid(new_repo.op_id())])
            );
            assert_eq!(
                f.session.snapshot().await.unwrap().manifest().published.0,
                before.published.0 + 1,
                "graph reads published authority"
            );
            assert!(full.stats.object_reads > 0 && full.stats.object_bytes > 0);
            assert!(!graphs.path().join(".jj").exists());
            assert!(!graphs.path().join(".git").exists());
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn graph_query_divergent_operations_bookmarks_and_explicit_workspace() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let cache = tempfile::tempdir().unwrap();
            let work = tempfile::tempdir().unwrap();
            let graphs = tempfile::tempdir().unwrap();
            let (repo, a) = initial(&f, cache.path(), work.path()).await;
            let (repo, b) = child(&repo, &a, "base").await;
            let (left, left_commit) = describe(&repo, &b, "left").await;
            let (right, right_commit) = describe(&repo, &b, "right").await;
            let before = f.session.snapshot().await.unwrap().manifest().clone();
            assert_eq!(before.heads.len(), 2);
            let exec = executor(graphs.path(), q::Limits::default());
            for (repo, name, other) in [(&left, "left", "right"), (&right, "right", "left")] {
                let read = pin(&f, oid(repo.op_id())).await;
                let result = read.graph(&exec, log("all()"), page(100)).await.unwrap();
                assert!(
                    commits(&result)
                        .iter()
                        .any(|n| n.commit.description == name)
                );
                assert!(
                    !commits(&result)
                        .iter()
                        .any(|n| n.commit.description == other)
                );
                assert!(matches!(
                    read.graph(&exec, log("@"), page(10)).await.unwrap_err(),
                    q::Error::InvalidQuery(diagnostic) if diagnostic.kind == "missing_context"
                ));
                let mut query = log("@");
                if let q::Request::Log { workspace, .. } = &mut query {
                    *workspace = Some("fixture".into());
                }
                assert_eq!(
                    commits(&read.graph(&exec, query, page(10)).await.unwrap()).len(),
                    1
                );
            }
            assert_eq!(f.session.snapshot().await.unwrap().manifest(), &before);
            let loader = f.stores.load_cache(&settings(), cache.path()).unwrap();
            let merged = f.stores.load_for_mutation(&loader).await.unwrap();
            let read = pin(&f, oid(merged.op_id())).await;
            let result = read
                .graph(&exec, log("divergent()"), page(100))
                .await
                .unwrap();
            for id in [cid(left_commit.id()), cid(right_commit.id())] {
                let node = commits(&result).iter().find(|n| n.commit.id == id).unwrap();
                assert!(node.commit.divergent && node.commit.visible);
                assert!(
                    node.commit
                        .bookmarks
                        .iter()
                        .any(|b| b.name == "main" && b.conflicted && b.added)
                );
            }
            let ops = read
                .graph(&exec, q::Request::Operations, page(100))
                .await
                .unwrap();
            let q::Nodes::Operations(ops) = ops.data else {
                unreachable!()
            };
            assert_eq!(
                ops[0].parents.iter().copied().collect::<BTreeSet<_>>(),
                before.heads
            );
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn graph_query_rebuild_limits_staged_rejection_and_current_revocation() {
    use jj_store_service::administration::{TokenChange, TokenRequest};
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let cache = tempfile::tempdir().unwrap();
            let work = tempfile::tempdir().unwrap();
            let graphs = tempfile::tempdir().unwrap();
            let (repo, a) = initial(&f, cache.path(), work.path()).await;
            let (repo, b) = child(&repo, &a, "B").await;
            let reader = f.session(f.reader.clone(), Protection::default());
            let OpenRead::Pinned(read) = reader
                .open_read(Some(oid(repo.op_id())), ReadLimits::default())
                .await
                .unwrap()
            else {
                unreachable!()
            };
            let read = Arc::new(read);
            let exec = executor(graphs.path(), q::Limits::default());
            let query = log("all()");
            let first = read.graph(&exec, query.clone(), page(100)).await.unwrap();
            let warm = read.graph(&exec, query.clone(), page(100)).await.unwrap();
            assert_eq!(first.data, warm.data);
            fs::remove_dir_all(graphs.path()).unwrap();
            let cold = read.graph(&exec, query.clone(), page(100)).await.unwrap();
            assert_eq!(first.data, cold.data);
            // Corrupted profile metadata must not select a different backend.
            let cache_path = cache_slot(graphs.path(), oid(repo.op_id()));
            fs::write(cache_path.join("store/type"), "git").unwrap();
            assert_eq!(
                read.graph(&exec, query.clone(), page(100))
                    .await
                    .unwrap_err(),
                q::Error::Unavailable
            );
            fs::remove_dir_all(&cache_path).unwrap();
            assert_eq!(
                read.graph(&exec, query.clone(), page(100))
                    .await
                    .unwrap()
                    .data,
                first.data
            );
            for (limits, error) in [
                (
                    q::Limits {
                        max_index_commits: 1,
                        ..q::Limits::default()
                    },
                    q::Error::IndexBudget,
                ),
                (
                    q::Limits {
                        max_work: 1,
                        ..q::Limits::default()
                    },
                    q::Error::Limit,
                ),
                (
                    q::Limits {
                        max_object_bytes: 1,
                        ..q::Limits::default()
                    },
                    q::Error::Limit,
                ),
                (
                    q::Limits {
                        max_output_bytes: 1,
                        ..q::Limits::default()
                    },
                    q::Error::Limit,
                ),
            ] {
                assert_eq!(
                    read.graph(&executor(graphs.path(), limits), query.clone(), page(100))
                        .await
                        .unwrap_err(),
                    error
                );
            }
            assert_eq!(
                read.graph(&exec, query.clone(), page(0)).await.unwrap_err(),
                q::Error::Limit
            );
            assert_eq!(
                read.graph(&exec, log(&"~".repeat(100)), page(2))
                    .await
                    .unwrap_err(),
                q::Error::Limit
            );
            assert_eq!(
                read.graph(
                    &exec,
                    query.clone(),
                    q::Page {
                        limit: 2,
                        after: Some(m::ObjectKey::Commit(m::CommitId::new([99; 64])))
                    }
                )
                .await
                .unwrap_err(),
                q::Error::Invalid
            );
            let mut staged = (**b.store_commit()).clone();
            staged.description = "staged".into();
            let (key, _) = f.objects.put(&staged).await.unwrap();
            let m::ObjectKey::Commit(id) = key else {
                unreachable!()
            };
            assert!(matches!(
                read.graph(&exec, log(&id.to_string()), page(2))
                    .await
                    .unwrap_err(),
                q::Error::InvalidQuery(diagnostic) if diagnostic.kind == "unresolved_symbol"
            ));
            assert_eq!(
                read.graph(&exec, q::Request::Evolution { commit: id }, page(2))
                    .await
                    .unwrap_err(),
                q::Error::NotCommitted
            );
            let before = f.session.snapshot().await.unwrap().manifest().snapshot();
            f.session
                .change_token(&TokenRequest {
                    request_id: m::RequestId::new([215; 16]),
                    expected: before,
                    change: TokenChange::Revoke {
                        token: f.reader.id(),
                    },
                })
                .await
                .unwrap();
            assert_eq!(
                read.graph(&exec, query.clone(), page(2)).await.unwrap_err(),
                q::Error::Denied
            );
            let denied = graphs.path().join("must-not-create");
            assert_eq!(
                read.graph(&executor(&denied, q::Limits::default()), query, page(2))
                    .await
                    .unwrap_err(),
                q::Error::Denied
            );
            assert!(!denied.exists());
        })
        .await;
}

struct PausedAccess {
    inner: Arc<CommittedRead>,
    calls: std::sync::atomic::AtomicUsize,
    entered: tokio::sync::Notify,
    release: tokio::sync::Notify,
}
#[async_trait::async_trait]
impl q::ReadAccess for PausedAccess {
    fn repository(&self) -> m::RepositoryId {
        self.inner.repository()
    }
    fn epoch(&self) -> m::EpochId {
        self.inner.epoch()
    }
    fn operation(&self) -> m::OperationId {
        self.inner.operation()
    }
    async fn contains(&self, key: m::ObjectKey) -> q::Result<bool> {
        q::ReadAccess::contains(self.inner.as_ref(), key).await
    }
    async fn inventory(&self) -> q::Result<Arc<BTreeSet<m::ObjectKey>>> {
        q::ReadAccess::inventory(self.inner.as_ref()).await
    }
    async fn authorize(&self) -> q::Result<()> {
        if self.calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst) == 1 {
            self.entered.notify_one();
            self.release.notified().await;
        }
        q::ReadAccess::authorize(self.inner.as_ref()).await
    }
}
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn graph_query_cancellation_retains_worker_permit_until_exit_and_deadlines_bound_waits() {
    let f = Fixture::new().await;
    let graphs = tempfile::tempdir().unwrap();
    let read = pin(&f, roots::ROOT_OPERATION).await;
    let paused = resource_acceptance::BlockingAccess::new(read);
    let shared = q::workers::Executors::new(
        graphs.path().to_owned(),
        q::Limits::default(),
        q::content::Limits::default(),
        q::cache::Policy::default(),
        1,
    )
    .unwrap();
    let exec = shared.graph().clone();
    let task = {
        let exec = exec.clone();
        let access = paused.clone();
        let store = f.store.clone();
        tokio::spawn(async move {
            exec.execute(store, access, q::Request::Operations, page(100))
                .await
        })
    };
    tokio::time::timeout(Duration::from_secs(3), paused.entered.notified())
        .await
        .unwrap();
    task.abort();
    assert!(task.await.unwrap_err().is_cancelled());
    assert_eq!(
        exec.execute(
            f.store.clone(),
            paused.clone(),
            q::Request::Operations,
            page(100)
        )
        .await
        .unwrap_err(),
        q::Error::Busy
    );
    assert_eq!(
        shared
            .content()
            .object(
                f.store.clone(),
                paused.clone(),
                m::ObjectKey::Commit(roots::ROOT_COMMIT)
            )
            .await
            .unwrap_err(),
        q::Error::Busy,
        "synchronous canceled worker must retain shared content capacity until exit"
    );
    assert_eq!(shared.workers().status().in_use, 1);
    let independent =
        q::GraphExecutor::new(graphs.path().to_owned(), q::Limits::default(), 1).unwrap();
    assert_eq!(
        independent
            .execute(
                f.store.clone(),
                paused.clone(),
                q::Request::Operations,
                page(100)
            )
            .await
            .unwrap_err(),
        q::Error::Busy,
        "canceled worker must retain namespace lock until exit"
    );
    paused.release();
    let deadline = tokio::time::Instant::now() + Duration::from_secs(3);
    loop {
        match exec
            .execute(
                f.store.clone(),
                paused.clone(),
                q::Request::Operations,
                page(100),
            )
            .await
        {
            Err(q::Error::Busy) => {
                assert!(tokio::time::Instant::now() < deadline);
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
            Ok(_) => break,
            other => panic!("worker failed: {other:?}"),
        }
    }
    let paused = Arc::new(PausedAccess {
        inner: pin(&f, roots::ROOT_OPERATION).await,
        calls: Default::default(),
        entered: Default::default(),
        release: Default::default(),
    });
    let limits = q::Limits {
        timeout: Duration::from_millis(100),
        ..q::Limits::default()
    };
    assert_eq!(
        executor(graphs.path(), limits)
            .execute(f.store.clone(), paused, q::Request::Operations, page(10))
            .await
            .unwrap_err(),
        q::Error::Limit
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn graph_query_marks_absent_evolution_data_and_rejects_missing_continuations() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let cache = tempfile::tempdir().unwrap();
            let work = tempfile::tempdir().unwrap();
            let graphs = tempfile::tempdir().unwrap();
            let (repo, original) = initial(&f, cache.path(), work.path()).await;
            let mut commit = (**original.store_commit()).clone();
            commit.description = "legacy predecessor".into();
            commit.predecessors = vec![original.id().clone()];
            let (key, _) = f.objects.put(&commit).await.unwrap();
            let m::ObjectKey::Commit(id) = key else {
                unreachable!()
            };
            let mut view = repo.view().store_view().clone();
            view.head_ids = [b::CommitId::from_bytes(id.as_bytes())].into();
            view.wc_commit_ids
                .insert("fixture".into(), b::CommitId::from_bytes(id.as_bytes()));
            let op = stage_view(
                &f,
                &repo,
                view,
                "legacy operation without predecessor records",
            )
            .await;
            f.session
                .publish(BTreeSet::from([oid(repo.op_id())]), op)
                .await
                .unwrap();
            let read = pin(&f, op).await;
            let exec = executor(graphs.path(), q::Limits::default());
            let result = read
                .graph(&exec, q::Request::Evolution { commit: id }, page(100))
                .await
                .unwrap();
            let q::Nodes::Evolution(nodes) = result.data else {
                unreachable!()
            };
            assert_eq!(nodes.len(), 1);
            assert!(!nodes[0].predecessors_known);
            assert_eq!(nodes[0].legacy_predecessors, vec![cid(original.id())]);
            let first = read
                .graph(&exec, q::Request::Operations, page(1))
                .await
                .unwrap();
            assert_eq!(first.next, Some(m::ObjectKey::Operation(op)));
            let remainder = read
                .graph(
                    &exec,
                    q::Request::Operations,
                    q::Page {
                        limit: 100,
                        after: first.next,
                    },
                )
                .await
                .unwrap();
            let q::Nodes::Operations(nodes) = remainder.data else {
                unreachable!()
            };
            assert_eq!(nodes[0].id, oid(repo.op_id()));
            assert_eq!(
                read.graph(
                    &exec,
                    q::Request::Operations,
                    q::Page {
                        limit: 10,
                        after: Some(m::ObjectKey::Operation(m::OperationId::new([241; 64])))
                    }
                )
                .await
                .unwrap_err(),
                q::Error::Invalid
            );
        })
        .await;
}

#[path = "graph_cache.rs"]
mod cache_acceptance;
use cache_acceptance::cache_slot;

#[path = "query_resources.rs"]
mod resource_acceptance;

#[path = "index_preparation.rs"]
mod index_acceptance;

#[path = "history_query.rs"]
mod history_query_acceptance;
