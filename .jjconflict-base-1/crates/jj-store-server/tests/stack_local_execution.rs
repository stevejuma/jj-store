#![cfg(unix)]
#[path = "../../jj-store-jj/tests/stack_foreground/fixture.rs"]
mod fixture;
use fixture::*;
use jj_store_jj::{local_browser as q, local_execution::Coordinator};
use jj_store_model as m;
use jj_store_server::{
    stack_command_journal::{Profile, Scope, StackCommandJournal},
    stack_preparation::{self as p, Authority, Context, RecoveryContext},
};
use jj_store_service::{stack_workspace as s, working_tree as tree};
use std::{path::PathBuf, sync::Arc};
#[path = "stack_local_execution/lifecycle.rs"]
mod lifecycle;

struct Allowed {
    access: Arc<dyn q::Access>,
    root: PathBuf,
    current: q::working_tree::CurrentState,
    physical: m::RecordDigest,
}
#[async_trait::async_trait]
impl Authority for Allowed {
    async fn authorize(&self, _: m::PrincipalId, _: &s::PrepareRequest) -> p::Result<()> {
        Ok(())
    }
    async fn context(&self, _: m::PrincipalId, request: &s::PrepareRequest) -> p::Result<Context> {
        Ok(Context {
            access: self.access.clone(),
            root: self.root.clone(),
            current: self.current.clone(),
            working_tree: self.physical,
            combination: request.selection.combination.clone().unwrap(),
            immutable_heads: vec![],
        })
    }
    async fn recovery(
        &self,
        _: m::PrincipalId,
        _: &s::PrepareRequest,
    ) -> p::Result<RecoveryContext> {
        Ok(RecoveryContext {
            access: self.access.clone(),
            root: self.root.clone(),
        })
    }
}
fn actor() -> m::PrincipalId {
    m::PrincipalId::new([72; 16])
}
async fn prepared(
    f: &Fixture,
) -> (
    tempfile::TempDir,
    StackCommandJournal,
    Allowed,
    s::ApplyRequest,
) {
    let current = f.witness().await;
    let physical = tree::scan(
        &f.root(),
        current.baseline.repository,
        current.baseline.generation,
        tree::Limits::default(),
        |_| Ok(()),
    )
    .unwrap()
    .fingerprint;
    let request = s::PrepareRequest::new(
        m::RequestId::new([73; 16]),
        s::Selection {
            version: 1,
            repository: current.baseline.repository,
            binding_generation: current.baseline.generation,
            physical_workspace: s::PhysicalWorkspaceId(current.physical_workspace),
            jj_workspace: current.checkout.workspace.clone(),
            code: s::CodeSelection::GitSha1Jj045 {
                operation: current.baseline.operation,
                view: current.baseline.view,
                working_copy: current.baseline.commit,
            },
            combination: Some(s::CombinationSelection {
                id: s::CombinationId(m::RequestId::new([74; 16])),
                generation: 1,
            }),
            working_tree: physical,
        },
        s::Command::Reword {
            commit: s::CommitIdentity::GitSha1(current.baseline.commit),
            message: "Durable real reword execution".into(),
        },
    )
    .unwrap();
    let private = tempfile::tempdir().unwrap();
    let journal = StackCommandJournal::create(
        &private.path().join("stacks"),
        Scope {
            installation: [75; 16],
            repository: current.baseline.repository,
            physical_workspace: s::PhysicalWorkspaceId(current.physical_workspace),
            jj_workspace: current.checkout.workspace.clone(),
            profile: Profile::GitSha1Jj045,
        },
    )
    .unwrap();
    let authority = Allowed {
        access: f.access(),
        root: f.root(),
        current,
        physical,
    };
    let preview = p::prepare_reword(
        &journal,
        &Coordinator::new(1).unwrap(),
        &authority,
        actor(),
        request.clone(),
    )
    .await
    .unwrap()
    .preview;
    let apply = s::ApplyRequest {
        version: 1,
        request_id: request.request_id,
        intent_digest: request.intent_digest,
        selection: request.selection,
        prepared: preview.prepared,
    };
    (private, journal, authority, apply)
}
#[tokio::test]
async fn real_retained_reword_execution_saves_checkout_witness_and_replays_receipt_without_writes()
{
    let f = Fixture::new().await;
    let (private, journal, authority, apply) = prepared(&f).await;
    let before = f.state();
    let observer = tree::Observer::new(q::Executor::new(1).unwrap(), 1).unwrap();
    let completed = p::apply_reword(
        &journal,
        &Coordinator::new(1).unwrap(),
        &observer,
        &authority,
        actor(),
        apply.clone(),
    )
    .await
    .unwrap();
    assert!(completed.receipt.completed());
    let after = f.state();
    assert_eq!(&after[..2], &before[..2]);
    assert_eq!(after[3], before[3]);
    assert_ne!(after[2], before[2]);
    let saved = journal
        .status(actor(), apply.request_id, apply.intent_digest)
        .unwrap()
        .unwrap();
    assert!(saved.execution.unwrap().checkout.is_some());
    drop(journal);
    let journal = StackCommandJournal::open(
        &private.path().join("stacks"),
        Scope {
            installation: [75; 16],
            repository: authority.current.baseline.repository,
            physical_workspace: s::PhysicalWorkspaceId(authority.current.physical_workspace),
            jj_workspace: authority.current.checkout.workspace.clone(),
            profile: Profile::GitSha1Jj045,
        },
    )
    .unwrap();
    // Historical completed retries return their receipt before any new physical
    // scan or execution, including after outside editor changes.
    std::fs::write(f.root().join("file.txt"), "new outside edit\n").unwrap();
    let after_edit = f.state();
    let replay = p::apply_reword(
        &journal,
        &Coordinator::new(1).unwrap(),
        &observer,
        &authority,
        actor(),
        apply,
    )
    .await
    .unwrap();
    assert!(replay.replayed);
    assert_eq!(replay.receipt, completed.receipt);
    assert_eq!(f.state(), after_edit);
}

