use super::{ReadBinding, jj};
use jj_store_jj::{local_browser as q, query::Cancellation};
use std::sync::Arc;

fn log(expression: &str) -> q::GraphRequest {
    q::GraphRequest::Log {
        revset: expression.into(),
        workspace: None,
        user_email: String::new(),
        at_time_millis: 0,
    }
}

#[tokio::test]
async fn prepared_all_log_pages_seek_deep_anchors_under_fixed_work() {
    let source = tempfile::tempdir().unwrap();
    let output = tempfile::tempdir().unwrap();
    jj(source.path(), &["git", "init", "--colocate"]);
    for n in 0..224 {
        jj(source.path(), &["new", "-m", &format!("history {n}")]);
    }
    let operation = jj(
        source.path(),
        &["op", "log", "--no-graph", "-n", "1", "-T", "id"],
    )
    .parse()
    .unwrap();
    let access: Arc<dyn q::Access> = Arc::new(ReadBinding(source.path().join(".jj/repo")));
    let ordinary = q::Executor::new(1).unwrap();
    let q::Response::Graph(expected) = ordinary
        .read(
            access.clone(),
            operation,
            q::Request::Graph(
                log("all() & all()"),
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
    let q::Nodes::Log(nodes) = expected.data else {
        panic!()
    };
    assert!(nodes.len() > 210);
    let prepared = Arc::new(
        ordinary
            .prepare_index(
                access.clone(),
                operation,
                output.path().into(),
                32 * 1024 * 1024,
                q::preparation::Limits::default(),
                Cancellation::default(),
            )
            .await
            .unwrap(),
    );
    let small = q::Executor::with_workers(
        jj_store_jj::query::workers::Workers::new(1).unwrap(),
        q::Limits {
            // Includes the fresh source-selection proof, without depth growth.
            max_work: 512,
            ..Default::default()
        },
    );
    let first = small
        .graph_prepared(
            access.clone(),
            prepared.clone(),
            log("all()"),
            q::GraphWindow {
                limit: 1,
                after: None,
            },
        )
        .await
        .unwrap();
    assert_eq!(first.data, q::Nodes::Log(nodes[..1].to_vec()));
    for index in [32, 128, 210, nodes.len() - 2, nodes.len() - 1] {
        for _ in 0..2 {
            let actual = small
                .graph_prepared(
                    access.clone(),
                    prepared.clone(),
                    log("all()"),
                    q::GraphWindow {
                        limit: 1,
                        after: Some(q::ObjectKey::Commit(nodes[index].commit.id)),
                    },
                )
                .await
                .unwrap();
            assert_eq!(
                actual.data,
                q::Nodes::Log(nodes[index + 1..(index + 2).min(nodes.len())].to_vec())
            );
            assert_eq!(
                actual.next,
                (index + 2 < nodes.len()).then(|| q::ObjectKey::Commit(nodes[index + 1].commit.id))
            );
            assert!(actual.stats.work <= 512, "{:?}", actual.stats);
            eprintln!("local log anchor{index}: {:?}", actual.stats);
            assert!(actual.stats.metadata_nodes <= 8, "{:?}", actual.stats);
        }
    }
    assert_eq!(
        small
            .graph_prepared(
                access,
                prepared,
                log("all()"),
                q::GraphWindow {
                    limit: 1,
                    after: Some(q::ObjectKey::Commit(jj_store_model::GitSha1Id::new(
                        [77; 20]
                    )))
                }
            )
            .await
            .unwrap_err(),
        q::Error::Invalid
    );
}
