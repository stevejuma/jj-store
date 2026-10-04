use super::{ReadBinding, jj};
use jj_store_jj::{local_browser as q, query::workers::Workers};
use std::sync::Arc;

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
    let q::Response::References(shared) = reader
        .read(
            access,
            operation,
            q::Request::References(q::references::Request {
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
    let reader = q::Executor::new(1).unwrap();
    let heads = reader.heads(access.clone()).await.unwrap();
    let operation = heads.heads[0];
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
                    access,
                    operation,
                    q::Request::Paths(q::paths::Request {
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
    let denied = reader
        .read(
            access,
            operation,
            q::Request::Paths(q::paths::Request {
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
