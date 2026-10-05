#![cfg(unix)]
#[path = "../../jj-store-jj/tests/stack_foreground/fixture.rs"]
mod fixture;
#[path = "stack_combinations/support.rs"]
mod support;
use fixture::*;
use jj_store_server::stack_combinations::{self as c, CombinationStore};
use jj_store_server::stack_preparation::CombinationAuthority;
use support::*;

#[tokio::test]
async fn reads_never_initialize_and_selected_combinations_survive_reopen() {
    let f = Prepared::new().await;
    let path = f.private.path().join("combinations");
    assert!(CombinationStore::open(&path, f.scope.clone(), f.actor).is_err());
    assert!(!path.exists());
    let store = CombinationStore::create(&path, f.scope.clone(), f.actor).unwrap();
    assert!(store.snapshot().unwrap().active.is_none());
    assert!(
        store
            .current(
                f.scope.repository,
                f.scope.physical_workspace,
                &f.scope.jj_workspace
            )
            .await
            .unwrap()
            .is_none()
    );
    let before = f.folder.state();
    let staged = store
        .stage(f.actor, 0, &f.preview, &f.after, &f.retained)
        .unwrap();
    assert_eq!(staged.phase, c::Phase::Pending);
    assert!(
        store
            .current(
                f.scope.repository,
                f.scope.physical_workspace,
                &f.scope.jj_workspace
            )
            .await
            .is_err()
    );
    assert!(store.finish(&f.journal, f.actor, &f.apply()).is_err());
    f.mark_completed(); // Metadata state-owner evidence; no actual code publication claimed.
    let complete = store.finish(&f.journal, f.actor, &f.apply()).unwrap();
    assert_eq!(complete.phase, c::Phase::Selected);
    let reopened = CombinationStore::open(&path, f.scope.clone(), f.actor).unwrap();
    assert_eq!(reopened.snapshot().unwrap(), f.after);
    let active = reopened
        .current(
            f.scope.repository,
            f.scope.physical_workspace,
            &f.scope.jj_workspace,
        )
        .await
        .unwrap()
        .unwrap();
    assert_eq!(active.selection, f.preview.plan.combination);
    assert_eq!(active.immutable_heads, vec![f.target]);
    let bytes = std::fs::read(path.join("state.json")).unwrap();
    assert_eq!(
        reopened
            .stage(f.actor, 0, &f.preview, &f.after, &f.retained)
            .unwrap()
            .phase,
        c::Phase::Selected
    );
    assert!(
        reopened
            .finish(&f.journal, f.actor, &f.apply())
            .unwrap()
            .replayed
    );
    assert_eq!(std::fs::read(path.join("state.json")).unwrap(), bytes);
    assert_eq!(f.folder.state(), before);
}

#[tokio::test]
async fn wrong_principal_scope_intent_and_unretained_code_fail_without_selection() {
    let f = Prepared::new().await;
    let path = f.private.path().join("combinations");
    let store = CombinationStore::create(&path, f.scope.clone(), f.actor).unwrap();
    let bytes = std::fs::read(path.join("state.json")).unwrap();
    assert!(
        store
            .stage(
                jj_store_model::PrincipalId::new([99; 16]),
                0,
                &f.preview,
                &f.after,
                &f.retained
            )
            .is_err()
    );
    let mut changed = f.after.clone();
    changed.combinations[0].merge = jj_store_model::GitSha1Id::new([99; 20]);
    assert!(
        store
            .stage(f.actor, 0, &f.preview, &changed, &f.retained)
            .is_err()
    );
    changed = f.after.clone();
    changed.combinations[0].target.head = jj_store_service::stack_workspace::CommitIdentity::Native(
        jj_store_model::CommitId::new([99; 64]),
    );
    assert!(
        store
            .stage(f.actor, 0, &f.preview, &changed, &f.retained)
            .is_err()
    );
    assert_eq!(std::fs::read(path.join("state.json")).unwrap(), bytes);
    store
        .stage(f.actor, 0, &f.preview, &f.after, &f.retained)
        .unwrap();
    let selected = std::fs::read(path.join("state.json")).unwrap();
    changed = f.after.clone();
    changed.combinations[0].name = "other intent".into();
    assert!(
        store
            .stage(f.actor, 0, &f.preview, &changed, &f.retained)
            .is_err()
    );
    let mut scope = f.scope.clone();
    scope.installation = [98; 16];
    assert!(CombinationStore::open(&path, scope, f.actor).is_err());
    assert!(
        store
            .current(
                jj_store_model::RepositoryId::new([99; 16]),
                f.scope.physical_workspace,
                &f.scope.jj_workspace
            )
            .await
            .is_err()
    );
    assert_eq!(std::fs::read(path.join("state.json")).unwrap(), selected);
}