#[derive(Clone, Copy)]
enum Boundary {
    Unknown,
    Published,
    Saved,
    Selected,
}
struct Stop {
    inner: Allowed,
    journal: StackCommandJournal,
    boundary: Boundary,
    original: Vec<u8>,
    stopped: std::sync::atomic::AtomicBool,
}
#[async_trait::async_trait]
impl Authority for Stop {
    async fn authorize(&self, actor: m::PrincipalId, request: &s::PrepareRequest) -> p::Result<()> {
        let record = self
            .journal
            .status(actor, request.request_id, request.intent_digest)?
            .unwrap();
        let stop = match self.boundary {
            Boundary::Unknown => record.receipt.state == s::ReceiptState::PublicationUnknown,
            Boundary::Published => {
                matches!(record.receipt.state, s::ReceiptState::Published { .. })
            }
            Boundary::Saved => record
                .execution
                .as_ref()
                .is_some_and(|e| e.checkout.is_some()),
            Boundary::Selected => {
                std::fs::read(self.inner.root.join(".jj/working_copy/checkout")).unwrap()
                    != self.original
            }
        };
        if stop && !self.stopped.swap(true, std::sync::atomic::Ordering::SeqCst) {
            return Err(p::Error::Forbidden);
        }
        Ok(())
    }
    async fn context(
        &self,
        actor: m::PrincipalId,
        request: &s::PrepareRequest,
    ) -> p::Result<Context> {
        self.inner.context(actor, request).await
    }
    async fn recovery(
        &self,
        actor: m::PrincipalId,
        request: &s::PrepareRequest,
    ) -> p::Result<RecoveryContext> {
        self.inner.recovery(actor, request).await
    }
}
#[tokio::test]
async fn interrupted_durable_boundaries_reopen_and_finish_the_same_original_candidate() {
    for boundary in [
        Boundary::Unknown,
        Boundary::Published,
        Boundary::Saved,
        Boundary::Selected,
    ] {
        let f = Fixture::new().await;
        let (private, journal, inner, apply) = prepared(&f).await;
        let before = f.state();
        let observer = tree::Observer::new(q::Executor::new(1).unwrap(), 1).unwrap();
        let stop = Stop {
            inner,
            journal: journal.clone(),
            boundary,
            original: before[2].clone(),
            stopped: Default::default(),
        };
        assert!(matches!(
            p::apply_reword(
                &journal,
                &Coordinator::new(1).unwrap(),
                &observer,
                &stop,
                actor(),
                apply.clone()
            )
            .await,
            Err(p::Error::Forbidden)
        ));
        let interrupted = journal
            .status(actor(), apply.request_id, apply.intent_digest)
            .unwrap()
            .unwrap();
        assert!(!interrupted.receipt.completed());
        assert_eq!(journal.pending(actor()).unwrap().len(), 1);
        let scope = Scope {
            installation: [75; 16],
            repository: stop.inner.current.baseline.repository,
            physical_workspace: s::PhysicalWorkspaceId(stop.inner.current.physical_workspace),
            jj_workspace: stop.inner.current.checkout.workspace.clone(),
            profile: Profile::GitSha1Jj045,
        };
        drop(journal);
        let reopened = StackCommandJournal::open(&private.path().join("stacks"), scope).unwrap();
        let completed = p::apply_reword(
            &reopened,
            &Coordinator::new(1).unwrap(),
            &observer,
            &stop.inner,
            actor(),
            apply.clone(),
        )
        .await
        .unwrap();
        assert!(completed.receipt.completed());
        assert_eq!(completed.preview, interrupted.preview);
        let selected = reopened
            .status(actor(), apply.request_id, apply.intent_digest)
            .unwrap()
            .unwrap();
        assert_eq!(
            selected.execution.as_ref().unwrap().key,
            interrupted.execution.as_ref().unwrap().key
        );
        if let Some(saved) = interrupted.execution.unwrap().checkout {
            assert_eq!(selected.execution.unwrap().checkout, Some(saved));
        }
        assert!(reopened.pending(actor()).unwrap().is_empty());
        assert_eq!(&f.state()[..2], &before[..2]);
        assert_eq!(f.state()[3], before[3]);
    }
}

