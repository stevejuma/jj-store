use super::*;
use jj_store_capture::{
    agent_capture::AgentCaptureSpool,
    codex_discovery::{ApprovedCodexRoot, CodexDirectoryScanner},
    profile::CaptureProfile,
    source_registry::QualifiedCodexSource,
};
use std::{fs, path::Path};
fn qualified(path: &Path) -> QualifiedCodexSource {
    let path = fs::canonicalize(path).unwrap();
    let mut scanner = CodexDirectoryScanner::new(
        ApprovedCodexRoot::open(path.parent().unwrap())
            .unwrap()
            .unwrap(),
    )
    .unwrap();
    let candidate = scanner
        .step(100, 100)
        .unwrap()
        .candidates
        .into_iter()
        .find(|c| c.path() == path)
        .unwrap();
    QualifiedCodexSource::inspect(CaptureProfile::CodexCliJsonlV1, candidate)
        .unwrap()
        .unwrap()
}

#[tokio::test]
async fn another_physical_reader_cannot_renumber_or_enqueue_a_parked_raw_batch() {
    let mut f = Fixture::new(1).await;
    let active = f.start();
    let source = CodexCaptureSource::acquire_active(
        f.sources.clone(),
        active,
        f.privacy(),
        &mut f.context(),
    )
    .await
    .unwrap();
    let partition = source.partition();
    drop(source);
    let header = fs::read_to_string(&f.transcript).unwrap();
    let archive = f.transcript.with_file_name("archive.jsonl");
    fs::rename(&f.transcript, &archive).unwrap();
    fs::write(&f.transcript, header).unwrap();
    let mut original = CodexCaptureSource::acquire_admitted(
        f.sources.clone(),
        qualified(&archive),
        f.privacy(),
        &mut f.context(),
    )
    .await
    .unwrap()
    .unwrap()
    .into_pipeline();
    // Allocate one exact raw archive frame, then stop before any durability.
    assert!(original.ingest_step(&mut f.context()).await.is_err());
    assert!(f.spool.pending().unwrap().is_empty());
    let mut parked = original.park();
    f.spool = AgentCaptureSpool::open(&f.root.path().join("spool"), 8 * 1024 * 1024).unwrap();
    f.message("independent", "independent current-file evidence");
    let mut current = CodexCaptureSource::acquire_admitted(
        f.sources.clone(),
        qualified(&f.transcript),
        f.privacy(),
        &mut f.context(),
    )
    .await
    .unwrap()
    .unwrap()
    .into_pipeline();
    assert_eq!(current.partition(), partition);
    assert_eq!(
        current.ingest_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    drop(current);
    let retained = f.spool.pending().unwrap();
    let progress = f
        .spool
        .latest_reader_progress(partition.integration, partition.source)
        .unwrap();
    for _ in 0..3 {
        assert!(
            parked
                .resume(f.sources.clone(), &mut f.context())
                .await
                .is_err()
        );
        assert_eq!(f.spool.pending().unwrap(), retained);
        assert_eq!(
            f.spool
                .latest_reader_progress(partition.integration, partition.source)
                .unwrap(),
            progress
        );
        // Failure retains raw state but cannot leak the fresh source/channel leases.
        drop(
            f.sources
                .acquire_admitted(qualified(&archive))
                .unwrap()
                .unwrap(),
        );
    }
}
