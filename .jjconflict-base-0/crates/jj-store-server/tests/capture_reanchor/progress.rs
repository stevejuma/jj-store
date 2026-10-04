use super::*;
use jj_store_server::capture_source::{CaptureResumeStep, CodexCaptureResume};

async fn deliver(
    pipeline: &mut jj_store_server::capture_source::CodexCapturePipeline,
    f: &mut Fixture,
) {
    for entry in f.spool.pending().unwrap() {
        for _ in 0..4 {
            if matches!(pipeline.publish_step(&entry, &mut f.context()).await.unwrap(), CaptureSourceStep::Published(report) if report.committed == 1)
            {
                break;
            }
        }
    }
    assert!(f.spool.pending().unwrap().is_empty());
}

#[tokio::test]
async fn missing_regressive_and_rebound_durable_state_cannot_authorize_current_handoff() {
    let mut f = Fixture::new(4 * 1024 * 1024).await;
    let mut parked = park(&mut f).await;
    let mut pipeline = parked
        .resume(f.sources.clone(), &mut f.context())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        pipeline.ingest_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    deliver(&mut pipeline, &mut f).await;
    let journal = fs::read_dir(f.root.path().join("spool/reader-progress"))
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    let previous = fs::read(&journal).unwrap();
    f.message("later-a", "second A cursor");
    assert_eq!(
        pipeline.ingest_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    deliver(&mut pipeline, &mut f).await;
    let latest = fs::read(&journal).unwrap();
    assert_ne!(latest, previous);
    let mut parked = pipeline.park();
    f.append(json!({"type":"turn_context","payload":{"cwd":f.workspace(B)}}));
    f.message("b", "admitted current body");
    f.catalog(&[B]);
    fs::remove_file(&journal).unwrap();
    assert!(
        parked
            .resume_current(f.sources.clone(), &mut f.context())
            .await
            .is_err()
    );
    fs::write(&journal, &previous).unwrap();
    use std::os::unix::fs::PermissionsExt as _;
    fs::set_permissions(&journal, fs::Permissions::from_mode(0o600)).unwrap();
    assert!(
        parked
            .resume_current(f.sources.clone(), &mut f.context())
            .await
            .is_err()
    );
    let mut changed: serde_json::Value = serde_json::from_slice(&latest).unwrap();
    changed["last_batch"] = json!("09".repeat(16));
    fs::write(&journal, serde_json::to_vec(&changed).unwrap()).unwrap();
    assert!(
        parked
            .resume_current(f.sources.clone(), &mut f.context())
            .await
            .is_err()
    );
    fs::write(&journal, &latest).unwrap();
    let spool_path = f.root.path().join("spool");
    let old_path = f.root.path().join("old-spool");
    fs::rename(&spool_path, &old_path).unwrap();
    f.spool = AgentCaptureSpool::open(&spool_path, 4 * 1024 * 1024).unwrap();
    assert!(
        parked
            .resume_current(f.sources.clone(), &mut f.context())
            .await
            .is_err()
    );
    // Restore the exact physical private root, never migrate or borrow its cursor.
    fs::remove_dir_all(&spool_path).unwrap();
    fs::rename(old_path, &spool_path).unwrap();
    f.spool = AgentCaptureSpool::open(&spool_path, 4 * 1024 * 1024).unwrap();
    drop(f.sources.acquire_admitted(f.qualified()).unwrap().unwrap());
    let CaptureParkedResumeStep::Ready(mut pipeline) = current(&mut parked, &mut f).await else {
        panic!("restored authoritative progress withheld")
    };
    assert_eq!(
        pipeline.ingest_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    assert!(bodies(&f.spool.pending().unwrap()[0]).contains("admitted current body"));
}

#[tokio::test]
async fn advanced_current_cursor_reopens_instead_of_reusing_the_old_raw_batch() {
    let mut f = Fixture::new(1).await;
    f.message("a", "old raw A body");
    let active = f.start();
    let mut pipeline = CodexCaptureSource::acquire_active(
        f.sources.clone(),
        active,
        f.privacy(),
        &mut f.context(),
    )
    .await
    .unwrap()
    .into_pipeline();
    assert!(pipeline.ingest_step(&mut f.context()).await.is_err());
    let mut parked = pipeline.park();
    f.append(json!({"type":"turn_context","payload":{"cwd":f.workspace(B)}}));
    f.message("b", "already durably accepted B body");
    f.catalog(&[B]);
    f.spool = AgentCaptureSpool::open(&f.root.path().join("spool"), 4 * 1024 * 1024).unwrap();
    let mut resume =
        CodexCaptureResume::prepare(f.sources.clone(), f.qualified(), f.privacy(), &f.spool)
            .unwrap()
            .unwrap();
    let CaptureResumeStep::Ready(source) = resume.step(&mut f.context()).await.unwrap() else {
        panic!("admitted independent reader")
    };
    let mut other = source.into_pipeline();
    assert_eq!(
        other.ingest_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    assert_eq!(
        other.ingest_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    let accepted = f.spool.pending().unwrap();
    let partition = other.partition();
    let progress = f
        .spool
        .latest_reader_progress(partition.integration, partition.source)
        .unwrap();
    drop(other);
    let CaptureParkedResumeStep::Ready(mut pipeline) = current(&mut parked, &mut f).await else {
        panic!("forward durable cursor withheld")
    };
    assert_eq!(
        pipeline.ingest_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Waiting
    );
    assert_eq!(f.spool.pending().unwrap(), accepted);
    assert_eq!(
        f.spool
            .latest_reader_progress(partition.integration, partition.source)
            .unwrap(),
        progress
    );
}