#[tokio::test]
async fn physical_edits_reject_prepublication_and_keep_saved_checkout_pending_after_publication() {
    for published in [false, true] {
        let f = Fixture::new().await;
        let (_private, journal, inner, apply) = prepared(&f).await;
        let observer = tree::Observer::new(q::Executor::new(1).unwrap(), 1).unwrap();
        if published {
            let stop = Stop {
                original: f.state()[2].clone(),
                inner,
                journal: journal.clone(),
                boundary: Boundary::Saved,
                stopped: Default::default(),
            };
            assert!(matches!(
                p::apply_reword(
                    &journal,
                    &Coordinator::new(1).unwrap(),
                    &observer,
                    &stop,
                    actor(),
                    apply.clone()
                )
                .await,
                Err(p::Error::Forbidden)
            ));
            std::fs::write(f.root().join("ignored.txt"), "new ignored bytes\n").unwrap();
            let before = f.state();
            assert!(matches!(
                p::apply_reword(
                    &journal,
                    &Coordinator::new(1).unwrap(),
                    &observer,
                    &stop.inner,
                    actor(),
                    apply.clone()
                )
                .await,
                Err(p::Error::Provider(q::Error::EpochChanged))
            ));
            assert_eq!(f.state(), before);
            assert_eq!(journal.pending(actor()).unwrap().len(), 1);
            assert!(
                journal
                    .status(actor(), apply.request_id, apply.intent_digest)
                    .unwrap()
                    .unwrap()
                    .execution
                    .unwrap()
                    .checkout
                    .is_some()
            );
        } else {
            std::fs::write(f.root().join("new-obstacle"), "retain this file\n").unwrap();
            let before = f.state();
            assert!(matches!(
                p::apply_reword(
                    &journal,
                    &Coordinator::new(1).unwrap(),
                    &observer,
                    &inner,
                    actor(),
                    apply.clone()
                )
                .await,
                Err(p::Error::Provider(q::Error::EpochChanged))
            ));
            assert_eq!(f.state(), before);
            assert_eq!(
                journal
                    .status(actor(), apply.request_id, apply.intent_digest)
                    .unwrap()
                    .unwrap()
                    .receipt
                    .state,
                s::ReceiptState::Prepared
            );
        }
    }
}

#[tokio::test]
async fn foreign_apply_identities_and_unproved_execution_receipts_cannot_publish() {
    let f = Fixture::new().await;
    let (_private, journal, authority, apply) = prepared(&f).await;
    let observer = tree::Observer::new(q::Executor::new(1).unwrap(), 1).unwrap();
    let before = f.state();
    for mode in ["actor", "prepared", "selection", "intent"] {
        let mut wrong = apply.clone();
        let mut caller = actor();
        match mode {
            "actor" => caller = m::PrincipalId::new([99; 16]),
            "prepared" => wrong.prepared = s::PreparedIdentity(m::RecordDigest::new([99; 32])),
            "selection" => wrong.selection.binding_generation += 1,
            _ => wrong.intent_digest = m::IntentDigest::new([99; 32]),
        }
        assert!(
            p::apply_reword(
                &journal,
                &Coordinator::new(1).unwrap(),
                &observer,
                &authority,
                caller,
                wrong
            )
            .await
            .is_err(),
            "{mode}"
        );
        assert_eq!(f.state(), before, "{mode}");
    }
    let record = journal
        .status(actor(), apply.request_id, apply.intent_digest)
        .unwrap()
        .unwrap();
    journal
        .transition(
            actor(),
            &apply,
            &s::ReceiptState::Prepared,
            &s::ReceiptState::PublicationUnknown,
        )
        .unwrap();
    for checkout in [
        s::CheckoutState::Required,
        s::CheckoutState::Verified {
            fingerprint: authority.physical,
        },
    ] {
        assert!(
            journal
                .transition(
                    actor(),
                    &apply,
                    &s::ReceiptState::PublicationUnknown,
                    &s::ReceiptState::Published {
                        operation: record.preview.plan.operation,
                        view: record.preview.plan.view,
                        checkout,
                    }
                )
                .is_err()
        );
    }
    assert_eq!(f.state(), before);
}

