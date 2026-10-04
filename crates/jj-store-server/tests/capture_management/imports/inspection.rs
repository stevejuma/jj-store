use super::*;

#[tokio::test]
async fn missing_reader_link_and_unqualified_publication_withhold_completion_without_repair() {
    let mut f = Fixture::new(true).await;
    let journal = attach(&mut f);
    f.source.message("history", "read-only evidence inspection");
    let mut runtime = f.runtime();
    let request = CaptureImportCommand {
        version: 1,
        request_id: RequestId::new([5; 16]),
        generation: hex::encode(f.binding.as_ref().unwrap().generation),
        expected_revision: 0,
        selector: ready(&f).await,
    };
    // Cold sanitizer preparation can exceed the foreground acknowledgement budget.
    // Observe only this original request; an unknown outcome does not permit a new one.
    match f.manager.import(f.actor, request.clone()).await {
        Ok(outcome) => assert_eq!(outcome.receipt.command, request),
        Err(Error::Unknown) => {}
        other => panic!("import acceptance: {other:?}"),
    }
    let completed = settled_with_timeout(&f, request.request_id, 240).await;
    assert_eq!(completed.receipt.as_ref().unwrap().command, request);
    runtime.pause().unwrap();
    tokio::time::timeout(Duration::from_secs(10), async {
        while runtime.snapshot().phase
            != jj_store_server::capture_worker::CaptureWorkerPhase::Paused
        {
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .unwrap();
    fs::remove_file(&f.source.transcript).unwrap();
    let path = f.config.state_dir.join("agent-capture/imports/state.json");
    let before = fs::read(&path).unwrap();
    let mut value: Value = serde_json::from_slice(&before).unwrap();
    value.as_object_mut().unwrap().remove("readers");
    let old = serde_json::to_vec(&value).unwrap();
    fs::write(&path, &old).unwrap();
    let status = f
        .manager
        .import_status(
            f.actor,
            CaptureImportQuery {
                request_id: request.request_id,
            },
        )
        .await
        .unwrap();
    assert!(status.receipt.is_some());
    assert_eq!(status.progress.phase, CaptureImportPhase::Unavailable);
    assert!(!status.progress.boundary_available);
    assert_eq!(fs::read(&path).unwrap(), old);
    fs::write(&path, &before).unwrap();
    let mut unverified: Value = serde_json::from_slice(&before).unwrap();
    unverified["readers"][0]
        .as_object_mut()
        .unwrap()
        .remove("prefix_verified");
    let old = serde_json::to_vec(&unverified).unwrap();
    fs::write(&path, &old).unwrap();
    let status = f
        .manager
        .import_status(
            f.actor,
            CaptureImportQuery {
                request_id: request.request_id,
            },
        )
        .await
        .unwrap();
    assert_eq!(status.progress.phase, CaptureImportPhase::Unavailable);
    assert_eq!(fs::read(&path).unwrap(), old);
    fs::write(&path, &before).unwrap();
    let catalog = f
        .source
        .root
        .path()
        .join("spool/reader-publication/catalog.json");
    let catalog_before = fs::read(&catalog).unwrap();
    fs::write(&catalog, b"corrupt evidence").unwrap();
    let status = f
        .manager
        .import_status(
            f.actor,
            CaptureImportQuery {
                request_id: request.request_id,
            },
        )
        .await
        .unwrap();
    assert!(status.receipt.is_some());
    assert_eq!(status.progress.phase, CaptureImportPhase::Unavailable);
    assert_eq!(fs::read(&catalog).unwrap(), b"corrupt evidence");
    assert_eq!(fs::read(&path).unwrap(), before);
    fs::write(&catalog, catalog_before).unwrap();
    assert_eq!(
        f.manager
            .import_status(
                f.actor,
                CaptureImportQuery {
                    request_id: request.request_id
                }
            )
            .await
            .unwrap()
            .progress
            .phase,
        CaptureImportPhase::Completed
    );
    assert_eq!(journal.entries().unwrap().len(), 1);
    runtime.shutdown().await.unwrap();
}

#[tokio::test]
async fn rebound_spool_and_missing_import_state_are_unavailable_without_new_owners_or_files() {
    let mut f = Fixture::new(true).await;
    attach(&mut f);
    let request = RequestId::new([5; 16]);
    let spool = f.source.root.path().join("spool");
    let retained = f.source.root.path().join("retained-spool");
    fs::rename(&spool, &retained).unwrap();
    use std::os::unix::fs::DirBuilderExt as _;
    fs::DirBuilder::new().mode(0o700).create(&spool).unwrap();
    assert_eq!(
        f.manager
            .import_status(
                f.actor,
                CaptureImportQuery {
                    request_id: request
                }
            )
            .await,
        Err(Error::Unavailable)
    );
    assert_eq!(fs::read_dir(&spool).unwrap().count(), 0);
    fs::remove_dir(&spool).unwrap();
    fs::rename(&retained, &spool).unwrap();
    let path = f.config.state_dir.join("agent-capture/imports/state.json");
    fs::remove_file(&path).unwrap();
    assert_eq!(
        f.manager
            .import_status(
                f.actor,
                CaptureImportQuery {
                    request_id: request
                }
            )
            .await,
        Err(Error::Unavailable)
    );
    assert!(!path.exists());
    assert!(f.binding.as_ref().unwrap().runtime.get().is_none());
}
