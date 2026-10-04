use super::*;

fn hint(command: &CaptureControlCommand) -> CaptureRescanCommand {
    CaptureRescanCommand {
        version: 1,
        generation: command.generation.clone(),
        expected_revision: 0,
    }
}

#[tokio::test(flavor = "current_thread")]
async fn cancelled_rescan_retains_both_gates_without_selecting_policy() {
    let (f, _, manager, actor, command, journal) = prepared().await;
    let state = f.root.path().join("control/state.json");
    let bytes = fs::read(&state).unwrap();
    let (entered, entry) = tokio::sync::oneshot::channel();
    let (release, wait) = std::sync::mpsc::channel();
    *manager.0.after_selection.lock().unwrap() = Some(Box::new(move || {
        let _ = entered.send(());
        wait.recv().unwrap();
    }));
    let owned = manager.clone();
    let request = hint(&command);
    let caller = tokio::spawn(async move { owned.rescan(actor, request).await });
    tokio::time::timeout(Duration::from_secs(10), entry)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        journal.current(),
        Err(crate::capture_control_state::Error::Unavailable)
    );
    caller.abort();
    assert!(caller.await.unwrap_err().is_cancelled());
    assert_eq!(manager.status(actor).await, Err(Error::Busy));
    assert_eq!(
        manager.rescan(actor, hint(&command)).await,
        Err(Error::Busy)
    );
    assert_eq!(fs::read(&state).unwrap(), bytes);
    release.send(()).unwrap();
    tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            match manager.status(actor).await {
                Err(Error::Busy) => tokio::time::sleep(Duration::from_millis(5)).await,
                Ok(status) => {
                    let policy = status.policy.unwrap();
                    assert_eq!(policy.revision, 0);
                    assert!(!policy.paused);
                    break;
                }
                other => panic!("unexpected rescan settlement: {other:?}"),
            }
        }
    })
    .await
    .unwrap();
    assert_eq!(fs::read(state).unwrap(), bytes);
    assert!(f.spool.pending().unwrap().is_empty());
}

#[tokio::test(flavor = "current_thread")]
async fn rescan_rechecks_revocation_and_physical_policy_before_runtime_request() {
    for replace in [false, true] {
        let (f, config, manager, actor, command, journal) = prepared().await;
        let state = f.root.path().join("control/state.json");
        let bytes = fs::read(&state).unwrap();
        let root = f.root.path().to_owned();
        *manager.0.after_selection.lock().unwrap() = Some(Box::new(move || {
            if replace {
                let control = root.join("control");
                let retained = root.join("retained");
                fs::rename(&control, &retained).unwrap();
                use std::os::unix::fs::DirBuilderExt as _;
                fs::DirBuilder::new().mode(0o700).create(&control).unwrap();
                for name in ["state.json", "control.lock"] {
                    fs::copy(retained.join(name), control.join(name)).unwrap();
                }
            } else {
                let mut revoked = config;
                let Access::LocalTrusted { permission, .. } = &mut revoked.access else {
                    unreachable!();
                };
                *permission = Permission::Write;
                fs::write(root.join("server.toml"), toml::to_string(&revoked).unwrap()).unwrap();
            }
        }));
        assert_eq!(
            manager.rescan(actor, hint(&command)).await,
            Err(if replace {
                Error::Unavailable
            } else {
                Error::Denied
            })
        );
        assert_eq!(fs::read(state).unwrap(), bytes);
        if !replace {
            assert_eq!(journal.current().unwrap().revision, 0);
        }
        assert!(f.spool.pending().unwrap().is_empty());
    }
}
