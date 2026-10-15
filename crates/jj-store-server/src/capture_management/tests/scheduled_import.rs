use super::import_policy::ImportFixture;
use super::*;
use crate::capture_test_fixture::{A, SECRET};
use jj_store_service::{auth::SystemClock, collaboration::AgentSessionPager, pagination};
mod handoff;
mod restart;

async fn schedule(f: &ImportFixture) -> crate::Result<()> {
    let dispatch = dispatch(f).await;
    f.runtime.controls().schedule_import(dispatch).await
}
async fn dispatch(f: &ImportFixture) -> crate::capture_management::CaptureImportDispatch {
    tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            match f.manager.begin_import(f.intent.clone(), f.imports.clone()) {
                Ok(policy) => return policy.into_dispatch().unwrap(),
                Err(Error::Busy) => tokio::time::sleep(Duration::from_millis(5)).await,
                Err(error) => panic!("dispatch admission: {error:?}"),
            }
        }
    })
    .await
    .unwrap_or_else(|error| {
        panic!(
            "{error}: runtime={:?}; import_receipts={:?}; pending_batches={:?}",
            f.runtime.snapshot(),
            f.imports.entries().map(|entries| entries.len()),
            f.f.spool.pending().map(|entries| entries.len())
        )
    })
}
async fn pause(f: &ImportFixture) {
    f.runtime.pause().unwrap();
    tokio::time::timeout(Duration::from_secs(10), async {
        while f.runtime.snapshot().phase != crate::capture_worker::CaptureWorkerPhase::Paused {
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .unwrap();
}
async fn resume(f: &ImportFixture) {
    f.runtime.resume().unwrap();
    tokio::time::timeout(Duration::from_secs(10), async {
        while !matches!(
            f.runtime.snapshot().phase,
            crate::capture_worker::CaptureWorkerPhase::Ready
                | crate::capture_worker::CaptureWorkerPhase::Running
        ) {
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .unwrap();
}
async fn published(f: &ImportFixture, text: &str) -> String {
    let session =
        f.f.host
            .endpoint(A)
            .unwrap()
            .bind_local_trusted(f.f.authority.principal(), Permission::Read)
            .unwrap()
            .into_session();
    let pager =
        AgentSessionPager::new(pagination::Limits::default(), Arc::new(SystemClock)).unwrap();
    tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            let page = pager.begin(session.clone(), 10).await.unwrap();
            if let Some(summary) = page.sessions.first() {
                let detail = pager
                    .detail(session.clone(), summary.session_id)
                    .await
                    .unwrap();
                let body = serde_json::to_string(&detail).unwrap();
                if body.contains(text) {
                    return body;
                }
            }
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .unwrap_or_else(|error| {
        panic!(
            "{error}: runtime={:?}; import_receipts={:?}; pending_batches={:?}",
            f.runtime.snapshot(),
            f.imports.entries().map(|entries| entries.len()),
            f.f.spool.pending().map(|entries| entries.len())
        )
    })
}

#[tokio::test]
async fn retained_runtime_import_publishes_inactive_history_with_existing_source_budgets() {
    let mut f = ImportFixture::new().await;
    f.f.message(
        "history",
        &format!("supervised inactive history token={SECRET}"),
    );
    assert!(f.imports.entries().unwrap().is_empty());
    assert!(
        f.f.sources
            .lookup_admitted(&f.f.qualified())
            .unwrap()
            .is_none()
    );
    schedule(&f).await.unwrap();
    let body = published(&f, "supervised inactive history").await;
    pause(&f).await;
    assert!(body.contains("[REDACTED]") && !body.contains(SECRET));
    let status = f.runtime.snapshot().status;
    assert_eq!(status.tracked, 1);
    assert!(status.running_sources <= 1 && status.resident_source_frames <= 2);
    assert!(
        f.f.sources
            .lookup_admitted(&f.f.qualified())
            .unwrap()
            .is_none()
    );
    assert_eq!(f.imports.entries().unwrap().len(), 1);
    f.runtime.shutdown().await.unwrap();
}

#[tokio::test]
async fn scheduled_import_pause_replay_and_tail_preserve_original_receipt_and_cursor() {
    let mut f = ImportFixture::new().await;
    f.f.message("old", "scheduled original history");
    schedule(&f).await.unwrap();
    published(&f, "scheduled original history").await;
    pause(&f).await;
    let receipt = f.imports.resolve(&f.intent).unwrap().unwrap();
    let pending = f.f.spool.pending().unwrap();
    f.f.message("tail", "scheduled continuing tail");
    assert!(
        f.manager
            .begin_import(f.intent.clone(), f.imports.clone())
            .unwrap()
            .into_dispatch()
            .is_err()
    );
    assert_eq!(f.imports.resolve(&f.intent).unwrap(), Some(receipt.clone()));
    assert_eq!(f.f.spool.pending().unwrap(), pending);
    resume(&f).await;
    schedule(&f).await.unwrap();
    published(&f, "scheduled continuing tail").await;
    pause(&f).await;
    assert_eq!(f.imports.entries().unwrap(), vec![receipt]);
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

#[tokio::test(flavor = "current_thread")]
async fn lost_caller_and_full_mailbox_keep_one_original_command_without_body_ownership() {
    let mut f = ImportFixture::new().await;
    f.f.message("history", "history after scheduling caller loss");
    let before = fs::read(f.f.root.path().join("sources/sources.json")).unwrap();
    let dispatch = dispatch(&f).await;
    let controls = f.runtime.controls();
    {
        let scheduled = controls.schedule_import(dispatch.clone());
        tokio::pin!(scheduled);
        tokio::select! {
            biased;
            _ = &mut scheduled => panic!("runtime cannot settle during the first synchronous poll"),
            _ = std::future::ready(()) => {},
        }
        assert!(
            controls.schedule_import(dispatch).await.is_err(),
            "mailbox accepted a second pending command"
        );
        assert_eq!(
            fs::read(f.f.root.path().join("sources/sources.json")).unwrap(),
            before
        );
        assert!(f.imports.entries().unwrap().is_empty());
        assert!(f.f.spool.pending().unwrap().is_empty());
        // Drop the caller after dispatch, before the runtime can acknowledge it.
    }
    published(&f, "history after scheduling caller loss").await;
    pause(&f).await;
    assert_eq!(f.imports.entries().unwrap().len(), 1);
    f.runtime.shutdown().await.unwrap();
}

#[tokio::test]
async fn accepted_source_refuses_other_request_without_replacing_original_owned_work() {
    let mut f = ImportFixture::new().await;
    f.f.message("history", "single retained import command");
    schedule(&f).await.unwrap();
    published(&f, "single retained import command").await;
    f.intent.request = RequestId::new([6; 16]);
    assert!(schedule(&f).await.is_err());
    pause(&f).await;
    let entries = f.imports.entries().unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].intent.request, RequestId::new([5; 16]));
    f.runtime.shutdown().await.unwrap();
}

#[tokio::test]
async fn multi_step_import_prefix_survives_worker_parking_without_starving_ingress() {
    let mut f = ImportFixture::new().await;
    // This deadline exercises prefix scheduling, not cold rule compilation.
    // Integration setup warms this same qualified cache before capture starts.
    jj_store_capture::agent_sanitization::warm_kingfisher_cache(&f.f.privacy().cache).unwrap();
    for _ in 0..5 {
        f.f.append_bytes(
            format!("{{\"unknown\":\"{}\"}}\n", "x".repeat(1024 * 1024 - 32)).as_bytes(),
        );
    }
    f.f.message("last", "history after multi step prefix");
    schedule(&f).await.unwrap();
    published(&f, "history after multi step prefix").await;
    pause(&f).await;
    let receipt = f.imports.entries().unwrap().remove(0);
    assert!(receipt.source.complete_byte_offset > 4 * 1024 * 1024);
    assert!(f.runtime.snapshot().status.resident_source_frames <= 2);
    f.runtime.shutdown().await.unwrap();
}

#[tokio::test]
async fn queued_dispatch_renews_revoked_installation_before_enrollment_or_body_work() {
    let mut f = ImportFixture::new().await;
    f.f.message("history", "withheld after installation revocation");
    let before = fs::read(f.f.root.path().join("sources/sources.json")).unwrap();
    let dispatch = dispatch(&f).await;
    let mut disabled = f.config.clone();
    disabled.agent_capture.enabled = false;
    f.save(&disabled);
    f.runtime
        .controls()
        .schedule_import(dispatch)
        .await
        .unwrap();
    tokio::time::timeout(Duration::from_secs(10), async {
        while f.runtime.snapshot().status.degraded_sources == 0 {
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .unwrap();
    pause(&f).await;
    assert!(f.imports.entries().unwrap().is_empty());
    assert!(f.f.spool.pending().unwrap().is_empty());
    assert_eq!(
        fs::read(f.f.root.path().join("sources/sources.json")).unwrap(),
        before
    );
    f.runtime.shutdown().await.unwrap();
}
