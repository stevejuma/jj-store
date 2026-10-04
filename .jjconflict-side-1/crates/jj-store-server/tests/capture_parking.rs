#![cfg(unix)]
#[path = "capture_source/fixture.rs"]
mod fixture;
#[path = "capture_parking/generations.rs"]
mod generations;
use fixture::*;
use jj_store_server::capture_source::{
    CaptureResumeStep, CaptureSourceStep, CodexCaptureResume, CodexCaptureSource,
};
use serde_json::json;

#[tokio::test]
async fn parked_pipeline_retains_verification_and_releases_leases() {
    let mut f = Fixture::new(16 * 1024 * 1024).await;
    let active = f.start();
    let sources = f.sources.clone();
    let privacy = f.privacy();
    let mut pipeline = CodexCaptureSource::acquire_active(
        sources.clone(),
        active,
        privacy.clone(),
        &mut f.context(),
    )
    .await
    .unwrap()
    .into_pipeline();
    let oversized = vec![b'x'; 3 * 1024 * 1024];
    for _ in 0..3 {
        f.append_bytes(&oversized);
        f.append_bytes(b"\n");
    }
    f.append(json!({"type":"turn_context","payload":{"cwd":f.workspace(A)}}));
    for _ in 0..24 {
        pipeline.ingest_step(&mut f.context()).await.unwrap();
    }
    let partition = pipeline.partition();
    let progress = f
        .spool
        .latest_reader_progress(partition.integration, partition.source)
        .unwrap()
        .unwrap();
    drop(pipeline);
    let qualified = f.qualified();
    let mut resume =
        CodexCaptureResume::prepare(sources.clone(), qualified.clone(), privacy, &f.spool)
            .unwrap()
            .unwrap();
    let mut pipeline = loop {
        match resume.step(&mut f.context()).await.unwrap() {
            CaptureResumeStep::Ready(source) => break source.into_pipeline(),
            CaptureResumeStep::Verifying => {}
            _ => panic!("resume unexpectedly withheld"),
        }
    };
    assert_eq!(
        pipeline.ingest_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Verifying
    );
    let mut parked = pipeline.park();
    // A parked frame grants no lease: another admitted owner can acquire it.
    drop(
        sources
            .acquire_admitted(qualified.clone())
            .unwrap()
            .unwrap(),
    );
    let mut pipeline = parked
        .resume(sources.clone(), &mut f.context())
        .await
        .unwrap()
        .unwrap();
    assert!(
        parked
            .resume(sources.clone(), &mut f.context())
            .await
            .is_err()
    );
    assert!(sources.acquire_admitted(qualified.clone()).is_err());
    assert_eq!(
        pipeline.ingest_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Verifying
    );
    let mut parked = pipeline.park();
    let mut pipeline = parked
        .resume(sources, &mut f.context())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        pipeline.ingest_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Waiting
    );
    assert_eq!(
        f.spool
            .latest_reader_progress(partition.integration, partition.source)
            .unwrap()
            .unwrap(),
        progress
    );
    f.message("after-verification", "retained reader can advance");
    // Appending requires renewed bounded prefix verification, even after parking.
    for _ in 0..4 {
        if pipeline.ingest_step(&mut f.context()).await.unwrap() == CaptureSourceStep::Enqueued {
            return;
        }
    }
    panic!("reader did not advance after verification");
}

#[tokio::test]
async fn parked_backpressure_keeps_the_exact_batch_and_rechecks_admission() {
    let mut f = Fixture::new(1).await;
    f.message("original", &format!("first pending body {SECRET}"));
    let active = f.start();
    let sources = f.sources.clone();
    let privacy = f.privacy();
    let mut pipeline =
        CodexCaptureSource::acquire_active(sources.clone(), active, privacy, &mut f.context())
            .await
            .unwrap()
            .into_pipeline();
    let end = std::fs::metadata(&f.transcript).unwrap().len();
    assert!(pipeline.ingest_step(&mut f.context()).await.is_err());
    let partition = pipeline.partition();
    assert!(f.spool.pending().unwrap().is_empty());
    let mut parked = pipeline.park();
    f.message("later", "later append must not enter retained batch");
    f.catalog(&[]);
    assert!(
        parked
            .resume(sources.clone(), &mut f.context())
            .await
            .is_err()
    );
    assert!(
        f.spool
            .latest_reader_progress(partition.integration, partition.source)
            .unwrap()
            .is_none()
    );
    f.catalog(&[A, B]);
    let host = f.host.clone();
    f.without_native_authority();
    assert!(
        parked
            .resume(sources.clone(), &mut f.context())
            .await
            .is_err()
    );
    f.host = host;
    let original = std::mem::replace(
        &mut f.spool,
        jj_store_capture::agent_capture::AgentCaptureSpool::open(
            &f.root.path().join("foreign-spool"),
            4 * 1024 * 1024,
        )
        .unwrap(),
    );
    assert!(
        parked
            .resume(sources.clone(), &mut f.context())
            .await
            .is_err()
    );
    f.spool = original;
    f.spool = jj_store_capture::agent_capture::AgentCaptureSpool::open(
        &f.root.path().join("spool"),
        4 * 1024 * 1024,
    )
    .unwrap();
    let mut pipeline = parked
        .resume(sources, &mut f.context())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        pipeline.ingest_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    let entries = f.spool.pending().unwrap();
    assert_eq!(entries.len(), 1);
    let encoded = entries
        .iter()
        .flat_map(|entry| &entry.batch.observations)
        .map(|observation| std::str::from_utf8(observation.payload.bytes()).unwrap())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(encoded.contains("first pending body"));
    assert!(!encoded.contains(SECRET));
    assert!(encoded.contains("[REDACTED]"));
    assert!(!encoded.contains(f.root.path().to_str().unwrap()));
    assert!(!encoded.contains("later append"));
    let position = serde_json::to_value(&entries[0].cursor.position).unwrap();
    assert_eq!(position["complete_byte_offset"].as_u64(), Some(end));
}

