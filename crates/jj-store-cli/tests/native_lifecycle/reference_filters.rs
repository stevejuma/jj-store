use super::*;
use c::references::{Kind, Reference, Request};

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn native_filtered_references_seek_pinned_text_before_selected_target_reads() {
    use jj_store_jj::query as q;
    let f = BrowsingFixture::new().await;
    let mut last = b::CommitId::from_bytes(f.head.as_bytes());
    for n in 40..72u8 {
        let mut commit = roots::commit();
        commit.change_id = b::ChangeId::from_bytes(&[n; 16]);
        commit.parents = vec![last];
        commit.description = if n == 71 {
            "ΟΣ İSTANBUL needle".into()
        } else {
            format!("ordinary {n}")
        };
        last = b::CommitId::from_bytes(f.f.objects.put(&commit).await.unwrap().0.as_bytes());
    }
    let mut hidden = roots::commit();
    hidden.change_id = b::ChangeId::from_bytes(&[90; 16]);
    hidden.description = "hidden-only".into();
    let hidden = b::CommitId::from_bytes(f.f.objects.put(&hidden).await.unwrap().0.as_bytes());
    let mut divergent = roots::commit();
    divergent.change_id = b::ChangeId::from_bytes(&[71; 16]);
    divergent.parents = vec![b::CommitId::from_bytes(f.base.as_bytes())];
    divergent.predecessors = vec![hidden.clone()];
    divergent.description = "divergent visible needle".into();
    let divergent =
        b::CommitId::from_bytes(f.f.objects.put(&divergent).await.unwrap().0.as_bytes());
    let mut view = o::View::make_root(last.clone());
    view.head_ids.insert(divergent.clone());
    for n in 0..128 {
        view.local_bookmarks.insert(
            format!("b/{n:04}").into(),
            o::RefTarget::normal(last.clone()),
        );
    }
    view.local_bookmarks.insert(
        "removed".into(),
        Merge::from_vec(vec![
            Some(last.clone()),
            Some(hidden),
            Some(divergent.clone()),
        ]),
    );
    view.wc_commit_ids.insert("desktop".into(), last);
    view.wc_commit_ids.insert("other".into(), divergent);
    let view = f.f.objects.put(&view).await.unwrap().0;
    let mut operation = o::Operation::make_root(o::ViewId::from_bytes(view.as_bytes()));
    operation.parents = vec![o::OperationId::from_bytes(f.metadata_operation.as_bytes())];
    let operation =
        m::OperationId::from_slice(f.f.objects.put(&operation).await.unwrap().0.as_bytes())
            .unwrap();
    f.f.session
        .publish(BTreeSet::from([f.metadata_operation]), operation)
        .await
        .unwrap();
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
    pin()
        .await
        .prepare_index(executors.graph(), q::Cancellation::default())
        .await
        .unwrap();
    let request = Request {
        operation,
        query: "needle".into(),
        exact: None,
        kind: Kind::Changes,
        limit: 1,
        anchor: None,
        direction: c::Direction::Forward,
    };
    let ready = pin()
        .await
        .references(executors.content(), request.clone())
        .await
        .unwrap();
    assert!(
        matches!(&ready.references[0], Reference::Change {divergent: true, targets, ..} if targets.len() == 2)
    );
    assert!(ready.stats.object_reads <= 4, "{:?}", ready.stats);
    let ordinary = reader();
    let mut windows = 0;
    for query in ["needle", "ΟΣ", "İST", "hidden-only", "b/012", "zz-missing"] {
        for kind in [Kind::Changes, Kind::Bookmarks, Kind::All] {
            // Derive all six direction/anchor windows from one complete ordinary
            // result. Rehydrating the identical oracle for each window obscures
            // the work bound this regression measures in the prepared reader.
            let oracle = pin()
                .await
                .references(
                    &ordinary,
                    Request {
                        query: query.into(),
                        kind,
                        limit: 500,
                        ..request.clone()
                    },
                )
                .await
                .unwrap();
            let key = |row: &Reference| match row {
                Reference::Bookmark { name, .. } => format!("bookmark:{name}"),
                Reference::Change { change, .. } => format!("change:{change}"),
                Reference::Workspace { .. } => unreachable!(),
            };
            for direction in [c::Direction::Forward, c::Direction::Backward] {
                for anchor in [
                    None,
                    Some("bookmark:b/0126".into()),
                    Some("change:2".into()),
                ] {
                    let req = Request {
                        query: query.into(),
                        kind,
                        direction,
                        anchor,
                        ..request.clone()
                    };
                    let rows = &oracle.references;
                    let (start, end) = match direction {
                        c::Direction::Forward => {
                            let start = req
                                .anchor
                                .as_ref()
                                .map_or(0, |a| rows.partition_point(|r| key(r) <= *a));
                            (start, (start + req.limit).min(rows.len()))
                        }
                        c::Direction::Backward => {
                            let end = req
                                .anchor
                                .as_ref()
                                .map_or(rows.len(), |a| rows.partition_point(|r| key(r) < *a));
                            (end.saturating_sub(req.limit), end)
                        }
                    };
                    let previous = (start > 0 && start < end).then(|| key(&rows[start]));
                    let next = (end < rows.len() && start < end).then(|| key(&rows[end - 1]));
                    let actual = pin()
                        .await
                        .references(executors.content(), req)
                        .await
                        .unwrap();
                    assert_eq!(
                        actual.references,
                        rows[start..end],
                        "{query}/{kind:?}/{direction:?}"
                    );
                    assert_eq!(actual.previous, previous);
                    assert_eq!(actual.next, next);
                    assert!(actual.stats.object_reads <= 5, "{:?}", actual.stats);
                    windows += 1;
                }
            }
        }
    }
    assert_eq!(windows, 108);
    eprintln!(
        "native filtered reference windows={windows}; selected stats={:?}",
        ready.stats
    );
}
