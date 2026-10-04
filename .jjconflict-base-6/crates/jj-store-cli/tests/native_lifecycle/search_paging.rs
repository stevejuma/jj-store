use super::*;
use jj_store_service::reads;
use jj_store_tantivy::{
    TantivyProjection,
    native::{Config, NativeExtractor},
    paging::{self, Pager, Request},
};

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn native_search_pages_pin_generation_view_and_query_across_writes() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let (a, _) = publish_text_scope(&f, roots::ROOT_COMMIT, 1, &[]).await;
            let (b, op) = publish_text_scope(&f, a, 2, &[]).await;
            let config = Config::default();
            let root = tempfile::tempdir().unwrap();
            let (projection, search) = TantivyProjection::open(
                root.path().join("index"),
                NativeExtractor::new(config.clone()).unwrap(),
                Default::default(),
            )
            .unwrap();
            let mut runner = a::Runner::new(
                f.session.adapter_source(),
                config.identity().unwrap(),
                projection,
                root.path(),
                Default::default(),
            )
            .unwrap();
            runner.step(a::CancellationToken::new()).await.unwrap();
            let pager = Pager::new(
                search,
                runner.subscribe(),
                ContentReader::new(Default::default(), 1).unwrap(),
                Default::default(),
                Arc::new(SystemClock),
            )
            .unwrap();
            let request = Request {
                q: "metafixture commit".into(),
                page_size: 1,
                ..Default::default()
            };
            let first = pager
                .begin(f.session.clone(), request.clone())
                .await
                .unwrap();
            assert_eq!(first.selected_view.as_ref().unwrap().operation_id, op);
            assert!(!first.is_stale);
            let cursor = first.next_cursor.clone().unwrap();
            publish_text_scope(&f, roots::ROOT_COMMIT, 3, &[]).await;
            runner.step(a::CancellationToken::new()).await.unwrap();
            let second = pager
                .resume(f.session.clone(), &cursor, request.clone())
                .await
                .unwrap();
            assert_eq!(first.index_generation, second.index_generation);
            assert_eq!(first.selected_view, second.selected_view);
            assert!(second.is_stale);
            assert_eq!(
                first
                    .results
                    .iter()
                    .chain(&second.results)
                    .map(|d| d.commit.unwrap())
                    .collect::<BTreeSet<_>>(),
                BTreeSet::from([a, b])
            );
            assert!(second.next_cursor.is_none());
            assert!(matches!(
                pager
                    .resume(
                        f.session.clone(),
                        &cursor,
                        Request {
                            q: "different".into(),
                            ..request.clone()
                        }
                    )
                    .await,
                Err(paging::Error::InvalidCursor)
            ));
            assert!(matches!(
                pager
                    .resume(f.session.clone(), &format!("{cursor}x"), request)
                    .await,
                Err(paging::Error::InvalidCursor)
            ));
        })
        .await;
}

