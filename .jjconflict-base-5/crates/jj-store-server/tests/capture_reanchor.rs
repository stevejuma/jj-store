#![cfg(unix)]
#[path = "capture_source/fixture.rs"]
mod fixture;
#[path = "capture_reanchor/progress.rs"]
mod progress;
#[path = "capture_reanchor/race.rs"]
mod race;
use fixture::*;
use jj_store_capture::agent_capture::AgentCaptureSpool;
use jj_store_capture::source_registry::SourceRegistry;
use jj_store_server::capture_source::ParkedCodexCapturePipeline;
use jj_store_server::capture_source::{
    CaptureParkedResumeStep, CaptureSourceStep, CodexCaptureSource,
};
use serde_json::json;
use std::{fs, sync::Arc};

async fn park(f: &mut Fixture) -> ParkedCodexCapturePipeline {
    let active = f.start();
    CodexCaptureSource::acquire_active(f.sources.clone(), active, f.privacy(), &mut f.context())
        .await
        .unwrap()
        .into_pipeline()
        .park()
}
async fn current(
    parked: &mut ParkedCodexCapturePipeline,
    f: &mut Fixture,
) -> CaptureParkedResumeStep {
    parked
        .resume_current(f.sources.clone(), &mut f.context())
        .await
        .unwrap()
}

fn bodies(entry: &jj_store_capture::agent_capture::PendingCaptureV1) -> String {
    entry
        .batch
        .observations
        .iter()
        .map(|observation| std::str::from_utf8(observation.payload.bytes()).unwrap())
        .collect::<Vec<_>>()
        .join("\n")
}

#[tokio::test]
async fn partial_current_context_and_missing_write_do_not_acquire_or_advance() {
    let mut f = Fixture::new(4 * 1024 * 1024).await;
    let mut parked = park(&mut f).await;
    let before = fs::read(f.root.path().join("sources/sources.json")).unwrap();
    let locks = fs::read_dir(f.root.path().join("sources")).unwrap().count();
    let context = format!(
        "{}\n",
        json!({"type":"turn_context","payload":{"cwd":f.workspace(B)}})
    );
    f.append_bytes(&context.as_bytes()[..context.len() - 1]);
    f.catalog(&[B]);
    assert!(matches!(
        current(&mut parked, &mut f).await,
        CaptureParkedResumeStep::Withheld
    ));
    assert_eq!(
        fs::read_dir(f.root.path().join("sources")).unwrap().count(),
        locks
    );
    let host = f.host.clone();
    f.without_native_authority();
    f.append_bytes(b"\n");
    assert!(
        parked
            .resume_current(f.sources.clone(), &mut f.context())
            .await
            .is_err()
    );
    assert_eq!(
        fs::read(f.root.path().join("sources/sources.json")).unwrap(),
        before
    );
    assert_eq!(
        fs::read_dir(f.root.path().join("sources")).unwrap().count(),
        locks
    );
    assert!(f.spool.pending().unwrap().is_empty());
    f.host = host;
    let CaptureParkedResumeStep::Ready(pipeline) = current(&mut parked, &mut f).await else {
        panic!("complete B withheld")
    };
    assert_eq!(pipeline.partition().repository, B);
    assert!(
        f.spool
            .latest_reader_progress(
                pipeline.partition().integration,
                pipeline.partition().source
            )
            .unwrap()
            .is_none()
    );
    assert!(
        parked
            .resume_current(f.sources.clone(), &mut f.context())
            .await
            .is_err()
    );
}

