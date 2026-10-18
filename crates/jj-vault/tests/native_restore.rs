use jj_vault::{Backup, Binding, NativeRepository, Phase, RestoreRequest, Timing};
use std::fs;
#[test]
fn the_worker_serializes_restore_with_capture_and_publishes_its_status() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(async {
        let temp = tempfile::tempdir().unwrap();
        let base = temp.path().canonicalize().unwrap();
        let binding = Binding {
            vault_id: "worker-restore".into(),
            root: base.join("notes"),
            state: base.join("private/history"),
        };
        fs::create_dir(&binding.root).unwrap();
        fs::write(binding.root.join("Note.md"), "first").unwrap();
        let backup = Backup::start_idle(
            NativeRepository {
                binding: binding.clone(),
                exclusions: vec![],
            },
            Timing::default(),
        )
        .unwrap();
        let first = backup.capture().await.unwrap().checkpoint.unwrap();
        fs::write(binding.root.join("Note.md"), "second").unwrap();
        let preview = backup
            .preview_restore(first.commit.clone(), "Note.md".into())
            .await
            .unwrap();
        let restored = backup
            .restore(RestoreRequest {
                request_id: "worker-restore-1".into(),
                checkpoint: first.commit,
                path: preview.path,
                expected: preview.expected,
            })
            .await
            .unwrap();
        assert_eq!(
            fs::read_to_string(binding.root.join("Note.md")).unwrap(),
            "first"
        );
        let status = backup.subscribe().borrow().clone();
        assert_eq!(status.phase, Phase::Ready);
        assert_eq!(
            status.checkpoint.unwrap().commit,
            restored.checkpoint.unwrap().commit
        );
        backup.close().await.unwrap();
    });
}
