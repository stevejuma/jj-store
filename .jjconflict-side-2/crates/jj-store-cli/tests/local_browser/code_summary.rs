use super::{ReadBinding, jj};
use jj_store_jj::{local_browser as q, query::workers::Workers};
use std::{collections::BTreeMap, fs, path::Path, sync::Arc};

fn files(root: &Path) -> BTreeMap<std::path::PathBuf, Vec<u8>> {
    let mut result = BTreeMap::new();
    for entry in fs::read_dir(root).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            result.extend(files(&path));
        } else {
            result.insert(path.clone(), fs::read(path).unwrap());
        }
    }
    result
}
fn selected(root: &Path) -> jj_store_model::OperationId {
    jj(root, &["op", "log", "--no-graph", "-n", "1", "-T", "id"])
        .parse()
        .unwrap()
}
fn reader() -> q::Executor {
    q::Executor::with_workers(
        Workers::new(1).unwrap(),
        q::Limits {
            max_history_visits: 2,
            ..Default::default()
        },
    )
}
async fn summary(
    reader: &q::Executor,
    access: Arc<dyn q::Access>,
    operation: jj_store_model::OperationId,
    limit: usize,
) -> q::code_summary::Summary {
    let q::Response::CodeSummary(value) = reader
        .read(access, operation, q::Request::CodeSummary { limit })
        .await
        .unwrap()
    else {
        panic!()
    };
    value
}

#[tokio::test]
async fn local_code_summary_skips_damaged_indexes_missing_extras_ancestry_and_trees() {
    let temp = tempfile::tempdir().unwrap();
    jj(temp.path(), &["git", "init", "--colocate"]);
    fs::write(temp.path().join("file.txt"), "Historical tree\n").unwrap();
    jj(temp.path(), &["describe", "-m", "Historical"]);
    let historical = selected(temp.path());
    for n in 0..12 {
        jj(temp.path(), &["new", "-m", &format!("Current {n}")]);
    }
    let operation = selected(temp.path());
    let metadata = temp.path().join(".jj/repo");
    for entry in fs::read_dir(metadata.join("store/extra/heads")).unwrap() {
        fs::remove_file(entry.unwrap().path()).unwrap();
    }
    for entry in fs::read_dir(metadata.join("index/segments")).unwrap() {
        fs::write(entry.unwrap().path(), b"damaged derived index").unwrap();
    }
    let before = files(&metadata);
    let access: Arc<dyn q::Access> = Arc::new(ReadBinding(metadata.clone()));
    let current = summary(&reader(), access.clone(), operation, 1).await;
    assert_eq!(current.commits, 1);
    assert!(current.commits_partial);
    assert_eq!(current.latest.unwrap().description.trim(), "Current 11");
    assert!(current.stats.metadata_nodes <= 3, "{:?}", current.stats);
    // A historical operation still needs retained-operation membership proof.
    assert_eq!(
        reader()
            .read(
                access.clone(),
                historical,
                q::Request::CodeSummary { limit: 50 }
            )
            .await
            .err()
            .unwrap(),
        q::Error::Limit
    );
    let old = summary(
        &q::Executor::new(1).unwrap(),
        access.clone(),
        historical,
        50,
    )
    .await;
    assert_eq!(old.commits, 1);
    assert!(!old.commits_partial);
    assert_eq!(old.latest.unwrap().description.trim(), "Historical");
    for limit in [0, 257] {
        assert_eq!(
            reader()
                .read(access.clone(), operation, q::Request::CodeSummary { limit })
                .await
                .err()
                .unwrap(),
            q::Error::Limit
        );
    }
    let limited = q::Executor::with_workers(
        Workers::new(1).unwrap(),
        q::Limits {
            max_output_bytes: 1,
            ..Default::default()
        },
    );
    assert_eq!(
        limited
            .read(access, operation, q::Request::CodeSummary { limit: 1 })
            .await
            .err()
            .unwrap(),
        q::Error::Limit
    );
    assert_eq!(files(&metadata), before);
}

#[tokio::test]
async fn local_code_summary_hydrates_only_the_lexical_window_and_marks_remaining_heads() {
    let temp = tempfile::tempdir().unwrap();
    jj(temp.path(), &["git", "init", "--colocate"]);
    for n in 0..8 {
        let timestamp = format!("debug.commit-timestamp='2026-10-04T10:0{n}:00Z'");
        jj(
            temp.path(),
            &[
                "--config",
                &timestamp,
                "new",
                "-r",
                "root()",
                "-m",
                &format!("Head {n}"),
            ],
        );
        fs::write(temp.path().join(format!("head{n}.txt")), "Nonempty head\n").unwrap();
        jj(
            temp.path(),
            &[
                "--config",
                &timestamp,
                "describe",
                "-m",
                &format!("Head {n}"),
            ],
        );
    }
    let operation = selected(temp.path());
    let ids = jj(
        temp.path(),
        &[
            "log",
            "--no-graph",
            "-r",
            "heads(all()) & ~root()",
            "-T",
            "commit_id ++ '\n'",
        ],
    );
    let mut ids = ids.lines().collect::<Vec<_>>();
    ids.sort();
    let access: Arc<dyn q::Access> = Arc::new(ReadBinding(temp.path().join(".jj/repo")));
    let first = summary(&reader(), access.clone(), operation, 1).await;
    assert_eq!(first.commits, 1);
    assert!(first.commits_partial);
    assert_eq!(first.latest.unwrap().id.to_string(), ids[0]);
    assert!(first.stats.metadata_nodes <= 3, "{:?}", first.stats);
    let all_reader = q::Executor::new(1).unwrap();
    let all = summary(&all_reader, access, operation, 256).await;
    assert_eq!(all.commits, ids.len());
    assert!(!all.commits_partial);
    assert_eq!(all.latest.unwrap().description.trim(), "Head 7");
}
