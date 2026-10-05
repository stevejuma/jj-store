use super::*;

pub(super) async fn prepared(f: &Installed) -> (StackCommandJournal, s::Preview, s::ApplyRequest) {
    let journal = f.journal();
    let saved = p::prepare_reword(
        &journal,
        &Coordinator::new(1).unwrap(),
        &f.authority,
        f.actor,
        f.request.clone(),
    )
    .await
    .unwrap();
    let apply = s::ApplyRequest {
        version: 1,
        request_id: f.request.request_id,
        intent_digest: f.request.intent_digest,
        prepared: saved.preview.prepared,
        selection: f.request.selection.clone(),
    };
    (journal, saved.preview, apply)
}
pub(super) fn lose_source(f: &Installed) {
    for name in ["op_store", "index"] {
        std::fs::remove_dir_all(f.folder.root().join(".jj/repo").join(name)).unwrap();
    }
    let gc = std::process::Command::new("git")
        .arg("--git-dir")
        .arg(f.folder.root().join(".git"))
        .args(["gc", "--prune=now"])
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .output()
        .unwrap();
    assert!(
        gc.status.success(),
        "{}",
        String::from_utf8_lossy(&gc.stderr)
    );
}
pub(super) async fn execute(
    f: &Installed,
    journal: &StackCommandJournal,
    apply: s::ApplyRequest,
) -> p::Result<jj_store_server::stack_command_journal::Selection> {
    p::apply_reword(
        journal,
        &Coordinator::new(1).unwrap(),
        &Observer::new(q::Executor::new(1).unwrap(), 1).unwrap(),
        &f.authority,
        f.actor,
        apply,
    )
    .await
}
fn assert_prepared(f: &Installed, journal: &StackCommandJournal, preview: &s::Preview) {
    let original = journal
        .status(f.actor, f.request.request_id, f.request.intent_digest)
        .unwrap()
        .unwrap();
    assert_eq!(original.preview, *preview);
    assert_eq!(original.receipt.state, s::ReceiptState::Prepared);
    assert!(original.execution.unwrap().checkout.is_none());
}

#[tokio::test]
async fn registered_first_apply_recovers_original_prepared_source_after_loss_and_gc() {
    let f = Installed::new().await;
    let (journal, preview, apply) = prepared(&f).await;
    let before = f.folder.state();
    lose_source(&f);
    drop(journal);
    let journal =
        StackCommandJournal::open(&f.state.join("stack-journal"), recovery::journal_scope(&f))
            .unwrap();
    let completed = execute(&f, &journal, apply.clone()).await.unwrap();
    assert_eq!(completed.preview, preview);
    assert!(completed.receipt.completed());
    let after = f.folder.state();
    assert_eq!(&after[..2], &before[..2]);
    assert_eq!(after[3], before[3]);
    assert_ne!(after[2], before[2]);
    assert!(f.folder.root().join(".jj/repo/op_store").exists());
    assert!(f.folder.root().join(".jj/repo/index").exists());
    // A terminal retry must remain historical even if source state is lost again.
    lose_source(&f);
    let replay = execute(&f, &journal, apply).await.unwrap();
    assert!(replay.replayed);
    assert_eq!(replay.receipt, completed.receipt);
    assert!(!f.folder.root().join(".jj/repo/op_store").exists());
    assert!(!f.folder.root().join(".jj/repo/index").exists());
    assert_eq!(f.folder.state(), after);
}

