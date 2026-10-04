use super::*;
use c::references::{Kind, Reference, Request};

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn workspace_windows_seek_filter_and_resolve_exact_names_without_history() {
    let f = BrowsingFixture::new().await;
    let mut view = o::View::make_root(b::CommitId::from_bytes(f.head.as_bytes()));
    for name in ["a/skip", "b/match", "c/skip", "d/match", "e/skip"] {
        view.wc_commit_ids
            .insert(name.into(), b::CommitId::from_bytes(f.head.as_bytes()));
    }
    let view = f.f.objects.put(&view).await.unwrap().0;
    let mut operation = o::Operation::make_root(o::ViewId::from_bytes(view.as_bytes()));
    operation.parents = vec![o::OperationId::from_bytes(f.metadata_operation.as_bytes())];
    let key = f.f.objects.put(&operation).await.unwrap().0;
    let operation = m::OperationId::from_slice(key.as_bytes()).unwrap();
    f.f.session
        .publish(BTreeSet::from([f.metadata_operation]), operation)
        .await
        .unwrap();
    let OpenRead::Pinned(read) =
        f.f.session
            .open_read(Some(operation), ReadLimits::default())
            .await
            .unwrap()
    else {
        panic!()
    };
    let read = Arc::new(read);
    let reader = ContentReader::new(
        c::Limits {
            max_history_visits: 1,
            ..Default::default()
        },
        1,
    )
    .unwrap();
    let request = Request {
        operation,
        query: "MATCH".into(),
        exact: None,
        kind: Kind::Workspaces,
        limit: 1,
        anchor: None,
        direction: c::Direction::Forward,
    };
    let first = read.references(&reader, request.clone()).await.unwrap();
    assert!(
        matches!(&first.references[0], Reference::Workspace {name, targets} if name == "b/match" && targets[0].workspaces == ["b/match"])
    );
    assert!(first.previous.is_none());
    assert_eq!(first.next.as_deref(), Some("workspace:b/match"));
    let second = read
        .references(
            &reader,
            Request {
                anchor: first.next.clone(),
                ..request.clone()
            },
        )
        .await
        .unwrap();
    assert!(matches!(&second.references[0], Reference::Workspace {name, ..} if name == "d/match"));
    assert!(second.next.is_none());
    let back = read
        .references(
            &reader,
            Request {
                anchor: second.previous,
                direction: c::Direction::Backward,
                ..request.clone()
            },
        )
        .await
        .unwrap();
    assert_eq!(back.references, first.references);
    for name in ["d/match", "missing"] {
        let exact = read
            .references(
                &reader,
                Request {
                    query: String::new(),
                    exact: Some(name.into()),
                    ..request.clone()
                },
            )
            .await
            .unwrap();
        assert_eq!(exact.references.len(), usize::from(name == "d/match"));
        assert!(exact.next.is_none() && exact.previous.is_none());
    }
    let all = read
        .references(
            &reader,
            Request {
                query: String::new(),
                limit: 5,
                ..request.clone()
            },
        )
        .await
        .unwrap();
    assert_eq!(all.references.len(), 5);
    // Shared target hydration is independent of workspace row count.
    assert!(all.stats.object_reads <= 3, "{:?}", all.stats);
    assert!(matches!(
        read.references(
            &reader,
            Request {
                exact: Some("d/match".into()),
                ..request
            }
        )
        .await,
        Err(jj_store_jj::query::Error::Invalid)
    ));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn operation_pages_do_not_admit_or_build_a_commit_index() {
    let f = BrowsingFixture::new().await;
    let cache = tempfile::tempdir().unwrap();
    let executor = jj_store_jj::query::GraphExecutor::new(
        cache.path().to_owned(),
        jj_store_jj::query::Limits {
            max_index_commits: 1,
            max_inline_rebuild_commits: 0,
            ..Default::default()
        },
        1,
    )
    .unwrap();
    let page = f
        .read
        .graph(
            &executor,
            jj_store_jj::query::Request::Operations,
            jj_store_jj::query::Page {
                limit: 1,
                after: None,
            },
        )
        .await
        .unwrap();
    let jj_store_jj::query::Nodes::Operations(nodes) = page.data else {
        panic!()
    };
    assert_eq!(nodes.len(), 1);
    assert_eq!(nodes[0].id, f.operation);
    assert_eq!(std::fs::read_dir(cache.path()).unwrap().count(), 0);
}
