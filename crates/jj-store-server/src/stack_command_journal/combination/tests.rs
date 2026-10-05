//! Metadata protocol/fault evidence only. Native completion is qualified by the
//! separate real-folder CLI coordination tests; this fixture does not attest it.
use super::*;
use crate::stack_checkout_plans as plans;
use crate::stack_combinations::tests::support;
use support::Prepared;
struct Metadata {
    fixture: Prepared,
    journal: StackCommandJournal,
    store: CombinationStore,
    apply: s::ApplyRequest,
    after: Snapshot,
    completion: NativeCompletionV1,
}
impl Metadata {
    async fn new() -> Self {
        let f = Prepared::new().await;
        let mut preview = f.preview.clone();
        preview.plan.request.request_id = m::RequestId::new([101; 16]);
        // A declared tree-changing protocol fixture; no engine effect claimed.
        preview.plan.checkout.after = s::TreeIdentity::GitSha1(m::GitSha1Id::new([96; 20])).into();
        preview.prepared = preview.plan.identity().unwrap();
        let metadata = jj_store_jj::local_metadata::MetadataStore::open(
            &f.folder.root().join(".git"),
            Default::default(),
        )
        .unwrap();
        let key = preparation_key(f.actor, &preview);
        let jj_store_jj::local_metadata::StackRetentionOutcome::Retained(retained) = metadata
            .retain_stack_execution(&key, f.retained.history.as_ref().unwrap())
            .unwrap()
        else {
            panic!()
        };
        let execution = LocalExecutionV1::from_retention(
            f.actor,
            &preview,
            f.journal
                .status(f.actor, f.apply().request_id, f.apply().intent_digest)
                .unwrap()
                .unwrap()
                .execution
                .unwrap()
                .witness,
            &retained,
        )
        .unwrap();
        let journal =
            StackCommandJournal::create(&f.private.path().join("joined-journal"), f.scope.clone())
                .unwrap();
        journal
            .record_local_prepared(f.actor, &preview, &execution)
            .unwrap();
        let store = CombinationStore::create(
            &f.private.path().join("combinations"),
            f.scope.clone(),
            f.actor,
        )
        .unwrap();
        let apply = s::ApplyRequest {
            version: 1,
            request_id: preview.plan.request.request_id,
            intent_digest: preview.plan.request.intent_digest,
            prepared: preview.prepared,
            selection: preview.plan.request.selection.clone(),
        };
        let mut after = f.after.clone();
        after.combinations[0].retention.key = key;
        after.combinations[0].retention.commit = retained.commit;
        journal
            .save_local_combination(&store, f.actor, &apply, 0, &after, &retained)
            .unwrap();
        let completion = NativeCompletionV1 {
            version: 1,
            physical: m::RecordDigest::new([97; 32]),
            proof: CompletionKind::Tree {
                plan: m::RecordDigest::new([98; 32]),
                inventory: m::RecordDigest::new([99; 32]),
                frame: m::RecordDigest::new([100; 32]),
            },
        };
        let gate = journal.root.gate().unwrap();
        let (mut catalog, stamp) = journal.read(&gate).unwrap();
        let record = &mut catalog.records[0];
        let execution = record.execution.as_mut().unwrap();
        execution.tree_checkout = Some(plans::TreeCheckoutReferenceV1 {
            id: m::RecordDigest::new([98; 32]),
            encoded_bytes: 1,
        });
        execution.physical_inventory = Some(plans::PhysicalInventoryReferenceV1 {
            id: m::RecordDigest::new([99; 32]),
            plan: m::RecordDigest::new([98; 32]),
            encoded_bytes: 1,
        });
        record.receipt.state = s::ReceiptState::Published {
            operation: preview.plan.operation,
            view: preview.plan.view,
            checkout: s::CheckoutState::Required,
        };
        journal
            .root
            .persist(&gate, &catalog.encode().unwrap(), Some(stamp))
            .unwrap();
        drop(gate);
        journal
            .completion(f.actor, &apply, |_| Ok(completion.clone()), None)
            .unwrap();
        Self {
            fixture: f,
            journal,
            store,
            apply,
            after,
            completion,
        }
    }
    fn record(&self) -> Record {
        self.journal
            .status(
                self.fixture.actor,
                self.apply.request_id,
                self.apply.intent_digest,
            )
            .unwrap()
            .unwrap()
    }
}
#[tokio::test]
async fn loss_after_combination_selection_keeps_receipt_pending_and_exact_retry_finishes() {
    let f = Metadata::new().await;
    let child = std::process::Command::new(std::env::current_exe().unwrap())
        .args([
            "--ignored",
            "--exact",
            "stack_command_journal::combination::tests::exit_after_combination_before_receipt",
            "--nocapture",
        ])
        .env("JJ_STORE_JOIN_LOSS_PATH", f.fixture.private.path())
        .env(
            "JJ_STORE_JOIN_LOSS_SCOPE",
            serde_json::to_string(&f.fixture.scope).unwrap(),
        )
        .env("JJ_STORE_JOIN_LOSS_ACTOR", f.fixture.actor.to_string())
        .env(
            "JJ_STORE_JOIN_LOSS_APPLY",
            serde_json::to_string(&f.apply).unwrap(),
        )
        .output()
        .unwrap();
    assert_eq!(
        child.status.code(),
        Some(99),
        "{}",
        String::from_utf8_lossy(&child.stderr)
    );
    assert_eq!(f.store.snapshot().unwrap(), f.after);
    assert!(!f.record().receipt.completed());
    assert_eq!(f.journal.pending(f.fixture.actor).unwrap().len(), 1);
    let combination_bytes =
        std::fs::read(f.fixture.private.path().join("combinations/state.json")).unwrap();
    let selected = f
        .journal
        .completion(
            f.fixture.actor,
            &f.apply,
            |_| Ok(f.completion.clone()),
            Some(&f.store),
        )
        .unwrap();
    assert!(selected.receipt.completed());
    assert_eq!(
        std::fs::read(f.fixture.private.path().join("combinations/state.json")).unwrap(),
        combination_bytes
    );
    assert!(
        f.journal
            .completion(
                f.fixture.actor,
                &f.apply,
                |_| Ok(f.completion.clone()),
                Some(&f.store)
            )
            .unwrap()
            .replayed
    );
}
#[test]
#[ignore = "isolated metadata worker exits after selected combination before journal success"]
fn exit_after_combination_before_receipt() {
    let path = std::path::PathBuf::from(std::env::var_os("JJ_STORE_JOIN_LOSS_PATH").unwrap());
    let scope: Scope =
        serde_json::from_str(&std::env::var("JJ_STORE_JOIN_LOSS_SCOPE").unwrap()).unwrap();
    let actor = std::env::var("JJ_STORE_JOIN_LOSS_ACTOR")
        .unwrap()
        .parse()
        .unwrap();
    let apply: s::ApplyRequest =
        serde_json::from_str(&std::env::var("JJ_STORE_JOIN_LOSS_APPLY").unwrap()).unwrap();
    let journal = StackCommandJournal::open(&path.join("joined-journal"), scope.clone()).unwrap();
    let store = CombinationStore::open(&path.join("combinations"), scope, actor).unwrap();
    let record = journal
        .status(actor, apply.request_id, apply.intent_digest)
        .unwrap()
        .unwrap();
    let _gate = journal.root.gate().unwrap();
    store.select_completed(&record).unwrap();
    std::process::exit(99);
}
#[tokio::test]
async fn final_receipt_sync_loss_is_unknown_and_replay_keeps_selected_combination() {
    let f = Metadata::new().await;
    let record = f.record();
    let gate = f.journal.root.gate().unwrap();
    let (mut catalog, stamp) = f.journal.read(&gate).unwrap();
    f.store.select_completed(&record).unwrap();
    catalog.records[0].receipt.state = s::ReceiptState::Published {
        operation: record.preview.plan.operation,
        view: record.preview.plan.view,
        checkout: s::CheckoutState::Verified {
            fingerprint: f.completion.physical(),
        },
    };
    let bytes = catalog.encode().unwrap();
    let lock = f.fixture.private.path().join("joined-journal/stacks.lock");
    let held = f.fixture.private.path().join("held");
    assert_eq!(
        f.journal
            .root
            .persist_with(&gate, &bytes, Some(stamp), || {
                std::fs::rename(&lock, &held).unwrap();
                Ok(())
            })
            .map_err(Error::from),
        Err(Error::OutcomeUnknown)
    );
    drop(gate);
    std::fs::rename(held, lock).unwrap();
    let reopened = StackCommandJournal::open(
        &f.fixture.private.path().join("joined-journal"),
        f.fixture.scope.clone(),
    )
    .unwrap();
    assert!(
        reopened
            .completion(
                f.fixture.actor,
                &f.apply,
                |_| Ok(f.completion.clone()),
                Some(&f.store)
            )
            .unwrap()
            .replayed
    );
    assert_eq!(f.store.snapshot().unwrap(), f.after);
    assert_eq!(
        std::fs::read(f.fixture.private.path().join("joined-journal/state.json")).unwrap(),
        bytes
    );
}
#[tokio::test]
async fn version_six_rejects_missing_binding_changed_proof_and_false_success() {
    let f = Metadata::new().await;
    let physical = f.fixture.folder.state();
    assert_eq!(f.after.combinations[0].merge, f.fixture.target);
    f.fixture.mark_completed();
    assert!(
        f.fixture
            .journal
            .status(
                f.fixture.actor,
                f.fixture.apply().request_id,
                f.fixture.apply().intent_digest
            )
            .unwrap()
            .unwrap()
            .receipt
            .completed()
    );
    assert!(!f.record().receipt.completed());
    assert_eq!(f.fixture.folder.state(), physical);
    let gate = f.journal.root.gate().unwrap();
    let (mut catalog, _) = f.journal.read(&gate).unwrap();
    let original = catalog.records[0].clone();
    catalog.records[0].execution.as_mut().unwrap().combination = None;
    assert!(catalog.encode().is_err());
    catalog.records[0] = original.clone();
    if let CompletionKind::Tree { inventory, .. } = &mut catalog.records[0]
        .execution
        .as_mut()
        .unwrap()
        .completion
        .as_mut()
        .unwrap()
        .proof
    {
        *inventory = m::RecordDigest::new([111; 32]);
    }
    assert!(catalog.encode().is_err());
    catalog.records[0] = original.clone();
    catalog.records[0].execution.as_mut().unwrap().completion = None;
    catalog.records[0].receipt.state = s::ReceiptState::Published {
        operation: original.preview.plan.operation,
        view: original.preview.plan.view,
        checkout: s::CheckoutState::Verified {
            fingerprint: f.completion.physical(),
        },
    };
    assert!(catalog.encode().is_err());
}
