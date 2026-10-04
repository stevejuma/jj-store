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
        .read(access, operation, q::Request::References(request))
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
            access,
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
}
