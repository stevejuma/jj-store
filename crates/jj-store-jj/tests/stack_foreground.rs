//! Foreground coordination uses a real stock-JJ fixture, without code writes.
#[path = "stack_foreground/fixture.rs"]
mod fixture;
#[path = "stack_foreground/lifecycle.rs"]
mod lifecycle;
use fixture::*;
use jj_store_jj::{local_browser::Error, local_execution::Coordinator};
use std::{fs, process::Command, time::Duration};

#[tokio::test]
async fn foreground_lease_matches_stock_jj_and_preserves_dirty_checkout() {
    let fixture = Fixture::new().await;
    let coordinator = Coordinator::new(2).unwrap();
    let before = fixture.state();
    let lease = coordinator
        .acquire(fixture.access(), fixture.root(), fixture.witness().await)
        .await
        .unwrap();
    assert_eq!(fixture.state(), before);
    lease.validate_metadata().await.unwrap();
    let mut child = Command::new("/opt/homebrew/bin/jj")
        .current_dir(fixture.root())
        .args([
            "--no-pager",
            "--config",
            "user.name='Fixture'",
            "--config",
            "user.email='fixture@invalid'",
            "describe",
            "-m",
            "CLI after lease",
        ])
        .env("JJ_CONFIG", "/dev/null")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .unwrap();
    tokio::time::sleep(Duration::from_millis(200)).await;
    assert!(
        child.try_wait().unwrap().is_none(),
        "stock JJ must wait on the same lock"
    );
    assert_eq!(fixture.state(), before);
    drop(lease);
    assert!(child.wait().unwrap().success());
    assert_eq!(
        fs::read(fixture.root().join("file.txt")).unwrap(),
        b"dirty external edit\n"
    );
    assert_eq!(
        fs::read(fixture.root().join("ignored.txt")).unwrap(),
        b"ignored obstacle\n"
    );
}

#[tokio::test]
async fn competing_foreground_lease_is_busy_and_release_restores_admission() {
    let fixture = Fixture::new().await;
    let first = Coordinator::new(1).unwrap();
    let second = Coordinator::new(1).unwrap();
    let witness = fixture.witness().await;
    let lease = first
        .acquire(fixture.access(), fixture.root(), witness.clone())
        .await
        .unwrap();
    assert!(matches!(
        second
            .acquire(fixture.access(), fixture.root(), witness.clone())
            .await,
        Err(Error::Busy)
    ));
    drop(lease);
    second
        .acquire(fixture.access(), fixture.root(), witness)
        .await
        .unwrap();
}

#[tokio::test]
async fn stale_selection_and_replaced_lock_fail_without_writing_checkout() {
    let fixture = Fixture::new().await;
    let coordinator = Coordinator::new(1).unwrap();
    let mut wrong = fixture.witness().await;
    wrong.baseline.view = jj_store_model::ViewId::new([91; 64]);
    let before = fixture.state();
    assert!(matches!(
        coordinator
            .acquire(fixture.access(), fixture.root(), wrong)
            .await,
        Err(Error::EpochChanged)
    ));
    let lease = coordinator
        .acquire(fixture.access(), fixture.root(), fixture.witness().await)
        .await
        .unwrap();
    let lock = fixture.root().join(".jj/working_copy/working_copy.lock");
    fs::remove_file(&lock).unwrap();
    fs::write(&lock, "replacement lock must be retained").unwrap();
    assert!(matches!(
        lease.validate_metadata().await,
        Err(Error::EpochChanged)
    ));
    drop(lease);
    assert_eq!(
        fs::read(&lock).unwrap(),
        b"replacement lock must be retained"
    );
    assert_eq!(fixture.state(), before);
}

#[tokio::test]
async fn unsafe_lock_entries_are_rejected_without_touching_their_targets() {
    let fixture = Fixture::new().await;
    let coordinator = Coordinator::new(1).unwrap();
    let witness = fixture.witness().await;
    let lock = fixture.root().join(".jj/working_copy/working_copy.lock");
    let target = fixture.root().join("ignored.txt");
    std::os::unix::fs::symlink(&target, &lock).unwrap();
    assert!(
        coordinator
            .acquire(fixture.access(), fixture.root(), witness.clone())
            .await
            .is_err()
    );
    assert_eq!(fs::read(&target).unwrap(), b"ignored obstacle\n");
    fs::remove_file(&lock).unwrap();
    fs::hard_link(&target, &lock).unwrap();
    assert!(matches!(
        coordinator
            .acquire(fixture.access(), fixture.root(), witness)
            .await,
        Err(Error::Invalid)
    ));
    assert_eq!(fs::read(&target).unwrap(), b"ignored obstacle\n");
}

#[tokio::test]
async fn missing_checkout_state_is_not_initialized_and_editor_bytes_are_separate() {
    let fixture = Fixture::new().await;
    let coordinator = Coordinator::new(1).unwrap();
    let expected = fixture.witness().await;
    let lease = coordinator
        .acquire(fixture.access(), fixture.root(), expected.clone())
        .await
        .unwrap();
    fs::write(fixture.root().join("file.txt"), "new editor bytes\n").unwrap();
    // A metadata check neither snapshots the edit nor claims a physical-byte guard.
    lease.validate_metadata().await.unwrap();
    assert_eq!(
        fs::read(fixture.root().join("file.txt")).unwrap(),
        b"new editor bytes\n"
    );
    drop(lease);
    let checkout = fixture.root().join(".jj/working_copy/checkout");
    fs::remove_file(&checkout).unwrap();
    assert!(
        coordinator
            .acquire(fixture.access(), fixture.root(), expected)
            .await
            .is_err()
    );
    assert!(!checkout.exists());
}
