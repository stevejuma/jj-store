use super::*;
use jj_store_tantivy::{
    Limits, SearchLimits, SearchRequest, TantivyProjection,
    native::{Config, NativeExtractor},
};

async fn visibility(
    f: &Fixture,
    operation: Option<m::OperationId>,
) -> jj_store_service::reads::VisibleView {
    let OpenRead::Pinned(read) = f
        .session
        .open_read(operation, ReadLimits::default())
        .await
        .unwrap()
    else {
        panic!("explicit operation required")
    };
    let read = Arc::new(read);
    let visible = read
        .search_visibility(&ContentReader::new(Default::default(), 1).unwrap())
        .await
        .unwrap();
    let cache = tempfile::tempdir().unwrap();
    let executor = q::GraphExecutor::new(cache.path().to_owned(), Default::default(), 1).unwrap();
    let graph = read
        .graph(
            &executor,
            q::Request::Log {
                revset: "all()".into(),
                workspace: None,
                user_email: String::new(),
                at_time_millis: 0,
            },
            q::Page {
                limit: 100,
                after: None,
            },
        )
        .await
        .unwrap();
    let q::Nodes::Log(nodes) = graph.data else {
        panic!("native graph")
    };
    assert_eq!(
        visible.commits(),
        &nodes.into_iter().map(|n| n.commit.id).collect()
    );
    visible
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn native_tantivy_selected_view_filters_hidden_commits_before_pagination() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let (first, first_op) = publish_text_scope(&f, roots::ROOT_COMMIT, 1, &[]).await;
            let mut latest = first;
            for version in 2..=6 {
                latest = publish_text_scope(&f, roots::ROOT_COMMIT, version, &[])
                    .await
                    .0;
            }
            let (child, current_op) = publish_text_scope(&f, latest, 7, &[]).await;
            let before = f.session.snapshot().await.unwrap().manifest().clone();
            let old = visibility(&f, Some(first_op)).await;
            let current = visibility(&f, None).await;
            assert_eq!(
                current.commits(),
                &BTreeSet::from([roots::ROOT_COMMIT, latest, child])
            );
            let config = Config::default();
            let root = tempfile::tempdir().unwrap();
            let (projection, search) = TantivyProjection::open(
                root.path().join("index"),
                NativeExtractor::new(config.clone()).unwrap(),
                Limits::default(),
            )
            .unwrap();
            let source = f.session.adapter_source();
            let mut runner = a::Runner::new(
                source.clone(),
                config.identity().unwrap(),
                projection,
                root.path(),
                Default::default(),
            )
            .unwrap();
            runner.step(a::CancellationToken::new()).await.unwrap();
            let pin = search.pin(&source).await.unwrap();
            let history = pin
                .query(
                    &source,
                    SearchRequest::historical("metafixture commit", 0, 20),
                    SearchLimits::default(),
                )
                .await
                .unwrap();
            assert_eq!(history.hits.len(), 7);
            assert!(history.selected_view.is_none());
            let page = pin
                .query(
                    &source,
                    SearchRequest::visible("metafixture commit", 0, 1, current.clone()),
                    SearchLimits::default(),
                )
                .await
                .unwrap();
            assert_eq!(page.hits.len(), 1);
            assert_eq!(page.next_offset, Some(1));
            assert_eq!(
                page.selected_view.as_ref().unwrap().operation_id,
                current_op
            );
            let second = pin
                .query(
                    &source,
                    SearchRequest::visible("metafixture commit", 1, 1, current.clone()),
                    SearchLimits::default(),
                )
                .await
                .unwrap();
            assert_eq!(second.hits.len(), 1);
            assert_eq!(second.next_offset, None);
            assert_eq!(
                page.hits
                    .iter()
                    .chain(&second.hits)
                    .map(|d| d.commit.unwrap())
                    .collect::<BTreeSet<_>>(),
                BTreeSet::from([latest, child])
            );
            let old_page = pin
                .query(
                    &source,
                    SearchRequest::visible("metafixture commit", 0, 20, old),
                    SearchLimits::default(),
                )
                .await
                .unwrap();
            assert_eq!(
                old_page
                    .hits
                    .iter()
                    .map(|d| d.commit.unwrap())
                    .collect::<Vec<_>>(),
                vec![first]
            );
            assert!(
                pin.query(
                    &source,
                    SearchRequest::visible(&first.to_string(), 0, 20, current.clone()),
                    SearchLimits::default()
                )
                .await
                .unwrap()
                .hits
                .is_empty()
            );
            // Operation history remains retained and explicitly distinguished from commit visibility.
            assert!(
                !pin.query(
                    &source,
                    SearchRequest::visible(&first_op.to_string(), 0, 20, current.clone()),
                    SearchLimits::default()
                )
                .await
                .unwrap()
                .hits
                .is_empty()
            );
            assert_eq!(f.session.snapshot().await.unwrap().manifest(), &before);
            publish_text_scope(&f, roots::ROOT_COMMIT, 8, &[]).await;
            runner.step(a::CancellationToken::new()).await.unwrap();
            assert_eq!(
                pin.query(
                    &source,
                    SearchRequest::visible("metafixture commit", 0, 20, current),
                    SearchLimits::default()
                )
                .await
                .unwrap()
                .hits
                .len(),
                2
            );
            let new_view = visibility(&f, None).await;
            // A lagging generation does not fabricate a hit for the latest native view.
            assert!(
                pin.query(
                    &source,
                    SearchRequest::visible("metafixture commit", 0, 20, new_view.clone()),
                    SearchLimits::default()
                )
                .await
                .unwrap()
                .hits
                .is_empty()
            );
            f.session
                .change_token(&jj_store_service::administration::TokenRequest {
                    request_id: m::RequestId::new([223; 16]),
                    expected: f.session.snapshot().await.unwrap().manifest().snapshot(),
                    change: jj_store_service::administration::TokenChange::Revoke {
                        token: f.token.id(),
                    },
                })
                .await
                .unwrap();
            assert!(matches!(
                pin.query(
                    &source,
                    SearchRequest::visible("metafixture commit", 0, 20, new_view),
                    SearchLimits::default()
                )
                .await,
                Err(a::Error::Denied)
            ));
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn native_search_visibility_excludes_predecessors_and_enforces_limits() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = ContentFixture::new().await;
            let reader = ContentReader::new(Default::default(), 1).unwrap();
            let view = f.read.search_visibility(&reader).await.unwrap();
            assert_eq!(view.commits().len(), 2);
            for predecessor in &f.commit.predecessors {
                assert!(
                    !view
                        .commits()
                        .contains(&m::CommitId::from_slice(predecessor.as_bytes()).unwrap())
                );
            }
            let limited = ContentReader::new(
                c::Limits {
                    max_history_visits: 1,
                    ..Default::default()
                },
                1,
            )
            .unwrap();
            assert!(matches!(
                f.read.search_visibility(&limited).await,
                Err(q::Error::Limit)
            ));
            let limited = ContentReader::new(
                c::Limits {
                    max_output_bytes: 1,
                    ..Default::default()
                },
                1,
            )
            .unwrap();
            assert!(matches!(
                f.read.search_visibility(&limited).await,
                Err(q::Error::Limit)
            ));
            f.f.session
                .change_token(&jj_store_service::administration::TokenRequest {
                    request_id: m::RequestId::new([224; 16]),
                    expected: f.f.session.snapshot().await.unwrap().manifest().snapshot(),
                    change: jj_store_service::administration::TokenChange::Revoke {
                        token: f.f.token.id(),
                    },
                })
                .await
                .unwrap();
            assert!(matches!(
                f.read.search_visibility(&reader).await,
                Err(q::Error::Denied)
            ));
        })
        .await;
}