struct Historical;
#[async_trait::async_trait]
impl Authority for Historical {
    async fn authorize(&self, _: m::PrincipalId, _: &s::PrepareRequest) -> p::Result<()> {
        Ok(())
    }
    async fn context(&self, _: m::PrincipalId, _: &s::PrepareRequest) -> p::Result<Context> {
        panic!("historical retry must not read live state")
    }
    async fn recovery(
        &self,
        _: m::PrincipalId,
        _: &s::PrepareRequest,
    ) -> p::Result<RecoveryContext> {
        panic!("historical retry must not open provider")
    }
}
#[tokio::test]
async fn retired_receipt_and_saved_witness_are_historical_after_later_real_cli_work() {
    let f = Fixture::new().await;
    let (_private, journal, authority, apply) = prepared(&f).await;
    let observer = tree::Observer::new(q::Executor::new(1).unwrap(), 1).unwrap();
    let done = p::apply_reword(
        &journal,
        &Coordinator::new(1).unwrap(),
        &observer,
        &authority,
        actor(),
        apply.clone(),
    )
    .await
    .unwrap();
    let retired = s::ReceiptState::Retired {
        operation: done.preview.plan.operation,
        reason: s::Retirement::Undone,
    };
    // Retirement metadata is an isolated outcome fixture, not proof of an undo
    // engine. Actual later CLI execution is independently performed below.
    journal
        .transition(actor(), &apply, &done.receipt.state, &retired)
        .unwrap();
    stock(&f.root(), &["describe", "-m", "Later real CLI operation"]);
    std::fs::write(f.root().join("file.txt"), "later editor bytes\n").unwrap();
    let before = f.state();
    let replay = p::apply_reword(
        &journal,
        &Coordinator::new(1).unwrap(),
        &observer,
        &Historical,
        actor(),
        apply,
    )
    .await
    .unwrap();
    assert!(replay.replayed);
    assert_eq!(replay.receipt.state, retired);
    assert_eq!(f.state(), before);
    assert!(journal.pending(actor()).unwrap().is_empty());
}

#[tokio::test]
async fn saved_checkout_is_immutable_and_malformed_linkage_cannot_reopen() {
    let f = Fixture::new().await;
    let (private, journal, inner, apply) = prepared(&f).await;
    let scope = Scope {
        installation: [75; 16],
        repository: inner.current.baseline.repository,
        physical_workspace: s::PhysicalWorkspaceId(inner.current.physical_workspace),
        jj_workspace: inner.current.checkout.workspace.clone(),
        profile: Profile::GitSha1Jj045,
    };
    let stop = Stop {
        original: f.state()[2].clone(),
        inner,
        journal: journal.clone(),
        boundary: Boundary::Saved,
        stopped: Default::default(),
    };
    let observer = tree::Observer::new(q::Executor::new(1).unwrap(), 1).unwrap();
    assert!(matches!(
        p::apply_reword(
            &journal,
            &Coordinator::new(1).unwrap(),
            &observer,
            &stop,
            actor(),
            apply.clone()
        )
        .await,
        Err(p::Error::Forbidden)
    ));
    let record = journal
        .status(actor(), apply.request_id, apply.intent_digest)
        .unwrap()
        .unwrap();
    let saved = record.execution.unwrap().checkout.unwrap();
    assert!(
        journal
            .save_local_checkout(actor(), &apply, &saved.witness)
            .unwrap()
            .replayed
    );
    let mut changed = serde_json::to_value(&saved.witness).unwrap();
    changed["before_fingerprint"] = serde_json::to_value(m::RecordDigest::new([99; 32])).unwrap();
    let different: jj_store_jj::local_execution::CheckoutWitnessV1 =
        serde_json::from_value(changed).unwrap();
    assert_ne!(different.id().unwrap(), saved.id);
    assert!(
        journal
            .save_local_checkout(actor(), &apply, &different)
            .is_err()
    );
    let path = private.path().join("stacks/state.json");
    let original = std::fs::read(&path).unwrap();
    let before = f.state();
    for mode in ["version", "digest", "key", "missing"] {
        let mut bad: serde_json::Value = serde_json::from_slice(&original).unwrap();
        assert_eq!(bad["version"], 3);
        let execution = &mut bad["records"][0]["execution"];
        match mode {
            "digest" => {
                execution["checkout"]["id"] =
                    serde_json::to_value(m::RecordDigest::new([99; 32])).unwrap()
            }
            "key" => {
                execution["checkout"]["witness"]["key"]["request"] =
                    serde_json::to_value(m::RequestId::new([99; 16])).unwrap()
            }
            "missing" => execution
                .as_object_mut()
                .unwrap()
                .remove("checkout")
                .map(|_| ())
                .unwrap(),
            _ => bad["version"] = serde_json::json!(2),
        }
        std::fs::write(&path, serde_json::to_vec(&bad).unwrap()).unwrap();
        assert!(
            StackCommandJournal::open(&private.path().join("stacks"), scope.clone()).is_err(),
            "{mode}"
        );
        assert_eq!(f.state(), before, "{mode}");
    }
    std::fs::write(&path, original).unwrap();
    StackCommandJournal::open(&private.path().join("stacks"), scope).unwrap();
}

