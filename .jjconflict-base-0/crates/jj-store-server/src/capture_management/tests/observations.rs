use super::*;

#[tokio::test(flavor = "current_thread")]
async fn cached_observations_queue_before_later_import_work() {
    for kind in 0..3 {
        let (_f, _, manager, actor, _, journal) = prepared().await;
        let before = journal.current().unwrap();
        let held = manager.0.gate.clone().lock_owned().await;
        let mut observation = Box::pin(async {
            match kind {
                0 => manager.status(actor).await.map(|_| ()),
                1 => manager
                    .sources(actor, CaptureSourceQuery::default())
                    .await
                    .map(|_| ()),
                _ => manager
                    .import_status(
                        actor,
                        CaptureImportQuery {
                            request_id: RequestId::new([1; 16]),
                        },
                    )
                    .await
                    .map(|_| ()),
            }
        });
        assert!(futures::poll!(observation.as_mut()).is_pending());
        let mut later = Box::pin(manager.0.gate.clone().lock_owned());
        assert!(futures::poll!(later.as_mut()).is_pending());
        drop(held);
        assert_eq!(
            observation.await,
            if kind == 0 {
                Ok(())
            } else {
                Err(Error::Unavailable)
            }
        );
        let permit = tokio::time::timeout(Duration::from_secs(2), later)
            .await
            .unwrap();
        drop(permit);
        assert_eq!(journal.current().unwrap(), before);
    }
}

#[tokio::test(flavor = "current_thread")]
async fn queued_observation_rechecks_current_authority() {
    let (f, mut config, manager, actor, _, _) = prepared().await;
    let held = manager.0.gate.clone().lock_owned().await;
    let mut observation = Box::pin(manager.status(actor));
    assert!(futures::poll!(observation.as_mut()).is_pending());
    let Access::LocalTrusted { permission, .. } = &mut config.access else {
        unreachable!()
    };
    *permission = Permission::Write;
    fs::write(
        f.root.path().join("server.toml"),
        toml::to_string(&config).unwrap(),
    )
    .unwrap();
    drop(held);
    assert_eq!(observation.await, Err(Error::Denied));
}

#[tokio::test(flavor = "current_thread")]
async fn timed_out_or_cancelled_observation_releases_its_queue_place() {
    let (_f, _, manager, actor, _, _) = prepared().await;
    let held = manager.0.gate.clone().lock_owned().await;
    assert_eq!(manager.status(actor).await, Err(Error::Busy));
    let mut cancelled = Box::pin(manager.status(actor));
    assert!(futures::poll!(cancelled.as_mut()).is_pending());
    drop(cancelled);
    drop(held);
    assert!(
        tokio::time::timeout(Duration::from_secs(2), manager.status(actor))
            .await
            .unwrap()
            .is_ok()
    );
}
