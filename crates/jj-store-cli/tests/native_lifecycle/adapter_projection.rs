use super::*;
use jj_store_adapters as a;
use jj_store_service::adapter::ProjectionRead;
struct ReadNative {
    seen: Arc<std::sync::Mutex<Vec<m::ObjectKey>>>,
    checkpoint: Option<a::Checkpoint>,
}
#[async_trait::async_trait]
impl a::Projection for ReadNative {
    async fn recover(&mut self, _: a::Context) -> a::Result<Option<a::Checkpoint>> {
        Ok(self.checkpoint.clone())
    }
    async fn bootstrap(
        &mut self,
        snapshot: a::Snapshot,
        identity: &a::Identity,
        _: a::Context,
    ) -> a::Result<a::Checkpoint> {
        let reader = ProjectionRead::new(
            &snapshot,
            *snapshot.heads.first().unwrap(),
            ContentReader::new(Default::default(), 1).unwrap(),
        )
        .await?;
        for key in snapshot.objects.entries().keys().copied() {
            let object = reader.object(key).await.unwrap();
            if let ObjectData::Commit(commit) = &object.data {
                assert!(!commit.root_tree.is_empty());
            }
            if let ObjectData::File { bytes } = object.data {
                let m::ObjectKey::File(id) = key else {
                    panic!("file")
                };
                let mut file = reader.file(id).await.unwrap();
                let mut count = 0;
                while let Some(chunk) = file.next_chunk().await.unwrap() {
                    count += chunk.len() as u64;
                }
                assert_eq!(count, bytes);
            }
            self.seen.lock().unwrap().push(key);
        }
        let checkpoint = a::Checkpoint {
            version: 1,
            identity: identity.clone(),
            source: snapshot.position,
            artifact: m::RecordDigest::new([7; 32]),
        };
        self.checkpoint = Some(checkpoint.clone());
        Ok(checkpoint)
    }
    async fn apply(
        &mut self,
        batch: a::Batch,
        identity: &a::Identity,
        context: a::Context,
    ) -> a::Result<a::Checkpoint> {
        self.bootstrap(batch.snapshot, identity, context).await
    }
}
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn native_adapter_projection_reader_decodes_committed_trees_and_files() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = ContentFixture::new().await;
            let root = tempfile::tempdir().unwrap();
            let seen = Arc::new(std::sync::Mutex::new(vec![]));
            let mut runner = a::Runner::new(
                f.f.session.adapter_source(),
                a::Identity {
                    name: "content".into(),
                    implementation: "test".into(),
                    schema: 1,
                    config: m::RecordDigest::new([1; 32]),
                },
                ReadNative {
                    seen: seen.clone(),
                    checkpoint: None,
                },
                root.path(),
                Default::default(),
            )
            .unwrap();
            runner.step(a::CancellationToken::new()).await.unwrap();
            let seen = seen.lock().unwrap();
            assert!(seen.iter().any(|key| matches!(key, m::ObjectKey::File(_))));
            assert!(seen.iter().any(|key| matches!(key, m::ObjectKey::Copy(_))));
            assert!(
                seen.iter()
                    .any(|key| matches!(key, m::ObjectKey::Symlink(_)))
            );
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn native_tantivy_extracts_committed_metadata_and_explicit_source_scopes() {
    use jj_store_tantivy::{
        native::{Config, NativeExtractor, Scope},
        *,
    };
    tokio::task::LocalSet::new().run_until(async {
        let f = ContentFixture::new().await;
        valid_scope_copy_paths(&f).await;
        let config = Config { scopes: [("workspace:laptop".into(), Scope::Workspace { name: "laptop".into() }), ("bookmark:conflicted".into(), Scope::Bookmark { name: "conflicted".into() })].into(), ..Default::default() };
        let identity = config.identity().unwrap(); let root = tempfile::tempdir().unwrap();
        let (projection, search) = TantivyProjection::open(root.path().join("index"), NativeExtractor::new(config).unwrap(), Limits::default()).unwrap();
        let source = f.f.session.adapter_source();
        let mut runner = a::Runner::new(source.clone(), identity, projection, root.path(), Default::default()).unwrap();
        let before = f.f.session.snapshot().await.unwrap().manifest().clone();
        runner.step(a::CancellationToken::new()).await.unwrap();
        let pin = search.pin(&source).await.unwrap();
        let page = pin.query(&source, "content fixture", 0, 50, SearchLimits::default()).await.unwrap();
        let operation = page.hits.iter().find(|d| matches!(d.key, DocumentKey::Operation { .. })).unwrap();
        assert_eq!(operation.actor.as_deref(), Some(ACTOR.to_string().as_str()));
        assert_eq!(page.scopes["bookmark:conflicted"].state, ScopeState::Conflicted);
        assert_eq!(page.scopes["workspace:laptop"].state, ScopeState::Ready);
        let page = pin.query(&source, "invisible predecessor", 0, 50, SearchLimits::default()).await.unwrap();
        assert!(page.hits.iter().any(|d| matches!(d.key, DocumentKey::Commit { .. })));
        let page = pin.query(&source, "empty", 0, 50, SearchLimits::default()).await.unwrap();
        assert_eq!(page.hits.len(), 1);
        assert!(matches!(&page.hits[0].key, DocumentKey::File { scope, path } if scope=="workspace:laptop" && path=="empty"));
        assert_eq!(f.f.session.snapshot().await.unwrap().manifest(), &before);
    }).await;
}

