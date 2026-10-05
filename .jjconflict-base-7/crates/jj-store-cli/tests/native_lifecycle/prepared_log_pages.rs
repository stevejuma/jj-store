use super::*;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn prepared_all_log_pages_seek_deep_anchors_under_fixed_work() {
    let f = Fixture::new().await;
    let mut parent = roots::ROOT_COMMIT;
    for n in 0..224u32 {
        let mut commit = roots::commit();
        commit.parents = vec![b::CommitId::from_bytes(parent.as_bytes())];
        commit.change_id = b::ChangeId::from_bytes(&(n + 1).to_be_bytes().repeat(4));
        commit.description = format!("history {n}");
        parent =
            m::CommitId::from_slice(f.objects.put(&commit).await.unwrap().0.as_bytes()).unwrap();
    }
    let mut view = o::View::make_root(b::CommitId::from_bytes(parent.as_bytes()));
    view.local_bookmarks.insert(
        "main".into(),
        o::RefTarget::normal(b::CommitId::from_bytes(parent.as_bytes())),
    );
    let view = f.objects.put(&view).await.unwrap().0;
    let mut operation = o::Operation::make_root(o::ViewId::from_bytes(view.as_bytes()));
    operation.parents = vec![o::OperationId::from_bytes(roots::ROOT_OPERATION.as_bytes())];
    let operation =
        m::OperationId::from_slice(f.objects.put(&operation).await.unwrap().0.as_bytes()).unwrap();
    f.session
        .publish(BTreeSet::from([roots::ROOT_OPERATION]), operation)
        .await
        .unwrap();
    let read = pin(&f, operation).await;
    let ordinary_cache = tempfile::tempdir().unwrap();
    let ordinary = executor(ordinary_cache.path(), q::Limits::default());
    // Equivalent syntax keeps the independent upstream filtered graph path.
    let expected = ordinary
        .execute(
            f.store.clone(),
            read.clone(),
            log("all() & all()"),
            page(256),
        )
        .await
        .unwrap();
    let nodes = commits(&expected);
    assert!(nodes.len() > 210);
    let cache = tempfile::tempdir().unwrap();
    let small = executor(
        cache.path(),
        q::Limits {
            // Covers fresh request admission; independent of anchor depth.
            max_work: 300,
            ..Default::default()
        },
    );
    small
        .prepare_index(f.store.clone(), read.clone(), q::Cancellation::default())
        .await
        .unwrap();
    let first = small
        .execute(f.store.clone(), read.clone(), log("all()"), page(1))
        .await
        .unwrap();
    assert_eq!(first.data, q::Nodes::Log(nodes[..1].to_vec()));
    for index in [32, 128, 210, nodes.len() - 2, nodes.len() - 1] {
        for _ in 0..2 {
            let actual = small
                .execute(
                    f.store.clone(),
                    read.clone(),
                    log("all()"),
                    q::Page {
                        limit: 1,
                        after: Some(m::ObjectKey::Commit(nodes[index].commit.id)),
                    },
                )
                .await
                .unwrap();
            assert_eq!(
                actual.data,
                q::Nodes::Log(nodes[index + 1..(index + 2).min(nodes.len())].to_vec())
            );
            assert_eq!(
                actual.next,
                (index + 2 < nodes.len()).then(|| m::ObjectKey::Commit(nodes[index + 1].commit.id))
            );
            assert!(actual.stats.work <= 300, "{:?}", actual.stats);
            eprintln!("native log anchor{index}: {:?}", actual.stats);
            assert!(actual.stats.object_reads <= 8, "{:?}", actual.stats);
        }
    }
    let mut hidden = roots::commit();
    hidden.description = "staged excluded".into();
    let hidden =
        m::CommitId::from_slice(f.objects.put(&hidden).await.unwrap().0.as_bytes()).unwrap();
    assert_eq!(
        small
            .execute(
                f.store.clone(),
                read.clone(),
                log("all()"),
                q::Page {
                    limit: 1,
                    after: Some(m::ObjectKey::Commit(hidden))
                }
            )
            .await
            .unwrap_err(),
        q::Error::Invalid
    );
}
