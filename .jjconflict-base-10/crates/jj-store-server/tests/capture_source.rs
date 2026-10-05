#![cfg(unix)]
mod capture_source {
    pub mod fixture;
}
use capture_source::fixture::*;
use jj_store_capture::agent_capture::AgentCaptureSpool;
use jj_store_server::capture_source::{CaptureSourceStep, CodexCaptureSource};
use jj_store_service::{auth::SystemClock, collaboration::AgentSessionPager, pagination};
use serde_json::json;
use std::{fs, sync::Arc};

#[tokio::test]
async fn admitted_codex_source_sanitizes_partitions_safe_gaps_and_resumes_offline_tail() {
    let mut fixture = Fixture::new(4 * 1024 * 1024).await;
    fixture.message("historical", "backfilled note");
    let active = fixture.start();
    fixture.message(
        "private",
        &format!(
            "token={SECRET} {}/src/file {}/home/private {}/temporary/file",
            fixture.workspace(A).display(),
            fixture.root.path().display(),
            fixture.root.path().display()
        ),
    );
    // Activity snapshot must be verified again after appending the actual body.
    let mut probe =
        jj_store_capture::codex_activity::CodexActivityProbe::arm(fixture.qualified()).unwrap();
    assert_eq!(
        probe.poll().unwrap(),
        jj_store_capture::codex_activity::ActivityPoll::Waiting
    );
    fixture
        .append(json!({"type":"event_msg","payload":{"type":"task_started","turn_id":"second"}}));
    assert_eq!(
        probe.poll().unwrap(),
        jj_store_capture::codex_activity::ActivityPoll::Active
    );
    assert!(active.revalidate().is_err());
    let sources = fixture.sources.clone();
    let privacy = fixture.privacy();
    let active = probe.proof().unwrap().unwrap();
    let mut source =
        CodexCaptureSource::acquire_active(sources, active, privacy, &mut fixture.context())
            .await
            .unwrap();
    let native = fixture
        .host
        .endpoint(A)
        .unwrap()
        .bind_local_trusted(
            fixture.authority.principal(),
            jj_store_service::auth::Permission::Read,
        )
        .unwrap();
    let heads = native.heads().await.unwrap();
    assert_eq!(
        source.step(&mut fixture.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    let partition = source.partition();
    let durable = fixture
        .spool
        .pending()
        .unwrap()
        .iter()
        .flat_map(|entry| &entry.batch.observations)
        .map(|observation| std::str::from_utf8(observation.payload.bytes()).unwrap())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(!durable.contains(SECRET));
    assert!(!durable.contains(fixture.root.path().to_str().unwrap()));
    for tag in ["$WORKSPACE", "$HOME", "$TEMP", "[REDACTED]"] {
        assert!(durable.contains(tag));
    }
    assert!(
        fixture
            .spool
            .latest_reader_progress(partition.integration, partition.source)
            .unwrap()
            .is_some()
    );
    fixture.catalog(&[]);
    let retained = fixture.spool.pending().unwrap();
    assert!(source.step(&mut fixture.context()).await.is_err());
    assert_eq!(fixture.spool.pending().unwrap(), retained);
    fixture.catalog(&[A, B]);
    assert!(
        matches!(source.step(&mut fixture.context()).await.unwrap(), CaptureSourceStep::Published(report) if report.committed==1)
    );
    fixture.append(json!({"type":"turn_context","payload":{"cwd":fixture.workspace(B)}}));
    fixture.append(json!({"type":"event_msg","payload":{"type":"task_started","turn_id":"b"}}));
    fixture.message("b", "second repository body");
    fixture.append_bytes(b"{invalid json}\n");
    fixture.message("withheld", "private unknown-context body");
    fixture.append(json!({"type":"turn_context","payload":{"cwd":fixture.workspace(A)}}));
    fixture.message("a-again", "valid after gap");
    fixture
        .append(json!({"type":"event_msg","payload":{"type":"task_complete","turn_id":"second"}}));
    for _ in 0..20 {
        let step = source.step(&mut fixture.context()).await.unwrap();
        if step == CaptureSourceStep::Waiting {
            break;
        }
    }
    assert!(fixture.spool.pending().unwrap().is_empty());
    let progress = fixture
        .spool
        .latest_reader_progress(partition.integration, partition.source)
        .unwrap()
        .unwrap();
    let position = serde_json::to_value(&progress.cursor.position).unwrap();
    assert_eq!(
        position["complete_byte_offset"],
        json!(fs::metadata(&fixture.transcript).unwrap().len())
    );
    let session = native.into_session();
    let pager =
        AgentSessionPager::new(pagination::Limits::default(), Arc::new(SystemClock)).unwrap();
    let page = pager.begin(session.clone(), 10).await.unwrap();
    assert_eq!(page.sessions.len(), 1);
    let detail = pager
        .detail(session.clone(), page.sessions[0].session_id)
        .await
        .unwrap();
    let body = serde_json::to_string(&detail).unwrap();
    assert!(body.contains("backfilled note"));
    assert!(body.contains("valid after gap"));
    assert!(!body.contains("second repository body"));
    assert!(!body.contains("private unknown-context body"));
    assert!(!body.contains(SECRET));
    assert!(!body.contains(fixture.root.path().to_str().unwrap()));
    let records: u64 = detail.transcripts.iter().map(|t| t.record_count).sum();
    let native_b = fixture
        .host
        .endpoint(B)
        .unwrap()
        .bind_local_trusted(
            fixture.authority.principal(),
            jj_store_service::auth::Permission::Read,
        )
        .unwrap()
        .into_session();
    let page_b = pager.begin(native_b.clone(), 10).await.unwrap();
    assert_eq!(page_b.sessions.len(), 1);
    let body_b = serde_json::to_string(
        &pager
            .detail(native_b, page_b.sessions[0].session_id)
            .await
            .unwrap(),
    )
    .unwrap();
    assert!(body_b.contains("second repository body"));
    assert!(!body_b.contains("backfilled note"));
    drop(source);
    fixture.message("offline", "final offline flush");
    fixture
        .append(json!({"type":"event_msg","payload":{"type":"task_complete","turn_id":"second"}}));
    let sources = fixture.sources.clone();
    let privacy = fixture.privacy();
    let qualified = fixture.qualified();
    let mut source =
        CodexCaptureSource::acquire_admitted(sources, qualified, privacy, &mut fixture.context())
            .await
            .unwrap()
            .unwrap();
    for _ in 0..20 {
        if source.step(&mut fixture.context()).await.unwrap() == CaptureSourceStep::Waiting {
            break;
        }
    }
    assert!(fixture.spool.pending().unwrap().is_empty());
    let detail = pager
        .detail(session.clone(), page.sessions[0].session_id)
        .await
        .unwrap();
    assert_eq!(
        detail
            .transcripts
            .iter()
            .map(|t| t.record_count)
            .sum::<u64>(),
        records + 1 // Selected transcript counts message chunks; lifecycle events are separate.
    );
    assert_eq!(
        serde_json::to_value(detail.session.state).unwrap(),
        json!("idle")
    );
    let final_progress = fixture
        .spool
        .latest_reader_progress(partition.integration, partition.source)
        .unwrap()
        .unwrap();
    assert_eq!(final_progress.last_sequence, progress.last_sequence + 2);
    assert_eq!(
        serde_json::to_value(&final_progress.cursor.position).unwrap()["complete_byte_offset"],
        json!(fs::metadata(&fixture.transcript).unwrap().len())
    );
    assert!(
        serde_json::to_string(&detail)
            .unwrap()
            .contains("final offline flush")
    );
    assert_eq!(
        fixture
            .host
            .endpoint(A)
            .unwrap()
            .bind_local_trusted(
                fixture.authority.principal(),
                jj_store_service::auth::Permission::Read
            )
            .unwrap()
            .heads()
            .await
            .unwrap(),
        heads
    );
}

#[tokio::test]
async fn spool_refusal_does_not_acknowledge_and_unadmitted_history_does_not_acquire() {
    let mut fixture = Fixture::new(128).await;
    let sources = fixture.sources.clone();
    let privacy = fixture.privacy();
    let qualified = fixture.qualified();
    assert!(
        CodexCaptureSource::acquire_admitted(sources, qualified, privacy, &mut fixture.context())
            .await
            .unwrap()
            .is_none()
    );
    let active = fixture.start();
    let sources = fixture.sources.clone();
    let privacy = fixture.privacy();
    let mut source =
        CodexCaptureSource::acquire_active(sources, active, privacy, &mut fixture.context())
            .await
            .unwrap();
    assert!(source.step(&mut fixture.context()).await.is_err());
    let partition = source.partition();
    assert!(fixture.spool.pending().unwrap().is_empty());
    assert!(
        fixture
            .spool
            .latest_reader_progress(partition.integration, partition.source)
            .unwrap()
            .is_none()
    );
    fixture.spool =
        AgentCaptureSpool::open(&fixture.root.path().join("spool"), 4 * 1024 * 1024).unwrap();
    assert_eq!(
        source.step(&mut fixture.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    assert!(
        matches!(source.step(&mut fixture.context()).await.unwrap(),CaptureSourceStep::Published(report) if report.committed==1)
    );
    assert!(fixture.spool.pending().unwrap().is_empty());
}

#[tokio::test]
async fn source_enrollment_requires_native_write_authority_and_pending_restart_requires_proof_recovery()
 {
    let mut fixture = Fixture::new(4 * 1024 * 1024).await;
    let active = fixture.start();
    fixture.without_native_authority();
    let state = fixture.root.path().join("sources");
    let before = fs::read(state.join("sources.json")).unwrap();
    let sources = fixture.sources.clone();
    let privacy = fixture.privacy();
    assert!(
        CodexCaptureSource::acquire_active(sources, active, privacy, &mut fixture.context())
            .await
            .is_err()
    );
    assert_eq!(fs::read(state.join("sources.json")).unwrap(), before);
    assert_eq!(fs::read_dir(state).unwrap().count(), 2);
    assert!(!fixture.root.path().join("rules").exists());

    let mut fixture = Fixture::new(4 * 1024 * 1024).await;
    let active = fixture.start();
    let sources = fixture.sources.clone();
    let privacy = fixture.privacy();
    let mut source =
        CodexCaptureSource::acquire_active(sources, active, privacy, &mut fixture.context())
            .await
            .unwrap();
    assert_eq!(
        source.step(&mut fixture.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    let retained = fixture.spool.pending().unwrap();
    drop(source);
    let sources = fixture.sources.clone();
    let privacy = fixture.privacy();
    let qualified = fixture.qualified();
    let mut resumed =
        CodexCaptureSource::acquire_admitted(sources, qualified, privacy, &mut fixture.context())
            .await
            .unwrap()
            .unwrap();
    assert_eq!(fixture.spool.pending().unwrap(), retained);
    fixture.catalog(&[]);
    assert_eq!(
        resumed.step(&mut fixture.context()).await.unwrap(),
        CaptureSourceStep::Withheld
    );
    assert_eq!(fixture.spool.pending().unwrap(), retained);
    fixture.catalog(&[A, B]);
    assert!(
        matches!(resumed.step(&mut fixture.context()).await.unwrap(), CaptureSourceStep::Published(report) if report.committed == 1)
    );
}

#[tokio::test]
async fn changed_spool_binding_never_clears_a_retained_source_intent() {
    let mut fixture = Fixture::new(4 * 1024 * 1024).await;
    let active = fixture.start();
    let sources = fixture.sources.clone();
    let privacy = fixture.privacy();
    let mut source =
        CodexCaptureSource::acquire_active(sources, active, privacy, &mut fixture.context())
            .await
            .unwrap();
    assert_eq!(
        source.step(&mut fixture.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    let retained = fixture.spool.pending().unwrap();
    fixture.spool =
        AgentCaptureSpool::open(&fixture.root.path().join("other-spool"), 4 * 1024 * 1024).unwrap();
    assert!(source.step(&mut fixture.context()).await.is_err());
    assert!(fixture.spool.pending().unwrap().is_empty());
    fixture.spool =
        AgentCaptureSpool::open(&fixture.root.path().join("spool"), 4 * 1024 * 1024).unwrap();
    assert_eq!(fixture.spool.pending().unwrap(), retained);
    assert!(
        matches!(source.step(&mut fixture.context()).await.unwrap(),CaptureSourceStep::Published(report) if report.committed==1)
    );
    fixture.message("later", "retained after physical root replacement");
    assert_eq!(
        source.step(&mut fixture.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    let retained = fixture.spool.pending().unwrap();
    let original = fixture.root.path().join("spool");
    let retired = fixture.root.path().join("retired-spool");
    fs::rename(&original, &retired).unwrap();
    fixture.spool = AgentCaptureSpool::open(&original, 4 * 1024 * 1024).unwrap();
    assert!(source.step(&mut fixture.context()).await.is_err());
    assert!(fixture.spool.pending().unwrap().is_empty());
    assert_eq!(
        AgentCaptureSpool::open(&retired, 4 * 1024 * 1024)
            .unwrap()
            .pending()
            .unwrap(),
        retained
    );
}
