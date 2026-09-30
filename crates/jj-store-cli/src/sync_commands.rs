//! Explicit local synchronization controls for native repositories.
use crate::{
    binding::Connection,
    commands::{Format, error},
    presentation,
};
use clap::{Args, Subcommand};
use jj_cli::{cli_util::CommandHelper, command_error::CommandError, ui::Ui};
use jj_store_model as m;
use jj_store_service::{commands::Command, mutations};
use jj_store_wal::publication::Snapshot;
use sha2::{Digest as _, Sha256};

#[derive(Clone, Debug, Args)]
#[command(disable_help_subcommand = true)]
pub struct SyncArgs {
    #[arg(long, global = true, value_enum, default_value_t=Format::Text)]
    pub format: Format,
    #[command(subcommand)]
    pub command: Option<SyncCommand>,
}

#[derive(Clone, Debug, Subcommand)]
pub enum SyncCommand {
    /// Merge the exact current operation heads through one receipt-backed mutation
    Reconcile,
}

fn request_id(repository: m::RepositoryId, snapshot: &Snapshot) -> m::RequestId {
    let mut hash = Sha256::new_with_prefix(b"jj-store/sync-reconcile-request/v1\0");
    hash.update(repository.as_bytes());
    hash.update(snapshot.epoch.as_bytes());
    hash.update(snapshot.position.0.to_le_bytes());
    hash.update(snapshot.code_state_version.to_le_bytes());
    hash.update((snapshot.heads.len() as u64).to_le_bytes());
    for head in &snapshot.heads {
        hash.update(head.as_bytes());
    }
    let digest = hash.finalize();
    let mut id = [0; 16];
    id.copy_from_slice(&digest[..16]);
    m::RequestId::new(id)
}

pub async fn run(ui: &mut Ui, helper: &CommandHelper, args: SyncArgs) -> Result<(), CommandError> {
    let Some(command) = args.command else {
        return presentation::help(ui, helper, &["sync".into()]);
    };
    match command {
        SyncCommand::Reconcile => reconcile(ui, helper, args.format).await,
    }
}

async fn reconcile(
    ui: &mut Ui,
    helper: &CommandHelper,
    format: Format,
) -> Result<(), CommandError> {
    let connection = Connection::read(helper.workspace_loader()?.repo_path()).map_err(error)?;
    let client = connection.connect().await.map_err(error)?;
    let expected = client.snapshot().await.map_err(error)?;
    if expected.heads.len() == 1 {
        return presentation::result(
            ui,
            helper,
            format,
            &serde_json::json!({"version":1,"status":"not_needed","snapshot":expected}),
            "Operation heads are already reconciled.\n",
        );
    }
    let base = *expected
        .heads
        .first()
        .ok_or_else(|| error("native snapshot contains no operation head"))?;
    let request = mutations::Request {
        version: 1,
        request_id: request_id(connection.repository, &expected),
        expected,
        command: Command::ReconcileOperations { base },
    };
    let outcome = client.apply_mutation(&request).await.map_err(error)?;
    match outcome {
        mutations::Outcome::Committed { receipt } => presentation::result(
            ui,
            helper,
            format,
            &mutations::Outcome::Committed {
                receipt: receipt.clone(),
            },
            &format!(
                "Reconciled operation heads at local position {}.\nOperation: {}\n",
                receipt.accepted.position.0, receipt.result.operation
            ),
        ),
        mutations::Outcome::OutcomeUnknown { request_id } => {
            let status = client.mutation_status(request_id).await.map_err(error)?;
            presentation::result(
                ui,
                helper,
                format,
                &status,
                &format!(
                    "Reconciliation outcome is unknown. Retain request ID {request_id} for retry.\n"
                ),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn reconcile_identity_is_stable_and_snapshot_bound() {
        let original = Snapshot {
            epoch: m::EpochId::new([2; 16]),
            position: m::WalPosition(7),
            code_state_version: 3,
            heads: BTreeSet::from([m::OperationId::new([4; 64]), m::OperationId::new([5; 64])]),
        };
        let repository = m::RepositoryId::new([1; 16]);
        assert_eq!(
            request_id(repository, &original),
            request_id(repository, &original)
        );
        let mut changed = original.clone();
        changed.position = m::WalPosition(8);
        assert_ne!(
            request_id(repository, &changed),
            request_id(repository, &original)
        );
    }
}
