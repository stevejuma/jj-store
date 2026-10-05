use super::*;
use c::references::{Kind, Request};
use jj_store_jj::query as q;
use std::sync::atomic::{AtomicBool, Ordering};

async fn pin(f: &Fixture, operation: m::OperationId) -> Arc<CommittedRead> {
    let OpenRead::Pinned(read) = f
        .session
        .open_read(Some(operation), ReadLimits::default())
        .await
        .unwrap()
    else {
        panic!("explicit selection must pin")
    };
    Arc::new(read)
}

struct PointAccess {
    inner: Arc<CommittedRead>,
    denied: AtomicBool,
}
#[async_trait::async_trait]
impl q::ReadAccess for PointAccess {
    fn repository(&self) -> m::RepositoryId {
        q::ReadAccess::repository(self.inner.as_ref())
    }
    fn epoch(&self) -> m::EpochId {
        q::ReadAccess::epoch(self.inner.as_ref())
    }
    fn operation(&self) -> m::OperationId {
        q::ReadAccess::operation(self.inner.as_ref())
    }
    async fn contains(&self, key: m::ObjectKey) -> q::Result<bool> {
        q::ReadAccess::contains(self.inner.as_ref(), key).await
    }
    async fn inventory(&self) -> q::Result<Arc<BTreeSet<m::ObjectKey>>> {
        panic!("embedded reference read enumerated native inventory")
    }
    async fn authorize(&self) -> q::Result<()> {
        if self.denied.load(Ordering::Acquire) {
            return Err(q::Error::Denied);
        }
        q::ReadAccess::authorize(self.inner.as_ref()).await
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn native_embedded_reference_operation_uses_its_own_prepared_window_and_outer_authority() {
    let f = BrowsingFixture::new().await;
    let mut head = b::CommitId::from_bytes(f.head.as_bytes());
    for n in 40..56u8 {
        let mut commit = roots::commit();
        commit.change_id = b::ChangeId::from_bytes(&[n; 16]);
        commit.parents = vec![head];
        commit.description = format!("historical needle {n}");
        head = b::CommitId::from_bytes(f.f.objects.put(&commit).await.unwrap().0.as_bytes());
    }
    let mut view = o::View::make_root(head.clone());
    for n in 0..16 {
        view.local_bookmarks.insert(
            format!("old/{n:02}").into(),
            o::RefTarget::normal(head.clone()),
        );
    }
    view.wc_commit_ids.insert("historical".into(), head.clone());
    let view_id = f.f.objects.put(&view).await.unwrap().0;
    let mut operation = o::Operation::make_root(o::ViewId::from_bytes(view_id.as_bytes()));
    operation.parents = vec![o::OperationId::from_bytes(f.metadata_operation.as_bytes())];
    let source =
        m::OperationId::from_slice(f.f.objects.put(&operation).await.unwrap().0.as_bytes())
            .unwrap();
    f.f.session
        .publish(BTreeSet::from([f.metadata_operation]), source)
        .await
        .unwrap();
    let old_outer = pin(&f.f, source).await;

    let mut future = roots::commit();
    future.change_id = b::ChangeId::from_bytes(&[99; 16]);
    future.parents = vec![head];
    future.description = "future-only".into();
    let future = b::CommitId::from_bytes(f.f.objects.put(&future).await.unwrap().0.as_bytes());
    view.head_ids = [future.clone()].into();
    view.local_bookmarks.clear();
    view.local_bookmarks
        .insert("new".into(), o::RefTarget::normal(future));
    view.wc_commit_ids.clear();
    let view_id = f.f.objects.put(&view).await.unwrap().0;
    let mut operation = o::Operation::make_root(o::ViewId::from_bytes(view_id.as_bytes()));
    operation.parents = vec![o::OperationId::from_bytes(source.as_bytes())];
    let outer = m::OperationId::from_slice(f.f.objects.put(&operation).await.unwrap().0.as_bytes())
        .unwrap();
    f.f.session
        .publish(BTreeSet::from([source]), outer)
        .await
        .unwrap();

    let cache = tempfile::tempdir().unwrap();
    let executors = q::workers::Executors::new(
        cache.path().into(),
        q::Limits {
            max_inline_rebuild_commits: 1,
            ..Default::default()
        },
        c::Limits {
            max_history_visits: 4,
            ..Default::default()
        },
        q::cache::Policy::default(),
        1,
    )
    .unwrap();
    let request = Request {
        operation: source,
        query: String::new(),
        exact: None,
        kind: Kind::Changes,
        limit: 1,
        anchor: None,
        direction: c::Direction::Backward,
    };
    assert_eq!(
        pin(&f.f, outer)
            .await
            .references(executors.content(), request.clone())
            .await
            .unwrap_err(),
        q::Error::IndexRebuildRequired,
        "cold recovery must select the embedded source operation"
    );
    let report = pin(&f.f, source)
        .await
        .prepare_index(executors.graph(), q::Cancellation::default())
        .await
        .unwrap();
    assert_eq!(report.operation, source);

    let before_outer_preparation = pin(&f.f, outer)
        .await
        .references(executors.content(), request.clone())
        .await
        .unwrap();
    assert_eq!(before_outer_preparation.operation, source);
    assert!(before_outer_preparation.stats.object_reads <= 3);

    // The outer generation can be independently prepared without changing the
    // selected historical result. A warm future cache grants no older read access.
    pin(&f.f, outer)
        .await
        .prepare_index(executors.graph(), q::Cancellation::default())
        .await
        .unwrap();
    assert_eq!(
        old_outer
            .references(
                executors.content(),
                Request {
                    operation: outer,
                    ..request.clone()
                }
            )
            .await
            .unwrap_err(),
        q::Error::NotCommitted,
    );
    let mut staged = operation;
    staged.metadata.description = "unpublished operation".into();
    let staged =
        m::OperationId::from_slice(f.f.objects.put(&staged).await.unwrap().0.as_bytes()).unwrap();
    assert_eq!(
        pin(&f.f, outer)
            .await
            .references(
                executors.content(),
                Request {
                    operation: staged,
                    ..request.clone()
                }
            )
            .await
            .unwrap_err(),
        q::Error::NotCommitted,
    );

    for query in ["", "needle", "future-only"] {
        for kind in [Kind::Changes, Kind::All, Kind::Bookmarks] {
            if query.is_empty() && kind == Kind::Bookmarks {
                continue;
            }
            for direction in [c::Direction::Forward, c::Direction::Backward] {
                for anchor in [
                    None,
                    Some("bookmark:old/07".into()),
                    Some("change:2".into()),
                ] {
                    let req = Request {
                        query: query.into(),
                        kind,
                        direction,
                        anchor,
                        ..request.clone()
                    };
                    let read = pin(&f.f, outer).await;
                    let snapshot = read.snapshot().await.unwrap().clone();
                    let expected = read.references(&reader(), req.clone()).await.unwrap();
                    let access = Arc::new(PointAccess {
                        inner: read.clone(),
                        denied: AtomicBool::new(false),
                    });
                    let actual = executors
                        .content()
                        .references(f.f.store.clone(), access, req)
                        .await
                        .unwrap();
                    assert_eq!(actual.operation, source);
                    assert_eq!(actual.references, expected.references);
                    assert_eq!(actual.previous, expected.previous);
                    assert_eq!(actual.next, expected.next);
                    assert!(actual.stats.object_reads <= 3, "{:?}", actual.stats);
                    assert_eq!(read.snapshot().await.unwrap(), &snapshot);
                }
            }
        }
    }
    let read = pin(&f.f, outer).await;
    let expected = read.references(&reader(), request.clone()).await.unwrap();
    let access = Arc::new(PointAccess {
        inner: read,
        denied: AtomicBool::new(false),
    });
    let actual = executors
        .content()
        .references(f.f.store.clone(), access.clone(), request.clone())
        .await
        .unwrap();
    assert_eq!(actual.references, expected.references);
    assert_eq!(actual.references, before_outer_preparation.references);
    access.denied.store(true, Ordering::Release);
    assert_eq!(
        executors
            .content()
            .references(f.f.store.clone(), access, request)
            .await
            .unwrap_err(),
        q::Error::Denied,
    );
}