#[tokio::test]
async fn another_owner_advancing_the_cursor_reopens_from_durable_progress() {
    let mut f = Fixture::new(4 * 1024 * 1024).await;
    let active = f.start();
    let sources = f.sources.clone();
    let privacy = f.privacy();
    let pipeline = CodexCaptureSource::acquire_active(
        sources.clone(),
        active,
        privacy.clone(),
        &mut f.context(),
    )
    .await
    .unwrap()
    .into_pipeline();
    let partition = pipeline.partition();
    let mut parked = pipeline.park();
    f.message("other-owner", "durably accepted by another owner");
    let mut other = CodexCaptureSource::acquire_admitted(
        sources.clone(),
        f.qualified(),
        privacy,
        &mut f.context(),
    )
    .await
    .unwrap()
    .unwrap()
    .into_pipeline();
    assert_eq!(
        other.ingest_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    let progress = f
        .spool
        .latest_reader_progress(partition.integration, partition.source)
        .unwrap()
        .unwrap();
    drop(other);
    let mut pipeline = parked
        .resume(sources, &mut f.context())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        pipeline.ingest_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Waiting
    );
    assert_eq!(f.spool.pending().unwrap().len(), 1);
    assert_eq!(
        f.spool
            .latest_reader_progress(partition.integration, partition.source)
            .unwrap()
            .unwrap(),
        progress
    );
}

#[tokio::test]
async fn replaced_source_cannot_reuse_a_parked_frame() {
    let mut f = Fixture::new(4 * 1024 * 1024).await;
    let active = f.start();
    let sources = f.sources.clone();
    let privacy = f.privacy();
    let pipeline =
        CodexCaptureSource::acquire_active(sources.clone(), active, privacy, &mut f.context())
            .await
            .unwrap()
            .into_pipeline();
    let mut parked = pipeline.park();
    let old = f.transcript.with_extension("old");
    std::fs::rename(&f.transcript, &old).unwrap();
    std::fs::copy(&old, &f.transcript).unwrap();
    assert!(
        parked
            .resume(sources.clone(), &mut f.context())
            .await
            .is_err()
    );
    // Restoring the exact physical source permits the still-retained frame.
    std::fs::remove_file(&f.transcript).unwrap();
    std::fs::rename(old, &f.transcript).unwrap();
    assert!(
        parked
            .resume(sources, &mut f.context())
            .await
            .unwrap()
            .is_some()
    );
}

#[tokio::test]
async fn missing_or_regressive_durable_progress_never_resets_a_parked_reader() {
    let mut f = Fixture::new(4 * 1024 * 1024).await;
    let active = f.start();
    let sources = f.sources.clone();
    let privacy = f.privacy();
    let mut pipeline =
        CodexCaptureSource::acquire_active(sources.clone(), active, privacy, &mut f.context())
            .await
            .unwrap()
            .into_pipeline();
    assert_eq!(
        pipeline.ingest_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    let entry = f.spool.pending().unwrap().remove(0);
    for _ in 0..4 {
        if matches!(pipeline.publish_step(&entry, &mut f.context()).await.unwrap(), CaptureSourceStep::Published(report) if report.committed == 1)
        {
            break;
        }
    }
    assert!(f.spool.pending().unwrap().is_empty());
    let journal = std::fs::read_dir(f.root.path().join("spool/reader-progress"))
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    let old = std::fs::read(&journal).unwrap();
    f.message("second", "a second durable cursor");
    assert_eq!(
        pipeline.ingest_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    let entry = f.spool.pending().unwrap().remove(0);
    for _ in 0..4 {
        if matches!(pipeline.publish_step(&entry, &mut f.context()).await.unwrap(), CaptureSourceStep::Published(report) if report.committed == 1)
        {
            break;
        }
    }
    assert!(f.spool.pending().unwrap().is_empty());
    let current = std::fs::read(&journal).unwrap();
    assert_ne!(old, current);
    let qualified = f.qualified();
    let mut parked = pipeline.park();
    std::fs::remove_file(&journal).unwrap();
    assert!(
        parked
            .resume(sources.clone(), &mut f.context())
            .await
            .is_err()
    );
    std::fs::write(&journal, &old).unwrap();
    std::fs::set_permissions(
        &journal,
        std::os::unix::fs::PermissionsExt::from_mode(0o600),
    )
    .unwrap();
    assert!(
        parked
            .resume(sources.clone(), &mut f.context())
            .await
            .is_err()
    );
    let mut changed: serde_json::Value = serde_json::from_slice(&current).unwrap();
    changed["last_batch"] = json!("09".repeat(16));
    std::fs::write(&journal, serde_json::to_vec(&changed).unwrap()).unwrap();
    assert!(
        parked
            .resume(sources.clone(), &mut f.context())
            .await
            .is_err()
    );
    // Each failed handoff has released any temporary acquisition and retained
    // the original frame. Restoring authoritative progress permits it once.
    drop(sources.acquire_admitted(qualified).unwrap().unwrap());
    std::fs::write(&journal, current).unwrap();
    let mut pipeline = parked
        .resume(sources, &mut f.context())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        pipeline.ingest_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Waiting
    );
    assert!(f.spool.pending().unwrap().is_empty());
}
