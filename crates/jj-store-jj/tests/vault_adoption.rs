#![cfg(feature = "vault-backup")]
use jj_store_jj::vault_workspace::{AdoptionRequest, Binding, FileContents, Session};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
};
fn jj(root: &Path, args: &[&str]) {
    let result = Command::new("jj")
        .current_dir(root)
        .args([
            "--no-pager",
            "--config",
            "user.name='History fixture'",
            "--config",
            "user.email='fixture@invalid'",
        ])
        .args(args)
        .env("JJ_CONFIG", "/dev/null")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
}
fn bytes(root: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    let mut result = BTreeMap::new();
    let mut pending = vec![root.to_owned()];
    while let Some(path) = pending.pop() {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_dir() {
                pending.push(entry.path());
            } else {
                result.insert(
                    entry.path().strip_prefix(root).unwrap().to_owned(),
                    fs::read(entry.path()).unwrap(),
                );
            }
        }
    }
    result
}
fn fixture(base: &Path, colocated: bool) -> Binding {
    let root = base.join("notes");
    fs::create_dir(&root).unwrap();
    jj(
        &root,
        &[
            "git",
            "init",
            if colocated {
                "--colocate"
            } else {
                "--no-colocate"
            },
            ".",
        ],
    );
    fs::write(root.join("Note.md"), "Recorded original version\n").unwrap();
    jj(&root, &["commit", "-m", "Original note history"]);
    fs::write(root.join("Note.md"), "Recorded second version\n").unwrap();
    jj(&root, &["describe", "-m", "Recorded working revision"]);
    Binding {
        vault_id: "adoption-vault".into(),
        root,
        state: base.join("private/history"),
    }
}
#[tokio::test]
async fn adopts_recorded_git_ancestry_without_touching_source_metadata_or_saved_files() {
    for colocated in [false, true] {
        let temp = tempfile::tempdir().unwrap();
        let base = temp.path().canonicalize().unwrap();
        let binding = fixture(&base, colocated);
        fs::write(
            binding.root.join("Note.md"),
            "Saved changes not recorded by original JJ\n",
        )
        .unwrap();
        let original = bytes(&binding.root);
        let source = binding.existing_history().await.unwrap().unwrap();
        assert_eq!(
            bytes(&binding.root),
            original,
            "discovery must be immutable"
        );
        let mut session = Session::open(binding.clone()).await.unwrap();
        let request = AdoptionRequest {
            request_id: "original-import".into(),
            source: source.clone(),
        };
        let adopted = session.adopt_history(&request, &[]).await.unwrap();
        assert_eq!(
            bytes(&binding.root),
            original,
            "import must not modify the source or notes"
        );
        assert_eq!(
            session.file_at(&source.commit, "Note.md").await.unwrap(),
            Some(FileContents::Regular {
                data: b"Recorded second version\n".to_vec(),
                executable: false
            })
        );
        assert_eq!(
            session.file_at(&adopted.commit, "Note.md").await.unwrap(),
            Some(FileContents::Regular {
                data: b"Saved changes not recorded by original JJ\n".to_vec(),
                executable: false
            })
        );
        let before = session.operation();
        let replay = session.adopt_history(&request, &[]).await.unwrap();
        assert_eq!(replay.commit, adopted.commit);
        assert_eq!(session.operation(), before);
        assert!(binding.pending_adoption().unwrap().is_none());
        let another = AdoptionRequest {
            request_id: "already-imported".into(),
            source,
        };
        session.adopt_history(&another, &[]).await.unwrap();
        assert_eq!(
            session.operation(),
            before,
            "already retained ancestry creates no duplicate commit"
        );
    }
}
#[tokio::test]
async fn startup_capture_recovers_prepared_or_published_adoption_before_recording_later_edits() {
    for phase in ["retained", "published"] {
        let temp = tempfile::tempdir().unwrap();
        let base = temp.path().canonicalize().unwrap();
        let binding = fixture(&base, false);
        let request = AdoptionRequest {
            request_id: "original-interrupted".into(),
            source: binding.existing_history().await.unwrap().unwrap(),
        };
        let mut session = Session::open(binding.clone()).await.unwrap();
        assert!(
            session
                .adopt_history_with(&request, &[], |value| {
                    if value == phase {
                        Err(jj_store_jj::vault_workspace::Error::Busy)
                    } else {
                        Ok(())
                    }
                })
                .await
                .is_err()
        );
        drop(session);
        assert_eq!(binding.pending_adoption().unwrap(), Some(request.clone()));
        fs::write(
            binding.root.join("Note.md"),
            "Later saved edits survive recovery\n",
        )
        .unwrap();
        let mut reopened = Session::open(binding.clone()).await.unwrap();
        let capture = reopened.capture(&[], "Later edits").await.unwrap();
        let latest = capture.checkpoint.unwrap();
        assert!(binding.pending_adoption().unwrap().is_none());
        let operation = reopened.operation();
        let adopted = reopened.adopt_history(&request, &[]).await.unwrap();
        assert_ne!(adopted.commit, latest.commit);
        assert_eq!(reopened.operation(), operation);
        assert_eq!(
            fs::read_to_string(binding.root.join("Note.md")).unwrap(),
            "Later saved edits survive recovery\n"
        );
        assert!(
            reopened
                .file_at(&request.source.commit, "Note.md")
                .await
                .unwrap()
                .is_some()
        );
    }
}
#[tokio::test]
async fn rejects_a_substituted_source_pin_and_does_not_initialize_during_discovery() {
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().canonicalize().unwrap();
    let binding = fixture(&base, false);
    let mut source = binding.existing_history().await.unwrap().unwrap();
    source.commit = "f".repeat(40);
    let mut session = Session::open(binding.clone()).await.unwrap();
    let before = session.operation();
    let never_started = AdoptionRequest {
        request_id: "stopped-before-delivery".into(),
        source: source.clone(),
    };
    session.stop_adoption(&never_started.request_id).unwrap();
    assert!(session.adopt_history(&never_started, &[]).await.is_err());
    assert_eq!(session.operation(), before);
    assert!(
        session
            .adopt_history(
                &AdoptionRequest {
                    request_id: "substituted".into(),
                    source
                },
                &[]
            )
            .await
            .is_err()
    );
    assert_eq!(session.operation(), before);
    assert!(
        !binding
            .state
            .join("adoption-receipts/substituted.json")
            .exists()
    );
    let plain = base.join("plain");
    fs::create_dir(&plain).unwrap();
    let fresh = Binding {
        vault_id: "plain".into(),
        root: plain,
        state: base.join("private/plain"),
    };
    assert!(fresh.existing_history().await.unwrap().is_none());
    assert!(!fresh.state.exists());
    assert!(!fresh.root.join(".jj").exists());
    let valid = binding.existing_history().await.unwrap().unwrap();
    for operation in ["", "00", "../escape"] {
        let mut source = valid.clone();
        source.operation = operation.into();
        assert!(
            session
                .adopt_history(
                    &AdoptionRequest {
                        request_id: "bad-width".into(),
                        source
                    },
                    &[]
                )
                .await
                .is_err()
        );
    }
}

