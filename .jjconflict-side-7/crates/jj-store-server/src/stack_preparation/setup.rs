//! Explicit initial combination creation from real retained provider output.
mod apply;
mod report;
use super::*;
use crate::stack_combinations::CombinationStore;
pub use apply::apply_setup;
use jj_store_jj::{
    local_execution::{PreparedSetup, Setup, SetupBookmark},
    local_metadata::StackRetention,
};
use jj_store_service::working_tree::Observer;

pub async fn prepare_setup(
    journal: &StackCommandJournal,
    store: &CombinationStore,
    coordinator: &Coordinator,
    observer: &Observer,
    authority: &dyn Authority,
    principal: m::PrincipalId,
    request: s::PrepareRequest,
) -> Result<j::Selection> {
    authority.authorize(principal, &request).await?;
    request.validate().map_err(|_| Error::Invalid)?;
    journal.check_scope(&request.selection)?;
    let target = target(&request)?;
    if let Some(original) = journal.status(principal, request.request_id, request.intent_digest)? {
        if original.preview.plan.request != request {
            return Err(j::Error::IntentMismatch.into());
        }
        if original.receipt.completed()
            || matches!(
                original.receipt.state,
                s::ReceiptState::Rejected { .. } | s::ReceiptState::Retired { .. }
            )
        {
            return Ok(j::Selection {
                preview: original.preview,
                receipt: original.receipt,
                replayed: true,
            });
        }
        let execution = original.execution.ok_or(Error::Unsupported)?;
        let apply = apply_request(&original.preview);
        if execution.combination.is_some() {
            journal.check_local_combination(store, principal, &apply)?;
            return Ok(j::Selection {
                preview: original.preview,
                receipt: original.receipt,
                replayed: true,
            });
        }
        if original.receipt.state != s::ReceiptState::Prepared {
            return Err(Error::Unsupported);
        }
        let context = authority.recovery(principal, &request).await?;
        if execution.retention.is_none() {
            let history = execution.history.ok_or(Error::Invalid)?;
            let retained = coordinator
                .retain_history(
                    context.access,
                    context.root,
                    execution.witness,
                    execution.key,
                    history,
                )
                .await?;
            authority.authorize(principal, &request).await?;
            return finish(journal, store, principal, &original.preview, retained);
        }
        let recovery = coordinator
            .resume_publication(
                context.access,
                context.root,
                execution.witness,
                execution.key,
            )
            .await?;
        let (_, retained) = recovery.retained_setup(target)?;
        authority.authorize(principal, &request).await?;
        let result = bind(journal, store, principal, &original.preview, &retained)?;
        drop(recovery);
        return Ok(result);
    }
    store.require_uninitialized(journal.scope(), principal)?;
    let context = authority.setup_context(principal, &request).await?;
    report::check(&request, &context)?;
    let physical = observer.physical_guard(context.access.clone(), context.root.clone());
    let lease = coordinator
        .acquire(context.access, context.root, context.current)
        .await?;
    let (lease, witness) = lease.recovery_witness().await?;
    let (lease, candidate) = lease
        .prepare_setup(
            Setup { principal, target },
            physical,
            request.selection.working_tree,
        )
        .await?;
    let preview = report::preview(request, &candidate)?;
    let execution = LocalExecutionV1::pending(
        principal,
        &preview,
        witness.clone(),
        candidate.history.clone(),
    )?;
    authority
        .authorize(principal, &preview.plan.request)
        .await?;
    journal.record_local_prepared(principal, &preview, &execution)?;
    let (lease, retained) = lease
        .retain_history(witness, execution.key, candidate.history)
        .await?;
    authority
        .authorize(principal, &preview.plan.request)
        .await?;
    let result = finish(journal, store, principal, &preview, retained)?;
    drop(lease);
    Ok(result)
}
fn target(request: &s::PrepareRequest) -> Result<SetupBookmark> {
    let s::Command::Setup { target, .. } = &request.command else {
        return Err(Error::Unsupported);
    };
    let s::CommitIdentity::GitSha1(head) = target.head else {
        return Err(Error::Unsupported);
    };
    Ok(SetupBookmark {
        name: target.name.clone(),
        remote: target.remote.clone(),
        head,
    })
}
fn apply_request(preview: &s::Preview) -> s::ApplyRequest {
    let r = &preview.plan.request;
    s::ApplyRequest {
        version: 1,
        request_id: r.request_id,
        intent_digest: r.intent_digest,
        prepared: preview.prepared,
        selection: r.selection.clone(),
    }
}
fn finish(
    journal: &StackCommandJournal,
    store: &CombinationStore,
    principal: m::PrincipalId,
    preview: &s::Preview,
    retained: StackRetentionOutcome,
) -> Result<j::Selection> {
    let StackRetentionOutcome::Retained(retained) = retained else {
        return Err(Error::OutcomeUnknown);
    };
    journal.acknowledge_local_retention(principal, preview, &retained)?;
    bind(journal, store, principal, preview, &retained)
}
fn bind(
    journal: &StackCommandJournal,
    store: &CombinationStore,
    principal: m::PrincipalId,
    preview: &s::Preview,
    retained: &StackRetention,
) -> Result<j::Selection> {
    let after = report::snapshot(preview, retained)?;
    Ok(journal.save_local_combination(
        store,
        principal,
        &apply_request(preview),
        0,
        &after,
        retained,
    )?)
}
