use super::*;
async fn observed(f: &ImportFixture) {
    tokio::time::timeout(Duration::from_secs(10), async {
        while f.runtime.snapshot().status.observed_sources != 1 {
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn import_waits_for_existing_raw_active_reader_and_keeps_the_same_session_identity() {
    let mut f = ImportFixture::new().await;
    f.runtime.shutdown().await.unwrap();
    f.config.agent_capture.max_pending_bytes = 128;
    f.save(&f.config);
    f.f.spool = jj_store_capture::agent_capture::AgentCaptureSpool::open(
        &f.f.root.path().join("spool"),
        128,
    )
    .unwrap();
    super::restart::start_runtime(&mut f);
    observed(&f).await;
    f.intent.selector = f
        .manager
        .sources(
            f.intent.principal,
            CaptureSourceQuery {
                limit: Some(100),
                cursor: None,
            },
        )
        .await
        .unwrap()
        .sources[0]
        .selector
        .clone();
    drop(f.f.start());
    f.f.message("active", "active raw batch retained before explicit import");
    tokio::time::timeout(Duration::from_secs(10), async {
        while f.runtime.snapshot().status.ingress_capacity_refusals != 1 {
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .unwrap();
    let original =
        f.f.sources
            .lookup_admitted(&f.f.qualified())
            .unwrap()
            .unwrap();
    assert_eq!(f.runtime.snapshot().status.resident_source_frames, 1);
    schedule(&f).await.unwrap();
    tokio::time::sleep(Duration::from_millis(100)).await;
    assert!(
        f.imports.entries().unwrap().is_empty(),
        "import replaced a noncompact active frame"
    );
    assert!(f.f.spool.pending().unwrap().is_empty());
    assert_eq!(f.runtime.snapshot().status.resident_source_frames, 1);
    assert_eq!(
        f.f.sources.lookup_admitted(&f.f.qualified()).unwrap(),
        Some(original)
    );
    f.runtime.shutdown().await.unwrap();
}

#[tokio::test]
async fn compact_active_handoff_reuses_cursor_alias_and_publishes_one_native_session() {
    let mut f = ImportFixture::new().await;
    observed(&f).await;
    drop(f.f.start());
    f.f.message("active", "active source before explicit import");
    published(&f, "active source before explicit import").await;
    tokio::time::timeout(Duration::from_secs(10), async {
        while f.runtime.snapshot().status.caught_up != 1 {
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .unwrap();
    let original =
        f.f.sources
            .lookup_admitted(&f.f.qualified())
            .unwrap()
            .unwrap();
    schedule(&f).await.unwrap();
    tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            if let Ok(entries) = f.imports.entries()
                && !entries.is_empty()
            {
                break;
            }
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .unwrap();
    f.f.message("tail", "tail after compact import handoff");
    published(&f, "tail after compact import handoff").await;
    pause(&f).await;
    assert_eq!(
        f.f.sources.lookup_admitted(&f.f.qualified()).unwrap(),
        Some(original)
    );
    assert_eq!(f.imports.entries().unwrap().len(), 1);
    let session =
        f.f.host
            .endpoint(A)
            .unwrap()
            .bind_local_trusted(f.f.authority.principal(), Permission::Read)
            .unwrap()
            .into_session();
    let pager =
        AgentSessionPager::new(pagination::Limits::default(), Arc::new(SystemClock)).unwrap();
    assert_eq!(pager.begin(session, 10).await.unwrap().sessions.len(), 1);
    f.runtime.shutdown().await.unwrap();
}