#[tokio::test]
async fn stopping_an_unfinished_import_retains_the_receipt_and_never_replays_it() {
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().canonicalize().unwrap();
    let binding = fixture(&base, false);
    let source = binding.existing_history().await.unwrap().unwrap();
    let request = AdoptionRequest {
        request_id: "stopped-original".into(),
        source: source.clone(),
    };
    let mut session = Session::open(binding.clone()).await.unwrap();
    session.capture(&[], "Before import").await.unwrap();
    let before = session.operation();
    assert!(
        session
            .adopt_history_with(&request, &[], |phase| {
                if phase == "prepared" {
                    Err(jj_store_jj::vault_workspace::Error::Busy)
                } else {
                    Ok(())
                }
            })
            .await
            .is_err()
    );
    session.stop_adoption(&request.request_id).unwrap();
    assert!(binding.pending_adoption().unwrap().is_none());
    assert!(session.adopt_history(&request, &[]).await.is_err());
    assert_eq!(session.operation(), before);
    let fresh = AdoptionRequest {
        request_id: "fresh-import".into(),
        source,
    };
    session.adopt_history(&fresh, &[]).await.unwrap();
    let current = session.operation();
    session.stop_adoption(&request.request_id).unwrap();
    assert_eq!(
        session.operation(),
        current,
        "old stop cannot undo accepted ancestry"
    );
}
