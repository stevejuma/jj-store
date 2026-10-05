use super::{ReadBinding, jj};
use jj_store_jj::{local_browser as q, query::Cancellation};
use std::sync::Arc;

#[tokio::test]
async fn prepared_evolution_continuations_resume_without_preceding_body_reads() {
    let source = tempfile::tempdir().unwrap();
    let output = tempfile::tempdir().unwrap();
    jj(source.path(), &["git", "init", "--colocate"]);
    for n in 0..32 {
        jj(source.path(), &["describe", "-m", &format!("rewrite {n}")]);
    }
    let operation = jj(
        source.path(),
        &["op", "log", "--no-graph", "-n", "1", "-T", "id"],
    )
    .parse()
    .unwrap();
    let commit = jj(
        source.path(),
        &["log", "--no-graph", "-r", "@", "-T", "commit_id"],
    )
    .parse()
    .unwrap();
    let access: Arc<dyn q::Access> = Arc::new(ReadBinding(source.path().join(".jj/repo")));
    let reader = q::Executor::new(1).unwrap();
    let query = q::GraphRequest::Evolution { commit };
    let q::Response::Graph(expected) = reader
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
    let q::Nodes::Evolution(nodes) = expected.data else {
        panic!()
    };
    assert!(nodes.len() > 31);
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
    for offset in [4, 18, nodes.len() - 3] {
        let seeded = reader
            .graph_prepared(
                access.clone(),
                prepared.clone(),
                query.clone(),
                q::GraphWindow {
                    limit: 1,
                    after: Some(q::ObjectKey::Commit(nodes[offset].commit.id)),
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
            Some(q::ObjectKey::Commit(nodes[offset + 1].commit.id))
        );
        for _ in 0..2 {
            let actual = reader
                .graph_prepared(
                    access.clone(),
                    prepared.clone(),
                    query.clone(),
                    q::GraphWindow {
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
                    .then(|| q::ObjectKey::Commit(nodes[offset + 2].commit.id))
            );
            eprintln!("local evolution anchor{}: {:?}", offset + 1, actual.stats);
            assert!(actual.stats.metadata_nodes <= 9, "{:?}", actual.stats);
            assert!(actual.stats.work <= 1024, "{:?}", actual.stats);
        }
    }
    assert_eq!(
        reader
            .graph_prepared(
                access.clone(),
                prepared.clone(),
                q::GraphRequest::Evolution {
                    commit: nodes[10].commit.id
                },
                q::GraphWindow {
                    limit: 1,
                    after: Some(q::ObjectKey::Commit(nodes[5].commit.id))
                }
            )
            .await
            .unwrap_err(),
        q::Error::Invalid
    );
    let terminal = reader
        .graph_prepared(
            access.clone(),
            prepared.clone(),
            query.clone(),
            q::GraphWindow {
                limit: 1,
                after: Some(q::ObjectKey::Commit(nodes.last().unwrap().commit.id)),
            },
        )
        .await
        .unwrap();
    assert_eq!(terminal.data, q::Nodes::Evolution(vec![]));
    assert_eq!(terminal.next, None);
    assert_eq!(
        reader
            .graph_prepared(
                access,
                prepared,
                query,
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
