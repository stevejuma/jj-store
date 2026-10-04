use super::*;
use crate::capture_worker::executor::OwnedWork;

async fn accepted() -> (
    ImportFixture,
    crate::capture_import_state::CaptureImportReceipt,
) {
    let mut f = ImportFixture::new().await;
    f.f.message("old", "history accepted before recovery policy checks");
    schedule(&f).await.unwrap();
    published(&f, "history accepted before recovery policy checks").await;
    pause(&f).await;
    let receipt = f.imports.entries().unwrap().remove(0);
    f.runtime.shutdown().await.unwrap();
    (f, receipt)
}
fn progress(f: &ImportFixture) -> jj_store_capture::reader_progress::ReaderSourceProgress {
    let reader = f.imports.reader_binding(&f.intent).unwrap().unwrap();
    f.f.spool
        .source_reader_progress(
            serde_json::from_value(serde_json::json!("codex")).unwrap(),
            reader.source,
            reader.generation_digest,
        )
        .unwrap()
}

#[tokio::test]
async fn accepted_restart_preserves_durable_pause_and_recovers_after_resume() {
    let (mut f, original) = accepted().await;
    let before = progress(&f);
    f.select_control(7, 0, true);
    start_runtime(&mut f);
    f.f.message("tail", "automatic recovery after durable pause resumes");
    tokio::time::sleep(Duration::from_millis(150)).await;
    assert_eq!(
        f.runtime.snapshot().phase,
        crate::capture_worker::CaptureWorkerPhase::Paused
    );
    assert_eq!(progress(&f), before);
    assert!(f.f.spool.pending().unwrap().is_empty());
    f.select_control(8, 1, false);
    resume(&f).await;
    published(&f, "automatic recovery after durable pause resumes").await;
    pause(&f).await;
    assert_eq!(f.imports.entries().unwrap(), vec![original]);
    f.runtime.shutdown().await.unwrap();
}

#[tokio::test]
async fn accepted_restart_renews_repository_binding_before_capturing_tail() {
    let (mut f, original) = accepted().await;
    let before = progress(&f);
    f.f.catalog(&[crate::capture_test_fixture::B]);
    start_runtime(&mut f);
    f.f.message("tail", "tail after restoring current repository binding");
    tokio::time::sleep(Duration::from_millis(1200)).await;
    pause(&f).await;
    assert_eq!(progress(&f), before);
    assert!(f.f.spool.pending().unwrap().is_empty());
    f.f.catalog(&[A, crate::capture_test_fixture::B]);
    resume(&f).await;
    published(&f, "tail after restoring current repository binding").await;
    pause(&f).await;
    assert_eq!(f.imports.entries().unwrap(), vec![original]);
    f.runtime.shutdown().await.unwrap();
}

#[tokio::test]
async fn accepted_restart_cannot_transfer_consent_to_a_replacement_physical_file() {
    let (mut f, original) = accepted().await;
    let before = progress(&f);
    f.f.message("tail", "replacement file cannot inherit historical consent");
    let replacement = f.f.root.path().join("replacement.jsonl");
    fs::write(&replacement, fs::read(&f.f.transcript).unwrap()).unwrap();
    fs::rename(replacement, &f.f.transcript).unwrap();
    start_runtime(&mut f);
    tokio::time::timeout(Duration::from_secs(10), async {
        while f.runtime.snapshot().status.unavailable == 0 {
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .unwrap();
    pause(&f).await;
    assert_eq!(progress(&f), before);
    assert!(f.f.spool.pending().unwrap().is_empty());
    assert!(
        f.f.sources
            .lookup_admitted(&f.f.qualified())
            .unwrap()
            .is_none()
    );
    assert_eq!(f.imports.entries().unwrap(), vec![original]);
    f.runtime.shutdown().await.unwrap();
}

#[tokio::test]
async fn recovery_withholds_disabled_and_missing_state_without_repair_or_enrollment() {
    let mut f = ImportFixture::new().await;
    f.runtime.shutdown().await.unwrap();
    start_runtime(&mut f);
    let mut recovery = f.manager.import_recovery().unwrap();
    let state = f.config.state_dir.join("agent-capture/imports/state.json");
    f.f.message(
        "unaccepted",
        "inactive history has no accepted import consent",
    );
    tokio::time::sleep(Duration::from_millis(150)).await;
    assert!(recovery.step(()).await.unwrap().is_none());
    assert!(
        f.f.sources
            .lookup_admitted(&f.f.qualified())
            .unwrap()
            .is_none()
    );
    assert!(f.f.spool.pending().unwrap().is_empty());
    let mut disabled = f.config.clone();
    disabled.agent_capture.enabled = false;
    f.save(&disabled);
    fs::remove_file(&state).unwrap();
    assert!(matches!(recovery.step(()).await, Err(Error::Disabled)));
    assert!(!state.exists());
    f.save(&f.config);
    assert!(matches!(recovery.step(()).await, Err(Error::Unavailable)));
    assert!(!state.exists());
    f.runtime.shutdown().await.unwrap();
}

#[tokio::test]
async fn paused_recovery_settles_while_management_holds_its_policy_gate() {
    let (mut f, original) = accepted().await;
    let before = progress(&f);
    start_runtime(&mut f);
    // The new runtime has not dispatched yet. Hold inspection coordination while
    // discovery settles and the recovery lane queues its fair policy acquisition.
    let permit = f.manager.0.gate.clone().lock_owned().await;
    f.f.message(
        "tail",
        "tail after cancelling queued recovery policy acquisition",
    );
    tokio::time::timeout(Duration::from_secs(10), async {
        while f.runtime.snapshot().status.observed_sources != 1 {
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .unwrap();
    tokio::time::sleep(Duration::from_millis(150)).await;
    pause(&f).await;
    assert_eq!(progress(&f), before);
    assert!(f.f.spool.pending().unwrap().is_empty());
    drop(permit);
    assert!(f.manager.0.gate.try_lock().is_ok());
    resume(&f).await;
    published(
        &f,
        "tail after cancelling queued recovery policy acquisition",
    )
    .await;
    pause(&f).await;
    assert_eq!(f.imports.entries().unwrap(), vec![original]);
    f.runtime.shutdown().await.unwrap();
}
