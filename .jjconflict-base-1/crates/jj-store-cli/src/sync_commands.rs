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
use std::time::Duration;

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
    /// Show local readiness and distinct local/upstream replication frontiers
    Status,
    /// Wake the background worker and return without waiting
    Now,
    /// Wait for the finite local and upstream frontiers captured at invocation
    Wait {
        #[arg(long, default_value = "30s", value_parser = parse_timeout)]
        timeout: Duration,
    },
    /// Pause background transfer without blocking local repository work
    Pause,
    /// Resume background transfer and wake the worker
    Resume,
    /// Merge the exact current operation heads through one receipt-backed mutation
    Reconcile,
}

#[derive(Clone, Debug, Args)]
#[command(disable_help_subcommand = true)]
pub struct ReplicaArgs {
    #[arg(long, global = true, value_enum, default_value_t=Format::Text)]
    pub format: Format,
    #[command(subcommand)]
    pub command: Option<ReplicaCommand>,
}

#[derive(Clone, Debug, Subcommand)]
pub enum ReplicaCommand {
    /// Inspect exact issuer-bound frontiers and the durable peer checkpoint
    Inspect,
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
        SyncCommand::Status => control(ui, helper, args.format, Control::Status).await,
        SyncCommand::Now => control(ui, helper, args.format, Control::Now).await,
        SyncCommand::Wait { timeout } => wait(ui, helper, args.format, timeout).await,
        SyncCommand::Pause => control(ui, helper, args.format, Control::Pause).await,
        SyncCommand::Resume => control(ui, helper, args.format, Control::Resume).await,
        SyncCommand::Reconcile => reconcile(ui, helper, args.format).await,
    }
}

pub async fn run_replica(
    ui: &mut Ui,
    helper: &CommandHelper,
    args: ReplicaArgs,
) -> Result<(), CommandError> {
    let Some(ReplicaCommand::Inspect) = args.command else {
        return presentation::help(ui, helper, &["replica".into()]);
    };
    let connection = Connection::read(helper.workspace_loader()?.repo_path()).map_err(error)?;
    let status = connection
        .connect()
        .await
        .map_err(error)?
        .replica_inspect()
        .await
        .map_err(error)?;
    presentation::result(
        ui,
        helper,
        args.format,
        &serde_json::to_value(&status).map_err(error)?,
        &status_text(&status, true),
    )
}

#[derive(Clone, Copy)]
enum Control {
    Status,
    Now,
    Pause,
    Resume,
}

async fn control(
    ui: &mut Ui,
    helper: &CommandHelper,
    format: Format,
    action: Control,
) -> Result<(), CommandError> {
    let connection = Connection::read(helper.workspace_loader()?.repo_path()).map_err(error)?;
    let client = connection.connect().await.map_err(error)?;
    let status = match action {
        Control::Status => client.replication_status().await,
        Control::Now => client.replication_now().await,
        Control::Pause => client.replication_pause().await,
        Control::Resume => client.replication_resume().await,
    }
    .map_err(error)?;
    presentation::result(
        ui,
        helper,
        format,
        &serde_json::to_value(&status).map_err(error)?,
        &status_text(&status, false),
    )
}

async fn wait(
    ui: &mut Ui,
    helper: &CommandHelper,
    format: Format,
    timeout: Duration,
) -> Result<(), CommandError> {
    let connection = Connection::read(helper.workspace_loader()?.repo_path()).map_err(error)?;
    let client = connection.connect().await.map_err(error)?;
    let barrier = client.replication_capture_barrier().await.map_err(error)?;
    let outcome = client
        .replication_wait(&jj_store_service::replication_control::WaitRequestV1 {
            version: 1,
            barrier,
            timeout_millis: u64::try_from(timeout.as_millis())
                .map_err(|_| error("sync wait timeout exceeds supported range"))?,
        })
        .await
        .map_err(error)?;
    presentation::result(
        ui,
        helper,
        format,
        &serde_json::to_value(&outcome).map_err(error)?,
        &format!(
            "Reached captured frontiers.\nPush: {}/{}/{}\nPull: {}/{}/{}\n",
            outcome.barrier.local_replica,
            outcome.barrier.push_epoch,
            outcome.barrier.push_through.0,
            outcome.barrier.peer_replica,
            outcome.barrier.pull_epoch,
            outcome.barrier.pull_through.0,
        ),
    )
}

