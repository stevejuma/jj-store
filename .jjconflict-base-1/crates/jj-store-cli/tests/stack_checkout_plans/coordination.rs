//! Real native completion joined to synthetic after-state: coordination evidence,
//! not qualification of the setup/merge/WIP command engine.
use super::*;
use installation::physical;
use jj_store_jj::local_execution::{PhysicalInventoryV1, RecoveredHistory, VerifiedTreeCheckout};
use jj_store_server::stack_combinations::{self as c, CombinationStore};

pub(super) struct Joined {
    pub(super) input: Input,
    pub(super) private: tempfile::TempDir,
    pub(super) journal: StackCommandJournal,
    pub(super) plans: StackCheckoutPlans,
    pub(super) store: CombinationStore,
    pub(super) after: c::Snapshot,
    pub(super) inventory: PhysicalInventoryV1,
}
impl Joined {
    pub(super) async fn new() -> Self {
        let input = Input::build_effects_for(
            true,
            vec![
                ("file.txt", Some((b"candidate\n", true))),
                ("src/new", Some((b"new\n", false))),
            ],
            |_| {},
            true,
        )
        .await;
        let private = tempfile::tempdir().unwrap();
        let journal = input.journal(&private.path().join("journal"));
        let plans =
            StackCheckoutPlans::create(&private.path().join("plans"), input.scope.clone()).unwrap();
        let store = CombinationStore::create(
            &private.path().join("combinations"),
            input.scope.clone(),
            actor(),
        )
        .unwrap();
        let retained = MetadataStore::open(&input.folder.root().join(".git"), Limits::default())
            .unwrap()
            .read_stack_prepared(&input.execution.key)
            .unwrap()
            .unwrap();
        let after = c::Snapshot {
            version: 1,
            revision: 1,
            active: Some(input.preview.plan.combination.id),
            combinations: vec![c::Combination {
                selection: input.preview.plan.combination.clone(),
                name: "Coordination fixture".into(),
                target: s::BookmarkTarget {
                    name: "main".into(),
                    remote: None,
                    head: s::CommitIdentity::GitSha1(m::GitSha1Id::new([0; 20])),
                },
                immutable_heads: vec![m::GitSha1Id::new([0; 20])],
                merge: m::GitSha1Id::new([0; 20]),
                code: s::CodeSelection::GitSha1Jj045 {
                    operation: input.preview.plan.operation,
                    view: input.preview.plan.view,
                    working_copy: input.plan.commits().1,
                },
                stacks: vec![],
                retention: c::RetainedCode {
                    key: retained.key.clone(),
                    commit: retained.commit,
                    archive: input.execution.archive,
                },
            }],
        };
        journal
            .save_local_combination(&store, actor(), &input.apply(), 0, &after, &retained)
            .unwrap();
        journal
            .save_local_tree_checkout(&plans, actor(), &input.apply(), &input.plan)
            .unwrap();
        let inventory = input.inventory().await;
        journal
            .save_local_physical_inventory(&plans, actor(), &input.apply(), &inventory)
            .unwrap();
        Self {
            input,
            private,
            journal,
            plans,
            store,
            after,
            inventory,
        }
    }
    pub(super) fn record(&self) -> jj_store_server::stack_command_journal::Record {
        self.journal
            .status(
                actor(),
                self.input.apply().request_id,
                self.input.apply().intent_digest,
            )
            .unwrap()
            .unwrap()
    }
    pub(super) async fn complete(&self) -> (RecoveredHistory, VerifiedTreeCheckout) {
        self.journal
            .check_local_combination(&self.store, actor(), &self.input.apply())
            .unwrap();
        self.journal
            .transition(
                actor(),
                &self.input.apply(),
                &s::ReceiptState::Prepared,
                &s::ReceiptState::PublicationUnknown,
            )
            .unwrap();
        let (recovery, head) = installation::published(&self.input).await;
        self.journal
            .acknowledge_local_publication(actor(), &self.input.apply(), &head)
            .unwrap();
        let (recovery, files) = recovery
            .install_tree_files(
                &head,
                self.input.plan.clone(),
                self.inventory.clone(),
                physical(&self.input),
            )
            .await
            .unwrap();
        recovery
            .complete_tree_checkout(
                files,
                self.input.plan.clone(),
                self.inventory.clone(),
                physical(&self.input),
            )
            .await
            .unwrap()
    }
    pub(super) async fn resume(
        &self,
    ) -> local_browser::Result<(RecoveredHistory, VerifiedTreeCheckout)> {
        let plan = self
            .journal
            .load_local_tree_checkout(&self.plans, actor(), &self.input.apply())
            .unwrap();
        let inventory = self
            .journal
            .load_local_physical_inventory(&self.plans, actor(), &self.input.apply())
            .unwrap();
        Coordinator::new(1)
            .unwrap()
            .resume_tree_checkout(
                self.input.folder.access(),
                self.input.folder.root(),
                self.input.execution.witness.clone(),
                self.input.execution.key.clone(),
                plan,
                inventory,
            )
            .await?
            .finish_tree_checkout(physical(&self.input))
            .await
    }
    pub(super) fn finish(
        &self,
        proof: &VerifiedTreeCheckout,
    ) -> Result<jj_store_server::stack_command_journal::Selection, Error> {
        self.journal
            .finish_local_tree_combination(&self.store, actor(), &self.input.apply(), proof)
    }
}

