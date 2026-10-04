use super::*;
use c::references::{Kind, Reference, Request};

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn native_prepared_references_select_visible_windows_before_target_reads() {
    use jj_store_jj::query as q;
    let f = BrowsingFixture::new().await;
    // A long visible chain, two divergent targets, a hidden predecessor, and
    // many bookmark labels make full history/body admission observable.
    let mut last = b::CommitId::from_bytes(f.head.as_bytes());
    for n in 40..72u8 {
        let mut commit = roots::commit();
        commit.change_id = b::ChangeId::from_bytes(&[n; 16]);
        commit.parents = vec![last];
        commit.description = format!("visible {n}");
        let key = f.f.objects.put(&commit).await.unwrap().0;
        last = b::CommitId::from_bytes(key.as_bytes());
    }
    let mut hidden = roots::commit();
    hidden.change_id = b::ChangeId::from_bytes(&[90; 16]);
    hidden.description = "hidden predecessor".into();
    let hidden_key = f.f.objects.put(&hidden).await.unwrap().0;
    let mut divergent = roots::commit();
    divergent.change_id = b::ChangeId::from_bytes(&[71; 16]);
    divergent.parents = vec![b::CommitId::from_bytes(f.base.as_bytes())];
    divergent.predecessors = vec![b::CommitId::from_bytes(hidden_key.as_bytes())];
    divergent.description = "divergent visible".into();
    let key = f.f.objects.put(&divergent).await.unwrap().0;
    let divergent = b::CommitId::from_bytes(key.as_bytes());
    let mut view = o::View::make_root(last.clone());
    view.head_ids.insert(divergent.clone());
    view.local_bookmarks.insert(
        "conflicted".into(),
        o::RefTarget::from_merge(Merge::from_vec(vec![
            Some(last.clone()),
            None,
            Some(divergent.clone()),
        ])),
    );
    for n in 0..256 {
        view.local_bookmarks.insert(
            format!("b/{n:04}").into(),
            o::RefTarget::normal(last.clone()),
        );
    }
    view.wc_commit_ids.insert("desktop".into(), last.clone());
    view.wc_commit_ids.insert("other".into(), divergent);
    let key = f.f.objects.put(&view).await.unwrap().0;
    let mut operation = o::Operation::make_root(o::ViewId::from_bytes(key.as_bytes()));
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
    let cache = tempfile::tempdir().unwrap();
    let executors = q::workers::Executors::new(
        cache.path().into(),
        q::Limits::default(),
        c::Limits {
            max_history_visits: 4,
            ..Default::default()
        },
        q::cache::Policy::default(),
        1,
    )
    .unwrap();
    read.prepare_index(executors.graph(), q::Cancellation::default())
        .await
        .unwrap();
    let ordinary = reader();
    let request = Request {
        operation,
        query: String::new(),
        exact: None,
        kind: Kind::All,
        limit: 1,
        anchor: None,
        direction: c::Direction::Forward,
    };
    let tiny = ContentReader::new(
        c::Limits {
            max_history_visits: 4,
            ..Default::default()
        },
        1,
    )
    .unwrap();
    assert!(matches!(
        read.references(&tiny, request.clone()).await,
        Err(q::Error::Limit)
    ));
    // Each simulated page is a new admitted request; the prepared generation
    // is reused without extending a short-lived CommittedRead deadline.
    let pin = || async {
        let OpenRead::Pinned(read) =
            f.f.session
                .open_read(Some(operation), ReadLimits::default())
                .await
                .unwrap()
        else {
            panic!()
        };
        Arc::new(read)
    };
    for kind in [Kind::All, Kind::Changes] {
        for direction in [c::Direction::Forward, c::Direction::Backward] {
            for anchor in [
                None,
                Some("bookmark:main".into()),
                Some("change:2".into()),
                Some("z".into()),
            ] {
                let req = Request {
                    kind,
                    direction,
                    anchor,
                    ..request.clone()
                };
                eprintln!(
                    "reference equivalence: {kind:?}/{direction:?}/{:?}",
                    req.anchor
                );
                let read = pin().await;
                let expected = read.references(&ordinary, req.clone()).await.unwrap();
                let actual = read.references(executors.content(), req).await.unwrap();
                assert_eq!(actual.references, expected.references);
                assert_eq!(actual.previous, expected.previous);
                assert_eq!(actual.next, expected.next);
                assert!(actual.stats.object_reads <= 4, "{:?}", actual.stats);
            }
        }
    }
    let read = pin().await;
    let measured = read
        .references(
            executors.content(),
            Request {
                kind: Kind::Changes,
                ..request.clone()
            },
        )
        .await
        .unwrap();
    eprintln!("ready native reference stats: {:?}", measured.stats);
    // The virtual-root row needs only the freshly checked operation and view.
    assert_eq!(measured.stats.object_reads, 2);
    let expected = read
        .references(
            &ordinary,
            Request {
                kind: Kind::Changes,
                limit: 100,
                ..request.clone()
            },
        )
        .await
        .unwrap();
    assert!(expected.references.iter().any(|r| matches!(
        r,
        Reference::Change {
            divergent: true,
            ..
        }
    )));
    assert!(
        !expected
            .references
            .iter()
            .any(|r| matches!(r, Reference::Change {change, ..} if change.as_bytes() == &[90; 16]))
    );
    for row in expected.references {
        let Reference::Change { change, .. } = row else {
            panic!()
        };
        let req = Request {
            kind: Kind::Changes,
            exact: Some(change.to_string()),
            ..request.clone()
        };
        let read = pin().await;
        let expected = read.references(&ordinary, req.clone()).await.unwrap();
        let actual = read.references(executors.content(), req).await.unwrap();
        assert_eq!(actual.references, expected.references);
        assert!(actual.previous.is_none() && actual.next.is_none());
    }
}

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
