use jj_vault::{
    Backup, Binding, NativeRepository, Phase, PushOutcome, RemoteTransfer, Timing,
    VaultRestoreRequest,
};
use std::{fs, process::Command};

#[test]
fn native_remote_import_and_restore_share_the_serialized_backup_worker() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(async {
        let temp = tempfile::tempdir().unwrap();
        let base = temp.path().canonicalize().unwrap();
        let origin = base.join("origin.git");
        assert!(
            Command::new("git")
                .args(["init", "--bare", "--quiet"])
                .arg(&origin)
                .status()
                .unwrap()
                .success()
        );
        let binding = |id: &str| {
            let root = base.join(id);
            fs::create_dir(&root).unwrap();
            Binding {
                vault_id: id.into(),
                root,
                state: base.join("private").join(id),
            }
        };
        let source = binding("source");
        fs::write(source.root.join("Note.md"), "backed up").unwrap();
        let mut session = jj_store_jj::vault_workspace::Session::open(source.clone())
            .await
            .unwrap();
        session.capture(&[], "Original").await.unwrap();
        drop(session);
        let transfer = RemoteTransfer::connect_local(source.clone(), &origin, "main")
            .await
            .unwrap();
        let session = jj_store_jj::vault_workspace::Session::open(source)
            .await
            .unwrap();
        let intent = session.prepare_push(&transfer, "seed", None).await.unwrap();
        drop(session);
        assert_eq!(
            transfer.publish(&intent).await.unwrap(),
            PushOutcome::Complete
        );
        let target = binding("target");
        let backup = Backup::start_idle(
            NativeRepository {
                binding: target.clone(),
                exclusions: vec![],
            },
            Timing::default(),
        )
        .unwrap();
        backup.capture().await.unwrap();
        let transfer = RemoteTransfer::connect_local(target.clone(), &origin, "main")
            .await
            .unwrap();
        let fetched = transfer.fetch().await.unwrap().unwrap();
        let (imported, captured) = tokio::join!(backup.import_remote(fetched), backup.capture());
        let imported = imported.unwrap();
        assert!(!captured.unwrap().changed);
        assert!(!target.root.join("Note.md").exists());
        let preview = backup
            .preview_vault_restore(imported.commit.clone())
            .await
            .unwrap();
        let request = VaultRestoreRequest {
            request_id: "remote-worker-restore".into(),
            checkpoint: imported.commit,
            expected: preview.expected,
        };
        let result = backup.restore_remote(request.clone()).await.unwrap();
        assert_eq!(
            fs::read_to_string(target.root.join("Note.md")).unwrap(),
            "backed up"
        );
        assert_eq!(backup.subscribe().borrow().phase, Phase::Ready);
        assert_eq!(
            backup
                .subscribe()
                .borrow()
                .checkpoint
                .as_ref()
                .unwrap()
                .commit,
            result.checkpoint.unwrap().commit
        );
        fs::write(target.root.join("Note.md"), "later local edit").unwrap();
        backup.capture().await.unwrap();
        backup.restore_remote(request).await.unwrap();
        assert_eq!(
            fs::read_to_string(target.root.join("Note.md")).unwrap(),
            "later local edit"
        );
        backup.close().await.unwrap();
    });
}
