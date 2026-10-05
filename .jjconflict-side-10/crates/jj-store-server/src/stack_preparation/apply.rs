use super::*;
use jj_store_service::working_tree::Observer;

/// Private original-request execution join. Production transport and combination
/// selection remain separately gated; this returns historical code/checkout state.
pub async fn apply_reword(
    journal: &StackCommandJournal,
    coordinator: &Coordinator,
    observer: &Observer,
    authority: &dyn Authority,
    principal: m::PrincipalId,
    apply: s::ApplyRequest,
) -> Result<j::Selection> {
    let mut record = journal
        .status(principal, apply.request_id, apply.intent_digest)?
        .ok_or(j::Error::NotFound)?;
    let request = record.preview.plan.request.clone();
    authority.authorize(principal, &request).await?;
    apply
        .validate(&request, record.preview.prepared)
        .map_err(|_| j::Error::IntentMismatch)?;
    journal.check_scope(&request.selection)?;
    if matches!(
        record.receipt.state,
        s::ReceiptState::Rejected { .. } | s::ReceiptState::Retired { .. }
    ) || record.receipt.completed()
    {
        return Ok(j::Selection {
            preview: record.preview,
            receipt: record.receipt,
            replayed: true,
        });
    }
    if !matches!(request.command, s::Command::Reword { .. })
        || record.preview.plan.checkout.before != record.preview.plan.checkout.after
        || !record.preview.plan.checkout.paths.is_empty()
    {
        return Err(Error::Unsupported);
    }
    let execution = record.execution.clone().ok_or(Error::Unsupported)?;
    if execution.retention.is_none() {
        return Err(Error::Unsupported);
    }
    // Registration is independent of source history. Current context may need
    // lost operation/index records, so obtain it only after exact restoration.
    let context = authority.recovery(principal, &request).await?;
    let physical = observer.physical_guard(context.access.clone(), context.root.clone());
    let recovery = if let Some(saved) = &execution.checkout {
        let recovery = coordinator
            .resume_unchanged_checkout(
                context.access,
                context.root,
                execution.witness.clone(),
                execution.key.clone(),
                saved.witness.clone(),
            )
            .await?;
        check_retained(&recovery, &execution, &record.preview)?;
        recovery.restore_extras().await?.restore_indexes().await?
    } else {
        let recovery = coordinator
            .resume_publication(
                context.access,
                context.root,
                execution.witness.clone(),
                execution.key.clone(),
            )
            .await?;
        check_retained(&recovery, &execution, &record.preview)?;
        let recovery = recovery.restore_extras().await?.restore_indexes().await?;
        if record.receipt.state == s::ReceiptState::Prepared {
            let context = authority.context(principal, &request).await?;
            report::check(&request, &context)?;
        }
        physical
            .validate(&execution.witness.binding(), request.selection.working_tree)
            .await?;
        authority.authorize(principal, &request).await?;
        if record.receipt.state == s::ReceiptState::Prepared {
            journal.transition(
                principal,
                &apply,
                &s::ReceiptState::Prepared,
                &s::ReceiptState::PublicationUnknown,
            )?;
        }
        let (recovery, proof) = recovery.publish_head().await?;
        authority.authorize(principal, &request).await?;
        if matches!(
            record.receipt.state,
            s::ReceiptState::Prepared | s::ReceiptState::PublicationUnknown
        ) {
            journal.acknowledge_local_publication(principal, &apply, &proof)?;
        }
        let (recovery, checkout) = recovery
            .prepare_unchanged_checkout(&proof, physical.clone(), request.selection.working_tree)
            .await?;
        authority.authorize(principal, &request).await?;
        journal.save_local_checkout(principal, &apply, &checkout)?;
        record = journal
            .status(principal, apply.request_id, apply.intent_digest)?
            .ok_or(j::Error::NotFound)?;
        recovery
    };
    let saved = record
        .execution
        .as_ref()
        .and_then(|e| e.checkout.as_ref())
        .ok_or(Error::Invalid)?;
    authority.authorize(principal, &request).await?;
    let (recovery, proof) = recovery
        .complete_unchanged_checkout(saved.witness.clone(), physical)
        .await?;
    authority.authorize(principal, &request).await?;
    let completed = journal.acknowledge_local_checkout(principal, &apply, &proof)?;
    drop(recovery);
    Ok(completed)
}
fn check_retained(
    recovery: &jj_store_jj::local_execution::RecoveredHistory,
    execution: &LocalExecutionV1,
    preview: &s::Preview,
) -> Result<()> {
    let retained = recovery.retained_identity();
    if Some(retained.commit) != execution.retention
        || retained.archive != execution.archive
        || retained.operation != preview.plan.operation
        || retained.view != preview.plan.view
    {
        return Err(Error::Invalid);
    }
    Ok(())
}
