use super::*;
use crate::{capture_source::CodexCaptureSource, capture_test_fixture::Fixture};

#[tokio::test]
async fn cancelled_live_raw_state_overrides_older_compact_snapshot_before_import_handoff() {
    let mut f = Fixture::new(128).await;
    f.message(
        "raw",
        "source raw batch must survive a stale compact observation",
    );
    let mut pipeline = CodexCaptureSource::acquire_active(
        f.sources.clone(),
        f.start(),
        f.privacy(),
        &mut f.context(),
    )
    .await
    .unwrap()
    .into_pipeline();
    assert!(
        pipeline
            .ingest_step(&mut f.context())
            .await
            .unwrap_err()
            .capacity_refusal()
            .is_some()
    );
    let config = crate::config::Config::load(&f.root.path().join("server.toml")).unwrap();
    let mut work = SourceWork::new(
        f.qualified(),
        CodexSupervisorResources {
            authority: Arc::new(CaptureAuthority::prepare(&config).unwrap().unwrap()),
            host: f.host.clone(),
            admission: f.admission.clone(),
            sources: f.sources.clone(),
            spool: f.spool.fork_handle(),
            privacy: f.privacy(),
        },
    );
    // The last completed step was compact; a cancelled newer step owns a raw batch.
    work.snapshot.compact = true;
    work.stage = Stage::Live(pipeline);
    assert!(
        !work.replaceable(),
        "cached compaction replaced retained raw work"
    );
    assert!(matches!(work.stage, Stage::Parked(_)));
    assert!(
        f.sources.acquire_admitted(f.qualified()).is_err(),
        "raw channel released during handoff check"
    );
    assert!(f.spool.pending().unwrap().is_empty());
    drop(work);
    assert!(f.sources.acquire_admitted(f.qualified()).unwrap().is_some());
}
