//! Frozen selected publication with native durability before external transport.
use super::*;
mod apply;
mod attempts;
mod prepare;
mod selection;
use apply::apply;
pub use attempts::{GitArgs, run_store};
use attempts::{record, stage_id};
use futures::TryStreamExt as _;
use jj_cli::cli_util::{RevisionArg, WorkspaceCommandHelper, has_tracked_remote_bookmarks};
use jj_lib::{
    refs::{LocalAndRemoteRef, RefPushAction, classify_ref_push_action},
    revset::{RemoteRefSymbolExpression, RevsetExpression, RevsetStreamExt as _},
    str_util::StringExpression,
};
use selection::explicit;
use sha2::{Digest as _, Sha256};
use std::io::Write as _;

pub async fn run(
    ui: &mut Ui,
    helper: &CommandHelper,
    registry: &crate::binding::FactoryRegistry,
    args: &clap::ArgMatches,
) -> std::result::Result<(), CommandError> {
    mutating(helper)?;
    if ["tag", "option", "allow_conflicts"]
        .iter()
        .any(|v| explicit(args, v))
    {
        return Err(unavailable("push selection or options"));
    }
    let loaded = helper.load_workspace()?;
    let context = registry.context(loaded.repo_path()).map_err(error)?;
    context.client.authorize(true).await.map_err(error)?;
    let state = context.client.git_bridge_state().await.map_err(error)?;
    let accepted = state
        .git
        .as_ref()
        .ok_or_else(|| unavailable("push without fetched remote"))?;
    let mut workspace = helper.workspace_helper(ui).await?;
    if workspace.settings().get_bool("git.sign-on-push")? {
        return Err(unavailable("sign on push"));
    }
    let (remote_name, remote_id) = remote(
        args,
        &accepted.remotes().map_err(error)?,
        workspace.settings(),
        false,
    )?;
    let device = device::Device::open(context.client.repository(), workspace.workspace_root())
        .map_err(error)?;
    let endpoint = device.endpoint(remote_id).map_err(error)?.clone();
    let selection::Selection { generated, updates } = selection::select(
        ui,
        &workspace,
        args,
        accepted,
        remote_id,
        &remote_name,
        &endpoint,
    )
    .await?;
    if updates.is_empty() {
        return crate::presentation::text(ui, helper, "Nothing changed.\n");
    }
    let prepare::Prepared {
        refs,
        cache,
        mut records,
    } = prepare::prepare(
        ui,
        &workspace,
        &context,
        prepare::Input {
            args,
            accepted,
            device: &device,
            remote_name: &remote_name,
        },
        updates,
    )
    .await?;
    let selection = refs.keys().cloned().collect::<Vec<_>>().join(", ");
    if flag(args, "dry_run") {
        return crate::presentation::text(
            ui,
            helper,
            &format!("Dry-run: {selection} -> {remote_name}; exact leases; no publication.\n"),
        );
    }
    if !generated.is_empty() {
        let mut tx = workspace.start_transaction();
        for (name, id) in &generated {
            tx.repo_mut()
                .set_local_bookmark_target(RefName::new(name), RefTarget::normal(id.clone()));
        }
        tx.finish(ui, "prepare selected native Git bookmarks")
            .await?;
    }
    let attempt = device::id().map_err(error)?;
    let intent = GitPushIntentV1 {
        id: attempt,
        executor: state.principal,
        remote: remote_id,
        binding: endpoint.fingerprint,
        operation: m::OperationId::from_slice(workspace.repo().op_id().as_bytes())
            .map_err(error)?,
        observations: accepted
            .observations(remote_id, endpoint.fingerprint)
            .iter()
            .map(|v| v.id)
            .collect(),
        refs,
    };
    records.push(GitRecordV1::PushIntent(intent.clone()));
    record(&context, attempt, records).await?;
    writeln!(ui.stderr(), "Prepared Git attempt {attempt}")?;
    let current = context.client.git_bridge_state().await.map_err(error)?;
    let current = current
        .git
        .as_ref()
        .ok_or_else(|| user_error("Git root unavailable before transport"))?;
    if current.remotes().map_err(error)?.get(&remote_name) != Some(&remote_id)
        || intent.refs.iter().any(|(name, value)| {
            current.expectation(remote_id, endpoint.fingerprint, name)
                != Some(value.expected.clone())
        })
    {
        return Err(user_error(format!(
            "Git destination evidence changed; replan attempt {attempt} before transport"
        )));
    }
    let started = stage_id(attempt, "started");
    record(
        &context,
        started,
        vec![GitRecordV1::PushEvent(GitPushEventV1 {
            id: started,
            attempt,
            predecessors: BTreeSet::new(),
            kind: GitPushEventKindV1::TransportStarted,
        })],
    )
    .await?;
    // The device lock pins the approved binding for the full attempt, while
    // ordinary native work remains free to advance on other operations.
    let outcomes = transport::push(&cache.bare, &endpoint.url, &intent.refs)
        .await
        .unwrap_or_else(|_| {
            intent
                .refs
                .keys()
                .map(|name| (name.clone(), GitPushRefOutcomeV1::Unknown))
                .collect()
        });
    let result = stage_id(attempt, "result");
    if let Err(failure) = record(
        &context,
        result,
        vec![GitRecordV1::PushEvent(GitPushEventV1 {
            id: result,
            attempt,
            predecessors: BTreeSet::from([started]),
            kind: GitPushEventKindV1::ResultRecorded {
                refs: outcomes.clone(),
            },
        })],
    )
    .await
    {
        return Err(failure.hinted(format!(
            "Git result recording pending; recover attempt {attempt}"
        )));
    }
    if !outcomes.values().all(|v| v.satisfied()) {
        return Err(user_error(format!(
            "Git push rejected or outcome unknown; fetch/review or recover attempt {attempt}"
        )));
    }
    apply(
        ui,
        &mut workspace,
        &context,
        &intent,
        &remote_name,
        BTreeSet::from([result]),
    )
    .await
    .map_err(|failure| {
        failure.hinted(format!(
            "Git remote satisfied; native apply pending. Recover attempt {attempt}"
        ))
    })?;
    crate::presentation::text(
        ui,
        helper,
        &format!("Push completed: {selection} -> {remote_name}\nAttempt: {attempt}\n"),
    )
}
