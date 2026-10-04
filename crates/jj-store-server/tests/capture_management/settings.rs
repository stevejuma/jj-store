use super::*;
#[tokio::test]
async fn configuration_saves_are_conditional_private_and_leave_capture_state_untouched() {
    let f = Fixture::new(false).await;
    let initial = f.manager.settings(f.actor).await.unwrap();
    assert!(!initial.desired.enabled && !initial.restart_required);
    assert_eq!(initial.defaults.len(), 2);
    let original: toml::Value =
        toml::from_str(&fs::read_to_string(f.source.root.path().join("server.toml")).unwrap())
            .unwrap();
    let mut command = SaveCaptureSettings {
        version: 1,
        expected_revision: initial.revision.clone(),
        configuration: initial.desired.clone(),
    };
    command.configuration.enabled = true;
    let saved = f
        .manager
        .save_settings(f.actor, command.clone())
        .await
        .unwrap();
    assert!(saved.desired.enabled && !saved.effective.enabled && saved.restart_required);
    assert_ne!(saved.revision, initial.revision);
    assert!(!f.config.state_dir.join("agent-capture").exists());
    let mut actual: toml::Value =
        toml::from_str(&fs::read_to_string(f.source.root.path().join("server.toml")).unwrap())
            .unwrap();
    actual.as_table_mut().unwrap().remove("agent_capture");
    let mut expected = original;
    expected.as_table_mut().unwrap().remove("agent_capture");
    assert_eq!(actual, expected);
    assert_eq!(
        f.manager.save_settings(f.actor, command.clone()).await,
        Err(Error::Stale)
    );
    assert_eq!(
        f.manager.settings(PrincipalId::new([9; 16])).await,
        Err(Error::Denied)
    );
    command.expected_revision = saved.revision.clone();
    command.configuration.max_readers = 5;
    assert_eq!(
        f.manager.save_settings(f.actor, command.clone()).await,
        Err(Error::Invalid)
    );
    command.configuration.max_readers = 1;
    command.configuration.harnesses[0].product_version = "unsupported".into();
    assert_eq!(
        f.manager.save_settings(f.actor, command.clone()).await,
        Err(Error::Invalid)
    );
    command.configuration.harnesses[0].product_version =
        initial.desired.harnesses[0].product_version.clone();
    command.configuration.harnesses[0].profile = CaptureHarnessProfile::CursorDesktopTranscriptV1;
    assert_eq!(
        f.manager.save_settings(f.actor, command.clone()).await,
        Err(Error::Invalid)
    );
    command.configuration = saved.desired.clone();
    for root in ["relative/path", "/tmp/../source", "/", "/tmp/\nsource"] {
        command.configuration.harnesses[0].roots = vec![CaptureRootPath(root.into())];
        assert_eq!(
            f.manager.save_settings(f.actor, command.clone()).await,
            Err(Error::Invalid)
        );
    }
    assert_eq!(
        f.manager.settings(f.actor).await.unwrap().revision,
        saved.revision
    );
    // A recreated manager reflects the startup selection without touching disabled state.
    let config = Config::load(&f.source.root.path().join("server.toml")).unwrap();
    let next = Manager::new(
        &f.source.root.path().join("server.toml"),
        &config,
        &f.identity,
        None,
    )
    .unwrap();
    assert!(!next.settings(f.actor).await.unwrap().restart_required);
}
#[tokio::test]
async fn configuration_revalidates_admin_identity_lock_and_regular_file() {
    use std::os::unix::fs::symlink;
    let f = Fixture::new(true).await;
    let before = f.bytes();
    let initial = f.manager.settings(f.actor).await.unwrap();
    let mut command = SaveCaptureSettings {
        version: 1,
        expected_revision: initial.revision,
        configuration: initial.desired,
    };
    command.configuration.max_sources = 8;
    let path = f.source.root.path().join("server.toml");
    let lock = jj_store_server::policy::exclusive_lock(
        &f.source.root.path().join("server.toml.settings.lock"),
    )
    .unwrap();
    assert_eq!(
        f.manager.save_settings(f.actor, command.clone()).await,
        Err(Error::Busy)
    );
    drop(lock);
    let saved = f
        .manager
        .save_settings(f.actor, command.clone())
        .await
        .unwrap();
    assert!(saved.restart_required);
    assert_eq!(before, f.bytes());
    assert_eq!(
        f.manager
            .control(f.actor, f.command(1, 0, CaptureControlAction::Pause))
            .await,
        Err(Error::Unavailable)
    );
    let mut foreign = f.config.clone();
    let Access::LocalTrusted { permission, .. } = &mut foreign.access else {
        unreachable!()
    };
    *permission = Permission::Write;
    f.save(&foreign);
    assert_eq!(f.manager.settings(f.actor).await, Err(Error::Denied));
    f.save(&f.config);
    let target = f.source.root.path().join("alternate.toml");
    fs::rename(&path, &target).unwrap();
    symlink(&target, &path).unwrap();
    assert_eq!(f.manager.settings(f.actor).await, Err(Error::Unavailable));
    assert_eq!(
        f.manager.save_settings(f.actor, command).await,
        Err(Error::Unavailable)
    );
    assert_eq!(before, f.bytes());
}