// The codec fixture deliberately reuses bin's copy ID at a nested path. Native
// diffs reject that inconsistent association. This scope fixture publishes a
// distinct correct lineage for the nested file, retaining bin's real copy ID.
async fn valid_scope_copy_paths(f: &ContentFixture) {
    let copy = b::CopyHistory {
        current_path: RepoPathBuf::from_internal_string("dir/nested.bin").unwrap(),
        parents: vec![b::CopyId::placeholder()],
        salt: vec![2],
    };
    let copy = f.f.objects.put(&copy).await.unwrap().0;
    let subtree = tree(vec![(
        "nested.bin",
        b::TreeValue::File {
            id: b::FileId::from_bytes(f.file.as_bytes()),
            executable: false,
            copy_id: b::CopyId::from_bytes(copy.as_bytes()),
        },
    )]);
    let subtree = f.f.objects.put(&subtree).await.unwrap().0;
    let root = b::Tree::from_sorted_entries(
        f.tree
            .entries()
            .map(|e| {
                (
                    e.name().to_owned(),
                    if e.name().as_internal_str() == "dir" {
                        b::TreeValue::Tree(b::TreeId::from_bytes(subtree.as_bytes()))
                    } else {
                        e.value().clone()
                    },
                )
            })
            .collect(),
    );
    let root = f.f.objects.put(&root).await.unwrap().0;
    let old_commit = f.f.objects.put(&f.commit).await.unwrap().0;
    let mut commit = f.commit.clone();
    commit.parents = vec![b::CommitId::from_bytes(old_commit.as_bytes())];
    commit.root_tree = Merge::resolved(b::TreeId::from_bytes(root.as_bytes()));
    let commit = f.f.objects.put(&commit).await.unwrap().0;
    let mut view = f.view.clone();
    let id = b::CommitId::from_bytes(commit.as_bytes());
    view.head_ids = [id.clone()].into();
    view.wc_commit_ids.insert("laptop".into(), id);
    let view = f.f.objects.put(&view).await.unwrap().0;
    let parent =
        *f.f.session
            .snapshot()
            .await
            .unwrap()
            .manifest()
            .heads
            .first()
            .unwrap();
    let mut op = f.operation.clone();
    op.parents = vec![o::OperationId::from_bytes(parent.as_bytes())];
    op.view_id = o::ViewId::from_bytes(view.as_bytes());
    op.metadata.description = "valid content fixture".into();
    let operation = f.f.objects.put(&op).await.unwrap().0;
    f.f.session
        .publish(
            [parent].into(),
            m::OperationId::from_slice(operation.as_bytes()).unwrap(),
        )
        .await
        .unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn native_tantivy_diffs_replay_equivalently_and_keep_exact_scope_context() {
    use jj_store_tantivy::{
        native::{Config, NativeExtractor, Scope},
        *,
    };
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let (first, _) = publish_text_scope(
                &f,
                roots::ROOT_COMMIT,
                1,
                &[
                    ("keep.rs", b"unchanged signal"),
                    ("remove.rs", b"removed signal"),
                    ("private.txt", b"secret signal"),
                    ("binary.bin", b"binary\0signal"),
                    ("large.txt", &[b'x'; 128]),
                ],
            )
            .await;
            let config = Config {
                scopes: [(
                    "main".into(),
                    Scope::Bookmark {
                        name: "main".into(),
                    },
                )]
                .into(),
                exclude_prefixes: ["private.txt".into()].into(),
                max_text_bytes: 64,
                ..Default::default()
            };
            let root = tempfile::tempdir().unwrap();
            let source = f.session.adapter_source();
            let (projection, search) = TantivyProjection::open(
                root.path().join("index"),
                NativeExtractor::new(config.clone()).unwrap(),
                Limits::default(),
            )
            .unwrap();
            let mut runner = a::Runner::new(
                source.clone(),
                config.identity().unwrap(),
                projection,
                root.path(),
                Default::default(),
            )
            .unwrap();
            runner.step(a::CancellationToken::new()).await.unwrap();
            let old = search.pin(&source).await.unwrap();
            let page = old
                .query(&source, "signal", 0, 100, SearchLimits::default())
                .await
                .unwrap();
            assert_eq!(page.hits.len(), 2);
            let (second, operation) = publish_text_scope(
                &f,
                first,
                2,
                &[
                    ("keep.rs", b"unchanged signal"),
                    ("new.rs", b"fresh signal"),
                ],
            )
            .await;
            runner.step(a::CancellationToken::new()).await.unwrap();
            let pin = search.pin(&source).await.unwrap();
            let page = pin
                .query(&source, "signal", 0, 100, SearchLimits::default())
                .await
                .unwrap();
            assert_eq!(page.hits.len(), 2);
            assert!(
                page.hits
                    .iter()
                    .all(|d| d.commit == Some(second) && d.operation == Some(operation))
            );
            assert!(
                page.hits
                    .iter()
                    .any(|d| matches!(&d.key,DocumentKey::File{path,..} if path=="new.rs"))
            );
            assert!(
                !page
                    .hits
                    .iter()
                    .any(|d| matches!(&d.key,DocumentKey::File{path,..} if path=="remove.rs"))
            );
            let by_operation = pin
                .query(
                    &source,
                    &operation.to_string(),
                    0,
                    100,
                    SearchLimits::default(),
                )
                .await
                .unwrap();
            assert_eq!(
                by_operation
                    .hits
                    .iter()
                    .filter(|d| matches!(d.key, DocumentKey::File { .. }))
                    .count(),
                2
            );
            let old_page = old
                .query(&source, "signal", 0, 100, SearchLimits::default())
                .await
                .unwrap();
            assert!(old_page.hits.iter().all(|d| d.commit == Some(first)));
            let rebuilt = tempfile::tempdir().unwrap();
            let (projection, fresh) = TantivyProjection::open(
                rebuilt.path().join("index"),
                NativeExtractor::new(config.clone()).unwrap(),
                Limits::default(),
            )
            .unwrap();
            let mut rebuild = a::Runner::new(
                source.clone(),
                config.identity().unwrap(),
                projection,
                rebuilt.path(),
                Default::default(),
            )
            .unwrap();
            rebuild.step(a::CancellationToken::new()).await.unwrap();
            for term in ["signal", "metafixture"] {
                let mut left = pin
                    .query(&source, term, 0, 100, SearchLimits::default())
                    .await
                    .unwrap()
                    .hits;
                let mut right = fresh
                    .pin(&source)
                    .await
                    .unwrap()
                    .query(&source, term, 0, 100, SearchLimits::default())
                    .await
                    .unwrap()
                    .hits;
                left.sort_by_key(|d| d.key.clone());
                right.sort_by_key(|d| d.key.clone());
                assert_eq!(left, right);
            }
            f.session
                .change_token(&jj_store_service::administration::TokenRequest {
                    request_id: m::RequestId::new([222; 16]),
                    expected: f.session.snapshot().await.unwrap().manifest().snapshot(),
                    change: jj_store_service::administration::TokenChange::Revoke {
                        token: f.token.id(),
                    },
                })
                .await
                .unwrap();
            assert!(matches!(
                pin.query(&source, "signal", 0, 100, SearchLimits::default())
                    .await,
                Err(a::Error::Denied)
            ));
        })
        .await;
}
async fn publish_text_scope(
    f: &Fixture,
    parent: m::CommitId,
    version: u8,
    files: &[(&str, &[u8])],
) -> (m::CommitId, m::OperationId) {
    let mut entries = std::collections::BTreeMap::new();
    for (name, bytes) in files {
        let file = f.objects.put_file(*bytes).await.unwrap().0;
        entries.insert(
            RepoPathComponentBuf::new(*name).unwrap(),
            b::TreeValue::File {
                id: b::FileId::from_bytes(file.as_bytes()),
                executable: false,
                copy_id: b::CopyId::placeholder(),
            },
        );
    }
    let tree = f
        .objects
        .put(&b::Tree::from_sorted_entries(entries.into_iter().collect()))
        .await
        .unwrap()
        .0;
    let mut commit = b::make_root_commit(
        b::ChangeId::from_bytes(&[version; 16]),
        b::TreeId::from_bytes(tree.as_bytes()),
    );
    commit.parents = vec![b::CommitId::from_bytes(parent.as_bytes())];
    commit.description = format!("metafixture commit {version}");
    let key = f.objects.put(&commit).await.unwrap().0;
    let commit = m::CommitId::from_slice(key.as_bytes()).unwrap();
    let mut view = o::View::make_root(b::CommitId::from_bytes(commit.as_bytes()));
    view.local_bookmarks.insert(
        "main".into(),
        o::RefTarget::normal(b::CommitId::from_bytes(commit.as_bytes())),
    );
    let view = f.objects.put(&view).await.unwrap().0;
    let heads = f.session.snapshot().await.unwrap().manifest().heads.clone();
    let mut op = o::Operation::make_root(o::ViewId::from_bytes(view.as_bytes()));
    op.parents = heads
        .iter()
        .map(|id| o::OperationId::from_bytes(id.as_bytes()))
        .collect();
    op.metadata.description = format!("metafixture operation {version}");
    op.metadata.username = "untrusted native actor".into();
    let key = f.objects.put(&op).await.unwrap().0;
    let op = m::OperationId::from_slice(key.as_bytes()).unwrap();
    f.session.publish(heads, op).await.unwrap();
    (commit, op)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn native_tantivy_clears_ambiguous_scopes_and_respects_source_disable() {
    use jj_store_tantivy::{
        native::{Config, NativeExtractor, Scope},
        *,
    };
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let (first, old_op) = publish_text_scope(
                &f,
                roots::ROOT_COMMIT,
                1,
                &[("code.rs", b"searchable signal")],
            )
            .await;
            publish_text_scope(&f, first, 2, &[("code.rs", b"searchable signal")]).await;
            let config = Config {
                scopes: [(
                    "main".into(),
                    Scope::Bookmark {
                        name: "main".into(),
                    },
                )]
                .into(),
                ..Default::default()
            };
            let source = f.session.adapter_source();
            let root = tempfile::tempdir().unwrap();
            let (projection, search) = TantivyProjection::open(
                root.path().join("index"),
                NativeExtractor::new(config.clone()).unwrap(),
                Limits::default(),
            )
            .unwrap();
            let mut runner = a::Runner::new(
                source.clone(),
                config.identity().unwrap(),
                projection,
                root.path(),
                Default::default(),
            )
            .unwrap();
            runner.step(a::CancellationToken::new()).await.unwrap();
            let mut view = o::View::make_root(b::CommitId::from_bytes(first.as_bytes()));
            view.local_bookmarks.insert(
                "main".into(),
                o::RefTarget::normal(b::CommitId::from_bytes(first.as_bytes())),
            );
            let view = f.objects.put(&view).await.unwrap().0;
            let mut operation = o::Operation::make_root(o::ViewId::from_bytes(view.as_bytes()));
            operation.parents = vec![o::OperationId::from_bytes(old_op.as_bytes())];
            operation.metadata.description = "sibling publication".into();
            let operation = f.objects.put(&operation).await.unwrap().0;
            f.session
                .publish(
                    BTreeSet::from([old_op]),
                    m::OperationId::from_slice(operation.as_bytes()).unwrap(),
                )
                .await
                .unwrap();
            let heads = f.session.snapshot().await.unwrap().manifest().heads.clone();
            assert_eq!(heads.len(), 2);
            runner.step(a::CancellationToken::new()).await.unwrap();
            let page = search
                .pin(&source)
                .await
                .unwrap()
                .query(&source, "signal", 0, 20, SearchLimits::default())
                .await
                .unwrap();
            assert!(page.hits.is_empty());
            assert_eq!(page.scopes["main"].state, ScopeState::AmbiguousHeads);
            assert_eq!(f.session.snapshot().await.unwrap().manifest().heads, heads);
            let disabled = Config {
                index_source: false,
                ..config
            };
            let root2 = tempfile::tempdir().unwrap();
            let (projection, search) = TantivyProjection::open(
                root2.path().join("index"),
                NativeExtractor::new(disabled.clone()).unwrap(),
                Limits::default(),
            )
            .unwrap();
            let mut disabled_runner = a::Runner::new(
                source.clone(),
                disabled.identity().unwrap(),
                projection,
                root2.path(),
                Default::default(),
            )
            .unwrap();
            disabled_runner
                .step(a::CancellationToken::new())
                .await
                .unwrap();
            let pin = search.pin(&source).await.unwrap();
            let page = pin
                .query(&source, "signal", 0, 20, SearchLimits::default())
                .await
                .unwrap();
            assert!(page.hits.is_empty());
            assert_eq!(page.scopes["main"].state, ScopeState::Disabled);
            assert!(
                !pin.query(&source, "sibling", 0, 20, SearchLimits::default())
                    .await
                    .unwrap()
                    .hits
                    .is_empty()
            );
        })
        .await;
}