#[tokio::test]
async fn rejected_update_retains_original_outcome_without_initializing_setup() {
    let f = Prepared::new().await;
    let path = f.private.path().join("combinations");
    let store = CombinationStore::create(&path, f.scope.clone(), f.actor).unwrap();
    store
        .stage(f.actor, 0, &f.preview, &f.after, &f.retained)
        .unwrap();
    f.journal
        .transition(
            f.actor,
            &f.apply(),
            &jj_store_service::stack_workspace::ReceiptState::Prepared,
            &jj_store_service::stack_workspace::ReceiptState::Rejected {
                reason: "Canceled before publication".into(),
            },
        )
        .unwrap();
    assert_eq!(
        store.finish(&f.journal, f.actor, &f.apply()).unwrap().phase,
        c::Phase::Rejected
    );
    assert!(store.snapshot().unwrap().active.is_none());
    let bytes = std::fs::read(path.join("state.json")).unwrap();
    assert_eq!(
        store
            .stage(f.actor, 0, &f.preview, &f.after, &f.retained)
            .unwrap()
            .phase,
        c::Phase::Rejected
    );
    assert!(
        store
            .finish(&f.journal, f.actor, &f.apply())
            .unwrap()
            .replayed
    );
    assert_eq!(std::fs::read(path.join("state.json")).unwrap(), bytes);
    assert!(
        store
            .current(
                f.scope.repository,
                f.scope.physical_workspace,
                &f.scope.jj_workspace
            )
            .await
            .unwrap()
            .is_none()
    );
}

#[tokio::test]
async fn selected_metadata_and_retained_code_reopen_after_source_loss_and_gc() {
    let f = Prepared::new().await;
    let path = f.private.path().join("combinations");
    let store = CombinationStore::create(&path, f.scope.clone(), f.actor).unwrap();
    store
        .stage(f.actor, 0, &f.preview, &f.after, &f.retained)
        .unwrap();
    f.mark_completed();
    store.finish(&f.journal, f.actor, &f.apply()).unwrap();
    let physical = f.folder.state();
    std::fs::remove_dir_all(f.folder.root().join(".jj/repo/op_store")).unwrap();
    std::fs::remove_dir_all(f.folder.root().join(".jj/repo/index")).unwrap();
    let gc = std::process::Command::new("/opt/homebrew/bin/git")
        .args(["gc", "--prune=now"])
        .current_dir(f.folder.root())
        .output()
        .unwrap();
    assert!(
        gc.status.success(),
        "{}",
        String::from_utf8_lossy(&gc.stderr)
    );
    let metadata = jj_store_jj::local_metadata::MetadataStore::open(
        &f.folder.root().join(".git"),
        jj_store_jj::local_metadata::Limits::default(),
    )
    .unwrap();
    let retained = metadata
        .read_stack_prepared(&f.retained.key)
        .unwrap()
        .unwrap();
    assert_eq!(retained, f.retained);
    let reopened = CombinationStore::open(&path, f.scope.clone(), f.actor).unwrap();
    assert_eq!(reopened.snapshot().unwrap(), f.after);
    assert!(
        reopened
            .stage(f.actor, 0, &f.preview, &f.after, &retained)
            .unwrap()
            .replayed
    );
    assert_eq!(f.folder.state(), physical);
    assert!(!f.folder.root().join(".jj/repo/op_store").exists());
    assert!(!f.folder.root().join(".jj/repo/index").exists());
}
