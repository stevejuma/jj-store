use super::*;
use crate::stack_combinations as c;
#[path = "../../../jj-store-jj/tests/stack_foreground/fixture.rs"]
pub(crate) mod fixture;
#[path = "../../tests/stack_combinations/support.rs"]
pub(crate) mod support;
use fixture::*;
use support::*;

#[tokio::test]
async fn acknowledgement_loss_and_replacement_recover_exact_original_pending_update() {
    let f = Prepared::new().await;
    let physical = f.folder.state();
    let path = f.private.path().join("combinations");
    let store = CombinationStore::create(&path, f.scope.clone(), f.actor).unwrap();
    let gate = store.root.gate().unwrap();
    let (mut catalog, stamp) = store.read(&gate).unwrap();
    let record = Record {
        preview: f.preview.clone(),
        before_revision: 0,
        after: f.after.clone(),
        retention: validation::retained(&f.scope, f.actor, &f.preview, &f.retained).unwrap(),
        phase: Phase::Pending,
    };
    catalog.records.push(record);
    let bytes = catalog.encode().unwrap();
    let held = f.private.path().join("held-lock");
    assert_eq!(
        store
            .root
            .persist_with(&gate, &bytes, Some(stamp), || {
                std::fs::rename(path.join("combinations.lock"), &held).unwrap();
                Ok(())
            })
            .map_err(Error::from),
        Err(Error::OutcomeUnknown)
    );
    drop(gate);
    assert_eq!(std::fs::read(path.join("state.json")).unwrap(), bytes);
    assert_eq!(store.snapshot(), Err(Error::Unavailable));
    std::fs::rename(held, path.join("combinations.lock")).unwrap();
    let reopened = CombinationStore::open(&path, f.scope.clone(), f.actor).unwrap();
    let outcome = reopened
        .stage(f.actor, 0, &f.preview, &f.after, &f.retained)
        .unwrap();
    assert!(outcome.replayed);
    assert_eq!(outcome.phase, Phase::Pending);
    assert_eq!(reopened.snapshot().unwrap().revision, 0);
    assert_eq!(std::fs::read(path.join("state.json")).unwrap(), bytes);
    f.mark_completed();
    reopened.finish(&f.journal, f.actor, &f.apply()).unwrap();
    assert_eq!(reopened.snapshot().unwrap(), f.after);
    assert_eq!(
        reopened.snapshot().unwrap().combinations[0].immutable_heads,
        vec![f.target]
    );
    assert_eq!(f.folder.state(), physical);
}

#[tokio::test]
async fn gate_stale_selection_and_malformed_generations_fail_before_replacement() {
    let f = Prepared::new().await;
    let path = f.private.path().join("combinations");
    let store = CombinationStore::create(&path, f.scope.clone(), f.actor).unwrap();
    let other = CombinationStore::open(&path, f.scope.clone(), f.actor).unwrap();
    let gate = store.root.gate().unwrap();
    assert_eq!(
        other.stage(f.actor, 0, &f.preview, &f.after, &f.retained),
        Err(Error::Unavailable)
    );
    let (catalog, stamp) = store.read(&gate).unwrap();
    let replacement = path.join(".stack-combination-replacement");
    std::fs::write(&replacement, catalog.encode().unwrap()).unwrap();
    use std::os::unix::fs::PermissionsExt as _;
    std::fs::set_permissions(&replacement, std::fs::Permissions::from_mode(0o600)).unwrap();
    std::fs::rename(replacement, path.join("state.json")).unwrap();
    assert_eq!(
        store
            .root
            .persist(&gate, &catalog.encode().unwrap(), Some(stamp))
            .map_err(Error::from),
        Err(Error::Unavailable)
    );
    drop(gate);
    let mut after = f.after.clone();
    after.combinations[0].selection.generation = 2;
    let bytes = std::fs::read(path.join("state.json")).unwrap();
    assert_eq!(
        store.stage(f.actor, 0, &f.preview, &after, &f.retained),
        Err(Error::IntentMismatch)
    );
    assert_eq!(
        store.stage(f.actor, 1, &f.preview, &f.after, &f.retained),
        Err(Error::Stale)
    );
    assert_eq!(std::fs::read(path.join("state.json")).unwrap(), bytes);
    store
        .stage(f.actor, 0, &f.preview, &f.after, &f.retained)
        .unwrap();
    let gate = store.root.gate().unwrap();
    let (mut catalog, _) = store.read(&gate).unwrap();
    catalog.records[0].after.combinations[0].code = s::CodeSelection::Native {
        operation: m::OperationId::new([1; 64]),
        view: m::ViewId::new([2; 64]),
        working_copy: m::CommitId::new([3; 64]),
    };
    assert!(catalog.encode().is_err());
}

