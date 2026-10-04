use super::*;

#[tokio::test(flavor = "current_thread")]
async fn cancelled_source_read_retains_gates_until_owned_observation_finishes() {
    let (f, _, manager, actor, _, journal) = prepared().await;
    let before = journal.current().unwrap();
    let (entered, entry) = tokio::sync::oneshot::channel();
    let (release, wait) = std::sync::mpsc::channel();
    *manager.0.after_selection.lock().unwrap() = Some(Box::new(move || {
        let _ = entered.send(());
        wait.recv().unwrap();
    }));
    let owned = manager.clone();
    let caller =
        tokio::spawn(async move { owned.sources(actor, CaptureSourceQuery::default()).await });
    tokio::time::timeout(Duration::from_secs(10), entry)
        .await
        .unwrap()
        .unwrap();
    caller.abort();
    assert!(caller.await.unwrap_err().is_cancelled());
    assert_eq!(
        journal.current(),
        Err(crate::capture_control_state::Error::Unavailable)
    );
    assert_eq!(
        manager.sources(actor, CaptureSourceQuery::default()).await,
        Err(Error::Busy)
    );
    release.send(()).unwrap();
    tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            match manager.status(actor).await {
                Err(Error::Busy) => tokio::time::sleep(Duration::from_millis(5)).await,
                Ok(_) => break,
                other => panic!("unexpected source read settlement: {other:?}"),
            }
        }
    })
    .await
    .unwrap();
    assert_eq!(journal.current().unwrap(), before);
    assert!(f.spool.pending().unwrap().is_empty());
}

#[tokio::test(flavor = "current_thread")]
async fn source_read_rechecks_revocation_disable_and_physical_policy_under_gate() {
    for kind in 0..3 {
        let (f, config, manager, actor, _, journal) = prepared().await;
        let before = journal.current().unwrap();
        let path = f.root.path().join("server.toml");
        let policy = f.root.path().join("control");
        let held = f.root.path().join("held-control");
        let moved = policy.clone();
        let saved = held.clone();
        *manager.0.after_selection.lock().unwrap() = Some(Box::new(move || {
            let mut changed = config;
            match kind {
                0 => {
                    let Access::LocalTrusted { permission, .. } = &mut changed.access else {
                        unreachable!()
                    };
                    *permission = Permission::Write;
                    fs::write(path, toml::to_string(&changed).unwrap()).unwrap();
                }
                1 => {
                    changed.agent_capture.enabled = false;
                    fs::write(path, toml::to_string(&changed).unwrap()).unwrap();
                }
                _ => {
                    fs::rename(moved, saved).unwrap();
                }
            }
        }));
        let expected = [Error::Denied, Error::Disabled, Error::Unavailable][kind];
        assert_eq!(
            manager.sources(actor, CaptureSourceQuery::default()).await,
            Err(expected)
        );
        if kind == 2 {
            fs::rename(held, policy).unwrap();
        }
        assert_eq!(journal.current().unwrap(), before);
        assert!(f.spool.pending().unwrap().is_empty());
    }
}
