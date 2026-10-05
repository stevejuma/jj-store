use super::*;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn prepared_evolution_seeks_predecessors_across_unrelated_bookmark_operations() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let local = tempfile::tempdir().unwrap();
            let work = tempfile::tempdir().unwrap();
            let cache = tempfile::tempdir().unwrap();
            let ordinary_cache = tempfile::tempdir().unwrap();
            let (mut repo, commit) = initial(&f, local.path(), work.path()).await;
            for n in 0..48 {
                let mut tx = repo.start_transaction();
                tx.repo_mut().set_local_bookmark_target(
                    jj_lib::ref_name::RefName::new(&format!("bookmark-{n}")),
                    o::RefTarget::normal(commit.id().clone()),
                );
                repo = tx.commit("unrelated bookmark").await.unwrap();
            }
            let read = pin(&f, oid(repo.op_id())).await;
            let query = q::Request::Evolution {
                commit: cid(commit.id()),
            };
            let ordinary = executor(ordinary_cache.path(), q::Limits::default());
            let expected = ordinary
                .execute(f.store.clone(), read.clone(), query.clone(), page(1))
                .await
                .unwrap();
            let exec = executor(cache.path(), q::Limits::default());
            exec.prepare_index(f.store.clone(), read.clone(), q::Cancellation::default())
                .await
                .unwrap();
            for _ in 0..2 {
                let actual = exec
                    .execute(f.store.clone(), read.clone(), query.clone(), page(1))
                    .await
                    .unwrap();
                assert_eq!(actual.data, expected.data);
                assert_eq!(actual.next, expected.next);
                eprintln!("native sparse predecessor lookup: {:?}", actual.stats);
                assert!(actual.stats.object_reads <= 8, "{:?}", actual.stats);
                assert!(actual.stats.work <= 1024, "{:?}", actual.stats);
            }
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn prepared_evolution_continuations_resume_without_preceding_body_reads() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let local = tempfile::tempdir().unwrap();
            let work = tempfile::tempdir().unwrap();
            let cache = tempfile::tempdir().unwrap();
            let ordinary_cache = tempfile::tempdir().unwrap();
            let (mut repo, mut commit) = initial(&f, local.path(), work.path()).await;
            for n in 0..32 {
                (repo, commit) = describe(&repo, &commit, &format!("rewrite {n}")).await;
            }
            let read = pin(&f, oid(repo.op_id())).await;
            let query = q::Request::Evolution {
                commit: cid(commit.id()),
            };
            let ordinary = executor(ordinary_cache.path(), q::Limits::default());
            let expected = ordinary
                .execute(f.store.clone(), read.clone(), query.clone(), page(256))
                .await
                .unwrap();
            let q::Nodes::Evolution(nodes) = expected.data else {
                panic!()
            };
            assert!(nodes.len() > 32);
            let exec = executor(cache.path(), q::Limits::default());
            exec.prepare_index(f.store.clone(), read.clone(), q::Cancellation::default())
                .await
                .unwrap();
            for offset in [4, 18, nodes.len() - 3] {
                let seeded = exec
                    .execute(
                        f.store.clone(),
                        read.clone(),
                        query.clone(),
                        q::Page {
                            limit: 1,
                            after: Some(m::ObjectKey::Commit(nodes[offset].commit.id)),
                        },
                    )
                    .await
                    .unwrap();
                assert_eq!(
                    seeded.data,
                    q::Nodes::Evolution(vec![nodes[offset + 1].clone()])
                );
                assert_eq!(
                    seeded.next,
                    Some(m::ObjectKey::Commit(nodes[offset + 1].commit.id))
                );
                for _ in 0..2 {
                    let actual = exec
                        .execute(
                            f.store.clone(),
                            read.clone(),
                            query.clone(),
                            q::Page {
                                limit: 1,
                                after: seeded.next,
                            },
                        )
                        .await
                        .unwrap();
                    assert_eq!(
                        actual.data,
                        q::Nodes::Evolution(vec![nodes[offset + 2].clone()])
                    );
                    assert_eq!(
                        actual.next,
                        (offset + 3 < nodes.len())
                            .then(|| m::ObjectKey::Commit(nodes[offset + 2].commit.id))
                    );
                    eprintln!("native evolution anchor{}: {:?}", offset + 1, actual.stats);
                    assert!(actual.stats.object_reads <= 8, "{:?}", actual.stats);
                    assert!(actual.stats.work <= 512, "{:?}", actual.stats);
                }
            }
            assert_eq!(
                exec.execute(
                    f.store.clone(),
                    read.clone(),
                    q::Request::Evolution {
                        commit: nodes[10].commit.id
                    },
                    q::Page {
                        limit: 1,
                        after: Some(m::ObjectKey::Commit(nodes[5].commit.id))
                    }
                )
                .await
                .unwrap_err(),
                q::Error::Invalid
            );
            let terminal = exec
                .execute(
                    f.store.clone(),
                    read.clone(),
                    query.clone(),
                    q::Page {
                        limit: 1,
                        after: Some(m::ObjectKey::Commit(nodes.last().unwrap().commit.id)),
                    },
                )
                .await
                .unwrap();
            assert_eq!(terminal.data, q::Nodes::Evolution(vec![]));
            assert_eq!(terminal.next, None);
            assert_eq!(
                exec.execute(
                    f.store.clone(),
                    read,
                    query,
                    q::Page {
                        limit: 1,
                        after: Some(m::ObjectKey::Commit(m::CommitId::new([77; 64])))
                    }
                )
                .await
                .unwrap_err(),
                q::Error::Invalid
            );
        })
        .await;
}
