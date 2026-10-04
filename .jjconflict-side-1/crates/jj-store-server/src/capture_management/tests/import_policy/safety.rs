use super::*;

#[tokio::test]
async fn disabled_denied_and_foreign_import_guards_read_no_unselected_capture_state() {
    let mut f = ImportFixture::new().await;
    let mut actor = f.intent.clone();
    actor.principal = PrincipalId::new([3; 16]);
    assert!(matches!(
        f.manager.begin_import(actor, f.imports.clone()),
        Err(Error::Denied)
    ));
    let foreign = CaptureImportJournal::create(
        &f.f.root.path().join("foreign-imports"),
        [0xab; 16],
        f.intent.generation,
    )
    .unwrap();
    fs::remove_file(f.f.root.path().join("foreign-imports/state.json")).unwrap();
    assert!(matches!(
        f.manager.begin_import(f.intent.clone(), foreign),
        Err(Error::Unavailable)
    ));
    let root = f.config.state_dir.join("agent-capture/imports");
    let original = f.f.root.path().join("held-imports");
    for (installation, generation) in [([0xcd; 16], f.intent.generation), ([0xab; 16], [7; 16])] {
        fs::rename(&root, &original).unwrap();
        let foreign = CaptureImportJournal::create(&root, installation, generation).unwrap();
        assert!(matches!(
            f.manager.begin_import(f.intent.clone(), foreign),
            Err(Error::Unavailable)
        ));
        fs::remove_dir_all(&root).unwrap();
        fs::rename(&original, &root).unwrap();
    }
    let imports = f.config.state_dir.join("agent-capture/imports/state.json");
    fs::remove_file(&imports).unwrap();
    let mut disabled = f.config.clone();
    disabled.agent_capture.enabled = false;
    f.save(&disabled);
    assert!(matches!(
        f.manager.begin_import(f.intent.clone(), f.imports.clone()),
        Err(Error::Disabled)
    ));
    assert!(!imports.exists());
    assert!(f.f.spool.pending().unwrap().is_empty());
    f.runtime.shutdown().await.unwrap();
}

#[tokio::test]
async fn byte_identical_control_file_replacement_before_application_withholds_import() {
    let mut f = ImportFixture::new().await;
    let policy = f
        .manager
        .begin_import(f.intent.clone(), f.imports.clone())
        .unwrap();
    let path = f.f.root.path().join("control/state.json");
    let replacement = f.f.root.path().join("control/.capture-control-replacement");
    *f.manager.0.after_selection.lock().unwrap() = Some(Box::new(move || {
        fs::copy(&path, &replacement).unwrap();
        fs::rename(replacement, path).unwrap();
    }));
    assert_eq!(
        policy.with_current(|_| panic!("rebound control applied import")),
        Err(Error::Unavailable)
    );
    assert!(f.imports.entries().unwrap().is_empty());
    drop(policy);
    f.runtime.shutdown().await.unwrap();
}

#[tokio::test]
async fn post_application_authority_loss_is_unknown_and_exact_selected_intent_survives() {
    let mut f = ImportFixture::new().await;
    let policy = f
        .manager
        .begin_import(f.intent.clone(), f.imports.clone())
        .unwrap();
    let binding = f.source_binding();
    let mut revoked = f.config.clone();
    let Access::LocalTrusted { permission, .. } = &mut revoked.access else {
        unreachable!()
    };
    *permission = Permission::Read;
    assert_eq!(
        policy.with_current(|check| {
            check().unwrap();
            let receipt = f.imports.record(&f.intent, &binding).unwrap();
            f.save(&revoked);
            receipt
        }),
        Err(Error::Unknown)
    );
    drop(policy);
    let receipt = f.imports.resolve(&f.intent).unwrap().unwrap();
    f.save(&f.config);
    let recovered = f
        .manager
        .begin_import(f.intent.clone(), f.imports.clone())
        .unwrap();
    assert_eq!(recovered.selected(), Some(&receipt));
    assert_eq!(recovered.with_current(|check| check()).unwrap(), Ok(()));
    drop(recovered);
    f.runtime.shutdown().await.unwrap();
}

#[tokio::test(flavor = "current_thread")]
async fn caller_cancellation_retains_import_policy_and_selection_gates_until_settlement() {
    let mut f = ImportFixture::new().await;
    let policy = f
        .manager
        .begin_import(f.intent.clone(), f.imports.clone())
        .unwrap();
    let journal = f.imports.clone();
    let intent = f.intent.clone();
    let binding = f.source_binding();
    let (entered, entry) = tokio::sync::oneshot::channel();
    let (release, wait) = std::sync::mpsc::channel();
    let caller = tokio::spawn(async move {
        tokio::task::spawn_blocking(move || {
            policy.with_current(|check| {
                journal
                    .record_then(&intent, &binding, |_, selected| {
                        entered.send(()).unwrap();
                        wait.recv().unwrap();
                        check().unwrap();
                        selected().unwrap();
                    })
                    .unwrap()
            })
        })
        .await
    });
    tokio::time::timeout(Duration::from_secs(10), entry)
        .await
        .unwrap()
        .unwrap();
    caller.abort();
    assert_eq!(f.manager.status(f.intent.principal).await, Err(Error::Busy));
    assert_eq!(
        f.control.current(),
        Err(crate::capture_control_state::Error::Unavailable)
    );
    assert_eq!(
        f.imports.entries(),
        Err(crate::capture_import_state::Error::Unavailable)
    );
    release.send(()).unwrap();
    tokio::time::timeout(Duration::from_secs(10), async {
        while f.manager.status(f.intent.principal).await == Err(Error::Busy) {
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .unwrap();
    assert!(f.imports.resolve(&f.intent).unwrap().is_some());
    assert!(
        f.f.sources
            .lookup_admitted(&f.f.qualified())
            .unwrap()
            .is_none()
    );
    f.runtime.shutdown().await.unwrap();
}
