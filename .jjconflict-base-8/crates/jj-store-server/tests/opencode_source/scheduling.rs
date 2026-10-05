use super::*;
#[tokio::test]
async fn settled_api_releases_acquisition_and_revoked_partition_yields_without_reordering() {
    let mut f = Fixture::new(4 * 1024 * 1024).await;
    let a = f.workspace(A).to_str().unwrap().to_owned();
    let b = f.workspace(B).to_str().unwrap().to_owned();
    let api = Api::new(
        a.clone(),
        vec![
            message("msg_a", Some(&a), "withheld A"),
            message("msg_b", Some(&b), "available B"),
            message("msg_b_tail", Some(&b), "B tail"),
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
    for _ in 0..3 {
        assert_eq!(
            source.ingest_step(&mut f.context()).await.unwrap(),
            CaptureSourceStep::Enqueued
        );
    }
    assert!(!source.compact_idle());
    assert_eq!(
        source.ingest_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Waiting
    );
    assert!(source.compact_idle());
    let a_entry = f
        .spool
        .pending()
        .unwrap()
        .into_iter()
        .find(|e| e.repository == A)
        .unwrap();
    f.catalog(&[B]);
    assert!(source.publish_step(&mut f.context()).await.is_err());
    for _ in 0..2 {
        assert!(
            matches!(source.publish_step(&mut f.context()).await.unwrap(),CaptureSourceStep::Published(report) if report.committed==1)
        );
        // The scheduler rotates A then B; A remains exact and cannot authorize B.
        if f.spool
            .pending()
            .unwrap()
            .iter()
            .filter(|e| e.repository == B)
            .count()
            == 1
        {
            assert!(source.publish_step(&mut f.context()).await.is_err());
        }
    }
    assert_eq!(f.spool.pending().unwrap(), vec![a_entry]);
    f.catalog(&[A, B]);
    assert!(
        matches!(source.publish_step(&mut f.context()).await.unwrap(),CaptureSourceStep::Published(report) if report.committed==1)
    );
    assert!(f.spool.pending().unwrap().is_empty());
}
