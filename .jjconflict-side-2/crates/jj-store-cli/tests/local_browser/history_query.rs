use super::*;
use jj_store_jj::{
    local_browser::{Access, Executor, GraphRequest, GraphWindow, Request, Response},
    query as q,
};
use std::sync::Arc;

#[tokio::test]
async fn local_history_queries_match_pinned_jj_aliases_and_preserve_heads_and_dirty_files() {
    let temp = tempfile::tempdir().unwrap();
    jj(temp.path(), &["git", "init", "--colocate"]);
    for message in ["first", "second"] {
        jj(temp.path(), &["describe", "-m", message]);
        jj(temp.path(), &["new", "-m", "noise"]);
    }
    let op = jj(
        temp.path(),
        &[
            "--ignore-working-copy",
            "op",
            "log",
            "--no-graph",
            "-n",
            "1",
            "-T",
            "id",
        ],
    )
    .parse()
    .unwrap();
    let aliases = [
        "trunk()",
        "immutable()",
        "mutable()",
        "visible()",
        "::@",
        "mine()",
    ];
    let oracle: Vec<_> = aliases
        .iter()
        .map(|expression| {
            jj(
                temp.path(),
                &[
                    "--ignore-working-copy",
                    "log",
                    "--no-graph",
                    "-r",
                    expression,
                    "-T",
                    "commit_id ++ \"\\n\"",
                ],
            )
        })
        .collect();
    jj(
        temp.path(),
        &[
            "config",
            "set",
            "--repo",
            r#"revset-aliases."mutable()""#,
            "none()",
        ],
    );
    let heads_path = temp.path().join(".jj/repo/op_heads/heads");
    let head_files = || {
        fs::read_dir(&heads_path)
            .unwrap()
            .map(|entry| entry.unwrap().file_name())
            .collect::<Vec<_>>()
    };
    let before = head_files();
    fs::write(temp.path().join("draft.txt"), "dirty unsnapshotted").unwrap();
    let access: Arc<dyn Access> = Arc::new(ReadBinding(temp.path().join(".jj/repo")));
    let reader = Executor::new(1).unwrap();
    let Response::QueryInfo(info) = reader
        .read(access.clone(), op, Request::QueryInfo)
        .await
        .unwrap()
    else {
        panic!()
    };
    assert_eq!(info.jj_version, "0.45.1");
    assert!(!info.callables.iter().any(|c| c.name == "files"));
    for (expression, expected) in aliases.into_iter().zip(oracle) {
        let Response::Graph(page) = reader
            .read(
                access.clone(),
                op,
                Request::Graph(
                    GraphRequest::Log {
                        revset: expression.into(),
                        workspace: Some("default".into()),
                        user_email: "browser@example.invalid".into(),
                        at_time_millis: 1_790_000_000_000,
                    },
                    GraphWindow {
                        limit: 100,
                        after: None,
                    },
                ),
            )
            .await
            .unwrap()
        else {
            panic!()
        };
        let jj_store_jj::local_browser::Nodes::Log(nodes) = page.data else {
            panic!()
        };
        assert_eq!(
            nodes
                .iter()
                .map(|n| format!("{}\n", n.commit.id))
                .collect::<String>(),
            expected,
            "{expression}"
        );
    }
    let query = GraphRequest::OperationsFiltered {
        query: "description:describe snapshot:false".into(),
    };
    let Response::Graph(first) = reader
        .read(
            access.clone(),
            op,
            Request::Graph(
                query.clone(),
                GraphWindow {
                    limit: 1,
                    after: None,
                },
            ),
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    let jj_store_jj::local_browser::Nodes::Operations(nodes) = &first.data else {
        panic!()
    };
    assert_eq!(nodes.len(), 1);
    assert!(nodes[0].description.contains("describe"));
    let Response::Graph(second) = reader
        .read(
            access.clone(),
            op,
            Request::Graph(
                query,
                GraphWindow {
                    limit: 1,
                    after: first.next,
                },
            ),
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    let jj_store_jj::local_browser::Nodes::Operations(nodes2) = second.data else {
        panic!()
    };
    assert!(nodes2[0].description.contains("describe"));
    assert!(!nodes[0].parents.contains(&nodes2[0].id));
    assert_eq!(second.next, None);
    let unsupported = Request::Graph(
        GraphRequest::Log {
            revset: "files('draft.txt')".into(),
            workspace: Some("default".into()),
            user_email: "".into(),
            at_time_millis: 1_790_000_000_000,
        },
        GraphWindow {
            limit: 10,
            after: None,
        },
    );
    assert!(
        matches!(reader.read(access.clone(), op, unsupported).await.err().unwrap(), q::Error::InvalidQuery(q::language::Diagnostic { kind, .. }) if kind == "unsupported_predicate")
    );
    assert_eq!(head_files(), before);
    assert_eq!(
        fs::read_to_string(temp.path().join("draft.txt")).unwrap(),
        "dirty unsnapshotted"
    );
}
