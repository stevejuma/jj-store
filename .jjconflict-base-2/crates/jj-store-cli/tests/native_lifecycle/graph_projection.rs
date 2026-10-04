use super::*;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn native_prepared_projection_is_recovered_explicitly_after_executor_restart() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let local = tempfile::tempdir().unwrap();
            let work = tempfile::tempdir().unwrap();
            let root = tempfile::tempdir().unwrap();
            let (repo, base) = initial(&f, local.path(), work.path()).await;
            let (repo, tip) = child(&repo, &base, "Restart fixture").await;
            let read = pin(&f, oid(repo.op_id())).await;
            let limits = q::Limits {
                max_inline_rebuild_commits: 1,
                ..Default::default()
            };
            let before = f.session.snapshot().await.unwrap().manifest().clone();
            let exec = executor(root.path(), limits);
            read.prepare_index(&exec, q::Cancellation::default())
                .await
                .unwrap();
            let expected = read.graph(&exec, log("all()"), page(100)).await.unwrap();
            drop(exec);
            let restarted = executor(root.path(), limits);
            assert!(matches!(
                read.graph(&restarted, log("all()"), page(1)).await,
                Err(q::Error::IndexRebuildRequired)
            ));
            read.prepare_index(&restarted, q::Cancellation::default())
                .await
                .unwrap();
            let result = read
                .graph(&restarted, log("all()"), page(100))
                .await
                .unwrap();
            assert_eq!(result.data, expected.data);
            assert_eq!(commits(&result)[0].commit.id, cid(tip.id()));
            assert_eq!(result.cache.state, q::cache::State::Reused);
            assert_eq!(*f.session.snapshot().await.unwrap().manifest(), before);
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn native_prepared_rows_reuse_visibility_and_charge_only_returned_labels() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let local = tempfile::tempdir().unwrap();
            let work = tempfile::tempdir().unwrap();
            let root = tempfile::tempdir().unwrap();
            let (repo, base) = initial(&f, local.path(), work.path()).await;
            let (repo, tip) = child(&repo, &base, "Selected row").await;
            let mut tx = repo.start_transaction();
            for n in 0..5_000 {
                tx.repo_mut().set_local_bookmark_target(
                    format!("unrelated/{n:05}").as_str().as_ref(),
                    o::RefTarget::normal(base.id().clone()),
                );
            }
            tx.repo_mut().set_remote_bookmark(
                jj_lib::ref_name::RefName::new("main")
                    .to_remote_symbol(jj_lib::ref_name::RemoteName::new("origin")),
                o::RemoteRef {
                    target: o::RefTarget::normal(tip.id().clone()),
                    state: o::RemoteRefState::Tracked,
                },
            );
            let repo = tx.commit("Decoration fixture").await.unwrap();
            let read = pin(&f, oid(repo.op_id())).await;
            let exec = executor(
                root.path(),
                q::Limits {
                    max_work: 300,
                    ..Default::default()
                },
            );
            let before = f.session.snapshot().await.unwrap().manifest().clone();
            read.prepare_index(&exec, q::Cancellation::default())
                .await
                .unwrap();
            for _ in 0..2 {
                let graph = read.graph(&exec, log("all()"), page(1)).await.unwrap();
                let row = &commits(&graph)[0].commit;
                assert_eq!(row.id, cid(tip.id()));
                assert!(row.visible);
                assert!(!row.divergent);
                assert_eq!(row.workspaces, ["fixture"]);
                assert_eq!(
                    row.bookmarks
                        .iter()
                        .map(|r| (r.name.as_str(), r.remote.as_deref()))
                        .collect::<Vec<_>>(),
                    [("main", None), ("main", Some("origin"))]
                );
                assert!(graph.stats.work <= 300, "{:?}", graph.stats);
                assert_eq!(graph.cache.state, q::cache::State::Reused);
            }
            // The selected base carries 5,000 labels. They are real returned work,
            // unlike the unrelated labels skipped by the preceding one-row page.
            assert!(matches!(
                read.graph(&exec, log(&base.id().hex()), page(1)).await,
                Err(q::Error::Limit)
            ));
            assert_eq!(*f.session.snapshot().await.unwrap().manifest(), before);
        })
        .await;
}