#[tokio::test]
async fn bounded_preparation_keeps_an_unfinished_reader_prefix_verification() {
    use jj_store_server::capture_source::{CaptureResumeStep, CodexCaptureResume};
    let mut f = Fixture::new(4 * 1024 * 1024).await;
    let mut parked = park(&mut f).await;
    let mut pipeline = parked
        .resume(f.sources.clone(), &mut f.context())
        .await
        .unwrap()
        .unwrap();
    f.append_bytes(&vec![b'x'; 9 * 1024 * 1024]);
    f.append_bytes(b"\n");
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
    let mut resume =
        CodexCaptureResume::prepare(f.sources.clone(), f.qualified(), f.privacy(), &f.spool)
            .unwrap()
            .unwrap();
    let mut pipeline = loop {
        match resume.step(&mut f.context()).await.unwrap() {
            CaptureResumeStep::Verifying => {}
            CaptureResumeStep::Ready(source) => break source.into_pipeline(),
            _ => panic!("admitted source withheld"),
        }
    };
    assert_eq!(
        pipeline.ingest_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Verifying
    );
    let mut parked = pipeline.park();
    f.append(json!({"type":"turn_context","payload":{"cwd":f.workspace(B)}}));
    f.catalog(&[B]);
    let sources_path = f.root.path().join("sources");
    let before = fs::read(sources_path.join("sources.json")).unwrap();
    for _ in 0..2 {
        assert!(matches!(
            current(&mut parked, &mut f).await,
            CaptureParkedResumeStep::Verifying
        ));
        assert_eq!(fs::read(sources_path.join("sources.json")).unwrap(), before);
        assert_eq!(
            f.spool
                .latest_reader_progress(partition.integration, partition.source)
                .unwrap(),
            Some(progress.clone())
        );
        // Preparation must not hold either reader lease.
        drop(f.sources.acquire_admitted(f.qualified()).unwrap().unwrap());
    }
    let CaptureParkedResumeStep::Ready(mut pipeline) = current(&mut parked, &mut f).await else {
        panic!("third bounded step")
    };
    assert_eq!(pipeline.partition().repository, B);
    // Changed metadata renews the retained reader's prefix proof in bounded steps.
    assert_eq!(
        pipeline.ingest_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Verifying
    );
    assert_eq!(
        pipeline.ingest_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Verifying
    );
    assert_eq!(
        pipeline.ingest_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    assert_eq!(
        f.spool
            .pending()
            .unwrap()
            .iter()
            .filter(|e| e.repository == B)
            .count(),
        1
    );
}

#[tokio::test]
async fn failed_current_handoffs_keep_exact_frame_across_foreign_spool_and_source_replacement() {
    let mut f = Fixture::new(1).await;
    f.message("a", "retained raw batch");
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
    f.catalog(&[B]);
    let spool = std::mem::replace(
        &mut f.spool,
        AgentCaptureSpool::open(&f.root.path().join("foreign"), 4 * 1024 * 1024).unwrap(),
    );
    assert!(
        parked
            .resume_current(f.sources.clone(), &mut f.context())
            .await
            .is_err()
    );
    f.spool = spool;
    let old = f.transcript.with_extension("old");
    fs::rename(&f.transcript, &old).unwrap();
    fs::copy(&old, &f.transcript).unwrap();
    assert!(
        parked
            .resume_current(f.sources.clone(), &mut f.context())
            .await
            .is_err()
    );
    fs::remove_file(&f.transcript).unwrap();
    fs::rename(old, &f.transcript).unwrap();
    // A different registry cannot supply the retained source identity.
    let foreign = Arc::new(
        SourceRegistry::open(&f.root.path().join("foreign-sources"), [0xff; 16], 4).unwrap(),
    );
    assert!(matches!(
        parked
            .resume_current(foreign, &mut f.context())
            .await
            .unwrap(),
        CaptureParkedResumeStep::Withheld
    ));
    let CaptureParkedResumeStep::Ready(mut pipeline) = current(&mut parked, &mut f).await else {
        panic!("restored source withheld")
    };
    assert!(pipeline.ingest_step(&mut f.context()).await.is_err());
    f.spool = AgentCaptureSpool::open(&f.root.path().join("spool"), 4 * 1024 * 1024).unwrap();
    assert_eq!(
        pipeline.ingest_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    assert!(!bodies(&f.spool.pending().unwrap()[0]).contains("retained raw batch"));
}

#[tokio::test]
async fn current_directory_handoff_preserves_the_exact_backpressured_batch_after_old_anchor_revocation()
 {
    let mut f = Fixture::new(1).await;
    f.message("original", &format!("old a body {SECRET}"));
    let active = f.start();
    let sources = f.sources.clone();
    let mut pipeline =
        CodexCaptureSource::acquire_active(sources.clone(), active, f.privacy(), &mut f.context())
            .await
            .unwrap()
            .into_pipeline();
    let original_end = std::fs::metadata(&f.transcript).unwrap().len();
    let original_source = pipeline.partition().source;
    let integration = pipeline.partition().integration;
    assert!(pipeline.ingest_step(&mut f.context()).await.is_err());
    let mut parked = pipeline.park();
    f.append(json!({"type":"turn_context","payload":{"cwd":f.workspace(B)}}));
    f.message("b", &format!("current b body {SECRET}"));
    f.catalog(&[B]);
    assert!(
        parked
            .resume(sources.clone(), &mut f.context())
            .await
            .is_err()
    );
    let mut pipeline = loop {
        match parked
            .resume_current(sources.clone(), &mut f.context())
            .await
            .unwrap()
        {
            CaptureParkedResumeStep::Verifying | CaptureParkedResumeStep::ChangedDuringRead => {}
            CaptureParkedResumeStep::Ready(pipeline) => break *pipeline,
            CaptureParkedResumeStep::Withheld => panic!("current linked writable B withheld"),
        }
    };
    assert_eq!(pipeline.partition().repository, B);
    assert_eq!(pipeline.partition().source, original_source);
    assert!(pipeline.ingest_step(&mut f.context()).await.is_err());
    assert!(f.spool.pending().unwrap().is_empty());
    assert!(
        f.spool
            .latest_reader_progress(integration, original_source)
            .unwrap()
            .is_none()
    );
    f.spool = AgentCaptureSpool::open(&f.root.path().join("spool"), 4 * 1024 * 1024).unwrap();
    assert_eq!(
        pipeline.ingest_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    let first = f.spool.pending().unwrap().remove(0);
    assert_eq!(first.repository, B);
    assert_eq!(
        serde_json::to_value(&first.cursor.position).unwrap()["complete_byte_offset"].as_u64(),
        Some(original_end)
    );
    let original_payloads = bodies(&first);
    assert!(!original_payloads.contains("old a body"));
    assert!(!original_payloads.contains("current b body"));
    assert!(!original_payloads.contains(SECRET));
    for _ in 0..8 {
        if pipeline.ingest_step(&mut f.context()).await.unwrap() == CaptureSourceStep::Enqueued {
            break;
        }
    }
    let queue = f.spool.pending().unwrap();
    assert_eq!(queue.len(), 2);
    let next = queue
        .iter()
        .find(|e| e.batch.batch_id != first.batch.batch_id)
        .unwrap();
    let body = bodies(next);
    assert!(body.contains("current b body"));
    assert!(!body.contains(SECRET));
    let mut queue = queue;
    queue.sort_by_key(|e| e.batch.observations[0].source_sequence);
    for entry in &queue {
        for _ in 0..8 {
            if matches!(pipeline.publish_step(entry, &mut f.context()).await.unwrap(), CaptureSourceStep::Published(report) if report.committed == 1)
            {
                break;
            }
        }
    }
    assert!(f.spool.pending().unwrap().is_empty());
}