#[tokio::test]
async fn process_loss_after_device_selection_reopens_exact_original_without_reexecution() {
    let f = Prepared::new().await;
    let path = f.private.path().join("combinations");
    let store = CombinationStore::create(&path, f.scope.clone(), f.actor).unwrap();
    store
        .stage(f.actor, 0, &f.preview, &f.after, &f.retained)
        .unwrap();
    f.mark_completed();
    let child = std::process::Command::new(std::env::current_exe().unwrap())
        .args([
            "--ignored",
            "--exact",
            "stack_combinations::tests::exit_after_device_selection",
            "--nocapture",
        ])
        .env("JJ_STORE_COMBINATION_LOSS_PATH", &path)
        .env(
            "JJ_STORE_COMBINATION_LOSS_SCOPE",
            serde_json::to_string(&f.scope).unwrap(),
        )
        .env("JJ_STORE_COMBINATION_LOSS_ACTOR", f.actor.to_string())
        .output()
        .unwrap();
    assert_eq!(
        child.status.code(),
        Some(94),
        "{}",
        String::from_utf8_lossy(&child.stderr)
    );
    let bytes = std::fs::read(path.join("state.json")).unwrap();
    let reopened = CombinationStore::open(&path, f.scope.clone(), f.actor).unwrap();
    assert_eq!(reopened.snapshot().unwrap(), f.after);
    assert!(
        reopened
            .finish(&f.journal, f.actor, &f.apply())
            .unwrap()
            .replayed
    );
    assert_eq!(std::fs::read(path.join("state.json")).unwrap(), bytes);
}
#[test]
#[ignore = "child exits after atomic selection before acknowledgement"]
fn exit_after_device_selection() {
    let path =
        std::path::PathBuf::from(std::env::var_os("JJ_STORE_COMBINATION_LOSS_PATH").unwrap());
    let scope: j::Scope =
        serde_json::from_str(&std::env::var("JJ_STORE_COMBINATION_LOSS_SCOPE").unwrap()).unwrap();
    let actor = std::env::var("JJ_STORE_COMBINATION_LOSS_ACTOR")
        .unwrap()
        .parse()
        .unwrap();
    let store = CombinationStore::open(&path, scope, actor).unwrap();
    let gate = store.root.gate().unwrap();
    let (mut catalog, stamp) = store.read(&gate).unwrap();
    catalog.selected = catalog.records[0].after.clone();
    catalog.records[0].phase = Phase::Selected;
    let bytes = catalog.encode().unwrap();
    let _ = store
        .root
        .persist_with(&gate, &bytes, Some(stamp), || std::process::exit(94));
    panic!("must exit after device-state selection");
}