use std::sync::atomic::{AtomicU64, Ordering};
struct TestClock(AtomicU64);
impl jj_store_service::auth::Clock for TestClock {
    fn unix_seconds(&self) -> jj_store_service::auth::Result<u64> {
        Ok(self.0.load(Ordering::SeqCst))
    }
}
struct SearchFixture {
    f: Fixture,
    root: tempfile::TempDir,
    runner: a::Runner<TantivyProjection<NativeExtractor>>,
    search: jj_store_tantivy::Search,
    clock: Arc<TestClock>,
    pager: Pager,
    faults: Arc<jj_store_tantivy::Faults>,
}
impl SearchFixture {
    async fn new(limits: paging::Limits) -> Self {
        let f = Fixture::new().await;
        let (first, _) =
            publish_text_scope(&f, roots::ROOT_COMMIT, 1, &[("code.rs", b"signal one")]).await;
        publish_text_scope(&f, first, 2, &[("code.rs", b"signal two")]).await;
        let config = Config {
            scopes: [(
                "main".into(),
                jj_store_tantivy::native::Scope::Bookmark {
                    name: "main".into(),
                },
            )]
            .into(),
            ..Default::default()
        };
        let root = tempfile::tempdir().unwrap();
        let (projection, search) = TantivyProjection::open(
            root.path().join("index"),
            NativeExtractor::new(config.clone()).unwrap(),
            Default::default(),
        )
        .unwrap();
        let faults = projection.faults();
        let mut runner = a::Runner::new(
            f.session.adapter_source(),
            config.identity().unwrap(),
            projection,
            root.path(),
            Default::default(),
        )
        .unwrap();
        runner.step(a::CancellationToken::new()).await.unwrap();
        let clock = Arc::new(TestClock(AtomicU64::new(1000)));
        let pager = Pager::new(
            search.clone(),
            runner.subscribe(),
            ContentReader::new(Default::default(), 1).unwrap(),
            limits,
            clock.clone(),
        )
        .unwrap();
        Self {
            f,
            root,
            runner,
            search,
            clock,
            pager,
            faults,
        }
    }
    fn request() -> Request {
        Request {
            q: "metafixture commit".into(),
            page_size: 1,
            ..Default::default()
        }
    }
    fn fresh_pager(&self, limits: paging::Limits) -> Pager {
        Pager::new(
            self.search.clone(),
            self.runner.subscribe(),
            ContentReader::new(Default::default(), 1).unwrap(),
            limits,
            self.clock.clone(),
        )
        .unwrap()
    }
}
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn native_search_freshness_waits_catch_up_and_reauthorize_under_admission() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let mut f = SearchFixture::new(paging::Limits {
                concurrency: 1,
                ..Default::default()
            })
            .await;
            let reader = f.f.session(f.f.reader.clone(), Protection::default());
            publish_text_scope(&f.f, roots::ROOT_COMMIT, 3, &[]).await;
            let request = Request {
                minimum_position: Some(m::WalPosition(3)),
                ..SearchFixture::request()
            };
            let error = f
                .pager
                .begin(reader.clone(), request.clone())
                .await
                .unwrap_err();
            let paging::Error::IndexLag(status) = error else {
                panic!("lag")
            };
            assert_eq!(status.indexed_position, Some(m::WalPosition(2)));
            assert_eq!(status.repository_position, m::WalPosition(3));
            assert!(status.is_stale);
            // Prepare the new durable generation before timing the serving
            // wait. Building an index is qualified separately from reader
            // catch-up and has no one-second performance contract.
            f.faults.once(jj_store_tantivy::Fault::BeforeReader);
            assert!(matches!(
                f.runner.step(a::CancellationToken::new()).await,
                Err(a::Error::Projection)
            ));
            let wait = Request {
                wait_millis: 5000,
                ..request
            };
            let (page, _) = tokio::join!(f.pager.begin(reader.clone(), wait), async {
                tokio::time::sleep(Duration::from_millis(20)).await;
                assert!(matches!(
                    f.pager.status(reader.clone()).await,
                    Err(paging::Error::Busy)
                ));
                f.runner.step(a::CancellationToken::new()).await.unwrap();
            });
            let page = page.unwrap();
            assert_eq!(page.indexed_position, m::WalPosition(3));
            assert!(!page.is_stale);
            let wait = Request {
                minimum_position: Some(m::WalPosition(99)),
                wait_millis: 5000,
                ..SearchFixture::request()
            };
            let (denied, _) = tokio::join!(f.pager.begin(reader.clone(), wait), async {
                tokio::time::sleep(Duration::from_millis(20)).await;
                f.f.session
                    .change_token(&jj_store_service::administration::TokenRequest {
                        request_id: m::RequestId::new([229; 16]),
                        expected: f.f.session.snapshot().await.unwrap().manifest().snapshot(),
                        change: jj_store_service::administration::TokenChange::Revoke {
                            token: f.f.reader.id(),
                        },
                    })
                    .await
                    .unwrap();
            });
            assert!(matches!(
                denied,
                Err(paging::Error::Read(reads::Error::Denied))
                    | Err(paging::Error::Adapter(a::Error::Denied))
            ));
            assert!(matches!(
                f.pager
                    .resume(reader, "malformed", SearchFixture::request())
                    .await,
                Err(paging::Error::Read(reads::Error::Denied))
            ));
            // The failed wait releases serving admission.
            assert!(f.pager.status(f.f.session.clone()).await.is_ok());
        })
        .await;
}
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn native_search_cursor_expiry_releases_idle_generations_and_restart_resets() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let mut f = SearchFixture::new(paging::Limits {
                pins: 1,
                ttl: Duration::from_secs(1),
                ..Default::default()
            })
            .await;
            let request = SearchFixture::request();
            let before = f
                .pager
                .begin(f.f.session.clone(), request.clone())
                .await
                .unwrap();
            let cursor = before.next_cursor.unwrap();
            assert!(matches!(
                f.pager.begin(f.f.session.clone(), request.clone()).await,
                Err(paging::Error::Busy)
            ));
            let restarted = f.fresh_pager(Default::default());
            assert!(matches!(
                restarted
                    .resume(f.f.session.clone(), &cursor, request.clone())
                    .await,
                Err(paging::Error::CursorReset)
            ));
            let index = f.root.path().join("index");
            let old = index.join(fs::read_to_string(index.join("active")).unwrap());
            publish_text_scope(&f.f, roots::ROOT_COMMIT, 3, &[]).await;
            f.runner.step(a::CancellationToken::new()).await.unwrap();
            assert!(old.exists());
            f.clock.0.store(1002, Ordering::SeqCst);
            // No search request drives this cleanup: idle cursor maintenance must release the old generation.
            tokio::time::timeout(Duration::from_secs(3), async {
                while old.exists() {
                    tokio::time::sleep(Duration::from_millis(20)).await;
                }
            })
            .await
            .unwrap();
            assert!(matches!(
                f.pager
                    .resume(f.f.session.clone(), &cursor, request.clone())
                    .await,
                Err(paging::Error::CursorReset)
            ));
            assert!(f.pager.begin(f.f.session.clone(), request).await.is_ok());
        })
        .await;
}
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn native_search_scope_filters_truncation_and_view_retention_limits() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = SearchFixture::new(Default::default()).await;
            let before = f.f.session.snapshot().await.unwrap().manifest().clone();
            let query = Request {
                q: "signal".into(),
                scope: Some("main".into()),
                ..Default::default()
            };
            let page = f
                .pager
                .begin(f.f.session.clone(), query.clone())
                .await
                .unwrap();
            assert_eq!(page.results.len(), 1);
            assert!(matches!(
                page.results[0].key,
                jj_store_tantivy::DocumentKey::File { .. }
            ));
            assert!(page.results[0].body.is_empty());
            assert!(
                f.pager
                    .begin(
                        f.f.session.clone(),
                        Request {
                            q: "metafixture".into(),
                            ..query.clone()
                        }
                    )
                    .await
                    .unwrap()
                    .results
                    .is_empty()
            );
            assert!(matches!(
                f.pager
                    .begin(
                        f.f.session.clone(),
                        Request {
                            scope: Some("absent".into()),
                            ..query
                        }
                    )
                    .await,
                Err(paging::Error::Adapter(a::Error::Invalid))
            ));
            let truncated = f.fresh_pager(paging::Limits {
                query: jj_store_tantivy::SearchLimits {
                    max_offset: 0,
                    ..Default::default()
                },
                ..Default::default()
            });
            let page = truncated
                .begin(f.f.session.clone(), SearchFixture::request())
                .await
                .unwrap();
            assert!(page.truncated);
            assert!(page.next_cursor.is_none());
            assert_eq!(page.results.len(), 1);
            let small = f.fresh_pager(paging::Limits {
                view_nodes: 1,
                ..Default::default()
            });
            assert!(matches!(
                small
                    .begin(f.f.session.clone(), SearchFixture::request())
                    .await,
                Err(paging::Error::Busy)
            ));
            // Historical pins retain no native visibility set and fit the same node budget.
            assert!(
                small
                    .begin(
                        f.f.session.clone(),
                        Request {
                            mode: paging::Mode::Historical,
                            ..SearchFixture::request()
                        }
                    )
                    .await
                    .unwrap()
                    .selected_view
                    .is_none()
            );
            assert_eq!(f.f.session.snapshot().await.unwrap().manifest(), &before);
        })
        .await;
}
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn native_search_cursor_epoch_and_request_changes_fail_closed() {
    assert!(serde_json::from_str::<Request>("{}").is_err());
    assert!(serde_json::from_str::<Request>(r#"{"q":"text","unknown":true}"#).is_err());
    assert_eq!(
        serde_json::from_str::<Request>(r#"{"q":"text"}"#).unwrap(),
        Request {
            q: "text".into(),
            ..Default::default()
        }
    );
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = SearchFixture::new(Default::default()).await;
            let request = SearchFixture::request();
            let cursor = f
                .pager
                .begin(f.f.session.clone(), request.clone())
                .await
                .unwrap()
                .next_cursor
                .unwrap();
            for changed in [
                Request {
                    mode: paging::Mode::Historical,
                    ..request.clone()
                },
                Request {
                    page_size: 2,
                    ..request.clone()
                },
                Request {
                    scope: Some("main".into()),
                    ..request.clone()
                },
                Request {
                    minimum_position: Some(m::WalPosition(1)),
                    ..request.clone()
                },
            ] {
                assert!(matches!(
                    f.pager.resume(f.f.session.clone(), &cursor, changed).await,
                    Err(paging::Error::InvalidCursor)
                ));
            }
            let key = Key::new(format!("repos/{REPO}/manifest.json")).unwrap();
            let download = f.f.store.get(&key, None).await.unwrap();
            let version = download.metadata.version.clone();
            let mut manifest: Manifest =
                serde_json::from_slice(&download.read_bounded(128 * 1024).await.unwrap()).unwrap();
            manifest.epoch = m::EpochId::new([230; 16]);
            let upload =
                StagedUpload::from_bytes(&serde_json::to_vec(&manifest).unwrap(), 128 * 1024)
                    .await
                    .unwrap();
            assert!(matches!(
                f.f.store
                    .put(&key, Condition::Matches(version), &upload)
                    .await
                    .unwrap(),
                WriteOutcome::Applied(_)
            ));
            assert!(matches!(
                f.pager.resume(f.f.session.clone(), &cursor, request).await,
                Err(paging::Error::EpochChanged)
            ));
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn native_search_reports_served_position_during_degraded_reader_recovery() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let mut f = SearchFixture::new(Default::default()).await;
            publish_text_scope(&f.f, roots::ROOT_COMMIT, 3, &[]).await;
            f.faults.once(jj_store_tantivy::Fault::BeforeReader);
            assert!(matches!(
                f.runner.step(a::CancellationToken::new()).await,
                Err(a::Error::Projection)
            ));
            let status = f.pager.status(f.f.session.clone()).await.unwrap();
            assert!(status.degraded);
            assert_eq!(status.durable_position, Some(m::WalPosition(3)));
            assert_eq!(status.indexed_position, Some(m::WalPosition(2)));
            let lag = f
                .pager
                .begin(
                    f.f.session.clone(),
                    Request {
                        minimum_position: Some(m::WalPosition(3)),
                        wait_millis: 10,
                        ..SearchFixture::request()
                    },
                )
                .await
                .unwrap_err();
            assert!(matches!(lag, paging::Error::IndexLag(_)));
            f.runner.step(a::CancellationToken::new()).await.unwrap();
            let status = f.pager.status(f.f.session.clone()).await.unwrap();
            assert!(!status.degraded);
            assert!(!status.is_stale);
            assert_eq!(status.indexed_position, Some(m::WalPosition(3)));
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn native_search_requires_head_selection_and_keeps_historical_mode_explicit() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let mut f = SearchFixture::new(Default::default()).await;
            let prior =
                f.f.session
                    .snapshot()
                    .await
                    .unwrap()
                    .manifest()
                    .heads
                    .clone();
            let current = *prior.first().unwrap();
            let operation =
                f.f.objects
                    .get::<o::Operation>(m::ObjectKey::Operation(current))
                    .await
                    .unwrap();
            let old = oid(&operation.parents[0]);
            let mut sibling = operation.clone();
            sibling.metadata.description = "search sibling".into();
            sibling.parents = vec![o::OperationId::from_bytes(old.as_bytes())];
            let key = f.f.objects.put(&sibling).await.unwrap().0;
            f.f.session
                .publish(
                    BTreeSet::from([old]),
                    m::OperationId::from_slice(key.as_bytes()).unwrap(),
                )
                .await
                .unwrap();
            f.runner.step(a::CancellationToken::new()).await.unwrap();
            let before = f.f.session.snapshot().await.unwrap().manifest().clone();
            let select = f
                .pager
                .begin(f.f.session.clone(), SearchFixture::request())
                .await
                .unwrap_err();
            let paging::Error::SelectHead(heads) = select else {
                panic!("selection")
            };
            assert_eq!(heads.heads.len(), 2);
            assert!(
                f.pager
                    .begin(
                        f.f.session.clone(),
                        Request {
                            operation: Some(current),
                            ..SearchFixture::request()
                        }
                    )
                    .await
                    .unwrap()
                    .selected_view
                    .is_some()
            );
            assert!(
                f.pager
                    .begin(
                        f.f.session.clone(),
                        Request {
                            mode: paging::Mode::Historical,
                            ..SearchFixture::request()
                        }
                    )
                    .await
                    .unwrap()
                    .selected_view
                    .is_none()
            );
            assert_eq!(f.f.session.snapshot().await.unwrap().manifest(), &before);
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn native_search_page_output_failure_does_not_exhaust_cursor_pins() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = SearchFixture::new(Default::default()).await;
            let request = SearchFixture::request();
            let response = f
                .pager
                .begin(f.f.session.clone(), request.clone())
                .await
                .unwrap();
            let limit = serde_json::to_vec(&response).unwrap().len() - 100;
            let OpenRead::Pinned(read) =
                f.f.session
                    .open_read(None, ReadLimits::default())
                    .await
                    .unwrap()
            else {
                panic!("pin")
            };
            let view = Arc::new(read)
                .search_visibility(&ContentReader::new(Default::default(), 1).unwrap())
                .await
                .unwrap();
            let source = f.f.session.adapter_source();
            let core = f
                .search
                .pin(&source)
                .await
                .unwrap()
                .query(
                    &source,
                    jj_store_tantivy::SearchRequest::visible(&request.q, 0, 1, view),
                    Default::default(),
                )
                .await
                .unwrap();
            assert!(
                serde_json::to_vec(&core).unwrap().len() < limit,
                "core output must fit to exercise failure after cursor retention"
            );
            let pager = f.fresh_pager(paging::Limits {
                pins: 1,
                query: jj_store_tantivy::SearchLimits {
                    output_bytes: limit,
                    ..Default::default()
                },
                ..Default::default()
            });
            for _ in 0..2 {
                assert!(matches!(
                    pager.begin(f.f.session.clone(), request.clone()).await,
                    Err(paging::Error::Adapter(a::Error::Limit))
                ));
            }
            assert!(
                pager
                    .begin(
                        f.f.session.clone(),
                        Request {
                            q: "no-match-for-this-query".into(),
                            ..request
                        }
                    )
                    .await
                    .unwrap()
                    .results
                    .is_empty()
            );
        })
        .await;
}

#[path = "http_search.rs"]
mod http_search;
