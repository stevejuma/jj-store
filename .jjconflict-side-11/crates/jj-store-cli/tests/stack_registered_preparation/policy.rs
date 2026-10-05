use super::*;
use apply::{execute, lose_source, prepared};
use std::sync::atomic::{AtomicBool, Ordering};

#[tokio::test]
async fn unpublished_unknown_retry_revalidates_current_immutable_policy_before_head_selection() {
    let f = Installed::new().await;
    let (journal, preview, apply) = prepared(&f).await;
    journal
        .transition(
            f.actor,
            &apply,
            &s::ReceiptState::Prepared,
            &s::ReceiptState::PublicationUnknown,
        )
        .unwrap();
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
    let reopened =
        StackCommandJournal::open(&f.state.join("stack-journal"), recovery::journal_scope(&f))
            .unwrap();
    assert!(matches!(
        execute(&f, &reopened, apply.clone()).await,
        Err(p::Error::Provider(q::Error::Denied))
    ));
    let saved = reopened
        .status(f.actor, apply.request_id, apply.intent_digest)
        .unwrap()
        .unwrap();
    assert_eq!(saved.preview, preview);
    assert_eq!(saved.receipt.state, s::ReceiptState::PublicationUnknown);
    assert!(saved.execution.unwrap().checkout.is_none());
    assert_eq!(f.folder.state(), before);
    f.combinations
        .0
        .lock()
        .unwrap()
        .as_mut()
        .unwrap()
        .immutable_heads = vec![m::GitSha1Id::new([0; 20])];
    let done = execute(&f, &reopened, apply).await.unwrap();
    assert_eq!(done.preview, preview);
    assert!(done.receipt.completed());
}
struct StopAfterHead {
    registered: RegisteredAuthority,
    journal: StackCommandJournal,
    root: std::path::PathBuf,
    stopped: AtomicBool,
    after_receipt: bool,
}
#[async_trait::async_trait]
impl Authority for StopAfterHead {
    async fn authorize(&self, actor: m::PrincipalId, request: &s::PrepareRequest) -> p::Result<()> {
        self.registered.authorize(actor, request).await?;
        if let Some(saved) =
            self.journal
                .status(actor, request.request_id, request.intent_digest)?
            && if self.after_receipt {
                matches!(
                    saved.receipt.state,
                    s::ReceiptState::Published {
                        checkout: s::CheckoutState::Required,
                        ..
                    }
                )
            } else {
                saved.receipt.state == s::ReceiptState::PublicationUnknown
            }
            && self
                .root
                .join(".jj/repo/op_heads/heads")
                .join(saved.preview.plan.operation.to_string())
                .exists()
            && !self.stopped.swap(true, Ordering::SeqCst)
        {
            return Err(p::Error::Forbidden);
        }
        Ok(())
    }
    async fn context(
        &self,
        actor: m::PrincipalId,
        request: &s::PrepareRequest,
    ) -> p::Result<p::Context> {
        self.registered.context(actor, request).await
    }
    async fn recovery(
        &self,
        actor: m::PrincipalId,
        request: &s::PrepareRequest,
    ) -> p::Result<p::RecoveryContext> {
        self.registered.recovery(actor, request).await
    }
}
#[tokio::test]
async fn already_published_original_retry_completes_checkout_after_live_policy_changes() {
    let f = Installed::new().await;
    let (journal, preview, apply) = prepared(&f).await;
    let before = f.folder.state();
    let stop = StopAfterHead {
        registered: f.authority.clone(),
        journal: journal.clone(),
        root: f.folder.root(),
        stopped: AtomicBool::new(false),
        after_receipt: false,
    };
    assert!(matches!(
        p::apply_reword(
            &journal,
            &Coordinator::new(1).unwrap(),
            &Observer::new(q::Executor::new(1).unwrap(), 1).unwrap(),
            &stop,
            f.actor,
            apply.clone()
        )
        .await,
        Err(p::Error::Forbidden)
    ));
    let selected = f.folder.state();
    assert_eq!(&selected[..4], &before[..4]);
    assert_ne!(selected[4..], before[4..]);
    let original = journal
        .status(f.actor, apply.request_id, apply.intent_digest)
        .unwrap()
        .unwrap();
    assert_eq!(original.receipt.state, s::ReceiptState::PublicationUnknown);
    assert!(original.execution.unwrap().checkout.is_none());
    // New policy cannot turn already-published execution into another rewrite
    // or prevent acknowledgement of the exact original historical selection.
    *f.combinations.0.lock().unwrap() = None;
    lose_source(&f);
    let done = execute(&f, &journal, apply).await.unwrap();
    assert_eq!(done.preview, preview);
    assert!(done.receipt.completed());
    let after = f.folder.state();
    assert_eq!(&after[..2], &before[..2]);
    assert_eq!(after[3], before[3]);
    assert_ne!(after[2], before[2]);
    assert_eq!(after[4..], selected[4..]);
}

#[tokio::test]
async fn typed_published_receipt_cannot_reannounce_lost_selection_before_witness_save() {
    let f = Installed::new().await;
    let (journal, preview, apply) = prepared(&f).await;
    let stop = StopAfterHead {
        registered: f.authority.clone(),
        journal: journal.clone(),
        root: f.folder.root(),
        stopped: AtomicBool::new(false),
        after_receipt: true,
    };
    assert!(matches!(
        p::apply_reword(
            &journal,
            &Coordinator::new(1).unwrap(),
            &Observer::new(q::Executor::new(1).unwrap(), 1).unwrap(),
            &stop,
            f.actor,
            apply.clone()
        )
        .await,
        Err(p::Error::Forbidden)
    ));
    let saved = journal
        .status(f.actor, apply.request_id, apply.intent_digest)
        .unwrap()
        .unwrap();
    assert!(matches!(
        saved.receipt.state,
        s::ReceiptState::Published {
            checkout: s::CheckoutState::Required,
            ..
        }
    ));
    assert!(saved.execution.unwrap().checkout.is_none());
    let s::CodeSelection::GitSha1Jj045 { operation, .. } = f.request.selection.code else {
        panic!()
    };
    let heads = f.folder.root().join(".jj/repo/op_heads/heads");
    std::fs::remove_file(heads.join(preview.plan.operation.to_string())).unwrap();
    std::fs::write(heads.join(operation.to_string()), []).unwrap();
    let lost = f.folder.state();
    let result = execute(&f, &journal, apply.clone()).await;
    assert!(
        matches!(result, Err(p::Error::Provider(q::Error::EpochChanged))),
        "{result:?}"
    );
    assert_eq!(f.folder.state(), lost);
    let still = journal
        .status(f.actor, apply.request_id, apply.intent_digest)
        .unwrap()
        .unwrap();
    assert_eq!(still.receipt, saved.receipt);
    assert_eq!(still.preview, preview);
    // Restore the original selected marker explicitly in this fault fixture;
    // recovery acknowledges it even when current combination policy is absent.
    std::fs::remove_file(heads.join(operation.to_string())).unwrap();
    std::fs::write(heads.join(preview.plan.operation.to_string()), []).unwrap();
    *f.combinations.0.lock().unwrap() = None;
    assert!(
        execute(&f, &journal, apply)
            .await
            .unwrap()
            .receipt
            .completed()
    );
}
