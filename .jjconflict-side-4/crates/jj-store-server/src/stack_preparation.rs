//! Exact unpublished local preparation composition; no publication/checkout route.
mod registered;
mod report;
use crate::stack_command_journal::{self as j, LocalExecutionV1, StackCommandJournal};
use jj_store_jj::{
    local_browser::{self as q, Access, working_tree::CurrentState},
    local_execution::{Coordinator, Reword},
    local_metadata::StackRetentionOutcome,
};
use jj_store_model as m;
use jj_store_service::stack_workspace as s;
pub use registered::{ActiveCombination, CombinationAuthority, RegisteredAuthority};
use std::{path::PathBuf, sync::Arc};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid local stack preparation")]
    Invalid,
    #[error("local stack selection changed")]
    Stale,
    #[error("local stack preparation is unauthorized")]
    Forbidden,
    #[error("local stack command is not qualified")]
    Unsupported,
    #[error("local stack preparation outcome is unknown; recover the original request")]
    OutcomeUnknown,
    #[error(transparent)]
    Journal(#[from] j::Error),
    #[error(transparent)]
    Provider(#[from] q::Error),
}
pub type Result<T> = std::result::Result<T, Error>;
pub struct Context {
    pub access: Arc<dyn Access>,
    pub root: PathBuf,
    pub current: CurrentState,
    pub working_tree: m::RecordDigest,
    pub combination: s::CombinationSelection,
    pub immutable_heads: Vec<m::GitSha1Id>,
}
pub struct RecoveryContext {
    pub access: Arc<dyn Access>,
    pub root: PathBuf,
}
/// Owning installation composition supplies current command authorization,
/// verified registration and actual combination/physical selection. No paths
/// or authority are inferred from incoming command fields. Production wiring
/// remains disabled until this authority and publication/checkout qualify.
#[async_trait::async_trait]
pub trait Authority: Send + Sync {
    async fn authorize(&self, principal: m::PrincipalId, request: &s::PrepareRequest)
    -> Result<()>;
    async fn context(
        &self,
        principal: m::PrincipalId,
        request: &s::PrepareRequest,
    ) -> Result<Context>;
    async fn recovery(
        &self,
        _principal: m::PrincipalId,
        _request: &s::PrepareRequest,
    ) -> Result<RecoveryContext> {
        Err(Error::Unsupported)
    }
}
pub async fn prepare_reword(
    journal: &StackCommandJournal,
    coordinator: &Coordinator,
    authority: &dyn Authority,
    principal: m::PrincipalId,
    request: s::PrepareRequest,
) -> Result<j::Selection> {
    authority.authorize(principal, &request).await?;
    request.validate().map_err(|_| Error::Invalid)?;
    journal.check_scope(&request.selection)?;
    if let Some(original) = journal.status(principal, request.request_id, request.intent_digest)? {
        if original.preview.plan.request != request {
            return Err(Error::Journal(j::Error::IntentMismatch));
        }
        let Some(execution) = original.execution else {
            return Err(Error::Unsupported);
        };
        if execution.retention.is_some() || original.receipt.state != s::ReceiptState::Prepared {
            return Ok(j::Selection {
                preview: original.preview,
                receipt: original.receipt,
                replayed: true,
            });
        }
        // A saved exact archive is resumed; neither context nor preparation is repeated.
        let context = authority.recovery(principal, &request).await?;
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
        return finish(journal, principal, &original.preview, retained);
    }
    let s::Command::Reword {
        commit: s::CommitIdentity::GitSha1(commit),
        message,
    } = &request.command
    else {
        return Err(Error::Unsupported);
    };
    let context = authority.context(principal, &request).await?;
    report::check(&request, &context)?;
    let lease = coordinator
        .acquire(context.access, context.root, context.current)
        .await?;
    let (lease, witness) = lease.recovery_witness().await?;
    let (lease, candidate) = lease
        .prepare_reword(Reword {
            principal,
            commit: *commit,
            message: message.clone(),
            immutable_heads: context.immutable_heads,
        })
        .await?;
    let preview = report::preview(request.clone(), &candidate)?;
    let execution = LocalExecutionV1::pending(
        principal,
        &preview,
        witness.clone(),
        candidate.history.clone(),
    )?;
    authority.authorize(principal, &request).await?;
    journal.record_local_prepared(principal, &preview, &execution)?;
    authority.authorize(principal, &request).await?;
    // Saving the full original archive precedes reserved-ref selection, so a
    // process/acknowledgement loss can only resume this exact original candidate.
    let (lease, retained) = lease
        .retain_history(witness, execution.key, candidate.history)
        .await?;
    authority.authorize(principal, &request).await?;
    let selected = finish(journal, principal, &preview, retained)?;
    drop(lease);
    Ok(selected)
}
fn finish(
    journal: &StackCommandJournal,
    principal: m::PrincipalId,
    preview: &s::Preview,
    retained: StackRetentionOutcome,
) -> Result<j::Selection> {
    match retained {
        StackRetentionOutcome::Retained(retained) => {
            Ok(journal.acknowledge_local_retention(principal, preview, &retained)?)
        }
        StackRetentionOutcome::Unknown { .. } => Err(Error::OutcomeUnknown),
    }
}
