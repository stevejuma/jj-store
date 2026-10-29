use jj_vault::{AdoptionRequest, Backup, Binding, NativeRepository, Timing};
use std::{fs, process::Command};
#[tokio::test]
async fn worker_adopts_original_history_and_receipt_replay_keeps_the_current_status() {
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().canonicalize().unwrap();
    let root = base.join("notes");
    fs::create_dir(&root).unwrap();
    for args in [
        vec!["git", "init", "--no-colocate", "."],
        vec!["describe", "-m", "Recorded original history"],
    ] {
        if args[0] == "describe" {
            fs::write(root.join("Note.md"), "Original version\n").unwrap();
        }
        let result = Command::new("jj")
            .current_dir(&root)
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
    let binding = Binding {
        vault_id: "worker-adoption".into(),
        root,
        state: base.join("private/history"),
    };
    let source = binding.existing_history().await.unwrap().unwrap();
    let worker = Backup::start_idle(
        NativeRepository {
            binding: binding.clone(),
            exclusions: vec![],
        },
        Timing::default(),
    )
    .unwrap();
    worker.capture().await.unwrap();
    let request = AdoptionRequest {
        request_id: "original-request".into(),
        source: source.clone(),
    };
    let first = worker.adopt_history(request.clone()).await.unwrap();
    assert_eq!(
        worker
            .subscribe()
            .borrow()
            .checkpoint
            .as_ref()
            .unwrap()
            .commit,
        first.commit
    );
    fs::write(binding.root.join("Note.md"), "Later saved version\n").unwrap();
    let replay = worker.adopt_history(request).await.unwrap();
    assert_eq!(replay.commit, first.commit);
    let latest = worker
        .subscribe()
        .borrow()
        .checkpoint
        .as_ref()
        .unwrap()
        .commit
        .clone();
    assert_ne!(
        latest, first.commit,
        "current status must follow saved files, not the old receipt"
    );
    worker.close().await.unwrap();
    let native = jj_store_jj::vault_workspace::Session::open(binding.clone())
        .await
        .unwrap();
    assert!(
        native
            .file_at(&source.commit, "Note.md")
            .await
            .unwrap()
            .is_some()
    );
    assert_eq!(
        fs::read_to_string(binding.root.join("Note.md")).unwrap(),
        "Later saved version\n"
    );
}
