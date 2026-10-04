use super::*;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn prepared_native_operation_history_hydrates_only_deep_anchor_and_page_candidates() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let local = tempfile::tempdir().unwrap();
            let work = tempfile::tempdir().unwrap();
            let cache = tempfile::tempdir().unwrap();
            let ordinary_cache = tempfile::tempdir().unwrap();
            let (mut repo, _) = initial(&f, local.path(), work.path()).await;
            let mut history = vec![repo.clone()];
            for n in 0..40 {
                repo = repo
                    .start_transaction()
                    .commit(format!("operation {n}"))
                    .await
                    .unwrap();
                history.push(repo.clone());
            }
            let selected = oid(repo.op_id());
            let read = pin(&f, selected).await;
            let ordinary = executor(ordinary_cache.path(), q::Limits::default());
            let queries = [
                q::Request::Operations,
                q::Request::OperationsFiltered {
                    query: "snapshot:false".into(),
                },
                q::Request::OperationsFiltered {
                    query: format!("from:{} snapshot:false", &history[30].op_id().hex()[..12]),
                },
            ];
            let mut expected = vec![];
            for query in &queries {
                let result = ordinary
                    .execute(f.store.clone(), read.clone(), query.clone(), page(256))
                    .await
                    .unwrap();
                let q::Nodes::Operations(nodes) = result.data else {
                    panic!()
                };
                assert!(nodes.len() > 25);
                expected.push(nodes);
            }
            let exec = executor(
                cache.path(),
                q::Limits {
                    metadata: q::admission::Limits {
                        max_nodes: 256,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
            exec.prepare_index(f.store.clone(), read.clone(), q::Cancellation::default())
                .await
                .unwrap();
            let access: Arc<dyn q::ReadAccess> = Arc::new(OperationReadsOnly(read));
            fs::remove_file(cache_slot(cache.path(), selected).join("index/operation-history"))
                .unwrap();
            for (query, nodes) in queries.iter().zip(&expected) {
                for index in [8, 17, nodes.len() - 2] {
                    let result = exec
                        .execute(
                            f.store.clone(),
                            access.clone(),
                            query.clone(),
                            q::Page {
                                limit: 1,
                                after: Some(m::ObjectKey::Operation(nodes[index].id)),
                            },
                        )
                        .await
                        .unwrap();
                    assert_eq!(
                        result.data,
                        q::Nodes::Operations(vec![nodes[index + 1].clone()])
                    );
                    assert_eq!(
                        result.next,
                        (index + 2 < nodes.len())
                            .then_some(m::ObjectKey::Operation(nodes[index + 1].id))
                    );
                    assert_eq!(result.operation, selected);
                    assert!(result.stats.object_reads <= 5, "{:?}", result.stats);
                    assert!(result.stats.metadata_nodes <= 256, "{:?}", result.stats);
                    if query != &queries[2] {
                        assert!(result.stats.work <= 128, "{:?}", result.stats);
                    }
                    eprintln!("native deep operation index{index}: {:?}", result.stats);
                    if query == &queries[2] && index + 2 < nodes.len() {
                        for _ in 0..2 {
                            let resumed = exec
                                .execute(
                                    f.store.clone(),
                                    access.clone(),
                                    query.clone(),
                                    q::Page {
                                        limit: 1,
                                        after: result.next,
                                    },
                                )
                                .await
                                .unwrap();
                            assert_eq!(
                                resumed.data,
                                q::Nodes::Operations(vec![nodes[index + 2].clone()])
                            );
                            assert_eq!(
                                resumed.next,
                                (index + 3 < nodes.len())
                                    .then_some(m::ObjectKey::Operation(nodes[index + 2].id))
                            );
                            assert!(resumed.stats.work <= 160, "{:?}", resumed.stats);
                            assert!(resumed.stats.object_reads <= 5, "{:?}", resumed.stats);
                            eprintln!(
                                "native resumed operation index{}: {:?}",
                                index + 1,
                                resumed.stats
                            );
                        }
                    }
                }
            }
            let later = repo
                .start_transaction()
                .commit("later excluded")
                .await
                .unwrap();
            for (query, after) in [
                (queries[0].clone(), oid(later.op_id())),
                (queries[2].clone(), selected),
                (
                    q::Request::OperationsFiltered {
                        query: "snapshot:true".into(),
                    },
                    expected[0][8].id,
                ),
            ] {
                assert!(matches!(
                    exec.execute(
                        f.store.clone(),
                        access.clone(),
                        query,
                        q::Page {
                            limit: 1,
                            after: Some(m::ObjectKey::Operation(after))
                        }
                    )
                    .await,
                    Err(q::Error::Invalid)
                ));
            }
        })
        .await;
}
