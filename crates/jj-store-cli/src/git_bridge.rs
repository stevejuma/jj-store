//! Parsed, backend-aware routing for explicitly requested native Git operations.
use jj_cli::{
    cli_util::CommandHelper,
    command_error::{CommandError, user_error},
};

/// Discover the backend without loading operation heads, snapshotting files or
/// creating a second WorkspaceCommandHelper. Creation remains upstream-owned.
pub fn native_operation(
    helper: &CommandHelper,
) -> std::result::Result<Option<String>, CommandError> {
    let Some(git) = helper.matches().subcommand_matches("git") else {
        return Ok(None);
    };
    let Some((name, args)) = git.subcommand() else {
        return Ok(None);
    };
    if matches!(name, "init" | "clone") {
        return Ok(None);
    }
    let workspace = helper.load_workspace()?;
    if workspace.repo_loader().store().backend().name() != "jj-store" {
        return Ok(None);
    }
    let operation = if name == "remote" {
        format!("remote {}", args.subcommand_name().unwrap_or(""))
    } else {
        name.to_owned()
    };
    Ok(Some(operation))
}

pub fn unavailable(operation: &str) -> CommandError {
    user_error(format!(
        "NATIVE_GIT_CAPABILITY: git {operation} is not yet qualified for native repositories"
    ))
    .hinted("Inspect `jj store capabilities` for the current native command matrix.")
}

mod configuration;
mod device;
mod fetch;
mod push;
mod transport;
mod watchdog;
use crate::{commands::error, remote::BridgeContext};
use jj_cli::{cli_util::WorkspaceCommandTransaction, ui::Ui};
use jj_lib::{
    config::ConfigGetResultExt as _,
    object_id::ObjectId as _,
    op_store::{RefTarget, RemoteRef, RemoteRefState},
    ref_name::{RefName, RemoteName, RemoteNameBuf},
    repo::Repo as _,
};
use jj_store_model as m;
use jj_store_wal::publication::*;
pub use push::{GitArgs, run_store};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};
pub use watchdog::entry as transport_worker;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn string(args: &clap::ArgMatches, name: &str) -> Option<String> {
    args.try_get_one::<String>(name)
        .ok()
        .flatten()
        .cloned()
        .or_else(|| {
            args.try_get_one::<RemoteNameBuf>(name)
                .ok()
                .flatten()
                .map(|v| v.as_str().to_owned())
        })
}
fn flag(args: &clap::ArgMatches, name: &str) -> bool {
    args.try_get_one::<bool>(name)
        .ok()
        .flatten()
        .copied()
        .unwrap_or(false)
}
fn strings(args: &clap::ArgMatches, name: &str) -> Vec<String> {
    args.try_get_many::<String>(name)
        .ok()
        .flatten()
        .into_iter()
        .flatten()
        .cloned()
        .collect()
}
fn mutating(helper: &CommandHelper) -> std::result::Result<(), CommandError> {
    if helper.global_args().at_operation.is_some() || helper.global_args().no_integrate_operation {
        return Err(user_error(
            "NATIVE_GIT_CAPABILITY: historical or non-integrating Git mutation is unsupported",
        ));
    }
    Ok(())
}
async fn accept(
    tx: WorkspaceCommandTransaction<'_>,
    ui: &Ui,
    context: &BridgeContext,
    records: Vec<GitRecordV1>,
    id: m::RequestId,
    description: String,
) -> std::result::Result<(), CommandError> {
    let request = jj_store_service::git_bridge::Request {
        version: 1,
        request_id: id,
        records,
        transition: None,
    };
    if tx.repo().has_changes() {
        let _pending = context.begin(request).map_err(error)?;
        tx.finish(ui, description).await
    } else {
        drop(tx);
        match context
            .client
            .accept_git_bridge(&request)
            .await
            .map_err(error)?
        {
            jj_store_service::git_bridge::Outcome::Committed { .. } => Ok(()),
            jj_store_service::git_bridge::Outcome::OutcomeUnknown { request_id } => {
                Err(user_error(format!(
                    "Git acceptance outcome unknown; request {request_id}"
                )))
            }
        }
    }
}
fn remote(
    args: &clap::ArgMatches,
    remotes: &BTreeMap<String, m::RequestId>,
    settings: &jj_lib::settings::UserSettings,
    fetch: bool,
) -> std::result::Result<(String, m::RequestId), CommandError> {
    let named = if fetch {
        strings(args, "remotes")
    } else {
        string(args, "remote").into_iter().collect()
    };
    let configured = if named.is_empty() {
        let key = if fetch { "git.fetch" } else { "git.push" };
        if let Ok(values) = settings.get::<Vec<String>>(key) {
            values
        } else {
            settings.get_string(key).optional()?.into_iter().collect()
        }
    } else {
        named
    };
    let name = match configured.as_slice() {
        [name] => name.clone(),
        [] if remotes.len() == 1 => remotes.keys().next().unwrap().clone(),
        [] => "origin".into(),
        _ => return Err(unavailable("multiple remotes")),
    };
    let id = remotes
        .get(&name)
        .copied()
        .ok_or_else(|| user_error("Git logical remote not found; add one explicitly"))?;
    Ok((name, id))
}
pub async fn run(
    ui: &mut Ui,
    helper: &CommandHelper,
    registry: &crate::binding::FactoryRegistry,
    operation: &str,
) -> std::result::Result<(), CommandError> {
    let git = helper.matches().subcommand_matches("git").unwrap();
    let (name, args) = git.subcommand().unwrap();
    if name == "push" {
        return push::run(ui, helper, registry, args).await;
    }
    if !matches!(name, "remote" | "fetch") {
        return Err(unavailable(operation));
    }
    if name == "remote" {
        return configuration::run(ui, helper, registry, args, operation).await;
    }
    fetch::run(ui, helper, registry, args).await
}
