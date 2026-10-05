#![cfg(unix)]
#[path = "capture_source/fixture.rs"]
mod fixture;
use fixture::*;
use jj_store_capture::codex_activity::{ActivityPoll, CodexActivityProbe};
use jj_store_server::capture_source::{
    CaptureResumeStep, CaptureSourceStep, CodexCaptureResume, CodexCaptureSource,
};
use serde_json::json;

#[tokio::test]
async fn fresh_start_admits_current_directory_after_event_directory_revocation() {
    let mut f = Fixture::new(4 * 1024 * 1024).await;
    f.message("history-a", "old repository A body");
    let mut activity = CodexActivityProbe::arm(f.qualified()).unwrap();
    assert_eq!(activity.poll().unwrap(), ActivityPoll::Waiting);
    f.append(json!({"type":"event_msg","payload":{"type":"task_started","turn_id":"active-a"}}));
    assert_eq!(activity.poll().unwrap(), ActivityPoll::Active);
    f.append(json!({"type":"turn_context","payload":{"cwd":f.workspace(B)}}));
    f.message("current-b", &format!("current repository B body {SECRET}"));
    assert_eq!(activity.poll().unwrap(), ActivityPoll::Active);
    let active = activity.proof().unwrap().unwrap();
    assert_eq!(active.directory(), f.workspace(A));
    f.catalog(&[B]);
    let sources = f.sources.clone();
    let privacy = f.privacy();
    assert!(
        CodexCaptureSource::acquire_active(
            sources.clone(),
            active,
            privacy.clone(),
            &mut f.context()
        )
        .await
        .is_err()
    );
    let qualified = f.qualified();
    assert!(!sources.previously_admitted(&qualified).unwrap());
    let before = std::fs::read(f.root.path().join("sources/sources.json")).unwrap();
    let active = activity.proof().unwrap().unwrap();
    let mut activation =
        CodexCaptureResume::prepare_active(sources.clone(), active, privacy, &f.spool).unwrap();
    assert_eq!(
        std::fs::read(f.root.path().join("sources/sources.json")).unwrap(),
        before
    );
    assert!(f.spool.pending().unwrap().is_empty());
    let CaptureResumeStep::Ready(source) = activation.step(&mut f.context()).await.unwrap() else {
        panic!("current B not admitted");
    };
    assert!(activation.step(&mut f.context()).await.is_err());
    assert!(sources.previously_admitted(&qualified).unwrap());
    let mut pipeline = source.into_pipeline();
    assert_eq!(pipeline.partition().repository, B);
    assert_eq!(
        pipeline.ingest_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    assert_eq!(
        pipeline.ingest_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    let mut entries = f.spool.pending().unwrap();
    entries.sort_by_key(|entry| entry.batch.observations[0].source_sequence);
    assert_eq!(entries.len(), 2);
    assert!(entries.iter().all(|entry| entry.repository == B));
    let body = entries
        .iter()
        .flat_map(|entry| &entry.batch.observations)
        .map(|observation| std::str::from_utf8(observation.payload.bytes()).unwrap())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(body.contains("current repository B body"));
    assert!(!body.contains("old repository A body"));
    assert!(!body.contains(SECRET));
    assert!(body.contains("[REDACTED]"));
    for entry in &entries {
        for _ in 0..4 {
            if matches!(pipeline.publish_step(entry, &mut f.context()).await.unwrap(), CaptureSourceStep::Published(report) if report.committed == 1)
            {
                break;
            }
        }
    }
    assert!(f.spool.pending().unwrap().is_empty());
}

#[tokio::test]
async fn fresh_preparation_is_bounded_and_does_not_enroll_before_current_write() {
    let mut f = Fixture::new(4 * 1024 * 1024).await;
    let mut activity = CodexActivityProbe::arm(f.qualified()).unwrap();
    assert_eq!(activity.poll().unwrap(), ActivityPoll::Waiting);
    f.append(json!({"type":"event_msg","payload":{"type":"task_started"}}));
    assert_eq!(activity.poll().unwrap(), ActivityPoll::Active);
    for _ in 0..7 {
        f.append(json!({"padding":"x".repeat(800 * 1024)}));
    }
    f.append(json!({"type":"turn_context","payload":{"cwd":f.workspace(B)}}));
    f.append_bytes(br#"{"type":"turn_context","payload":{"cwd":"/unbound/partial"#);
    assert_eq!(activity.poll().unwrap(), ActivityPoll::Active);
    f.catalog(&[B]);
    let sources = f.sources.clone();
    let before = std::fs::read(f.root.path().join("sources/sources.json")).unwrap();
    let mut activation = CodexCaptureResume::prepare_active(
        sources.clone(),
        activity.proof().unwrap().unwrap(),
        f.privacy(),
        &f.spool,
    )
    .unwrap();
    assert!(matches!(
        activation.step(&mut f.context()).await.unwrap(),
        CaptureResumeStep::Verifying
    ));
    assert_eq!(
        std::fs::read(f.root.path().join("sources/sources.json")).unwrap(),
        before
    );
    let host = f.host.clone();
    f.without_native_authority();
    assert!(activation.step(&mut f.context()).await.is_err());
    assert_eq!(
        std::fs::read(f.root.path().join("sources/sources.json")).unwrap(),
        before
    );
    assert!(!sources.previously_admitted(&f.qualified()).unwrap());
    f.host = host;
    let CaptureResumeStep::Ready(source) = activation.step(&mut f.context()).await.unwrap() else {
        panic!("current complete B directory not admitted");
    };
    assert_eq!(source.partition().repository, B);
    assert!(f.spool.pending().unwrap().is_empty());
    assert!(
        f.spool
            .latest_reader_progress(source.partition().integration, source.partition().source)
            .unwrap()
            .is_none()
    );
}

#[tokio::test]
async fn changed_activity_snapshot_requires_renewal_from_the_original_probe() {
    let mut f = Fixture::new(4 * 1024 * 1024).await;
    let mut activity = CodexActivityProbe::arm(f.qualified()).unwrap();
    assert_eq!(activity.poll().unwrap(), ActivityPoll::Waiting);
    f.append(json!({"type":"event_msg","payload":{"type":"task_started"}}));
    assert_eq!(activity.poll().unwrap(), ActivityPoll::Active);
    let active = activity.proof().unwrap().unwrap();
    let sources = f.sources.clone();
    let before = std::fs::read(f.root.path().join("sources/sources.json")).unwrap();
    let mut activation =
        CodexCaptureResume::prepare_active(sources.clone(), active.clone(), f.privacy(), &f.spool)
            .unwrap();
    f.append(json!({"type":"turn_context","payload":{"cwd":f.workspace(B)}}));
    assert!(activation.step(&mut f.context()).await.is_err());
    assert!(
        CodexCaptureResume::prepare_active(sources.clone(), active, f.privacy(), &f.spool).is_err()
    );
    assert_eq!(
        std::fs::read(f.root.path().join("sources/sources.json")).unwrap(),
        before
    );
    assert!(!sources.previously_admitted(&f.qualified()).unwrap());
    // A newly armed probe cannot inherit the older genuine start.
    let mut historical = CodexActivityProbe::arm(f.qualified()).unwrap();
    assert_eq!(historical.poll().unwrap(), ActivityPoll::Waiting);
    assert!(historical.proof().unwrap().is_none());
    // The original probe verifies its complete prefix and renews that start
    // against the current snapshot without inventing a new start event.
    assert_eq!(activity.poll().unwrap(), ActivityPoll::Active);
    f.catalog(&[B]);
    let mut activation = CodexCaptureResume::prepare_active(
        sources,
        activity.proof().unwrap().unwrap(),
        f.privacy(),
        &f.spool,
    )
    .unwrap();
    let CaptureResumeStep::Ready(source) = activation.step(&mut f.context()).await.unwrap() else {
        panic!("renewed current snapshot not admitted");
    };
    assert_eq!(source.partition().repository, B);
    assert!(f.spool.pending().unwrap().is_empty());
}

#[tokio::test]
async fn fresh_start_cannot_enroll_an_unbound_directory_or_use_another_spool() {
    let mut f = Fixture::new(4 * 1024 * 1024).await;
    let active = f.start();
    let sources = f.sources.clone();
    let before = std::fs::read(f.root.path().join("sources/sources.json")).unwrap();
    let mut activation =
        CodexCaptureResume::prepare_active(sources.clone(), active, f.privacy(), &f.spool).unwrap();
    f.catalog(&[]);
    assert!(matches!(
        activation.step(&mut f.context()).await.unwrap(),
        CaptureResumeStep::Withheld
    ));
    assert_eq!(
        std::fs::read(f.root.path().join("sources/sources.json")).unwrap(),
        before
    );
    f.catalog(&[A, B]);
    let original = std::mem::replace(
        &mut f.spool,
        jj_store_capture::agent_capture::AgentCaptureSpool::open(
            &f.root.path().join("foreign-spool"),
            4 * 1024 * 1024,
        )
        .unwrap(),
    );
    assert!(activation.step(&mut f.context()).await.is_err());
    assert_eq!(
        std::fs::read(f.root.path().join("sources/sources.json")).unwrap(),
        before
    );
    f.spool = original;
    let CaptureResumeStep::Ready(source) = activation.step(&mut f.context()).await.unwrap() else {
        panic!("restored binding unavailable");
    };
    assert_eq!(source.partition().repository, A);
    assert!(f.spool.pending().unwrap().is_empty());
}
