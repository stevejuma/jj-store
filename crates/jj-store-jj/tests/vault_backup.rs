#![cfg(feature = "vault-backup")]
use jj_lib::{
    backend::CommitId, config::StackedConfig, default_backend_factories::default_backend_factories,
    repo::RepoLoader, settings::UserSettings,
};
use jj_store_jj::vault_workspace::{Binding, FileContents, Session};
use std::{fs, path::Path};
fn binding(root: &Path) -> Binding {
    Binding {
        vault_id: "test-vault".into(),
        root: root.join("notes"),
        state: root.join("private/history"),
    }
}
#[tokio::test]
async fn external_capture_retains_edit_bursts_and_never_creates_vault_metadata() {
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().canonicalize().unwrap();
    fs::create_dir(base.join("notes")).unwrap();
    let binding = binding(&base);
    fs::write(binding.root.join("Note.md"), "First version\n").unwrap();
    fs::write(binding.root.join(".gitignore"), "attachment.bin\n").unwrap();
    fs::write(binding.root.join("attachment.bin"), [0, 1, 2]).unwrap();
    fs::create_dir(binding.root.join(".lapis")).unwrap();
    fs::write(binding.root.join(".lapis/cache"), "private runtime").unwrap();
    let mut session = Session::open(binding.clone()).await.unwrap();
    let first = session.capture(&[], "First save").await.unwrap();
    assert!(first.changed);
    assert_ne!(
        first.checkpoint.as_ref().unwrap().commit,
        first.working_copy
    );
    let unchanged = session.capture(&[], "Unchanged").await.unwrap();
    assert!(!unchanged.changed);
    assert_eq!(unchanged.operation, first.operation);
    fs::write(binding.root.join("Note.md"), "Second version\n").unwrap();
    let second = session.capture(&[], "Second save").await.unwrap();
    drop(session);
    let reopened = Session::open(binding.clone()).await.unwrap();
    assert_eq!(
        reopened
            .file_at(&first.checkpoint.as_ref().unwrap().commit, "Note.md")
            .await
            .unwrap(),
        Some(FileContents::Regular {
            data: b"First version\n".to_vec(),
            executable: false
        })
    );
    assert_eq!(
        reopened
            .file_at(&first.checkpoint.as_ref().unwrap().commit, "attachment.bin")
            .await
            .unwrap(),
        Some(FileContents::Regular {
            data: vec![0, 1, 2],
            executable: false
        })
    );
    assert_eq!(
        reopened.checkpoint().unwrap().unwrap().commit,
        second.checkpoint.as_ref().unwrap().commit
    );
    assert!(!binding.root.join(".jj").exists());
    assert!(!binding.root.join(".git").exists());
    let settings =
        UserSettings::from_config_and_home_dir(StackedConfig::with_defaults(), None).unwrap();
    let loader = RepoLoader::init_from_file_system(
        &settings,
        &binding.metadata(),
        &default_backend_factories(),
    )
    .unwrap();
    let saved = |hex: &str| {
        loader
            .store()
            .get_commit(&CommitId::try_from_hex(hex).unwrap())
            .unwrap()
    };
    let before = saved(&first.checkpoint.unwrap().commit);
    let after = saved(&second.checkpoint.unwrap().commit);
    assert_eq!(after.parent_ids(), &[before.id().clone()]);
    let paths: Vec<_> = before
        .tree()
        .entries()
        .map(|(p, _)| p.as_internal_file_string().to_owned())
        .collect();
    assert!(
        paths.contains(&"attachment.bin".to_string()),
        "Git ignores cannot hide backup content"
    );
    assert!(!paths.iter().any(|p| p.starts_with(".lapis/")));
}
#[tokio::test]
async fn captures_nested_repo_contents_but_excludes_its_metadata_and_deleted_files() {
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().canonicalize().unwrap();
    fs::create_dir_all(base.join("notes/nested/.git")).unwrap();
    let binding = binding(&base);
    fs::write(binding.root.join("nested/Note.md"), "Nested note").unwrap();
    fs::write(binding.root.join("nested/.git/config"), "metadata").unwrap();
    let mut session = Session::open(binding.clone()).await.unwrap();
    assert!(session.capture(&[], "Nested").await.unwrap().changed);
    fs::remove_file(binding.root.join("nested/Note.md")).unwrap();
    assert!(session.capture(&[], "Deleted").await.unwrap().changed);
    assert!(!session.capture(&[], "Still deleted").await.unwrap().changed);
}
#[tokio::test]
async fn rejects_concurrent_commands_and_another_vault_binding() {
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().canonicalize().unwrap();
    fs::create_dir(base.join("notes")).unwrap();
    let binding = binding(&base);
    let session = Session::open(binding.clone()).await.unwrap();
    assert!(Session::open(binding.clone()).await.is_err());
    drop(session);
    let mut other = binding.clone();
    fs::create_dir(base.join("other")).unwrap();
    other.root = base.join("other");
    assert!(Session::open(other).await.is_err());
}
#[cfg(unix)]
#[tokio::test]
async fn relinking_the_same_physical_vault_preserves_history_and_rejects_replacement() {
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().canonicalize().unwrap();
    fs::create_dir(base.join("notes")).unwrap();
    let mut binding = binding(&base);
    fs::write(binding.root.join("Note.md"), "Retained version").unwrap();
    let mut session = Session::open(binding.clone()).await.unwrap();
    let checkpoint = session
        .capture(&[], "Before moving")
        .await
        .unwrap()
        .checkpoint
        .unwrap();
    drop(session);
    fs::rename(&binding.root, base.join("moved")).unwrap();
    binding.root = base.join("moved");
    assert!(Session::open(binding.clone()).await.is_err());
    binding.relink().unwrap();
    let mut session = Session::open(binding.clone()).await.unwrap();
    assert_eq!(
        session.checkpoint().unwrap().unwrap().commit,
        checkpoint.commit
    );
    assert!(!session.capture(&[], "After moving").await.unwrap().changed);
    drop(session);
    fs::rename(&binding.root, base.join("retained")).unwrap();
    fs::create_dir(&binding.root).unwrap();
    assert!(binding.relink().is_err());
}