#[tokio::test]
async fn saved_pending_checkout_repairs_lost_source_dependencies_without_repreparing() {
    let f = Fixture::new().await;
    let (_private, journal, inner, apply) = prepared(&f).await;
    let stop = Stop {
        original: f.state()[2].clone(),
        inner,
        journal: journal.clone(),
        boundary: Boundary::Saved,
        stopped: Default::default(),
    };
    let observer = tree::Observer::new(q::Executor::new(1).unwrap(), 1).unwrap();
    assert!(matches!(
        p::apply_reword(
            &journal,
            &Coordinator::new(1).unwrap(),
            &observer,
            &stop,
            actor(),
            apply.clone()
        )
        .await,
        Err(p::Error::Forbidden)
    ));
    let record = journal
        .status(actor(), apply.request_id, apply.intent_digest)
        .unwrap()
        .unwrap();
    let saved = record.execution.unwrap().checkout.unwrap();
    std::fs::remove_dir_all(f.root().join(".jj/repo/op_store")).unwrap();
    std::fs::remove_dir_all(f.root().join(".jj/repo/index")).unwrap();
    let before = f.state();
    let done = p::apply_reword(
        &journal,
        &Coordinator::new(1).unwrap(),
        &observer,
        &stop.inner,
        actor(),
        apply.clone(),
    )
    .await
    .unwrap();
    assert!(done.receipt.completed());
    assert_eq!(
        journal
            .status(actor(), apply.request_id, apply.intent_digest)
            .unwrap()
            .unwrap()
            .execution
            .unwrap()
            .checkout,
        Some(saved)
    );
    assert_eq!(&f.state()[..2], &before[..2]);
    assert_eq!(f.state()[3], before[3]);
}

#[tokio::test]
async fn journal_retention_mismatch_is_rejected_before_publication() {
    let f = Fixture::new().await;
    let (private, journal, authority, apply) = prepared(&f).await;
    let path = private.path().join("stacks/state.json");
    let original = std::fs::read(&path).unwrap();
    let before = f.state();
    let observer = tree::Observer::new(q::Executor::new(1).unwrap(), 1).unwrap();
    for field in ["retention", "archive"] {
        let mut bad: serde_json::Value = serde_json::from_slice(&original).unwrap();
        bad["records"][0]["execution"][field] = if field == "retention" {
            serde_json::json!("9999999999999999999999999999999999999999")
        } else {
            serde_json::to_value(m::RecordDigest::new([99; 32])).unwrap()
        };
        std::fs::write(&path, serde_json::to_vec(&bad).unwrap()).unwrap();
        assert!(
            matches!(
                p::apply_reword(
                    &journal,
                    &Coordinator::new(1).unwrap(),
                    &observer,
                    &authority,
                    actor(),
                    apply.clone()
                )
                .await,
                Err(p::Error::Invalid)
            ),
            "{field}"
        );
        assert_eq!(f.state(), before, "{field}");
        assert_eq!(
            journal
                .status(actor(), apply.request_id, apply.intent_digest)
                .unwrap()
                .unwrap()
                .receipt
                .state,
            s::ReceiptState::Prepared
        );
    }
    std::fs::write(path, original).unwrap();
}
