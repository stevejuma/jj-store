use super::*;
use jj_store_service::events::{Cursor, Error, EventKind, Limits as EventLimits, Subscription};
use std::time::Duration;

fn limits() -> EventLimits {
    EventLimits {
        poll_interval: Duration::from_millis(20),
        ..Default::default()
    }
}
fn change(f: &Fixture, number: u8) -> Request {
    Request {
        principal: ACTOR,
        request_id: m::RequestId::new([number; 16]),
        publication_id: m::PublicationId::new([number; 16]),
        transition: None,
        settings: Some(f.policy.encode().unwrap()),
        result: vec![],
        kind: SemanticKind::SettingsChanged,
    }
}
async fn publish(f: &Fixture, number: u8) {
    assert!(matches!(
        f.session.publish_request(&change(f, number)).await.unwrap(),
        PublishOutcome::Committed(_)
    ));
}
async fn next(feed: &mut Subscription) -> jj_store_service::events::Event {
    tokio::time::timeout(Duration::from_secs(2), feed.next())
        .await
        .unwrap()
        .unwrap()
        .unwrap()
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn events_replay_only_committed_positions_across_restart_and_checkpoint() {
    let f = Fixture::new().await;
    let session = f.session(f.reader.clone(), Protection::default());
    let mut feed = Subscription::open(session, None, limits()).await.unwrap();
    let initial = next(&mut feed).await;
    assert!(matches!(initial.kind, EventKind::Ready { .. }));
    let zero = initial.cursor;
    assert_eq!(zero.position.0, 0);
    // Staging and a failed publication cannot appear in the stream.
    f.objects
        .put_file(b"orphan native bytes".as_slice())
        .await
        .unwrap();
    let mut invalid = change(&f, 90);
    invalid.principal = m::PrincipalId::new([8; 16]);
    assert!(f.session.publish_request(&invalid).await.is_err());
    assert!(matches!(next(&mut feed).await.kind, EventKind::Heartbeat));
    publish(&f, 91).await;
    publish(&f, 92).await;
    for position in 1..=2 {
        let event = next(&mut feed).await;
        assert_eq!(event.cursor.position.0, position);
        assert!(matches!(event.kind, EventKind::Changed));
    }
    // Replaying an accepted request cannot create another event.
    publish(&f, 91).await;
    assert!(matches!(next(&mut feed).await.kind, EventKind::Heartbeat));
    struct Deny;
    #[async_trait::async_trait]
    impl PublicationGuard for Deny {
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
    Coordinator::new(
        f.store.clone(),
        REPO,
        f.objects.clone(),
        Arc::new(Deny),
        Limits::default(),
    )
    .unwrap()
    .checkpoint()
    .await
    .unwrap();
    assert!(matches!(next(&mut feed).await.kind, EventKind::Heartbeat));
    drop(feed);
    let encoded = zero.to_string();
    assert_eq!(encoded.parse::<Cursor>().unwrap(), zero);
    let mut cold = Subscription::open(
        f.session(f.reader.clone(), Protection::default()),
        Some(encoded.parse().unwrap()),
        limits(),
    )
    .await
    .unwrap();
    assert_eq!(next(&mut cold).await.cursor, zero);
    assert_eq!(next(&mut cold).await.cursor.position.0, 1);
    assert_eq!(next(&mut cold).await.cursor.position.0, 2);
    assert_eq!(
        f.session.snapshot().await.unwrap().manifest().published.0,
        2
    );
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn events_reject_foreign_cursors_and_reset_expired_future_and_replaced_epochs() {
    let f = Fixture::new().await;
    let mut feed = Subscription::open(f.session.clone(), None, limits())
        .await
        .unwrap();
    let zero = next(&mut feed).await.cursor;
    assert!("v1:invalid".parse::<Cursor>().is_err());
    assert!(format!("{zero}0").parse::<Cursor>().is_err());
    let mut foreign = zero;
    foreign.repository = m::RepositoryId::new([9; 16]);
    assert!(matches!(
        Subscription::open(f.session.clone(), Some(foreign), limits()).await,
        Err(Error::InvalidCursor)
    ));
    for number in 93..=95 {
        publish(&f, number).await;
    }
    for cursor in [
        zero,
        Cursor {
            position: m::WalPosition(4),
            ..zero
        },
        Cursor {
            epoch: m::EpochId::new([5; 16]),
            ..zero
        },
    ] {
        let mut feed = Subscription::open(
            f.session.clone(),
            Some(cursor),
            EventLimits {
                replay_window: 2,
                ..limits()
            },
        )
        .await
        .unwrap();
        let reset = next(&mut feed).await;
        assert!(matches!(reset.kind, EventKind::Reset { .. }));
        assert_eq!(reset.cursor.position.0, 3);
        assert!(feed.next().await.unwrap().is_none());
    }
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn events_reauthorize_queued_records_after_revocation() {
    let mut f = Fixture::new().await;
    let mut feed = Subscription::open(
        f.session(f.reader.clone(), Protection::default()),
        None,
        limits(),
    )
    .await
    .unwrap();
    next(&mut feed).await;
    for number in 96..=98 {
        publish(&f, number).await;
    }
    assert_eq!(next(&mut feed).await.cursor.position.0, 1);
    f.policy.revoke(f.reader.id());
    publish(&f, 99).await;
    assert!(matches!(feed.next().await, Err(Error::Denied)));
    assert!(feed.next().await.unwrap().is_none());
}

fn host(f: &Fixture) -> jj_store_service::administration::RepositoryHost {
    struct NoOperators;
    #[async_trait::async_trait]
    impl jj_store_service::administration::OperatorPolicies for NoOperators {
        async fn current(&self) -> jj_store_service::auth::Result<TokenPolicy> {
            panic!("event reads accessed operator policy")
        }
    }
    jj_store_service::administration::RepositoryHost::new(
        f.store.clone(),
        Arc::new(NoOperators),
        Arc::new(SystemClock),
        Protection::default(),
        SessionLimits::default(),
        4 * 1024 * 1024,
    )
    .unwrap()
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn events_local_publication_wakes_shared_host_without_waiting_for_poll() {
    let f = Fixture::new().await;
    let host = host(&f);
    let endpoint = host.endpoint(REPO).unwrap();
    let writer = host
        .clone()
        .endpoint(REPO)
        .unwrap()
        .bind(f.token.clone())
        .unwrap()
        .into_session();
    let reader = endpoint.bind(f.reader.clone()).unwrap().into_session();
    let mut feed = Subscription::open(
        reader,
        None,
        EventLimits {
            poll_interval: Duration::from_secs(60),
            ..limits()
        },
    )
    .await
    .unwrap();
    next(&mut feed).await;
    assert!(matches!(
        writer.publish_request(&change(&f, 100)).await.unwrap(),
        PublishOutcome::Committed(_)
    ));
    assert_eq!(next(&mut feed).await.cursor.position.0, 1);
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn events_corrupt_selected_record_fails_without_skipping_or_resetting() {
    use jj_store_object::{Condition, ObjectStore as _, StagedUpload};
    let f = Fixture::new().await;
    let mut feed = Subscription::open(f.session.clone(), None, limits())
        .await
        .unwrap();
    next(&mut feed).await;
    publish(&f, 101).await;
    let snapshot = f.session.snapshot().await.unwrap();
    let key = snapshot.manifest().tail.unwrap().key(REPO).unwrap();
    let token = f.store.head(&key).await.unwrap().version;
    f.store
        .put(
            &key,
            Condition::Matches(token),
            &StagedUpload::from_bytes(b"corrupt selected record", 128)
                .await
                .unwrap(),
        )
        .await
        .unwrap();
    assert!(matches!(feed.next().await, Err(Error::Unavailable)));
    assert!(feed.next().await.unwrap().is_none());
}

#[path = "http_events.rs"]
mod http_acceptance;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn events_epoch_replacement_resets_before_queued_delivery() {
    use jj_store_object::{Condition, Key, ObjectStore as _, StagedUpload};
    let f = Fixture::new().await;
    let mut feed = Subscription::open(f.session.clone(), None, limits())
        .await
        .unwrap();
    next(&mut feed).await;
    for number in 114..=116 {
        publish(&f, number).await;
    }
    assert_eq!(next(&mut feed).await.cursor.position.0, 1);
    let key = Key::new(format!("repos/{REPO}/manifest.json")).unwrap();
    let version = f.store.head(&key).await.unwrap().version;
    let mut manifest = f.session.snapshot().await.unwrap().manifest().clone();
    manifest.epoch = m::EpochId::new([12; 16]);
    let bytes = serde_json::to_vec(&manifest).unwrap();
    f.store
        .put(
            &key,
            Condition::Matches(version),
            &StagedUpload::from_bytes(&bytes, 128 * 1024).await.unwrap(),
        )
        .await
        .unwrap();
    let reset = next(&mut feed).await;
    assert!(matches!(
        reset.kind,
        EventKind::Reset {
            reason: jj_store_service::events::ResetReason::EpochChanged,
            ..
        }
    ));
    assert_eq!(reset.cursor.epoch, manifest.epoch);
    assert!(feed.next().await.unwrap().is_none());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn events_storage_deadlines_close_without_advancing_resume_position() {
    use jj_store_object::{
        ByteRange, Condition, Download, Key, Metadata, ObjectStore, StagedUpload, WriteOutcome,
    };
    use std::sync::atomic::{AtomicBool, Ordering};
    struct Pending {
        inner: Arc<MemoryStore>,
        stall: Arc<AtomicBool>,
    }
    #[async_trait::async_trait]
    impl ObjectStore for Pending {
        async fn head(&self, key: &Key) -> jj_store_object::Result<Metadata> {
            self.inner.head(key).await
        }
        async fn get(
            &self,
            key: &Key,
            range: Option<ByteRange>,
        ) -> jj_store_object::Result<Download> {
            if self.stall.load(Ordering::SeqCst) {
                std::future::pending::<()>().await;
            }
            self.inner.get(key, range).await
        }
        async fn put(
            &self,
            key: &Key,
            condition: Condition,
            upload: &StagedUpload,
        ) -> jj_store_object::Result<WriteOutcome> {
            self.inner.put(key, condition, upload).await
        }
    }
    let f = Fixture::new().await;
    let stall = Arc::new(AtomicBool::new(false));
    let store = Arc::new(Pending {
        inner: f.store.clone(),
        stall: stall.clone(),
    });
    let objects = Arc::new(NativeObjects::new(store.clone(), REPO, 4 * 1024 * 1024).unwrap());
    let reader = Arc::new(
        NativeSession::new(
            store,
            objects,
            f.reader.clone(),
            Arc::new(SystemClock),
            Protection::default(),
            SessionLimits::default(),
        )
        .unwrap(),
    );
    let mut feed = Subscription::open(
        reader,
        None,
        EventLimits {
            read_timeout: Duration::from_millis(20),
            ..limits()
        },
    )
    .await
    .unwrap();
    let cursor = next(&mut feed).await.cursor;
    publish(&f, 117).await;
    stall.store(true, Ordering::SeqCst);
    assert!(matches!(
        tokio::time::timeout(Duration::from_secs(1), feed.next())
            .await
            .unwrap(),
        Err(Error::Limit)
    ));
    assert!(feed.next().await.unwrap().is_none());
    let mut retry = Subscription::open(f.session.clone(), Some(cursor), limits())
        .await
        .unwrap();
    next(&mut retry).await;
    assert_eq!(next(&mut retry).await.cursor.position.0, 1);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn events_lagged_wake_hints_replay_every_selected_record_in_bounded_pages() {
    let f = Fixture::new().await;
    let host = host(&f);
    let writer = host
        .endpoint(REPO)
        .unwrap()
        .bind(f.token.clone())
        .unwrap()
        .into_session();
    let reader = host
        .endpoint(REPO)
        .unwrap()
        .bind(f.reader.clone())
        .unwrap()
        .into_session();
    let mut feed = Subscription::open(
        reader,
        None,
        EventLimits {
            poll_interval: Duration::from_secs(60),
            page_size: 2,
            ..limits()
        },
    )
    .await
    .unwrap();
    next(&mut feed).await;
    for number in 1..=70 {
        writer.publish_request(&change(&f, number)).await.unwrap();
    }
    for position in 1..=70 {
        let event = next(&mut feed).await;
        assert_eq!(event.cursor.position.0, position);
        assert!(matches!(event.kind, EventKind::Changed));
    }
    assert_eq!(
        f.session.snapshot().await.unwrap().manifest().published.0,
        70
    );
}
