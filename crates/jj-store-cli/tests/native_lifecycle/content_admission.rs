use super::*;

async fn publish(f: &Fixture, commit: &b::Commit, large_ref: bool) -> Arc<CommittedRead> {
    let key = f.objects.put(commit).await.unwrap().0;
    assert_eq!(f.objects.get::<b::Commit>(key).await.unwrap(), *commit);
    let id = b::CommitId::from_bytes(key.as_bytes());
    let mut view = roots::view();
    view.head_ids = [id.clone()].into();
    if large_ref {
        view.local_bookmarks
            .insert("amplified".into(), Merge::from_vec(vec![Some(id); 1025]));
    }
    let view_key = f.objects.put(&view).await.unwrap().0;
    assert_eq!(f.objects.get::<o::View>(view_key).await.unwrap(), view);
    let mut op = o::Operation::make_root(o::ViewId::from_bytes(view_key.as_bytes()));
    op.parents = vec![o::OperationId::from_bytes(roots::ROOT_OPERATION.as_bytes())];
    let op_key = f.objects.put(&op).await.unwrap().0;
    f.session
        .publish(
            BTreeSet::from([roots::ROOT_OPERATION]),
            m::OperationId::from_slice(op_key.as_bytes()).unwrap(),
        )
        .await
        .unwrap();
    let OpenRead::Pinned(read) = f
        .session
        .open_read(None, ReadLimits::default())
        .await
        .unwrap()
    else {
        panic!("sole head")
    };
    Arc::new(read)
}
fn commit() -> b::Commit {
    let mut commit = roots::commit();
    commit.parents = vec![b::CommitId::from_bytes(roots::ROOT_COMMIT.as_bytes())];
    commit.change_id = b::ChangeId::from_bytes(&[71; 16]);
    commit
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn query_shape_rejects_commit_merge_amplification_without_narrowing_persistence() {
    let f = Fixture::new().await;
    let mut commit = commit();
    commit.root_tree = Merge::from_vec(vec![commit.root_tree.first().clone(); 1025]);
    commit.conflict_labels = Merge::from_vec(vec![String::new(); 1025]);
    let read = publish(&f, &commit, false).await;
    let before = f.session.snapshot().await.unwrap().manifest().clone();
    assert_eq!(
        read.object(&reader(), commit.key()).await.unwrap_err(),
        q::Error::Limit
    );
    assert_eq!(
        read.tree(
            &reader(),
            c::TreeRequest {
                commit: m::CommitId::from_slice(commit.key().as_bytes()).unwrap(),
                path: String::new(),
                limit: 1,
                after: None
            }
        )
        .await
        .unwrap_err(),
        q::Error::Limit
    );
    let cache = tempfile::tempdir().unwrap();
    let graph = q::GraphExecutor::new(cache.path().to_owned(), q::Limits::default(), 1).unwrap();
    assert_eq!(
        read.graph(
            &graph,
            q::Request::Log {
                revset: "all()".into(),
                workspace: None,
                user_email: String::new(),
                at_time_millis: 0,
            },
            q::Page {
                limit: 1,
                after: None
            }
        )
        .await
        .unwrap_err(),
        q::Error::Limit
    );
    assert_eq!(f.session.snapshot().await.unwrap().manifest(), &before);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn query_shape_rejects_reference_merge_amplification() {
    let f = Fixture::new().await;
    let read = publish(&f, &commit(), true).await;
    let cache = tempfile::tempdir().unwrap();
    let graph = q::GraphExecutor::new(cache.path().to_owned(), q::Limits::default(), 1).unwrap();
    assert_eq!(
        read.graph(
            &graph,
            q::Request::Log {
                revset: "all()".into(),
                workspace: None,
                user_email: String::new(),
                at_time_millis: 0,
            },
            q::Page {
                limit: 1,
                after: None
            }
        )
        .await
        .unwrap_err(),
        q::Error::Limit
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn query_shape_rejects_dense_trees_before_native_expansion() {
    let f = Fixture::new().await;
    let tree = b::Tree::from_sorted_entries(
        (0..50_001)
            .map(|i| {
                (
                    RepoPathComponentBuf::new(format!("path{i:06}")).unwrap(),
                    b::TreeValue::GitSubmodule(b::CommitId::from_bytes(&[7])),
                )
            })
            .collect(),
    );
    let key = f.objects.put(&tree).await.unwrap().0;
    assert_eq!(f.objects.get::<b::Tree>(key).await.unwrap(), tree);
    let mut commit = commit();
    commit.root_tree = Merge::resolved(b::TreeId::from_bytes(key.as_bytes()));
    let read = publish(&f, &commit, false).await;
    assert_eq!(
        read.tree(
            &reader(),
            c::TreeRequest {
                commit: m::CommitId::from_slice(commit.key().as_bytes()).unwrap(),
                path: String::new(),
                limit: 1,
                after: None
            }
        )
        .await
        .unwrap_err(),
        q::Error::Limit
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn query_shape_node_admission_is_request_local_and_preserves_file_streaming() {
    let f = ContentFixture::new().await;
    let ordinary = reader();
    let first = f.read.object(&ordinary, f.commit.key()).await.unwrap();
    assert!(first.stats.metadata_nodes > 1);
    let bounded = ContentReader::new(
        c::Limits {
            metadata: q::admission::Limits {
                max_nodes: 1,
                ..Default::default()
            },
            ..Default::default()
        },
        1,
    )
    .unwrap();
    assert_eq!(
        f.read.object(&bounded, f.commit.key()).await.unwrap_err(),
        q::Error::Limit
    );
    let mut stream = f.read.file(&bounded, f.file, None).await.unwrap();
    let mut bytes = Vec::new();
    while let Some(chunk) = stream.next_chunk().await.unwrap() {
        assert!(chunk.len() <= 64 * 1024);
        bytes.extend(chunk);
    }
    assert_eq!(bytes, f.binary);
    drop(stream);
    let second = f.read.object(&ordinary, f.commit.key()).await.unwrap();
    assert_eq!(second.stats.metadata_nodes, first.stats.metadata_nodes);
}
