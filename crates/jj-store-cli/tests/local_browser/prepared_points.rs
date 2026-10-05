use super::{ReadBinding, jj};
use jj_store_jj::{local_browser as q, query::Cancellation};
use std::{fs, sync::Arc};

fn without_stats(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Object(fields) => {
            fields.remove("stats");
            for field in fields.values_mut() {
                without_stats(field);
            }
        }
        serde_json::Value::Array(values) => values.iter_mut().for_each(without_stats),
        _ => {}
    }
}

#[tokio::test]
async fn prepared_historical_points_skip_ancestry_and_preserve_content_and_conflicts() {
    let source = tempfile::tempdir().unwrap();
    let output = tempfile::tempdir().unwrap();
    jj(source.path(), &["git", "init", "--colocate"]);
    fs::write(source.path().join("selected.txt"), "before\n").unwrap();
    jj(source.path(), &["describe", "-m", "Before"]);
    let commit_id = || {
        jj(
            source.path(),
            &["log", "--no-graph", "-r", "@", "-T", "commit_id"],
        )
        .parse::<jj_store_model::GitSha1Id>()
        .unwrap()
    };
    let from = commit_id();
    jj(source.path(), &["new", "-m", "Left"]);
    fs::write(source.path().join("selected.txt"), "left\n").unwrap();
    jj(source.path(), &["describe", "-m", "Left"]);
    let left = commit_id();
    jj(source.path(), &["new", &from.to_string(), "-m", "Right"]);
    fs::write(source.path().join("selected.txt"), "right\n").unwrap();
    jj(source.path(), &["describe", "-m", "Right"]);
    let right = commit_id();
    jj(
        source.path(),
        &[
            "new",
            &left.to_string(),
            &right.to_string(),
            "-m",
            "Conflict",
        ],
    );
    let to = commit_id();
    for n in 0..12 {
        jj(
            source.path(),
            &["new", "-m", &format!("Unrelated later history {n}")],
        );
    }
    let operation = jj(
        source.path(),
        &["op", "log", "--no-graph", "-n", "1", "-T", "id"],
    )
    .parse()
    .unwrap();
    let metadata = source.path().join(".jj/repo");
    let access: Arc<dyn q::Access> = Arc::new(ReadBinding(metadata.clone()));
    let ordinary = q::Executor::new(1).unwrap();
    let paths = q::Request::Paths(q::paths::Request {
        commit: from,
        path: String::new(),
        recursive: false,
        include_last_change: false,
        last_change_mode: Default::default(),
        limit: 1,
        anchor: None,
        direction: Default::default(),
    });
    let q::Response::Paths(rows) = ordinary
        .read(access.clone(), operation, paths.clone())
        .await
        .unwrap()
    else {
        panic!()
    };
    let Some(q::dto::TreeValue::File { id: file, .. }) = rows.entries[0].terms[0] else {
        panic!()
    };
    let requests = vec![
        q::Request::CommitSummary { commit: from },
        paths,
        q::Request::FileAt {
            commit: from,
            path: "selected.txt".into(),
            file,
        },
        q::Request::Diff(q::diff::Request {
            from,
            to,
            path: "selected.txt".into(),
            content: true,
        }),
        q::Request::DiffPaths(q::diff_summary::Request {
            from,
            to,
            path: String::new(),
            limit: 1,
            anchor: None,
            direction: Default::default(),
        }),
        q::Request::Conflicts(q::conflicts::Request {
            commit: to,
            path: "selected.txt".into(),
        }),
    ];
    let mut expected = vec![];
    for request in &requests {
        let response = ordinary
            .read(access.clone(), operation, request.clone())
            .await
            .unwrap();
        if let q::Response::Conflicts(result) = &response {
            assert_eq!(result.conflicts.len(), 1);
            assert_eq!(result.conflicts[0].terms.len(), 3);
        }
        expected.push(serde_json::to_value(response).unwrap());
    }
    let prepared = Arc::new(
        ordinary
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
    let small = q::Executor::with_workers(
        jj_store_jj::query::workers::Workers::new(1).unwrap(),
        q::Limits {
            max_history_visits: 4,
            ..Default::default()
        },
    );
    assert!(matches!(
        small
            .read(access.clone(), operation, requests[0].clone())
            .await,
        Err(q::Error::Limit)
    ));
    jj(source.path(), &["new", "-m", "Outside captured operation"]);
    let later = commit_id();
    let mut source_segments = std::collections::BTreeMap::new();
    for entry in fs::read_dir(metadata.join("index/segments")).unwrap() {
        let path = entry.unwrap().path();
        fs::write(&path, b"unavailable source index").unwrap();
        source_segments.insert(path, b"unavailable source index".to_vec());
    }
    fs::remove_dir_all(prepared.path().join("segments")).unwrap();
    let extra_heads = || {
        fs::read_dir(metadata.join("store/extra/heads"))
            .unwrap()
            .map(|e| e.unwrap().file_name())
            .collect::<std::collections::BTreeSet<_>>()
    };
    let heads = extra_heads();
    for (request, mut expected) in requests.into_iter().zip(expected) {
        let mut actual = serde_json::to_value(
            small
                .read_prepared(access.clone(), prepared.clone(), request)
                .await
                .unwrap(),
        )
        .unwrap();
        let stats = actual["data"].get("stats");
        if let Some(stats) = stats {
            assert!(stats["metadata_nodes"].as_u64().unwrap() < 16, "{stats}");
        }
        without_stats(&mut expected);
        without_stats(&mut actual);
        assert_eq!(actual, expected);
    }
    for request in [
        q::Request::CommitSummary { commit: later },
        q::Request::Paths(q::paths::Request {
            commit: later,
            path: String::new(),
            recursive: false,
            include_last_change: false,
            last_change_mode: Default::default(),
            limit: 1,
            anchor: None,
            direction: Default::default(),
        }),
        q::Request::FileAt {
            commit: later,
            path: "selected.txt".into(),
            file,
        },
        q::Request::Diff(q::diff::Request {
            from,
            to: later,
            path: "selected.txt".into(),
            content: true,
        }),
        q::Request::DiffPaths(q::diff_summary::Request {
            from: later,
            to,
            path: String::new(),
            limit: 1,
            anchor: None,
            direction: Default::default(),
        }),
        q::Request::Conflicts(q::conflicts::Request {
            commit: later,
            path: "selected.txt".into(),
        }),
        q::Request::FileAt {
            commit: from,
            path: "selected.txt".into(),
            file: jj_store_model::GitSha1Id::new([1; 20]),
        },
    ] {
        assert!(matches!(
            small
                .read_prepared(access.clone(), prepared.clone(), request)
                .await,
            Err(q::Error::NotCommitted)
        ));
    }
    assert_eq!(heads, extra_heads());
    for (path, bytes) in source_segments {
        assert_eq!(fs::read(path).unwrap(), bytes);
    }
}