#[tokio::test]
async fn multiple_combinations_preserve_anchors_and_historical_replay_never_rewinds() {
    let f = Prepared::new().await;
    let path = f.private.path().join("combinations");
    let store = CombinationStore::create(&path, f.scope.clone(), f.actor).unwrap();
    store
        .stage(f.actor, 0, &f.preview, &f.after, &f.retained)
        .unwrap();
    f.mark_completed();
    store.finish(&f.journal, f.actor, &f.apply()).unwrap();
    let mut preview = f.preview.clone();
    preview.plan.request.request_id = m::RequestId::new([77; 16]);
    preview.plan.request.selection.combination = Some(f.after.combinations[0].selection.clone());
    preview.plan.request.selection.code = f.after.combinations[0].code.clone();
    preview.plan.request.command = s::Command::CreateCombination {
        name: "Other".into(),
    };
    preview.plan.request.intent_digest = preview.plan.request.intent().unwrap();
    preview.plan.combination = s::CombinationSelection {
        id: s::CombinationId(m::RequestId::new([78; 16])),
        generation: 1,
    };
    preview.prepared = preview.plan.identity().unwrap();
    let key = j::preparation_key(f.actor, &preview);
    let metadata = jj_store_jj::local_metadata::MetadataStore::open(
        &f.folder.root().join(".git"),
        jj_store_jj::local_metadata::Limits::default(),
    )
    .unwrap();
    let jj_store_jj::local_metadata::StackRetentionOutcome::Retained(retained) = metadata
        .retain_stack_execution(&key, f.retained.history.as_ref().unwrap())
        .unwrap()
    else {
        panic!()
    };
    let mut other = f.after.combinations[0].clone();
    other.selection = preview.plan.combination.clone();
    other.name = "Other".into();
    other.retention = validation::retained(&f.scope, f.actor, &preview, &retained).unwrap();
    let mut after = f.after.clone();
    after.revision = 2;
    after.active = Some(other.selection.id);
    after.combinations.push(other);
    store
        .stage(f.actor, 1, &preview, &after, &retained)
        .unwrap();
    // Private catalog/reader acceptance only; real CreateCombination publication
    // is intentionally not inferred from this codec fixture selection.
    let gate = store.root.gate().unwrap();
    let (mut catalog, stamp) = store.read(&gate).unwrap();
    catalog.records[1].phase = Phase::Selected;
    catalog.selected = after.clone();
    store
        .root
        .persist(&gate, &catalog.encode().unwrap(), Some(stamp))
        .unwrap();
    drop(gate);
    let reopened = CombinationStore::open(&path, f.scope.clone(), f.actor).unwrap();
    assert_eq!(
        reopened.snapshot().unwrap().combinations[0],
        f.after.combinations[0]
    );
    assert!(
        reopened
            .stage(f.actor, 0, &f.preview, &f.after, &f.retained)
            .unwrap()
            .replayed
    );
    assert!(
        reopened
            .finish(&f.journal, f.actor, &f.apply())
            .unwrap()
            .replayed
    );
    assert_eq!(reopened.snapshot().unwrap(), after);
    let mut stale = preview.clone();
    stale.plan.request.request_id = m::RequestId::new([79; 16]);
    stale.prepared = stale.plan.identity().unwrap();
    let key = j::preparation_key(f.actor, &stale);
    let jj_store_jj::local_metadata::StackRetentionOutcome::Retained(stale_retained) = metadata
        .retain_stack_execution(&key, f.retained.history.as_ref().unwrap())
        .unwrap()
    else {
        panic!()
    };
    assert_eq!(
        reopened.stage(f.actor, 1, &stale, &after, &stale_retained),
        Err(Error::Stale)
    );
    let mut invalid = after.clone();
    invalid.combinations[1].name = invalid.combinations[0].name.clone();
    assert_eq!(
        validation::snapshot(&f.scope, f.actor, &invalid),
        Err(Error::Invalid)
    );
    invalid = after.clone();
    invalid.combinations[1].selection.id = invalid.combinations[0].selection.id;
    assert_eq!(
        validation::snapshot(&f.scope, f.actor, &invalid),
        Err(Error::Invalid)
    );
}

#[tokio::test]
async fn capacity_never_evicts_original_updates_or_selects_a_partial_record() {
    let f = Prepared::new().await;
    let path = f.private.path().join("combinations");
    let store = CombinationStore::create(&path, f.scope.clone(), f.actor).unwrap();
    let gate = store.root.gate().unwrap();
    let (mut catalog, stamp) = store.read(&gate).unwrap();
    for index in 0..MAX_RECORDS {
        let mut preview = f.preview.clone();
        let mut id = [200; 16];
        id[0] = index as u8;
        preview.plan.request.request_id = m::RequestId::new(id);
        preview.prepared = preview.plan.identity().unwrap();
        let mut retention = f.after.combinations[0].retention.clone();
        retention.key = j::preparation_key(f.actor, &preview);
        let mut after = f.after.clone();
        after.combinations[0].retention = retention.clone();
        catalog.records.push(Record {
            preview,
            before_revision: 0,
            after,
            retention,
            phase: Phase::Rejected,
        });
    }
    store
        .root
        .persist(&gate, &catalog.encode().unwrap(), Some(stamp))
        .unwrap();
    drop(gate);
    let before = std::fs::read(path.join("state.json")).unwrap();
    assert_eq!(
        store.stage(f.actor, 0, &f.preview, &f.after, &f.retained),
        Err(Error::Capacity)
    );
    assert_eq!(std::fs::read(path.join("state.json")).unwrap(), before);
    assert_eq!(store.snapshot().unwrap(), Snapshot::empty());
    let mut excessive = f.after.clone();
    excessive.combinations = vec![excessive.combinations[0].clone(); MAX_COMBINATIONS + 1];
    assert_eq!(
        validation::snapshot(&f.scope, f.actor, &excessive),
        Err(Error::Invalid)
    );
    assert_eq!(
        store.stage(f.actor, 0, &f.preview, &excessive, &f.retained),
        Err(Error::Invalid)
    );
    assert_eq!(std::fs::read(path.join("state.json")).unwrap(), before);
}
