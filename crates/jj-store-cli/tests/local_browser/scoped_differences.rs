use super::{ReadBinding, jj};
use jj_store_jj::{local_browser as q, query::workers::Workers};
use std::{fs, sync::Arc};

#[tokio::test]
async fn selected_diffs_and_conflicts_do_not_preflight_unrelated_history() {
    let temp = tempfile::tempdir().unwrap();
    jj(temp.path(), &["git", "init", "--colocate"]);
    for n in 0..12 {
        fs::write(temp.path().join("selected.txt"), format!("old {n}\n")).unwrap();
        jj(temp.path(), &["new", "-m", "Older history"]);
    }
    fs::write(temp.path().join("selected.txt"), "before\n").unwrap();
    jj(temp.path(), &["describe", "-m", "Before"]);
    let from = jj(
        temp.path(),
        &["log", "--no-graph", "-r", "@", "-T", "commit_id"],
    )
    .parse()
    .unwrap();
    jj(temp.path(), &["new", "-m", "Selected"]);
    fs::write(temp.path().join("selected.txt"), "after\n").unwrap();
    jj(temp.path(), &["describe", "-m", "Selected"]);
    let to = jj(
        temp.path(),
        &["log", "--no-graph", "-r", "@", "-T", "commit_id"],
    )
    .parse()
    .unwrap();
    let operation = jj(
        temp.path(),
        &["op", "log", "--no-graph", "-n", "1", "-T", "id"],
    )
    .parse()
    .unwrap();
    let metadata = temp.path().join(".jj/repo");
    let heads_before = fs::read_dir(metadata.join("store/extra/heads"))
        .unwrap()
        .map(|e| e.unwrap().file_name())
        .collect::<Vec<_>>();
    let reader = q::Executor::with_workers(
        Workers::new(1).unwrap(),
        q::Limits {
            max_history_visits: 4,
            ..Default::default()
        },
    );
    let access: Arc<dyn q::Access> = Arc::new(ReadBinding(metadata.clone()));
    let q::Response::Diff(diff) = reader
        .read(
            access.clone(),
            operation,
            q::Request::Diff(q::diff::Request {
                from,
                to,
                path: "selected.txt".into(),
                content: true,
            }),
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    assert_eq!(diff.changes.len(), 1);
    assert!(
        matches!(&diff.changes[0].content, q::diff::Content::Text { hunks } if hunks.iter().any(|h| h.before.contains("before") && h.after.contains("after")))
    );
    assert!(diff.stats.metadata_nodes < 16, "{:?}", diff.stats);
    let q::Response::DiffPaths(page) = reader
        .read(
            access.clone(),
            operation,
            q::Request::DiffPaths(q::diff_summary::Request {
                from,
                to,
                path: "selected.txt".into(),
                limit: 1,
                anchor: None,
                direction: Default::default(),
            }),
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    assert_eq!(page.changes.len(), 1);
    let q::Response::Conflicts(conflicts) = reader
        .read(
            access,
            operation,
            q::Request::Conflicts(q::conflicts::Request {
                commit: to,
                path: "selected.txt".into(),
            }),
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    assert!(conflicts.conflicts.is_empty());
    let heads_after = fs::read_dir(metadata.join("store/extra/heads"))
        .unwrap()
        .map(|e| e.unwrap().file_name())
        .collect::<Vec<_>>();
    assert_eq!(heads_after, heads_before);
}
