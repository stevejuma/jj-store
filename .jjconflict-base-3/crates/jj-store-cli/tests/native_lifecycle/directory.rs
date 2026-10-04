use super::*;
use jj_store_service::{
    administration::{TokenChange, TokenRequest},
    pagination::{self as p, directory as d},
};
use std::sync::atomic::{AtomicU64, Ordering};

struct PageClock(AtomicU64);
impl Clock for PageClock {
    fn unix_seconds(&self) -> jj_store_service::auth::Result<u64> {
        Ok(self.0.load(Ordering::SeqCst))
    }
}
fn directory(f: &ContentFixture) -> d::Request {
    d::Request {
        commit: m::CommitId::from_slice(f.commit.key().as_bytes()).unwrap(),
        path: String::new(),
        limit: 2,
    }
}
fn page(start: d::Start) -> d::Page {
    let d::Start::Page(page) = start else {
        panic!("expected directory page")
    };
    *page
}
async fn later(f: &ContentFixture, description: &str) -> m::OperationId {
    let mut op = f.operation.clone();
    op.parents = vec![o::OperationId::from_bytes(f.operation.key().as_bytes())];
    op.metadata.description = description.into();
    let key = f.f.objects.put(&op).await.unwrap().0;
    let id = m::OperationId::from_slice(key.as_bytes()).unwrap();
    f.f.session
        .publish(
            BTreeSet::from([m::OperationId::from_slice(f.operation.key().as_bytes()).unwrap()]),
            id,
        )
        .await
        .unwrap();
    id
}
struct NoPublication;
#[async_trait::async_trait]
impl PublicationGuard for NoPublication {
    async fn authorize(&self, _: &ManifestSnapshot, _: &Request) -> wal::Result<()> {
        Err(wal::Error::Denied)
    }
    async fn validate_transition(
        &self,
        _: &ManifestSnapshot,
        _: &Request,
        _: &Snapshot,
    ) -> wal::Result<()> {
        Err(wal::Error::Denied)
    }
}
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn directory_cursor_retains_snapshot_through_writes_checkpoint_and_retries() {
    let f = ContentFixture::new().await;
    let pager =
        d::DirectoryPager::new(reader(), p::Limits::default(), Arc::new(SystemClock)).unwrap();
    let request = directory(&f);
    let first = page(
        pager
            .begin(f.f.session.clone(), None, request.clone())
            .await
            .unwrap(),
    );
    let token = first.next_cursor.clone().unwrap();
    assert!(first.tree.next.is_none());
    let expected = f
        .read
        .tree(
            &reader(),
            c::TreeRequest {
                commit: request.commit,
                path: String::new(),
                limit: 500,
                after: None,
            },
        )
        .await
        .unwrap()
        .entries;
    later(&f, "after first directory page").await;
    let checkpoint = Coordinator::new(
        f.f.store.clone(),
        REPO,
        f.f.objects.clone(),
        Arc::new(NoPublication),
        Limits::default(),
    )
    .unwrap()
    .checkpoint()
    .await
    .unwrap();
    let before = checkpoint.manifest().clone();
    let mut entries = first.tree.entries;
    let mut next = Some(token.clone());
    while let Some(cursor) = next {
        let result = pager
            .resume(
                f.f.session(f.f.reader.clone(), Protection::default()),
                &cursor,
                Some(first.snapshot.operation_id),
                request.clone(),
            )
            .await
            .unwrap();
        assert_eq!(result.snapshot, first.snapshot);
        assert!(result.tree.next.is_none());
        let again = pager
            .resume(f.f.session.clone(), &cursor, None, request.clone())
            .await
            .unwrap();
        assert_eq!(again.tree.entries, result.tree.entries);
        assert_eq!(again.next_cursor, result.next_cursor);
        entries.extend(result.tree.entries);
        next = result.next_cursor;
    }
    assert_eq!(entries, expected);
    assert_eq!(*f.f.session.snapshot().await.unwrap().manifest(), before);
    assert!(format!("{pager:?}").contains("[REDACTED]"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn directory_cursor_binds_all_inputs_and_rechecks_revocation_before_lookup() {
    let f = ContentFixture::new().await;
    let pager =
        d::DirectoryPager::new(reader(), p::Limits::default(), Arc::new(SystemClock)).unwrap();
    let request = directory(&f);
    let session = f.f.session(f.f.reader.clone(), Protection::default());
    let first = page(
        pager
            .begin(session.clone(), None, request.clone())
            .await
            .unwrap(),
    );
    let token = first.next_cursor.unwrap();
    for field in 0..3 {
        let mut changed = request.clone();
        match field {
            0 => changed.commit = roots::ROOT_COMMIT,
            1 => changed.path = "dir".into(),
            _ => changed.limit += 1,
        }
        assert_eq!(
            pager
                .resume(session.clone(), &token, None, changed)
                .await
                .unwrap_err(),
            p::Error::InvalidCursor
        );
    }
    assert_eq!(
        pager
            .resume(
                session.clone(),
                &token,
                Some(roots::ROOT_OPERATION),
                request.clone()
            )
            .await
            .unwrap_err(),
        p::Error::InvalidCursor
    );
    let mut altered = token.clone().into_bytes();
    let i = token.rfind('.').unwrap() + 1;
    altered[i] = if altered[i] == b'A' { b'B' } else { b'A' };
    for token in [
        String::from_utf8(altered).unwrap(),
        token[..token.len() - 1].into(),
        "x".repeat(4097),
    ] {
        assert_eq!(
            pager
                .resume(session.clone(), &token, None, request.clone())
                .await
                .unwrap_err(),
            p::Error::InvalidCursor
        );
    }
    f.f.session
        .change_token(&TokenRequest {
            request_id: m::RequestId::new([231; 16]),
            expected: f.f.session.snapshot().await.unwrap().manifest().snapshot(),
            change: TokenChange::Revoke {
                token: f.f.reader.id(),
            },
        })
        .await
        .unwrap();
    for cursor in [token.as_str(), "invalid"] {
        assert_eq!(
            pager
                .resume(session.clone(), cursor, None, request.clone())
                .await
                .unwrap_err(),
            p::Error::Denied
        );
    }
    assert_eq!(
        pager.begin(session, None, request).await.unwrap_err(),
        p::Error::Denied
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn directory_cursor_shares_graph_capacity_and_rejects_cross_query_restart_and_expiry() {
    let f = ContentFixture::new().await;
    let clock = Arc::new(PageClock(AtomicU64::new(1000)));
    let limits = p::Limits {
        snapshots: 1,
        ttl: Duration::from_secs(2),
        ..p::Limits::default()
    };
    let cursors = Arc::new(p::SnapshotCursors::new(limits, clock.clone()).unwrap());
    let pager = d::DirectoryPager::with_cursors(reader(), cursors.clone());
    let cache = tempfile::tempdir().unwrap();
    let graphs = p::GraphPager::with_cursors(
        q::GraphExecutor::new(cache.path().to_owned(), q::Limits::default(), 2).unwrap(),
        cursors,
    );
    let request = directory(&f);
    let first = page(
        pager
            .begin(f.f.session.clone(), None, request.clone())
            .await
            .unwrap(),
    );
    let token = first.next_cursor.unwrap();
    assert_eq!(
        graphs
            .resume(f.f.session.clone(), &token, None, q::Request::Operations, 2)
            .await
            .unwrap_err(),
        p::Error::InvalidCursor
    );
    assert_eq!(
        graphs
            .begin(f.f.session.clone(), None, q::Request::Operations, 1)
            .await
            .unwrap_err(),
        p::Error::Busy
    );
    // A complete listing needs no cursor capacity.
    let mut nested = request.clone();
    nested.path = "dir".into();
    assert!(
        page(
            pager
                .begin(f.f.session.clone(), None, nested)
                .await
                .unwrap()
        )
        .next_cursor
        .is_none()
    );
    let restarted = d::DirectoryPager::new(reader(), limits, clock.clone()).unwrap();
    assert_eq!(
        restarted
            .resume(f.f.session.clone(), &token, None, request.clone())
            .await
            .unwrap_err(),
        p::Error::CursorReset
    );
    clock.0.store(1002, Ordering::SeqCst);
    assert_eq!(
        pager
            .resume(f.f.session.clone(), &token, None, request.clone())
            .await
            .unwrap_err(),
        p::Error::CursorReset
    );
    let p::Start::Page(graph) = graphs
        .begin(f.f.session.clone(), None, q::Request::Operations, 1)
        .await
        .unwrap()
    else {
        panic!("graph page")
    };
    let graph_token = graph.next_cursor.unwrap();
    assert_eq!(
        pager
            .resume(f.f.session.clone(), &graph_token, None, request.clone())
            .await
            .unwrap_err(),
        p::Error::InvalidCursor
    );
    assert_eq!(
        pager
            .begin(f.f.session.clone(), None, request.clone())
            .await
            .unwrap_err(),
        p::Error::Busy
    );
    clock.0.store(1004, Ordering::SeqCst);
    let fresh = page(
        pager
            .begin(f.f.session.clone(), None, request.clone())
            .await
            .unwrap(),
    );
    // Epoch changes invalidate an otherwise signed, unexpired directory cursor.
    let key = Key::new(format!("repos/{REPO}/manifest.json")).unwrap();
    let download = f.f.store.get(&key, None).await.unwrap();
    let version = download.metadata.version.clone();
    let mut manifest: Manifest =
        serde_json::from_slice(&download.read_bounded(128 * 1024).await.unwrap()).unwrap();
    manifest.epoch = m::EpochId::new([234; 16]);
    let upload = StagedUpload::from_bytes(&serde_json::to_vec(&manifest).unwrap(), 128 * 1024)
        .await
        .unwrap();
    assert!(matches!(
        f.f.store
            .put(&key, Condition::Matches(version), &upload)
            .await
            .unwrap(),
        WriteOutcome::Applied(_)
    ));
    assert_eq!(
        pager
            .resume(
                f.f.session.clone(),
                fresh.next_cursor.as_ref().unwrap(),
                None,
                request
            )
            .await
            .unwrap_err(),
        p::Error::Read(jj_store_service::reads::Error::EpochChanged)
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn directory_cursor_keeps_head_selection_explicit_and_rejects_invalid_paths() {
    let f = ContentFixture::new().await;
    let pager =
        d::DirectoryPager::new(reader(), p::Limits::default(), Arc::new(SystemClock)).unwrap();
    let request = directory(&f);
    for path in ["../dir", "dir/..", "./dir", "dir\0", "/dir", "dir/"] {
        let mut invalid = request.clone();
        invalid.path = path.into();
        assert_eq!(
            pager
                .begin(f.f.session.clone(), None, invalid)
                .await
                .unwrap_err(),
            p::Error::Query(q::Error::Invalid)
        );
    }
    let mut huge = request.clone();
    huge.path = "x".repeat(4097);
    assert_eq!(
        pager
            .begin(f.f.session.clone(), None, huge)
            .await
            .unwrap_err(),
        p::Error::Limit
    );
    let left = later(&f, "left").await;
    let right = later(&f, "right").await;
    let before = f.f.session.snapshot().await.unwrap().manifest().clone();
    assert_eq!(before.heads, BTreeSet::from([left, right]));
    let d::Start::SelectHead { snapshot } = pager
        .begin(f.f.session.clone(), None, request.clone())
        .await
        .unwrap()
    else {
        panic!("must select head")
    };
    assert_eq!(snapshot.heads, before.heads);
    let selected = page(
        pager
            .begin(f.f.session.clone(), Some(left), request)
            .await
            .unwrap(),
    );
    assert_eq!(selected.snapshot.operation_id, left);
    assert_eq!(*f.f.session.snapshot().await.unwrap().manifest(), before);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn directory_cursor_oversized_name_fails_without_consuming_snapshot_capacity() {
    let f = ContentFixture::new().await;
    let name = "a".repeat(4096);
    let value = b::TreeValue::Symlink(b::SymlinkId::from_bytes(f.symlink.as_bytes()));
    let tree = tree(vec![(&name, value.clone()), ("z", value)]);
    let key = f.f.objects.put(&tree).await.unwrap().0;
    let mut commit = f.commit.clone();
    commit.parents = vec![b::CommitId::from_bytes(f.commit.key().as_bytes())];
    commit.predecessors.clear();
    commit.change_id = b::ChangeId::from_bytes(&[235; 16]);
    commit.root_tree = Merge::resolved(b::TreeId::from_bytes(key.as_bytes()));
    commit.secure_sig = None;
    let key = f.f.objects.put(&commit).await.unwrap().0;
    let view = o::View::make_root(b::CommitId::from_bytes(key.as_bytes()));
    let view_key = f.f.objects.put(&view).await.unwrap().0;
    let mut operation = f.operation.clone();
    operation.parents = vec![o::OperationId::from_bytes(f.operation.key().as_bytes())];
    operation.view_id = o::ViewId::from_bytes(view_key.as_bytes());
    let op_key = f.f.objects.put(&operation).await.unwrap().0;
    f.f.session
        .publish(
            BTreeSet::from([m::OperationId::from_slice(f.operation.key().as_bytes()).unwrap()]),
            m::OperationId::from_slice(op_key.as_bytes()).unwrap(),
        )
        .await
        .unwrap();
    let before = f.f.session.snapshot().await.unwrap().manifest().clone();
    let pager = d::DirectoryPager::new(
        reader(),
        p::Limits {
            snapshots: 1,
            ..p::Limits::default()
        },
        Arc::new(SystemClock),
    )
    .unwrap();
    let request = d::Request {
        commit: m::CommitId::from_slice(key.as_bytes()).unwrap(),
        path: String::new(),
        limit: 1,
    };
    assert_eq!(
        pager
            .begin(f.f.session.clone(), None, request.clone())
            .await
            .unwrap_err(),
        p::Error::Limit
    );
    let complete = page(
        pager
            .begin(
                f.f.session.clone(),
                None,
                d::Request {
                    limit: 2,
                    ..request
                },
            )
            .await
            .unwrap(),
    );
    assert_eq!(complete.tree.entries[0].name, name);
    assert!(complete.next_cursor.is_none());
    let original = page(
        pager
            .begin(f.f.session.clone(), None, directory(&f))
            .await
            .unwrap(),
    );
    assert!(original.next_cursor.is_some());
    assert_eq!(*f.f.session.snapshot().await.unwrap().manifest(), before);
}