#[tokio::test]
async fn native_completion_remains_pending_until_exact_combination_selection() {
    let f = Joined::new().await;
    let (recovery, proof) = f.complete().await;
    assert!(f.finish(&proof).is_err()); // Original native evidence must be durable first.
    f.journal
        .save_local_tree_completion(actor(), &f.input.apply(), &proof)
        .unwrap();
    assert!(!f.record().receipt.completed());
    assert!(f.record().execution.unwrap().completion.is_some());
    assert_eq!(f.journal.pending(actor()).unwrap().len(), 1);
    assert!(f.store.snapshot().unwrap().active.is_none());
    assert_eq!(
        f.store.finish(&f.journal, actor(), &f.input.apply()),
        Err(c::Error::Pending)
    );
    assert_eq!(
        f.journal.transition(
            actor(),
            &f.input.apply(),
            &f.record().receipt.state,
            &s::ReceiptState::Published {
                operation: proof.operation,
                view: proof.view,
                checkout: s::CheckoutState::Verified {
                    fingerprint: proof.physical()
                }
            }
        ),
        Err(Error::InvalidTransition)
    );
    let selected = f.finish(&proof).unwrap();
    assert!(selected.receipt.completed());
    assert_eq!(f.store.snapshot().unwrap(), f.after);
    assert!(f.journal.pending(actor()).unwrap().is_empty());
    drop(recovery);
    let native = f.input.folder.state();
    let (recovery, same) = f.resume().await.unwrap();
    assert_eq!(proof, same);
    assert!(f.finish(&same).unwrap().replayed);
    assert_eq!(f.input.folder.state(), native);
    drop(recovery);
}

#[tokio::test]
async fn changed_editor_files_and_missing_original_state_cannot_finish_a_pending_command() {
    let f = Joined::new().await;
    let (recovery, proof) = f.complete().await;
    f.journal
        .save_local_tree_completion(actor(), &f.input.apply(), &proof)
        .unwrap();
    drop(recovery);
    fs::write(
        f.input.folder.root().join("ignored.txt"),
        b"new editor contents",
    )
    .unwrap();
    assert!(matches!(
        f.resume().await,
        Err(local_browser::Error::EpochChanged)
    ));
    assert!(!f.record().receipt.completed());
    assert!(f.store.snapshot().unwrap().active.is_none());
    let missing = CombinationStore::create(
        &f.private.path().join("missing"),
        f.input.scope.clone(),
        actor(),
    )
    .unwrap();
    assert!(
        f.journal
            .check_local_combination(&missing, actor(), &f.input.apply())
            .is_err()
    );
    assert!(
        f.journal
            .finish_local_tree_combination(&missing, actor(), &f.input.apply(), &proof)
            .is_err()
    );
    assert!(missing.snapshot().unwrap().active.is_none());
}

#[tokio::test]
async fn proof_linkage_and_cross_actor_checks_cannot_change_native_or_device_state() {
    let f = Joined::new().await;
    let (recovery, proof) = f.complete().await;
    let state = f.input.folder.state();
    let bytes = fs::read(f.private.path().join("journal/state.json")).unwrap();
    let mut changed = proof.clone();
    changed.operation = m::OperationId::new([99; 64]);
    assert_eq!(
        f.journal
            .save_local_tree_completion(actor(), &f.input.apply(), &changed),
        Err(Error::IntentMismatch)
    );
    assert_eq!(
        f.journal.save_local_tree_completion(
            m::PrincipalId::new([99; 16]),
            &f.input.apply(),
            &proof
        ),
        Err(Error::IntentMismatch)
    );
    let mut apply = f.input.apply();
    apply.selection.binding_generation += 1;
    assert_eq!(
        f.journal
            .save_local_tree_completion(actor(), &apply, &proof),
        Err(Error::IntentMismatch)
    );
    assert_eq!(
        fs::read(f.private.path().join("journal/state.json")).unwrap(),
        bytes
    );
    assert_eq!(f.input.folder.state(), state);
    assert!(f.store.snapshot().unwrap().active.is_none());
    drop(recovery);
}
