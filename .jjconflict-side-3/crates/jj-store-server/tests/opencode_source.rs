#![cfg(unix)]
#[path = "capture_source/fixture.rs"]
#[allow(dead_code)]
mod native;
#[path = "opencode_source/scheduling.rs"]
mod scheduling;
mod opencode_source {
    pub mod fixture;
}
use jj_store_capture::agent_capture::AgentCaptureSpool;
use jj_store_capture::opencode_ingress::IntegrationId;
use jj_store_server::{capture_source::CaptureSourceStep, opencode_source::OpenCodeCaptureSource};
use native::{A, B, Fixture, SECRET};
use opencode_source::fixture::{Api, message};
#[tokio::test]
async fn api_ingestion_partitions_sanitizes_publishes_and_resumes_exact_offline_history() {
    let mut f = Fixture::new(4 * 1024 * 1024).await;
    let a = f.workspace(A).to_str().unwrap().to_owned();
    let b = f.workspace(B).to_str().unwrap().to_owned();
    let api = Api::new(
        a.clone(),
        vec![
            message("msg_a", Some(&a), &format!("{SECRET} {a}/src")),
            message("msg_b", Some(&b), "second repository"),
            message("msg_gap", None, SECRET),
        ],
        true,
    )
    .await;
    let native_a = f
        .host
        .endpoint(A)
        .unwrap()
        .bind_local_trusted(
            f.authority.principal(),
            jj_store_service::auth::Permission::Read,
        )
        .unwrap();
    let code_heads = native_a.heads().await.unwrap();
    let mut source = OpenCodeCaptureSource::acquire(
        api.api.clone(),
        f.sources.clone(),
        "ses_first",
        f.privacy(),
        &mut f.context(),
    )
    .await
    .unwrap()
    .unwrap();
    let id = source.source_instance_id();
    assert!(
        OpenCodeCaptureSource::acquire(
            api.api.clone(),
            f.sources.clone(),
            "ses_first",
            f.privacy(),
            &mut f.context()
        )
        .await
        .is_err()
    );
    for _ in 0..3 {
        assert_eq!(
            source.ingest_step(&mut f.context()).await.unwrap(),
            CaptureSourceStep::Enqueued
        );
    }
    let pending = f.spool.pending().unwrap();
    assert_eq!(pending.len(), 3);
    let repo_b = pending.iter().filter(|e| e.repository == B).count();
    assert_eq!(repo_b, 1);
    for entry in &pending {
        let text = std::str::from_utf8(entry.batch.observations[0].payload.bytes()).unwrap();
        assert!(!text.contains(SECRET));
        assert!(!text.contains(&a));
        assert!(!text.contains(&b));
    }
    let before = f
        .spool
        .latest_reader_progress(IntegrationId::OpenCode, id)
        .unwrap()
        .unwrap();
    drop(source);
    drop(f.spool);
    f.spool = AgentCaptureSpool::open(&f.root.path().join("spool"), 4 * 1024 * 1024).unwrap();
    api.data.lock().unwrap().busy = false;
    let mut source = OpenCodeCaptureSource::acquire(
        api.api.clone(),
        f.sources.clone(),
        "ses_first",
        f.privacy(),
        &mut f.context(),
    )
    .await
    .unwrap()
    .unwrap();
    assert_eq!(source.source_instance_id(), id);
    for _ in 0..3 {
        assert_eq!(
            source.ingest_step(&mut f.context()).await.unwrap(),
            CaptureSourceStep::Verifying
        );
    }
    assert_eq!(f.spool.pending().unwrap(), pending);
    assert_eq!(
        f.spool
            .latest_reader_progress(IntegrationId::OpenCode, id)
            .unwrap(),
        Some(before.clone())
    );
    for _ in 0..3 {
        assert!(
            matches!(source.publish_step(&mut f.context()).await.unwrap(),CaptureSourceStep::Published(report) if report.committed==1)
        );
    }
    assert!(f.spool.pending().unwrap().is_empty());
    let pager = jj_store_service::collaboration::AgentSessionPager::new(
        jj_store_service::pagination::Limits::default(),
        std::sync::Arc::new(jj_store_service::auth::SystemClock),
    )
    .unwrap();
    for (repository, required, excluded) in [
        (A, "[REDACTED]", "second repository"),
        (B, "second repository", "[REDACTED]"),
    ] {
        let native = f
            .host
            .endpoint(repository)
            .unwrap()
            .bind_local_trusted(
                f.authority.principal(),
                jj_store_service::auth::Permission::Read,
            )
            .unwrap()
            .into_session();
        let page = pager.begin(native.clone(), 10).await.unwrap();
        assert_eq!(page.sessions.len(), 1);
        let detail = pager
            .detail(native, page.sessions[0].session_id)
            .await
            .unwrap();
        let body = serde_json::to_string(&detail).unwrap();
        assert!(body.contains(required));
        assert!(!body.contains(excluded));
        assert!(!body.contains(SECRET));
        assert!(!body.contains(&a));
        assert!(!body.contains(&b));
        // Unbound omissions select source coverage, without inventing session activity.
        assert_eq!(
            detail
                .transcripts
                .iter()
                .map(|t| t.record_count)
                .sum::<u64>(),
            1
        );
    }
    assert_eq!(native_a.heads().await.unwrap(), code_heads);
    assert_eq!(
        source.ingest_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Waiting
    );
    api.data
        .lock()
        .unwrap()
        .messages
        .push(message("msg_offline", Some(&a), "offline tail"));
    source.refresh(&mut f.context()).unwrap();
    for _ in 0..3 {
        assert_eq!(
            source.ingest_step(&mut f.context()).await.unwrap(),
            CaptureSourceStep::Verifying
        );
    }
    assert_eq!(
        source.ingest_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    let after = f
        .spool
        .latest_reader_progress(IntegrationId::OpenCode, id)
        .unwrap()
        .unwrap();
    assert_eq!(after.last_sequence, before.last_sequence + 1);
    assert!(
        matches!(source.publish_step(&mut f.context()).await.unwrap(),CaptureSourceStep::Published(report) if report.committed==1)
    );
}
#[tokio::test]
async fn inactive_history_revocation_and_spool_pressure_never_grant_or_advance_admission() {
    let mut f = Fixture::new(128).await;
    let a = f.workspace(A).to_str().unwrap().to_owned();
    let api = Api::new(
        a.clone(),
        vec![message("msg_first", Some(&a), "record")],
        false,
    )
    .await;
    assert!(
        OpenCodeCaptureSource::acquire(
            api.api.clone(),
            f.sources.clone(),
            "ses_first",
            f.privacy(),
            &mut f.context()
        )
        .await
        .unwrap()
        .is_none()
    );
    api.data.lock().unwrap().busy = true;
    f.catalog(&[]);
    assert!(
        OpenCodeCaptureSource::acquire(
            api.api.clone(),
            f.sources.clone(),
            "ses_first",
            f.privacy(),
            &mut f.context()
        )
        .await
        .is_err()
    );
    f.catalog(&[A]);
    let mut source = OpenCodeCaptureSource::acquire(
        api.api.clone(),
        f.sources.clone(),
        "ses_first",
        f.privacy(),
        &mut f.context(),
    )
    .await
    .unwrap()
    .unwrap();
    let id = source.source_instance_id();
    assert!(source.ingest_step(&mut f.context()).await.is_err());
    assert!(f.spool.pending().unwrap().is_empty());
    assert!(
        f.spool
            .latest_reader_progress(IntegrationId::OpenCode, id)
            .unwrap()
            .is_none()
    );
    drop(f.spool);
    f.spool = AgentCaptureSpool::open(&f.root.path().join("spool"), 4 * 1024 * 1024).unwrap();
    // A retained source cannot silently switch spool ownership/capacity binding.
    assert!(source.ingest_step(&mut f.context()).await.is_err());
    drop(source);
    let mut source = OpenCodeCaptureSource::acquire(
        api.api.clone(),
        f.sources.clone(),
        "ses_first",
        f.privacy(),
        &mut f.context(),
    )
    .await
    .unwrap()
    .unwrap();
    assert_eq!(
        source.ingest_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    let pending = f.spool.pending().unwrap();
    f.catalog(&[]);
    assert!(source.publish_step(&mut f.context()).await.is_err());
    assert_eq!(f.spool.pending().unwrap(), pending);
    f.catalog(&[A]);
    assert!(
        matches!(source.publish_step(&mut f.context()).await.unwrap(),CaptureSourceStep::Published(report) if report.committed==1)
    );
}
#[tokio::test]
async fn rewritten_snapshots_restart_a_generation_and_withhold_old_pending_intents() {
    let mut f = Fixture::new(4 * 1024 * 1024).await;
    let a = f.workspace(A).to_str().unwrap().to_owned();
    let api = Api::new(a.clone(), vec![message("msg_first", Some(&a), "old")], true).await;
    let mut source = OpenCodeCaptureSource::acquire(
        api.api.clone(),
        f.sources.clone(),
        "ses_first",
        f.privacy(),
        &mut f.context(),
    )
    .await
    .unwrap()
    .unwrap();
    let id = source.source_instance_id();
    assert_eq!(
        source.ingest_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    let old = f.spool.pending().unwrap();
    drop(source);
    api.data.lock().unwrap().messages[0] = message("msg_first", Some(&a), "rewritten");
    let mut source = OpenCodeCaptureSource::acquire(
        api.api.clone(),
        f.sources.clone(),
        "ses_first",
        f.privacy(),
        &mut f.context(),
    )
    .await
    .unwrap()
    .unwrap();
    assert_eq!(
        source.ingest_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::ChangedDuringRead
    );
    assert_eq!(f.spool.pending().unwrap(), old);
    assert_eq!(
        source.publish_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Withheld
    );
    assert_eq!(
        source.ingest_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    let progress = f
        .spool
        .latest_reader_progress(IntegrationId::OpenCode, id)
        .unwrap()
        .unwrap();
    assert_eq!(progress.cursor.generation, 2);
    assert_eq!(progress.last_sequence, 2);
    assert_eq!(
        source.publish_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Withheld
    );
}

#[tokio::test]
async fn a_changed_queued_prefix_starts_reconstruction_without_skipping_to_the_last_record() {
    let mut f = Fixture::new(4 * 1024 * 1024).await;
    let a = f.workspace(A).to_str().unwrap().to_owned();
    let api = Api::new(
        a.clone(),
        vec![
            message("msg_first", Some(&a), "original"),
            message("msg_second", Some(&a), "retained"),
        ],
        true,
    )
    .await;
    let mut source = OpenCodeCaptureSource::acquire(
        api.api.clone(),
        f.sources.clone(),
        "ses_first",
        f.privacy(),
        &mut f.context(),
    )
    .await
    .unwrap()
    .unwrap();
    for _ in 0..2 {
        assert_eq!(
            source.ingest_step(&mut f.context()).await.unwrap(),
            CaptureSourceStep::Enqueued
        );
    }
    let original = f.spool.pending().unwrap();
    drop(source);
    api.data.lock().unwrap().messages[0] = message("msg_first", Some(&a), "rewritten first row");
    let mut source = OpenCodeCaptureSource::acquire(
        api.api.clone(),
        f.sources.clone(),
        "ses_first",
        f.privacy(),
        &mut f.context(),
    )
    .await
    .unwrap()
    .unwrap();
    assert_eq!(
        source.ingest_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::ChangedDuringRead
    );
    assert_eq!(f.spool.pending().unwrap(), original);
    assert_eq!(
        source.publish_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Withheld
    );
}

#[tokio::test]
async fn native_write_and_changed_api_binding_withhold_allocation_and_publication() {
    let mut f = Fixture::new(4 * 1024 * 1024).await;
    let a = f.workspace(A).to_str().unwrap().to_owned();
    let api = Api::new(
        a.clone(),
        vec![message("msg_first", Some(&a), "bound evidence")],
        true,
    )
    .await;
    let registry_before = std::fs::read(f.root.path().join("sources/sources.json")).unwrap();
    let original_host = f.host.clone();
    f.without_native_authority();
    assert!(
        OpenCodeCaptureSource::acquire(
            api.api.clone(),
            f.sources.clone(),
            "ses_first",
            f.privacy(),
            &mut f.context()
        )
        .await
        .is_err()
    );
    assert_eq!(
        std::fs::read(f.root.path().join("sources/sources.json")).unwrap(),
        registry_before
    );
    assert!(f.spool.pending().unwrap().is_empty());
    f.host = original_host;
    let mut source = OpenCodeCaptureSource::acquire(
        api.api.clone(),
        f.sources.clone(),
        "ses_first",
        f.privacy(),
        &mut f.context(),
    )
    .await
    .unwrap()
    .unwrap();
    assert_eq!(
        source.ingest_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    let pending = f.spool.pending().unwrap();
    api.data.lock().unwrap().directory = format!("{a}/rebound");
    assert!(source.publish_step(&mut f.context()).await.is_err());
    assert_eq!(f.spool.pending().unwrap(), pending);
    api.data.lock().unwrap().directory = a;
    assert!(
        matches!(source.publish_step(&mut f.context()).await.unwrap(), CaptureSourceStep::Published(report) if report.committed == 1)
    );
}
