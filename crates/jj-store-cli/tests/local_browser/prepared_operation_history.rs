use super::{ReadBinding, jj};
use jj_store_jj::{local_browser as q, query::Cancellation};
use std::{fs, sync::Arc};

#[tokio::test]
async fn prepared_operation_history_skips_prior_bodies_for_filtered_and_historical_pages() {
    let source = tempfile::tempdir().unwrap();
    let output = tempfile::tempdir().unwrap();
    jj(source.path(), &["git", "init", "--colocate"]);
    let mut history = vec![];
    for n in 0..26 {
        jj(source.path(), &["new", "-m", &format!("History {n}")]);
        history.push(jj(
            source.path(),
            &["op", "log", "--no-graph", "-n", "1", "-T", "id"],
        ));
    }
    jj(
        source.path(),
        &["--at-op", &history[12], "new", "-m", "Sibling"],
    );
    jj(source.path(), &["log", "--no-graph", "-r", "all()"]);
    let operation = jj(
        source.path(),
        &["op", "log", "--no-graph", "-n", "1", "-T", "id"],
    )
    .parse()
    .unwrap();
    let metadata = source.path().join(".jj/repo");
    let access: Arc<dyn q::Access> = Arc::new(ReadBinding(metadata.clone()));
    let reader = q::Executor::new(1).unwrap();
    let queries = [
        q::GraphRequest::Operations,
        q::GraphRequest::OperationsFiltered {
            query: "snapshot:false".into(),
        },
        q::GraphRequest::OperationsFiltered {
            query: format!("from:{} snapshot:false", &history[20][..12]),
        },
    ];
    let mut expected = vec![];
    for query in &queries {
        let q::Response::Graph(page) = reader
            .read(
                access.clone(),
                operation,
                q::Request::Graph(
                    query.clone(),
                    q::GraphWindow {
                        limit: 256,
                        after: None,
                    },
                ),
            )
            .await
            .unwrap()
        else {
            panic!()
        };
        let q::Nodes::Operations(nodes) = page.data else {
            panic!()
        };
        assert!(nodes.len() > 15);
        expected.push(nodes);
    }
    let prepared = Arc::new(
        reader
            .prepare_index(
                access.clone(),
                operation,
                output.path().into(),
                16 * 1024 * 1024,
                q::preparation::Limits::default(),
                Cancellation::default(),
            )
            .await
            .unwrap(),
    );
    fs::rename(
        metadata.join("op_store/views"),
        metadata.join("withheld-views"),
    )
    .unwrap();
    fs::rename(
        metadata.join("index/segments"),
        metadata.join("withheld-segments"),
    )
    .unwrap();
    fs::rename(
        source.path().join(".git/objects"),
        source.path().join("withheld-objects"),
    )
    .unwrap();
    fs::create_dir(source.path().join(".git/objects")).unwrap();
    let small = q::Executor::with_workers(
        jj_store_jj::query::workers::Workers::new(1).unwrap(),
        q::Limits {
            // Head, historical start, anchor, emitted row and final probe.
            max_history_visits: 5,
            ..Default::default()
        },
    );
    let seek = q::Executor::with_workers(
        jj_store_jj::query::workers::Workers::new(1).unwrap(),
        q::Limits {
            max_history_visits: 5,
            max_work: 800,
            ..Default::default()
        },
    );
    for (query, nodes) in queries.iter().zip(&expected) {
        let reader = if query == &queries[2] { &small } else { &seek };
        for index in [8, 13, nodes.len() - 2] {
            let after = Some(q::ObjectKey::Operation(nodes[index].id));
            let q::Response::Graph(actual) = reader
                .read_prepared(
                    access.clone(),
                    prepared.clone(),
                    q::Request::Graph(query.clone(), q::GraphWindow { limit: 1, after }),
                )
                .await
                .unwrap()
            else {
                panic!()
            };
            assert_eq!(
                actual.data,
                q::Nodes::Operations(vec![nodes[index + 1].clone()])
            );
            assert_eq!(
                actual.next,
                (index + 2 < nodes.len()).then_some(q::ObjectKey::Operation(nodes[index + 1].id))
            );
            assert!(actual.stats.metadata_nodes <= 6, "{:?}", actual.stats);
            if query != &queries[2] {
                assert!(actual.stats.work <= 800, "{:?}", actual.stats);
            }
            eprintln!("deep operation page index{index}: {:?}", actual.stats);
        }
    }
    let request = |query, after| {
        q::Request::Graph(
            query,
            q::GraphWindow {
                limit: 1,
                after: Some(q::ObjectKey::Operation(after)),
            },
        )
    };
    assert!(matches!(
        small
            .read_prepared(
                access.clone(),
                prepared.clone(),
                request(
                    queries[0].clone(),
                    jj_store_model::OperationId::new([99; 64])
                )
            )
            .await,
        Err(q::Error::Invalid)
    ));
    assert!(matches!(
        small
            .read_prepared(
                access.clone(),
                prepared.clone(),
                request(queries[2].clone(), expected[0][0].id)
            )
            .await,
        Err(q::Error::Invalid)
    ));
    assert!(matches!(
        small
            .read_prepared(
                access.clone(),
                prepared.clone(),
                request(
                    q::GraphRequest::OperationsFiltered {
                        query: "snapshot:true".into()
                    },
                    expected[0][8].id
                )
            )
            .await,
        Err(q::Error::Invalid)
    ));
    assert!(!metadata.join("op_store/views").exists());
    assert!(!metadata.join("index/segments").exists());
}
