use super::*;

#[tokio::test]
async fn progress_snapshot_retains_exact_capacity_refusal_from_an_independent_spool_handle() {
    let mut f = Fixture::new(8 * 1024 * 1024).await;
    let small = AgentCaptureSpool::open(&f.root.path().join("spool"), 4 * 1024 * 1024).unwrap();
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
    for id in 0..4 {
        f.message(
            &format!("pending-{id}"),
            "accepted independent-handle evidence",
        );
        assert_eq!(
            pipeline.ingest_step(&mut f.context()).await.unwrap(),
            CaptureSourceStep::Enqueued
        );
    }
    let queue = f.spool.pending().unwrap();
    let partition = pipeline.partition();
    let progress = f
        .spool
        .latest_reader_progress(partition.integration, partition.source)
        .unwrap();
    let required: u64 = fs::read_dir(f.root.path().join("spool/pending"))
        .unwrap()
        .map(|p| p.unwrap().metadata().unwrap().len())
        .sum();
    assert!(required > small.pending_capacity());
    let large = f.spool.fork_handle();
    f.spool = small;
    let error = pipeline.ingest_step(&mut f.context()).await.unwrap_err();
    assert!(
        matches!(error, jj_store_server::Error::CaptureCapacity(refusal)
        if refusal.limit_bytes == 4 * 1024 * 1024 && refusal.required_bytes == required),
        "{error:?}"
    );
    assert_eq!(large.pending().unwrap(), queue);
    assert_eq!(
        large
            .latest_reader_progress(partition.integration, partition.source)
            .unwrap(),
        progress
    );
}