async fn foreign_visibility(
    repo: m::RepositoryId,
    epoch: m::EpochId,
) -> jj_store_service::reads::VisibleView {
    let store = Arc::new(MemoryStore::default());
    let objects = Arc::new(NativeObjects::new(store.clone(), repo, 4 * 1024 * 1024).unwrap());
    let mut policy = TokenPolicy::new(jj_store_service::auth::Scope::Repository(repo));
    let token = policy.issue(ACTOR, Permission::Admin, None).unwrap();
    let mut operator = TokenPolicy::new(jj_store_service::auth::Scope::Operator);
    let op = operator
        .issue(ACTOR, Permission::CreateRepository, None)
        .unwrap();
    create_repository(
        store.clone(),
        objects.clone(),
        &operator,
        &op,
        &policy,
        epoch,
        &SystemClock,
        jj_store_wal::publication::Limits::default(),
    )
    .await
    .unwrap();
    let session = Arc::new(
        NativeSession::new(
            store,
            objects,
            token,
            Arc::new(SystemClock),
            Protection::default(),
            SessionLimits::default(),
        )
        .unwrap(),
    );
    let OpenRead::Pinned(read) = session
        .open_read(None, ReadLimits::default())
        .await
        .unwrap()
    else {
        panic!("root")
    };
    Arc::new(read)
        .search_visibility(&ContentReader::new(Default::default(), 1).unwrap())
        .await
        .unwrap()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn native_tantivy_rejects_foreign_view_and_changed_epoch_filters() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            publish_text_scope(&f, roots::ROOT_COMMIT, 1, &[]).await;
            let config = Config::default();
            let root = tempfile::tempdir().unwrap();
            let (projection, search) = TantivyProjection::open(
                root.path().join("index"),
                NativeExtractor::new(config.clone()).unwrap(),
                Limits::default(),
            )
            .unwrap();
            let source = f.session.adapter_source();
            let mut runner = a::Runner::new(
                source.clone(),
                config.identity().unwrap(),
                projection,
                root.path(),
                Default::default(),
            )
            .unwrap();
            runner.step(a::CancellationToken::new()).await.unwrap();
            let pin = search.pin(&source).await.unwrap();
            for (repo, epoch) in [
                (m::RepositoryId::new([250; 16]), m::EpochId::new([4; 16])),
                (REPO, m::EpochId::new([249; 16])),
            ] {
                let view = foreign_visibility(repo, epoch).await;
                assert!(matches!(
                    pin.query(
                        &source,
                        SearchRequest::visible("metafixture", 0, 10, view),
                        SearchLimits::default()
                    )
                    .await,
                    Err(a::Error::Denied)
                ));
            }
        })
        .await;
}
