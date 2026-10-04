use super::*;
use crate::{
    capture_test_fixture::Fixture,
    config::{Access, Config},
    installation::Identity,
};
use jj_store_model::RequestId;
use jj_store_service::auth::Permission;
use std::{fs, time::Duration};
mod import_policy;
mod import_settlement;
mod native_import;
mod observations;
mod rescan;
mod scheduled_import;
mod sources;

async fn prepared() -> (
    Fixture,
    Config,
    Manager,
    PrincipalId,
    CaptureControlCommand,
    CaptureControlJournal,
) {
    let f = Fixture::new(4 * 1024 * 1024).await;
    let path = f.root.path().join("server.toml");
    let mut config = Config::load(&path).unwrap();
    let Access::LocalTrusted { permission, .. } = &mut config.access else {
        unreachable!();
    };
    *permission = Permission::Admin;
    fs::write(&path, toml::to_string(&config).unwrap()).unwrap();
    let identity = Identity::load(&f.root.path().join("installation.json")).unwrap();
    let actor = identity.principal(b"loopback-http");
    let journal =
        CaptureControlJournal::create(&f.root.path().join("control"), [0xab; 16]).unwrap();
    let generation = journal.current().unwrap().generation;
    let manager = Manager::new(
        &path,
        &config,
        &identity,
        Some(CaptureManagementBinding {
            journal: journal.clone(),
            generation,
            runtime: Default::default(),
            imports: None,
        }),
    )
    .unwrap();
    let command = CaptureControlCommand {
        version: 1,
        request_id: RequestId::new([1; 16]),
        generation: hex::encode(generation),
        expected_revision: 0,
        action: CaptureControlAction::Pause,
    };
    (f, config, manager, actor, command, journal)
}

#[tokio::test(flavor = "current_thread")]
async fn caller_cancellation_retains_journal_gate_and_operation_until_owned_application_finishes() {
    let (f, _, manager, actor, command, journal) = prepared().await;
    let (entered, entry) = tokio::sync::oneshot::channel();
    let (release, wait) = std::sync::mpsc::channel();
    *manager.0.after_selection.lock().unwrap() = Some(Box::new(move || {
        let _ = entered.send(());
        wait.recv().unwrap();
    }));
    let owned = manager.clone();
    let request = command.clone();
    let caller = tokio::spawn(async move { owned.control(actor, request).await });
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
    release.send(()).unwrap();
    tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            match manager.status(actor).await {
                Err(Error::Busy) => tokio::time::sleep(Duration::from_millis(5)).await,
                Ok(status) => {
                    assert!(status.policy.unwrap().paused);
                    break;
                }
                other => panic!("unexpected settled control state: {other:?}"),
            }
        }
    })
    .await
    .unwrap();
    let replay = manager.control(actor, command).await.unwrap();
    assert!(replay.replayed && !replay.runtime_requested);
    assert_eq!(replay.receipt.revision, 1);
    assert!(f.spool.pending().unwrap().is_empty());
}

#[tokio::test(flavor = "current_thread")]
async fn revocation_after_selection_returns_unknown_and_exact_retry_recovers_receipt() {
    let (f, config, manager, actor, command, journal) = prepared().await;
    let path = f.root.path().join("server.toml");
    let mut denied = config.clone();
    let Access::LocalTrusted { permission, .. } = &mut denied.access else {
        unreachable!();
    };
    *permission = Permission::Write;
    let mutate = path.clone();
    *manager.0.after_selection.lock().unwrap() = Some(Box::new(move || {
        fs::write(mutate, toml::to_string(&denied).unwrap()).unwrap();
    }));
    assert_eq!(
        manager.control(actor, command.clone()).await,
        Err(Error::Unknown)
    );
    let selected = journal.current().unwrap();
    assert!(selected.paused && selected.revision == 1);
    assert_eq!(manager.status(actor).await, Err(Error::Denied));
    fs::write(path, toml::to_string(&config).unwrap()).unwrap();
    let replay = manager.control(actor, command).await.unwrap();
    assert!(replay.replayed && !replay.runtime_requested);
    assert_eq!(replay.receipt.revision, selected.revision);
    assert!(f.spool.pending().unwrap().is_empty());
}

#[tokio::test(flavor = "current_thread")]
async fn application_task_panic_preserves_selected_receipt_for_exact_replay() {
    let (f, _, manager, actor, command, journal) = prepared().await;
    *manager.0.after_selection.lock().unwrap() =
        Some(Box::new(|| panic!("capture application lost")));
    assert_eq!(
        manager.control(actor, command.clone()).await,
        Err(Error::Unknown)
    );
    assert_eq!(journal.current().unwrap().revision, 1);
    let replay = manager.control(actor, command).await.unwrap();
    assert!(replay.replayed && replay.receipt.paused && !replay.runtime_requested);
    assert_eq!(replay.status.policy.unwrap().revision, 1);
    assert!(f.spool.pending().unwrap().is_empty());
}

#[tokio::test(flavor = "current_thread")]
async fn policy_directory_rebinding_after_selection_withholds_application_until_exact_replay() {
    let (f, _, manager, actor, command, journal) = prepared().await;
    let control = f.root.path().join("control");
    let retained = f.root.path().join("retained-control");
    let replace = control.clone();
    let keep = retained.clone();
    *manager.0.after_selection.lock().unwrap() = Some(Box::new(move || {
        fs::rename(&replace, &keep).unwrap();
        use std::os::unix::fs::DirBuilderExt as _;
        fs::DirBuilder::new().mode(0o700).create(&replace).unwrap();
        for name in ["state.json", "control.lock"] {
            fs::copy(keep.join(name), replace.join(name)).unwrap();
        }
    }));
    assert_eq!(
        manager.control(actor, command.clone()).await,
        Err(Error::Unknown)
    );
    let bytes = fs::read(retained.join("state.json")).unwrap();
    assert_eq!(
        journal.current(),
        Err(crate::capture_control_state::Error::Unavailable)
    );
    fs::remove_dir_all(&control).unwrap();
    fs::rename(retained, &control).unwrap();
    let replay = manager.control(actor, command).await.unwrap();
    assert!(replay.replayed && replay.receipt.paused && !replay.runtime_requested);
    assert_eq!(fs::read(control.join("state.json")).unwrap(), bytes);
    assert!(f.spool.pending().unwrap().is_empty());
}

#[tokio::test]
async fn configuration_remains_editable_while_capture_inspection_holds_the_runtime_gate() {
    let (_fixture, _, manager, actor, _, journal) = prepared().await;
    let before = journal.current().unwrap();
    let _inspection = manager.0.gate.lock().await;
    let initial = manager.settings(actor).await.unwrap();
    let mut desired = initial.desired;
    desired.max_readers = 2;
    let saved = manager
        .save_settings(
            actor,
            SaveCaptureSettings {
                version: 1,
                expected_revision: initial.revision,
                configuration: desired,
            },
        )
        .await
        .unwrap();
    assert_eq!(saved.desired.max_readers, 2);
    assert!(saved.restart_required);
    assert_eq!(journal.current().unwrap(), before);
}
