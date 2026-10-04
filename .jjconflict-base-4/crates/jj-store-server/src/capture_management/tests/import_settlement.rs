use super::import_policy::ImportFixture;
use super::*;
use crate::{capture_import_state::Error as ImportError, capture_test_fixture::A};

fn settlement(f: &ImportFixture) -> CaptureImportSettlement {
    f.manager
        .begin_import(f.intent.clone(), f.imports.clone())
        .unwrap()
        .into_settlement()
        .unwrap()
}
#[tokio::test]
async fn owned_settlement_retains_policy_and_receipt_gates_across_native_authorization() {
    let mut f = ImportFixture::new().await;
    let binding = f.source_binding();
    let mut owned = settlement(&f);
    owned.check().unwrap();
    assert!(f.control.current().is_err());
    assert!(matches!(
        f.manager.begin_import(f.intent.clone(), f.imports.clone()),
        Err(Error::Busy)
    ));
    let selected = owned.select(&binding).unwrap();
    assert!(!selected.replayed);
    assert_eq!(owned.receipt(), Some(&selected.receipt));
    assert_eq!(f.imports.entries(), Err(ImportError::Unavailable));
    tokio::task::yield_now().await;
    let proof =
        f.f.admission
            .inspect(f.f.workspace(A).to_str().unwrap())
            .await
            .unwrap();
    let authority = crate::capture_authority::CaptureAuthority::prepare(&f.config)
        .unwrap()
        .unwrap();
    let request = authority
        .bind(&f.f.host, &f.f.admission, &proof)
        .await
        .unwrap();
    request.authorize(true).await.unwrap();
    owned.check().unwrap();
    assert!(f.control.current().is_err());
    assert_eq!(f.imports.entries(), Err(ImportError::Unavailable));
    assert!(
        f.f.sources
            .lookup_admitted(&f.f.qualified())
            .unwrap()
            .is_none()
    );
    assert!(f.f.spool.pending().unwrap().is_empty());
    drop(owned);
    assert_eq!(
        f.imports.resolve(&f.intent).unwrap(),
        Some(selected.receipt)
    );
    assert_eq!(f.control.current().unwrap().revision, 0);
    f.runtime.shutdown().await.unwrap();
}

#[tokio::test]
async fn selected_settlement_replay_ignores_offered_metadata_and_releases_exact_locks() {
    let mut f = ImportFixture::new().await;
    let binding = f.source_binding();
    let selected = f.imports.record(&f.intent, &binding).unwrap();
    f.select_control(8, 0, false);
    let mut original = settlement(&f);
    let mut unavailable = binding.clone();
    unavailable.prefix_digest = [0; 32];
    unavailable.complete_records = 0;
    let replayed = original.select(&unavailable).unwrap();
    assert!(replayed.replayed);
    assert_eq!(replayed.receipt, selected.receipt);
    let repeated = original.select(&unavailable).unwrap();
    assert!(repeated.replayed);
    assert_eq!(repeated.receipt, selected.receipt);
    original.check().unwrap();
    drop(original);
    assert_eq!(f.imports.entries().unwrap().len(), 1);
    assert_eq!(f.control.current().unwrap().revision, 1);
    f.runtime.shutdown().await.unwrap();
}

#[tokio::test]
async fn preselection_pause_disable_and_physical_loss_refuse_owned_settlement() {
    for mode in ["pause", "disable", "control", "imports"] {
        let mut f = ImportFixture::new().await;
        let policy = f
            .manager
            .begin_import(f.intent.clone(), f.imports.clone())
            .unwrap();
        match mode {
            "pause" => f.select_control(8, 0, true),
            "disable" => {
                let mut changed = f.config.clone();
                changed.agent_capture.enabled = false;
                f.save(&changed);
            }
            "control" => {
                fs::remove_file(f.f.root.path().join("control/state.json")).unwrap();
            }
            "imports" => {
                fs::remove_file(f.config.state_dir.join("agent-capture/imports/state.json"))
                    .unwrap();
            }
            _ => unreachable!(),
        }
        assert!(policy.into_settlement().is_err());
        if mode != "imports" {
            assert!(f.imports.entries().unwrap().is_empty());
        }
        assert!(f.f.spool.pending().unwrap().is_empty());
        f.runtime.shutdown().await.unwrap();
    }
}

#[tokio::test]
async fn postselection_policy_runtime_and_selected_file_loss_preserve_original_receipt() {
    for mode in ["authority", "pause", "control", "imports"] {
        let mut f = ImportFixture::new().await;
        let binding = f.source_binding();
        let mut owned = settlement(&f);
        let selected = owned.select(&binding).unwrap();
        match mode {
            "authority" => {
                let mut changed = f.config.clone();
                let Access::LocalTrusted { permission, .. } = &mut changed.access else {
                    unreachable!()
                };
                *permission = Permission::Read;
                f.save(&changed);
            }
            "pause" => {
                f.runtime.controls().pause().unwrap();
            }
            "control" | "imports" => {
                let path = if mode == "control" {
                    f.f.root.path().join("control")
                } else {
                    f.config.state_dir.join("agent-capture/imports")
                };
                let candidate = path.join(if mode == "control" {
                    ".capture-control-replacement"
                } else {
                    ".capture-import-replacement"
                });
                fs::copy(path.join("state.json"), &candidate).unwrap();
                fs::rename(candidate, path.join("state.json")).unwrap();
            }
            _ => unreachable!(),
        }
        assert_eq!(owned.check(), Err(Error::Unknown), "{mode}");
        drop(owned);
        assert_eq!(
            f.imports.resolve(&f.intent).unwrap(),
            Some(selected.receipt)
        );
        assert!(f.f.spool.pending().unwrap().is_empty());
        f.runtime.shutdown().await.unwrap();
    }
}

#[tokio::test(flavor = "current_thread")]
async fn caller_cancellation_does_not_release_settlement_gates_held_by_owned_admission() {
    let mut f = ImportFixture::new().await;
    let binding = f.source_binding();
    let owned = settlement(&f);
    let (entered, entry) = tokio::sync::oneshot::channel();
    let (release, wait) = tokio::sync::oneshot::channel();
    let (done, finished) = tokio::sync::oneshot::channel();
    let caller = tokio::spawn(async move {
        tokio::spawn(async move {
            let mut owned = owned;
            let selected = owned.select(&binding).unwrap();
            entered.send(()).unwrap();
            wait.await.unwrap();
            owned.check().unwrap();
            drop(owned);
            done.send(selected).unwrap();
        })
        .await
        .unwrap()
    });
    tokio::time::timeout(Duration::from_secs(10), entry)
        .await
        .unwrap()
        .unwrap();
    caller.abort();
    assert!(caller.await.unwrap_err().is_cancelled());
    assert!(f.control.current().is_err());
    assert_eq!(f.imports.entries(), Err(ImportError::Unavailable));
    assert!(matches!(
        f.manager.begin_import(f.intent.clone(), f.imports.clone()),
        Err(Error::Busy)
    ));
    release.send(()).unwrap();
    let selected = tokio::time::timeout(Duration::from_secs(10), finished)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        f.imports.resolve(&f.intent).unwrap(),
        Some(selected.receipt)
    );
    assert!(f.control.current().is_ok());
    f.runtime.shutdown().await.unwrap();
}