#[tokio::test]
async fn transcript_files_are_backed_up_without_private_runtime_siblings() {
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().canonicalize().unwrap();
    let binding = binding(&base);
    let id = "7297795f-c00b-475e-b430-5b0e2e11a2e3";
    let relative = format!("Notes/.lapis/agents/sessions/{id}/transcript.jsonl");
    let directory = binding
        .root
        .join(format!("Notes/.lapis/agents/sessions/{id}"));
    fs::create_dir_all(&directory).unwrap();
    fs::write(directory.join("metadata.yaml"), "version: 1\n").unwrap();
    fs::write(directory.join("agents.jsonl"), "{}\n").unwrap();
    fs::write(directory.join("transcript.jsonl"), "first\n").unwrap();
    fs::write(directory.join("private.tmp"), "must stay private").unwrap();
    fs::create_dir_all(binding.root.join(".lapis/cache")).unwrap();
    fs::write(binding.root.join(".lapis/cache/private"), "secret").unwrap();
    let root_relative = format!(".lapis/agents/sessions/{id}/transcript.jsonl");
    fs::create_dir_all(binding.root.join(&root_relative).parent().unwrap()).unwrap();
    fs::write(binding.root.join(&root_relative), "root transcript\n").unwrap();
    let mut session = Session::open(binding.clone()).await.unwrap();
    let first = session
        .capture(&[], "Chat")
        .await
        .unwrap()
        .checkpoint
        .unwrap();
    assert_eq!(
        session.file_at(&first.commit, &relative).await.unwrap(),
        Some(FileContents::Regular {
            data: b"first\n".to_vec(),
            executable: false
        })
    );
    assert!(
        session
            .file_at(&first.commit, ".lapis/cache/private")
            .await
            .unwrap()
            .is_none()
    );
    assert!(
        session
            .file_at(&first.commit, &root_relative)
            .await
            .unwrap()
            .is_some()
    );
    for file in ["metadata.yaml", "agents.jsonl"] {
        let path = format!("Notes/.lapis/agents/sessions/{id}/{file}");
        assert!(
            session
                .file_at(&first.commit, &path)
                .await
                .unwrap()
                .is_some()
        );
    }
    assert!(
        session
            .file_at(
                &first.commit,
                &format!("Notes/.lapis/agents/sessions/{id}/private.tmp")
            )
            .await
            .unwrap()
            .is_none()
    );
    fs::write(directory.join("transcript.jsonl"), "second\n").unwrap();
    assert!(session.capture(&[], "Changed chat").await.unwrap().changed);
    assert!(
        !session
            .capture(&[], "Unchanged chat")
            .await
            .unwrap()
            .changed
    );
    let preview = session
        .preview_file_restore(&first.commit, &relative)
        .await
        .unwrap();
    session
        .restore_file(
            &jj_store_jj::vault_workspace::RestoreRequest {
                request_id: "restore-transcript".into(),
                checkpoint: first.commit.clone(),
                path: relative.clone(),
                expected: preview.expected,
            },
            &[],
        )
        .await
        .unwrap();
    assert_eq!(
        fs::read_to_string(directory.join("transcript.jsonl")).unwrap(),
        "first\n"
    );
    let excluded = session
        .capture(&["Notes/.lapis".into()], "Explicit exclusion")
        .await
        .unwrap()
        .checkpoint
        .unwrap();
    assert!(
        session
            .file_at(&excluded.commit, &relative)
            .await
            .unwrap()
            .is_none()
    );
    assert!(
        session
            .file_at(&excluded.commit, &root_relative)
            .await
            .unwrap()
            .is_some()
    );
    drop(session);
    let reopened = Session::open(binding).await.unwrap();
    assert!(
        reopened
            .file_at(&first.commit, &relative)
            .await
            .unwrap()
            .is_some()
    );
}
