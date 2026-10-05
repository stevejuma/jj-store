#![cfg(unix)]
#[path = "opencode_source/fixture.rs"]
mod api;
#[path = "capture_source/fixture.rs"]
#[allow(dead_code)]
mod native;
use api::{Api, message};
use jj_store_capture::{agent_capture::AgentCaptureSpool, opencode_ingress::IntegrationId};
use jj_store_server::{capture_source::CaptureSourceStep, opencode_source::OpenCodeCaptureSource};
use native::{A, B, Fixture};
async fn acquire(f: &mut Fixture, api: &Api) -> OpenCodeCaptureSource {
    OpenCodeCaptureSource::acquire(
        api.api.clone(),
        f.sources.clone(),
        "ses_first",
        f.privacy(),
        &mut f.context(),
    )
    .await
    .unwrap()
    .unwrap()
}
fn witness(f: &Fixture, source: &OpenCodeCaptureSource) -> [u8; 32] {
    f.spool
        .latest_reader_progress(IntegrationId::OpenCode, source.source_instance_id())
        .unwrap()
        .unwrap()
        .metadata
        .generation_digest
}
#[tokio::test]
async fn empty_lifecycle_snapshot_coverage_requires_native_receipts_and_survives_reopen() {
    let mut f = Fixture::new(8 * 1024 * 1024).await;
    let api = Api::new(f.workspace(A).to_str().unwrap().into(), vec![], true).await;
    let mut s = acquire(&mut f, &api).await;
    assert_eq!(
        s.lifecycle_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    let id = s.source_instance_id();
    let digest = witness(&f, &s);
    let p = f
        .spool
        .inspect_snapshot_publication(IntegrationId::OpenCode, id, digest, 1)
        .unwrap()
        .unwrap();
    assert!(p.boundary_available);
    assert!(!p.completed);
    assert_eq!(p.captured_sequence, 1);
    assert_eq!(p.committed_sequence, 0);
    assert!(
        matches!(s.publish_step(&mut f.context()).await.unwrap(),CaptureSourceStep::Published(r) if r.committed==1)
    );
    api.data.lock().unwrap().busy = false;
    assert_eq!(
        s.lifecycle_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    let p = f
        .spool
        .inspect_snapshot_publication(IntegrationId::OpenCode, id, digest, 1)
        .unwrap()
        .unwrap();
    assert!(p.completed);
    let tail = p.tail.unwrap();
    assert_eq!(tail.captured_sequence, 2);
    assert_eq!(tail.committed_sequence, 1);
    assert_eq!(tail.committed_logical_records, Some(0));
    assert_eq!(tail.outstanding_ranges, 1);
    assert!(
        matches!(s.publish_step(&mut f.context()).await.unwrap(),CaptureSourceStep::Published(r) if r.committed==1)
    );
    drop(s);
    drop(f.spool);
    f.spool = AgentCaptureSpool::open(&f.root.path().join("spool"), 8 * 1024 * 1024).unwrap();
    let p = f
        .spool
        .inspect_snapshot_publication(IntegrationId::OpenCode, id, digest, 2)
        .unwrap()
        .unwrap();
    assert!(p.completed);
    assert_eq!(p.committed_sequence, 2);
    let tail = p.tail.unwrap();
    assert_eq!(tail.receipts, 2);
    assert_eq!(tail.committed_logical_records, Some(0));
    assert_eq!(tail.outstanding_ranges, 0);
}
#[tokio::test]
async fn later_snapshot_partition_receipt_cannot_cover_an_earlier_unpublished_status() {
    let mut f = Fixture::new(8 * 1024 * 1024).await;
    let a = f.workspace(A).to_str().unwrap().to_owned();
    let b = f.workspace(B).to_str().unwrap().to_owned();
    let api = Api::new(
        a,
        vec![message("msg_b", Some(&b), "separate repository receipt")],
        true,
    )
    .await;
    let mut s = acquire(&mut f, &api).await;
    assert_eq!(
        s.lifecycle_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    assert_eq!(
        s.ingest_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    let id = s.source_instance_id();
    let digest = witness(&f, &s);
    f.catalog(&[B]);
    assert!(s.publish_step(&mut f.context()).await.is_err());
    assert!(
        matches!(s.publish_step(&mut f.context()).await.unwrap(),CaptureSourceStep::Published(r) if r.committed==1)
    );
    let p = f
        .spool
        .inspect_snapshot_publication(IntegrationId::OpenCode, id, digest, 2)
        .unwrap()
        .unwrap();
    assert!(!p.completed);
    assert_eq!(p.committed_sequence, 0);
    assert_eq!(p.tail.unwrap().outstanding_ranges, 2);
    f.catalog(&[A, B]);
    assert!(
        matches!(s.publish_step(&mut f.context()).await.unwrap(),CaptureSourceStep::Published(r) if r.committed==1)
    );
    let p = f
        .spool
        .inspect_snapshot_publication(IntegrationId::OpenCode, id, digest, 2)
        .unwrap()
        .unwrap();
    assert!(p.completed);
    assert_eq!(p.committed_sequence, 2);
    assert_eq!(p.tail.unwrap().receipts, 2);
}