#[tokio::test]
async fn registered_recovered_first_apply_rechecks_combination_before_publication() {
    let f = Installed::new().await;
    let (journal, preview, apply) = prepared(&f).await;
    let before = f.folder.state();
    let selected = f.combinations.0.lock().unwrap().clone().unwrap();
    let mut generation = selected.clone();
    generation.selection.generation += 1;
    let mut code = selected.clone();
    code.code = s::CodeSelection::GitSha1Jj045 {
        operation: m::OperationId::new([99; 64]),
        view: m::ViewId::new([98; 64]),
        working_copy: m::GitSha1Id::new([97; 20]),
    };
    for (active, unavailable) in [(None, true), (Some(generation), false), (Some(code), false)] {
        lose_source(&f);
        *f.combinations.0.lock().unwrap() = active;
        let result = execute(&f, &journal, apply.clone()).await;
        if unavailable {
            assert!(matches!(result, Err(p::Error::Unsupported)), "{result:?}");
        } else {
            assert!(matches!(result, Err(p::Error::Stale)), "{result:?}");
        }
        assert_eq!(f.folder.state(), before);
        assert_prepared(&f, &journal, &preview);
    }
    *f.combinations.0.lock().unwrap() = Some(selected);
    assert!(
        execute(&f, &journal, apply)
            .await
            .unwrap()
            .receipt
            .completed()
    );
}

#[tokio::test]
async fn registered_recovered_first_apply_preserves_editor_changes_and_exact_physical_guard() {
    let f = Installed::new().await;
    let (journal, preview, apply) = prepared(&f).await;
    let before = f.folder.state();
    lose_source(&f);
    std::fs::write(f.folder.root().join("file.txt"), "new editor bytes\n").unwrap();
    let edited = f.folder.state();
    assert!(matches!(
        execute(&f, &journal, apply.clone()).await,
        Err(p::Error::Stale)
    ));
    assert_eq!(f.folder.state(), edited);
    assert_prepared(&f, &journal, &preview);
    std::fs::write(f.folder.root().join("file.txt"), &before[0]).unwrap();
    // Exact physical guards include identity/timestamps. Reverting bytes is
    // still an external edit, not proof that the original checkout is intact.
    let reverted = f.folder.state();
    assert!(matches!(
        execute(&f, &journal, apply).await,
        Err(p::Error::Stale)
    ));
    assert_eq!(f.folder.state(), reverted);
    assert_prepared(&f, &journal, &preview);
}

#[tokio::test]
async fn revoked_registration_cannot_restore_prepared_source_or_publish() {
    let f = Installed::new().await;
    let (journal, preview, apply) = prepared(&f).await;
    let before = f.folder.state();
    lose_source(&f);
    f.manager
        .remove(
            f.binding.repository,
            RemoveBinding {
                version: 1,
                expected_revision: f.manager.list().unwrap().revision,
            },
        )
        .unwrap();
    assert!(matches!(
        execute(&f, &journal, apply).await,
        Err(p::Error::Forbidden)
    ));
    assert_eq!(f.folder.state(), before);
    assert!(!f.folder.root().join(".jj/repo/op_store").exists());
    assert!(!f.folder.root().join(".jj/repo/index").exists());
    assert_prepared(&f, &journal, &preview);
}

#[tokio::test]
async fn newly_immutable_commit_rejects_original_prepared_apply_without_publication() {
    let f = Installed::new().await;
    let (journal, preview, apply) = prepared(&f).await;
    let before = f.folder.state();
    lose_source(&f);
    let s::Command::Reword {
        commit: s::CommitIdentity::GitSha1(commit),
        ..
    } = f.request.command
    else {
        panic!()
    };
    f.combinations
        .0
        .lock()
        .unwrap()
        .as_mut()
        .unwrap()
        .immutable_heads = vec![commit];
    assert!(matches!(
        execute(&f, &journal, apply.clone()).await,
        Err(p::Error::Provider(q::Error::Denied))
    ));
    assert_eq!(f.folder.state(), before);
    assert_prepared(&f, &journal, &preview);
    // Removing the new boundary permits the same retained candidate, not a new
    // preview. The source and physical selection have remained unchanged.
    f.combinations
        .0
        .lock()
        .unwrap()
        .as_mut()
        .unwrap()
        .immutable_heads = vec![m::GitSha1Id::new([0; 20])];
    let done = execute(&f, &journal, apply).await.unwrap();
    assert_eq!(done.preview, preview);
    assert!(done.receipt.completed());
}