fn status_text(status: &jj_store_service::replication_control::StatusV1, inspect: bool) -> String {
    let frontier = |value: Option<jj_store_service::replication_control::FrontierV1>| {
        value.map_or_else(
            || "unavailable".into(),
            |value| {
                format!(
                    "{}/{}/{} (observed {})",
                    value.replica, value.log_epoch, value.through.0, value.observed_upper_bound.0
                )
            },
        )
    };
    let mut text = format!(
        "Local repository: {}\nReplication: {:?}\nLocal pending WAL positions: {}\nUpstream pending observed WAL positions: {}\nOperation heads: {}{}\nSearch: {}\n",
        if status.offline_ready {
            "available, offline-ready"
        } else {
            "unavailable"
        },
        status.phase,
        status
            .local_pending_positions
            .map_or_else(|| "unknown".into(), |value| value.to_string()),
        status
            .peer_pending_positions
            .map_or_else(|| "unknown".into(), |value| value.to_string()),
        status.operation_heads,
        if status.reconciliation_required {
            " (local reconciliation available)"
        } else {
            ""
        },
        if !status.search.configured {
            "not configured".into()
        } else if status.search.degraded {
            "degraded".into()
        } else {
            status.search.indexed_position.map_or_else(
                || "not ready".into(),
                |position| format!("indexed through local publication {}", position.0),
            )
        },
    );
    if inspect {
        text.push_str(&format!(
            "Local frontier: {}\nUpstream frontier: {}\nPushed checkpoint: {}\nPulled checkpoint: {}\nCheckpoint revision: {}\n",
            frontier(status.local_frontier),
            frontier(status.peer_frontier),
            frontier(status.pushed),
            frontier(status.pulled),
            status.checkpoint_revision,
        ));
    }
    text
}

fn parse_timeout(value: &str) -> Result<Duration, String> {
    let (number, unit) = if let Some(value) = value.strip_suffix("ms") {
        (value, "ms")
    } else if let Some(value) = value.strip_suffix('s') {
        (value, "s")
    } else if let Some(value) = value.strip_suffix('m') {
        (value, "m")
    } else {
        return Err("timeout must use ms, s, or m".into());
    };
    let number: u64 = number
        .parse()
        .map_err(|_| "timeout must be a positive integer".to_owned())?;
    let duration = match unit {
        "ms" => Duration::from_millis(number),
        "s" => Duration::from_secs(number),
        "m" => Duration::from_secs(number.saturating_mul(60)),
        _ => unreachable!(),
    };
    if duration.is_zero() || duration > Duration::from_secs(300) {
        return Err("timeout must be between 1ms and 5m".into());
    }
    Ok(duration)
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
            &serde_json::to_value(mutations::Outcome::Committed {
                receipt: receipt.clone(),
            })
            .map_err(error)?,
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
                &serde_json::to_value(&status).map_err(error)?,
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

    #[test]
    fn wait_timeout_parser_is_explicit_and_bounded() {
        assert_eq!(parse_timeout("250ms").unwrap(), Duration::from_millis(250));
        assert_eq!(parse_timeout("30s").unwrap(), Duration::from_secs(30));
        assert_eq!(parse_timeout("2m").unwrap(), Duration::from_secs(120));
        for invalid in ["30", "0s", "6m", "1h", "-1s"] {
            assert!(parse_timeout(invalid).is_err(), "accepted {invalid}");
        }
    }
}
