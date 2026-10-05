use super::{ReadBinding, jj};
use jj_store_jj::{local_browser as q, query::workers::Workers};
use std::sync::Arc;

#[tokio::test]
async fn local_path_metadata_crosses_merges_using_first_parent_history() {
    let temp = tempfile::tempdir().unwrap();
    jj(temp.path(), &["git", "init", "--colocate"]);
    for name in ["a.txt", "b.txt", "c.txt"] {
        std::fs::write(temp.path().join(name), "base\n").unwrap();
    }
    jj(temp.path(), &["describe", "-m", "Base"]);
    jj(temp.path(), &["bookmark", "create", "base"]);
    jj(temp.path(), &["new", "base", "-m", "Left edit"]);
    std::fs::write(temp.path().join("a.txt"), "left\n").unwrap();
    jj(temp.path(), &["bookmark", "create", "left"]);
    jj(temp.path(), &["new", "base", "-m", "Right edit"]);
    std::fs::write(temp.path().join("b.txt"), "right\n").unwrap();
    jj(temp.path(), &["bookmark", "create", "right"]);
    jj(
        temp.path(),
        &["new", "left", "right", "-m", "Merge feature"],
    );
    let commit = jj(
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
    let access: Arc<dyn q::Access> = Arc::new(ReadBinding(temp.path().join(".jj/repo")));
    let q::Response::Paths(page) = q::Executor::new(1)
        .unwrap()
        .read(
            access,
            operation,
            q::Request::Paths(q::paths::Request {
                commit,
                path: String::new(),
                recursive: false,
                include_last_change: true,
                limit: 20,
                anchor: None,
                direction: Default::default(),
            }),
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    assert_eq!(page.entries.len(), 3);
    for (entry, expected) in page
        .entries
        .iter()
        .zip(["Left edit", "Merge feature", "Base"])
    {
        assert!(
            matches!(&entry.last_change, q::paths::LastChange::Available { description, .. } if description.trim() == expected),
            "{}: {:?}",
            entry.path,
            entry.last_change
        );
    }
}

#[tokio::test]
async fn ordinary_point_reads_do_not_import_missing_extra_metadata() {
    let temp = tempfile::tempdir().unwrap();
    jj(temp.path(), &["git", "init", "--colocate"]);
    std::fs::write(temp.path().join("selected.txt"), "selected\n").unwrap();
    jj(temp.path(), &["describe", "-m", "Selected"]);
    jj(temp.path(), &["bookmark", "create", "main"]);
    let operation = jj(
        temp.path(),
        &["op", "log", "--no-graph", "-n", "1", "-T", "id"],
    )
    .parse()
    .unwrap();
    let commit = jj(
        temp.path(),
        &["log", "--no-graph", "-r", "@", "-T", "commit_id"],
    )
    .parse()
    .unwrap();
    let metadata = temp.path().join(".jj/repo");
    let heads = metadata.join("store/extra/heads");
    for entry in std::fs::read_dir(&heads).unwrap() {
        std::fs::remove_file(entry.unwrap().path()).unwrap();
    }
    let extra_files = std::fs::read_dir(metadata.join("store/extra"))
        .unwrap()
        .count();
    let reader = q::Executor::new(1).unwrap();
    let access: Arc<dyn q::Access> = Arc::new(ReadBinding(metadata.clone()));
    let summary = reader
        .read(
            access.clone(),
            operation,
            q::Request::CommitSummary { commit },
        )
        .await
        .unwrap();
    let q::Response::CommitSummary(summary) = summary else {
        panic!()
    };
    let q::ObjectData::Commit(data) = summary.data else {
        panic!()
    };
    assert_eq!(data.description.trim(), "Selected");
    assert_eq!(
        std::fs::read_dir(&heads).unwrap().count(),
        0,
        "ordinary reads must not create extra heads"
    );
    assert_eq!(
        std::fs::read_dir(metadata.join("store/extra"))
            .unwrap()
            .count(),
        extra_files
    );
    let q::Response::References(page) = reader
        .read(
            access,
            operation,
            q::Request::References(q::references::Request {
                operation,
                query: String::new(),
                exact: None,
                kind: q::references::Kind::Bookmarks,
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
    assert_eq!(page.references.len(), 1);
    assert_eq!(std::fs::read_dir(&heads).unwrap().count(), 0);
}

#[tokio::test]
async fn historical_file_reads_stop_after_proving_the_selected_commit() {
    let temp = tempfile::tempdir().unwrap();
    jj(temp.path(), &["git", "init", "--colocate"]);
    std::fs::write(temp.path().join("selected.txt"), "historical content\n").unwrap();
    jj(temp.path(), &["describe", "-m", "Historical"]);
    let parent = jj(
        temp.path(),
        &["log", "--no-graph", "-r", "@", "-T", "commit_id"],
    )
    .parse()
    .unwrap();
    jj(temp.path(), &["new", "-m", "Second"]);
    jj(temp.path(), &["new", "-m", "Third"]);
    jj(temp.path(), &["new", "-m", "Current"]);
    let operation = jj(
        temp.path(),
        &["op", "log", "--no-graph", "-n", "1", "-T", "id"],
    )
    .parse()
    .unwrap();
    let index_dir = temp.path().join(".jj/repo/index/segments");
    let mut original = std::collections::BTreeMap::new();
    for entry in std::fs::read_dir(&index_dir).unwrap() {
        let path = entry.unwrap().path();
        std::fs::write(&path, b"unavailable derived index").unwrap();
        original.insert(path, b"unavailable derived index".to_vec());
    }
    let access: Arc<dyn q::Access> = Arc::new(ReadBinding(temp.path().join(".jj/repo")));
    let reader = q::Executor::with_workers(
        Workers::new(1).unwrap(),
        q::Limits {
            max_history_visits: 4,
            ..Default::default()
        },
    );
    let q::Response::Paths(rows) = reader
        .read(
            access.clone(),
            operation,
            q::Request::Paths(q::paths::Request {
                include_last_change: false,
                commit: parent,
                path: String::new(),
                recursive: false,
                limit: 10,
                anchor: None,
                direction: Default::default(),
            }),
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    assert_eq!(rows.entries.len(), 1);
    assert!(rows.stats.metadata_nodes <= 7, "{:?}", rows.stats);
    let Some(q::dto::TreeValue::File { id: file, .. }) = rows.entries[0].terms[0] else {
        panic!()
    };
    let q::Response::FileAt(contents) = reader
        .read(
            access.clone(),
            operation,
            q::Request::FileAt {
                commit: parent,
                path: "selected.txt".into(),
                file,
            },
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    assert_eq!(contents.text.as_deref(), Some("historical content\n"));
    let q::Response::CommitSummary(summary) = reader
        .read(
            access,
            operation,
            q::Request::CommitSummary { commit: parent },
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    let q::ObjectData::Commit(commit) = summary.data else {
        panic!()
    };
    assert_eq!(commit.description.trim(), "Historical");
    let after: std::collections::BTreeMap<_, _> = std::fs::read_dir(index_dir)
        .unwrap()
        .map(|entry| {
            let path = entry.unwrap().path();
            let bytes = std::fs::read(&path).unwrap();
            (path, bytes)
        })
        .collect();
    assert_eq!(after, original);
}

#[tokio::test]
async fn historical_point_membership_retains_abandoned_commits_and_excludes_later_operations() {
    let temp = tempfile::tempdir().unwrap();
    jj(temp.path(), &["git", "init", "--colocate"]);
    jj(temp.path(), &["describe", "-m", "Retained abandoned"]);
    let abandoned = jj(
        temp.path(),
        &["log", "--no-graph", "-r", "@", "-T", "commit_id"],
    )
    .parse()
    .unwrap();
    jj(temp.path(), &["abandon", "@"]);
    jj(temp.path(), &["new", "-m", "Selected"]);
    let operation = jj(
        temp.path(),
        &["op", "log", "--no-graph", "-n", "1", "-T", "id"],
    )
    .parse()
    .unwrap();
    jj(temp.path(), &["new", "-m", "Future"]);
    let future = jj(
        temp.path(),
        &["log", "--no-graph", "-r", "@", "-T", "commit_id"],
    )
    .parse()
    .unwrap();
    let access: Arc<dyn q::Access> = Arc::new(ReadBinding(temp.path().join(".jj/repo")));
    let reader = q::Executor::new(1).unwrap();
    let q::Response::CommitSummary(summary) = reader
        .read(
            access.clone(),
            operation,
            q::Request::CommitSummary { commit: abandoned },
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    let q::ObjectData::Commit(commit) = summary.data else {
        panic!()
    };
    assert_eq!(commit.description.trim(), "Retained abandoned");
    assert!(matches!(
        reader
            .read(
                access,
                operation,
                q::Request::CommitSummary { commit: future }
            )
            .await,
        Err(q::Error::NotCommitted)
    ));
}

#[tokio::test]
async fn local_metadata_and_bookmark_pages_do_not_load_history_or_index() {
    let temp = tempfile::tempdir().unwrap();
    jj(temp.path(), &["git", "init", "--colocate"]);
    jj(temp.path(), &["describe", "-m", "Ancestor"]);
    jj(temp.path(), &["new", "-m", "Selected"]);
    let names: Vec<_> = (0..10_000).map(|i| format!("bookmark-{i:04}")).collect();
    let mut args = vec!["bookmark", "create"];
    args.extend(names.iter().map(String::as_str));
    jj(temp.path(), &args);
    let operation = jj(
        temp.path(),
        &["op", "log", "--no-graph", "-n", "1", "-T", "id"],
    )
    .parse()
    .unwrap();
    let access: Arc<dyn q::Access> = Arc::new(ReadBinding(temp.path().join(".jj/repo")));
    // The selected operation is a head; neither its ancestry nor any commit's
    // ancestry is necessary to return metadata or one bookmark target.
    let reader = q::Executor::with_workers(
        Workers::new(1).unwrap(),
        q::Limits {
            max_history_visits: 1,
            ..Default::default()
        },
    );
    // A corrupt derived index must not prevent operation/view-only reads.
    let index = temp.path().join(".jj/repo/index");
    let mut corrupted = 0;
    for entry in std::fs::read_dir(index.join("segments")).unwrap() {
        let path = entry.unwrap().path();
        if path.is_file() {
            std::fs::write(path, b"invalid derived index").unwrap();
            corrupted += 1;
        }
    }
    assert!(corrupted > 0);
    assert!(matches!(
        reader
            .read(access.clone(), operation, q::Request::QueryInfo)
            .await
            .unwrap(),
        q::Response::QueryInfo(_)
    ));
    let request = q::references::Request {
        exact: None,
        operation,
        query: String::new(),
        kind: q::references::Kind::Bookmarks,
        limit: 1,
        anchor: Some("bookmark:bookmark-8999".into()),
        direction: Default::default(),
    };
    let q::Response::References(page) = reader
        .read(
            access.clone(),
            operation,
            q::Request::References(request.clone()),
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    assert_eq!(page.references.len(), 1);
    let q::references::Reference::Bookmark { name, targets, .. } = &page.references[0] else {
        panic!()
    };
    assert_eq!(name, "bookmark-9000");
    assert_eq!(targets[0].description.trim(), "Selected");
    assert!(page.previous.is_some());
    assert!(page.next.is_some());
    assert!(page.stats.metadata_nodes <= 3, "{:?}", page.stats);
    assert!(page.stats.work < 16, "{:?}", page.stats);
    // The workspace picker must not discover workspaces by walking change history.
    let workspace: q::references::Request = serde_json::from_value(serde_json::json!({
        "operation":operation, "query":"", "kind":"workspaces", "limit":1,
        "anchor":null, "direction":"forward"
    }))
    .unwrap();
    let q::Response::References(workspaces) = reader
        .read(access.clone(), operation, q::Request::References(workspace))
        .await
        .unwrap()
    else {
        panic!()
    };
    assert_eq!(workspaces.references.len(), 1);
    assert!(
        workspaces.stats.metadata_nodes <= 3,
        "{:?}",
        workspaces.stats
    );
    // Exact bookmark lookup must work outside the first page and hydrate one target.
    let exact: q::references::Request = serde_json::from_value(serde_json::json!({
        "operation":operation, "query":"", "kind":"bookmarks", "exact":"bookmark-9999",
        "limit":1, "anchor":null, "direction":"forward"
    }))
    .unwrap();
    let q::Response::References(exact) = reader
        .read(access.clone(), operation, q::Request::References(exact))
        .await
        .unwrap()
    else {
        panic!()
    };
    let q::references::Reference::Bookmark { name, .. } = &exact.references[0] else {
        panic!()
    };
    assert_eq!(name, "bookmark-9999");
    assert!(exact.next.is_none() && exact.previous.is_none());
    assert!(exact.stats.work < 16, "{:?}", exact.stats);
    let q::Response::References(shared) = reader
        .read(
            access,
            operation,
            q::Request::References(q::references::Request {
                exact: None,
                limit: 50,
                ..request
            }),
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    assert_eq!(shared.references.len(), 50);
    assert!(shared.stats.metadata_nodes <= 3, "{:?}", shared.stats);
}

/// Read-only smoke for an explicitly provided registered Git-backed JJ folder.
#[tokio::test]
#[ignore = "requires JJ_STORE_LARGE_REPO pointing at an existing JJ metadata directory"]
async fn registered_large_repository_metadata_smoke() {
    let metadata = std::env::var_os("JJ_STORE_LARGE_REPO").expect("explicit metadata path");
    let access: Arc<dyn q::Access> = Arc::new(ReadBinding(metadata.into()));
    // Point proof may inspect multiple reference roots; keep an explicit budget
    // far below the repository's complete history, including for parent browsing.
    let reader = q::Executor::with_workers(
        Workers::new(1).unwrap(),
        q::Limits {
            max_history_visits: 1024,
            max_work: 100_000,
            ..Default::default()
        },
    );
    let heads = reader.heads(access.clone()).await.unwrap();
    let operation = heads.heads[0];
    let op_started = std::time::Instant::now();
    let q::Response::Graph(op_page) = reader
        .read(
            access.clone(),
            operation,
            q::Request::Graph(
                q::GraphRequest::OperationsFiltered {
                    query: "from:@".into(),
                },
                q::GraphWindow {
                    limit: 10,
                    after: None,
                },
            ),
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    eprintln!(
        "operation page: {:?}; {:?}",
        op_started.elapsed(),
        op_page.stats
    );
    let q::Nodes::Operations(nodes) = op_page.data else {
        panic!()
    };
    assert!(!nodes.is_empty());
    let started = std::time::Instant::now();
    assert!(matches!(
        reader
            .read(access.clone(), operation, q::Request::QueryInfo)
            .await
            .unwrap(),
        q::Response::QueryInfo(_)
    ));
    let q::Response::References(page) = reader
        .read(
            access.clone(),
            operation,
            q::Request::References(q::references::Request {
                exact: None,
                operation,
                query: String::new(),
                kind: q::references::Kind::Bookmarks,
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
    eprintln!(
        "metadata + one bookmark: {:?}; {:?}; rows={}",
        started.elapsed(),
        page.stats,
        page.references.len()
    );
    if let Some(q::references::Reference::Bookmark { targets, .. }) = page.references.first() {
        if let Some(target) = targets.first() {
            let started = std::time::Instant::now();
            let q::Response::Paths(paths) = reader
                .read(
                    access.clone(),
                    operation,
                    q::Request::Paths(q::paths::Request {
                        include_last_change: false,
                        commit: target.commit,
                        path: String::new(),
                        recursive: false,
                        limit: 10,
                        anchor: None,
                        direction: Default::default(),
                    }),
                )
                .await
                .unwrap()
            else {
                panic!()
            };
            eprintln!(
                "Linux directory: {:?}; {:?}; rows={}",
                started.elapsed(),
                paths.stats,
                paths.entries.len()
            );
            let q::Response::CommitSummary(header) = reader
                .read(
                    access.clone(),
                    operation,
                    q::Request::CommitSummary {
                        commit: target.commit,
                    },
                )
                .await
                .unwrap()
            else {
                panic!()
            };
            let q::ObjectData::Commit(header) = header.data else {
                panic!()
            };
            if let Some(parent) = header.parents.first() {
                let started = std::time::Instant::now();
                let q::Response::Paths(history) = reader
                    .read(
                        access.clone(),
                        operation,
                        q::Request::Paths(q::paths::Request {
                            include_last_change: false,
                            commit: *parent,
                            path: String::new(),
                            recursive: false,
                            limit: 10,
                            anchor: None,
                            direction: Default::default(),
                        }),
                    )
                    .await
                    .unwrap()
                else {
                    panic!()
                };
                eprintln!(
                    "Linux historical directory: {:?}; {:?}; rows={}",
                    started.elapsed(),
                    history.stats,
                    history.entries.len()
                );
            }
            if let Some(entry) = paths.entries.iter().find(|entry| {
                matches!(
                    entry.terms.first(),
                    Some(Some(q::dto::TreeValue::File { .. }))
                )
            }) {
                let Some(q::dto::TreeValue::File { id: file, .. }) = entry.terms[0] else {
                    panic!()
                };
                let q::Response::FileAt(contents) = reader
                    .read(
                        access.clone(),
                        operation,
                        q::Request::FileAt {
                            commit: target.commit,
                            path: entry.path.clone(),
                            file,
                        },
                    )
                    .await
                    .unwrap()
                else {
                    panic!()
                };
                eprintln!("Linux scoped file {}: {} bytes", entry.path, contents.bytes);
            }
        }
    }
}

#[tokio::test]
async fn local_file_rows_read_only_the_selected_commit_and_traversed_trees() {
    let temp = tempfile::tempdir().unwrap();
    jj(temp.path(), &["git", "init", "--colocate"]);
    std::fs::write(temp.path().join("a.txt"), "base\n").unwrap();
    std::fs::create_dir(temp.path().join("nested")).unwrap();
    std::fs::write(temp.path().join("nested/deep.txt"), "deep\n").unwrap();
    jj(temp.path(), &["describe", "-m", "Base"]);
    jj(temp.path(), &["new", "-m", "Selected"]);
    std::fs::write(temp.path().join("a.txt"), "selected\n").unwrap();
    std::fs::write(temp.path().join("nested/deep.txt"), "selected deep\n").unwrap();
    jj(temp.path(), &["describe", "-m", "Selected"]);
    let commit = jj(
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
    let access: Arc<dyn q::Access> = Arc::new(ReadBinding(temp.path().join(".jj/repo")));
    let reader = q::Executor::with_workers(
        Workers::new(1).unwrap(),
        q::Limits {
            max_history_visits: 2,
            ..Default::default()
        },
    );
    let q::Response::Paths(page) = reader
        .read(
            access.clone(),
            operation,
            q::Request::Paths(q::paths::Request {
                include_last_change: true,
                commit,
                path: String::new(),
                recursive: false,
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
    assert_eq!(page.entries[0].path, "a.txt");
    assert!(
        matches!(page.entries[0].last_change, q::paths::LastChange::Available { commit: id, .. } if id == commit)
    );
    assert!(page.stats.metadata_nodes <= 5, "{:?}", page.stats);
    assert!(page.next.is_some());
    let q::Response::Paths(second) = reader
        .read(
            access.clone(),
            operation,
            q::Request::Paths(q::paths::Request {
                include_last_change: true,
                commit,
                path: String::new(),
                recursive: false,
                limit: 1,
                anchor: page.next,
                direction: Default::default(),
            }),
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    assert_eq!(second.entries[0].path, "nested");
    assert!(matches!(
        second.entries[0].terms[0],
        Some(q::dto::TreeValue::Tree { .. })
    ));
    let rows_only = q::Executor::with_workers(
        Workers::new(1).unwrap(),
        q::Limits {
            max_history_visits: 1,
            ..Default::default()
        },
    );
    let q::Response::Paths(rows) = rows_only
        .read(
            access.clone(),
            operation,
            q::Request::Paths(q::paths::Request {
                include_last_change: false,
                commit,
                path: String::new(),
                recursive: false,
                limit: 20,
                anchor: None,
                direction: Default::default(),
            }),
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    assert_eq!(rows.entries.len(), 2);
    assert!(rows.entries.iter().all(|row| matches!(
        row.last_change,
        q::paths::LastChange::Unavailable {
            reason: q::paths::LastChangeUnavailable::NotRequested
        }
    )));
    let q::Response::CommitSummary(summary) = rows_only
        .read(
            access.clone(),
            operation,
            q::Request::CommitSummary { commit },
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    assert!(matches!(summary.data, q::ObjectData::Commit(_)));
    let Some(q::dto::TreeValue::File { id: file, .. }) = rows.entries[0].terms[0] else {
        panic!()
    };
    let q::Response::FileAt(contents) = rows_only
        .read(
            access.clone(),
            operation,
            q::Request::FileAt {
                commit,
                path: "a.txt".into(),
                file,
            },
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    assert_eq!(contents.text.as_deref(), Some("selected\n"));
    assert!(matches!(
        rows_only
            .read(
                access.clone(),
                operation,
                q::Request::FileAt {
                    commit,
                    path: "nested/deep.txt".into(),
                    file
                }
            )
            .await,
        Err(q::Error::NotCommitted)
    ));
    let denied = reader
        .read(
            access,
            operation,
            q::Request::Paths(q::paths::Request {
                include_last_change: true,
                commit: jj_store_model::GitSha1Id::new([9; 20]),
                path: String::new(),
                recursive: false,
                limit: 1,
                anchor: None,
                direction: Default::default(),
            }),
        )
        .await;
    assert!(denied.is_err());
}

#[tokio::test]
async fn operation_pages_ignore_corrupt_commit_indexes() {
    let temp = tempfile::tempdir().unwrap();
    jj(temp.path(), &["git", "init", "--colocate"]);
    jj(temp.path(), &["describe", "-m", "Selected operation"]);
    let operation = jj(
        temp.path(),
        &["op", "log", "--no-graph", "-n", "1", "-T", "id"],
    )
    .parse()
    .unwrap();
    let index_dir = temp.path().join(".jj/repo/index/segments");
    let mut original = std::collections::BTreeMap::new();
    for entry in std::fs::read_dir(temp.path().join(".jj/repo/index/segments")).unwrap() {
        let path = entry.unwrap().path();
        if path.is_file() {
            std::fs::write(&path, b"corrupt derived index").unwrap();
            original.insert(path, b"corrupt derived index".to_vec());
        }
    }
    // Operation metadata is usable even when the selected view cannot be read.
    for entry in std::fs::read_dir(temp.path().join(".jj/repo/op_store/views")).unwrap() {
        let path = entry.unwrap().path();
        if path.is_file() {
            std::fs::write(path, b"unavailable view").unwrap();
        }
    }
    let reader = q::Executor::with_workers(Workers::new(1).unwrap(), q::Limits::default());
    let access: Arc<dyn q::Access> = Arc::new(ReadBinding(temp.path().join(".jj/repo")));
    let q::Response::Graph(page) = reader
        .read(
            access,
            operation,
            q::Request::Graph(
                q::GraphRequest::Operations,
                q::GraphWindow {
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
    let q::Nodes::Operations(nodes) = page.data else {
        panic!()
    };
    assert_eq!(nodes.len(), 1);
    assert_eq!(nodes[0].id, operation);
    let after: std::collections::BTreeMap<_, _> = std::fs::read_dir(index_dir)
        .unwrap()
        .map(|entry| {
            let path = entry.unwrap().path();
            let bytes = std::fs::read(&path).unwrap();
            (path, bytes)
        })
        .collect();
    assert_eq!(
        after, original,
        "operation browsing must not rebuild commit indexes"
    );
}
