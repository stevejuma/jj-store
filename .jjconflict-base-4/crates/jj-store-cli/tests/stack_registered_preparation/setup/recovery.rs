use super::*;
use std::sync::atomic::{AtomicBool, Ordering};
#[derive(Clone, Copy)]
enum Boundary {
    Retained,
    Head,
    Published,
    Pointer,
}
struct Pause {
    authority: RegisteredAuthority,
    journal: StackCommandJournal,
    root: std::path::PathBuf,
    pointer: Vec<u8>,
    boundary: Boundary,
    paused: AtomicBool,
}
#[async_trait::async_trait]
impl Authority for Pause {
    async fn authorize(&self, actor: m::PrincipalId, request: &s::PrepareRequest) -> p::Result<()> {
        self.authority.authorize(actor, request).await?;
        if let Some(saved) =
            self.journal
                .status(actor, request.request_id, request.intent_digest)?
        {
            let stop = match self.boundary {
                Boundary::Retained => saved
                    .execution
                    .as_ref()
                    .is_some_and(|e| e.history.is_some()),
                Boundary::Head => {
                    saved.receipt.state == s::ReceiptState::PublicationUnknown
                        && self
                            .root
                            .join(".jj/repo/op_heads/heads")
                            .join(saved.preview.plan.operation.to_string())
                            .exists()
                }
                Boundary::Published => matches!(
                    saved.receipt.state,
                    s::ReceiptState::Published {
                        checkout: s::CheckoutState::Required,
                        ..
                    }
                ),
                Boundary::Pointer => {
                    std::fs::read(self.root.join(".jj/working_copy/checkout")).unwrap()
                        != self.pointer
                }
            };
            if stop && !self.paused.swap(true, Ordering::SeqCst) {
                return Err(p::Error::Forbidden);
            }
        }
        Ok(())
    }
    async fn context(
        &self,
        actor: m::PrincipalId,
        request: &s::PrepareRequest,
    ) -> p::Result<p::Context> {
        self.authority.context(actor, request).await
    }
    async fn setup_context(
        &self,
        actor: m::PrincipalId,
        request: &s::PrepareRequest,
    ) -> p::Result<p::SetupContext> {
        self.authority.setup_context(actor, request).await
    }
    async fn recovery(
        &self,
        actor: m::PrincipalId,
        request: &s::PrepareRequest,
    ) -> p::Result<p::RecoveryContext> {
        self.authority.recovery(actor, request).await
    }
}
fn pause(f: &SetupInstalled, boundary: Boundary) -> Pause {
    Pause {
        authority: f.authority.clone(),
        journal: f.journal(),
        root: f.f.folder.root(),
        pointer: std::fs::read(f.f.folder.root().join(".jj/working_copy/checkout")).unwrap(),
        boundary,
        paused: AtomicBool::new(false),
    }
}
#[tokio::test]
async fn setup_original_retention_retry_keeps_saved_candidate_and_combination_identity() {
    let f = SetupInstalled::new().await;
    let journal = f.journal();
    let paused = pause(&f, Boundary::Retained);
    let before = f.f.folder.state();
    assert!(matches!(
        p::prepare_setup(
            &journal,
            &f.store,
            &Coordinator::new(1).unwrap(),
            &f.observer,
            &paused,
            f.f.actor,
            f.request.clone()
        )
        .await,
        Err(p::Error::Forbidden)
    ));
    let pending = journal
        .status(f.f.actor, f.request.request_id, f.request.intent_digest)
        .unwrap()
        .unwrap();
    assert_eq!(pending.receipt.state, s::ReceiptState::Prepared);
    assert!(pending.execution.as_ref().unwrap().history.is_some());
    assert_eq!(f.f.folder.state(), before);
    let resumed = f.prepare(&f.journal()).await.unwrap();
    assert_eq!(resumed.preview, pending.preview);
    let done = f
        .apply(&journal, f.apply_request(&resumed.preview))
        .await
        .unwrap();
    assert!(done.receipt.completed());
    assert_eq!(
        f.store.snapshot().unwrap().active,
        Some(pending.preview.plan.combination.id)
    );
}
#[tokio::test]
async fn setup_replays_original_head_receipt_and_pointer_interruptions_without_repreparing() {
    for boundary in [Boundary::Head, Boundary::Published, Boundary::Pointer] {
        let f = SetupInstalled::new().await;
        let journal = f.journal();
        let prepared = f.prepare(&journal).await.unwrap();
        let paused = pause(&f, boundary);
        let before = f.f.folder.state();
        let apply = f.apply_request(&prepared.preview);
        assert!(matches!(
            p::apply_setup(
                &f.store,
                &journal,
                &Coordinator::new(1).unwrap(),
                &f.observer,
                &paused,
                f.f.actor,
                apply.clone()
            )
            .await,
            Err(p::Error::Forbidden)
        ));
        assert_eq!(f.store.snapshot().unwrap().revision, 0);
        assert_eq!(&f.f.folder.state()[..2], &before[..2]);
        assert!(
            !journal
                .status(f.f.actor, f.request.request_id, f.request.intent_digest)
                .unwrap()
                .unwrap()
                .receipt
                .completed()
        );
        let done = f.apply(&f.journal(), apply).await.unwrap();
        assert!(done.receipt.completed());
        assert_eq!(done.preview, prepared.preview);
        assert_eq!(
            f.store.snapshot().unwrap().active,
            Some(prepared.preview.plan.combination.id)
        );
        assert_eq!(&f.f.folder.state()[..2], &before[..2]);
    }
}
#[tokio::test]
async fn setup_revoked_registration_prevents_original_preparation_and_apply_recovery() {
    let f = SetupInstalled::new().await;
    let journal = f.journal();
    let prepared = f.prepare(&journal).await.unwrap();
    f.f.manager
        .remove(
            f.f.binding.repository,
            RemoveBinding {
                version: 1,
                expected_revision: f.f.manager.list().unwrap().revision,
            },
        )
        .unwrap();
    let before = f.f.folder.state();
    assert!(matches!(
        f.prepare(&journal).await,
        Err(p::Error::Forbidden)
    ));
    assert!(matches!(
        f.apply(&journal, f.apply_request(&prepared.preview)).await,
        Err(p::Error::Forbidden)
    ));
    assert_eq!(f.f.folder.state(), before);
    assert_eq!(f.store.snapshot().unwrap().revision, 0);
}

#[tokio::test]
async fn setup_original_apply_recovers_retained_source_after_metadata_loss_and_git_gc() {
    let f = SetupInstalled::new().await;
    let journal = f.journal();
    let prepared = f.prepare(&journal).await.unwrap();
    let before = f.f.folder.state();
    apply::lose_source(&f.f);
    let resumed = f.prepare(&f.journal()).await.unwrap();
    assert_eq!(resumed.preview, prepared.preview);
    assert!(!f.f.folder.root().join(".jj/repo/op_store").exists());
    let done = f
        .apply(&f.journal(), f.apply_request(&prepared.preview))
        .await
        .unwrap();
    assert!(done.receipt.completed());
    assert_eq!(&f.f.folder.state()[..2], &before[..2]);
    assert_eq!(
        f.store.snapshot().unwrap().active,
        Some(prepared.preview.plan.combination.id)
    );
}
