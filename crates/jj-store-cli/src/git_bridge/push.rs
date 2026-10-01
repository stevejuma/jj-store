//! Frozen selected publication with native durability before external transport.
use super::*;
use futures::TryStreamExt as _;
use jj_cli::cli_util::{RevisionArg, WorkspaceCommandHelper, has_tracked_remote_bookmarks};
use jj_lib::{
    refs::{LocalAndRemoteRef, RefPushAction, classify_ref_push_action},
    revset::{RemoteRefSymbolExpression, RevsetExpression, RevsetStreamExt as _},
    str_util::StringExpression,
};
use sha2::{Digest as _, Sha256};
use std::io::Write as _;

pub fn stage_id(attempt: m::RequestId, stage: &str) -> m::RequestId {
    let mut hash = Sha256::new_with_prefix(b"jj-store/git-push-stage/v1\0");
    hash.update(attempt.as_bytes());
    hash.update(stage.as_bytes());
    let mut bytes = [0; 16];
    bytes.copy_from_slice(&hash.finalize()[..16]);
    m::RequestId::new(bytes)
}
async fn record(
    context: &BridgeContext,
    id: m::RequestId,
    records: Vec<GitRecordV1>,
) -> std::result::Result<(), CommandError> {
    let request = jj_store_service::git_bridge::Request {
        version: 1,
        request_id: id,
        records,
        transition: None,
    };
    match context
        .client
        .accept_git_bridge(&request)
        .await
        .map_err(error)?
    {
        jj_store_service::git_bridge::Outcome::Committed { .. } => Ok(()),
        jj_store_service::git_bridge::Outcome::OutcomeUnknown { request_id } => Err(user_error(
            format!("Git native outcome unknown; request {request_id}"),
        )),
    }
}
fn explicit(args: &clap::ArgMatches, name: &str) -> bool {
    args.try_contains_id(name).unwrap_or(false)
        && args.value_source(name) == Some(clap::parser::ValueSource::CommandLine)
}
fn add_generated(
    workspace: &WorkspaceCommandHelper,
    generated: &mut BTreeMap<String, jj_lib::backend::CommitId>,
    name: String,
    id: jj_lib::backend::CommitId,
    allow_matching: bool,
) -> std::result::Result<(), CommandError> {
    if !valid_git_ref(&format!("refs/heads/{name}")) {
        return Err(user_error("Unsupported generated Git bookmark"));
    }
    let target = workspace
        .repo()
        .view()
        .get_local_bookmark(RefName::new(&name));
    if !(allow_matching && target.as_normal() == Some(&id))
        && (target.is_present()
            || has_tracked_remote_bookmarks(workspace.repo().as_ref(), RefName::new(&name)))
    {
        return Err(user_error(format!(
            "Generated bookmark {name} already exists or has tracked remote state"
        )));
    }
    if generated
        .insert(name, id.clone())
        .is_some_and(|old| old != id)
    {
        return Err(user_error("Generated bookmark name collision"));
    }
    Ok(())
}
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
    let patterns = strings(args, "bookmark");
    let revisions: Vec<_> = args
        .try_get_many::<RevisionArg>("revisions")
        .ok()
        .flatten()
        .into_iter()
        .flatten()
        .cloned()
        .collect();
    let changes: Vec<_> = args
        .try_get_many::<RevisionArg>("change")
        .ok()
        .flatten()
        .into_iter()
        .flatten()
        .cloned()
        .collect();
    let mut generated = BTreeMap::new();
    if !changes.is_empty() {
        let template_text = workspace
            .settings()
            .get_string("templates.git_push_bookmark")?;
        let template = workspace.parse_commit_template(ui, &template_text)?;
        for id in workspace.resolve_some_revsets(ui, &changes).await? {
            let commit = workspace.repo().store().get_commit_async(&id).await?;
            let name = String::from_utf8(template.format_plain_text(&commit)).map_err(error)?;
            add_generated(&workspace, &mut generated, name, id, true)?;
        }
    }
    for named in strings(args, "named") {
        let (name, revision) = named
            .split_once('=')
            .filter(|(name, revision)| !name.is_empty() && !revision.is_empty())
            .ok_or_else(|| user_error("Named bookmark requires NAME=REVISION"))?;
        let name = jj_cli::revset_util::parse_bookmark_name(name).map_err(error)?;
        let commit = workspace
            .resolve_single_rev(ui, &RevisionArg::from(revision.to_owned()))
            .await?;
        add_generated(
            &workspace,
            &mut generated,
            name.as_str().to_owned(),
            commit.id().clone(),
            false,
        )?;
    }
    let matcher = jj_cli::revset_util::parse_union_name_patterns(ui, &patterns)?.to_matcher();
    let specific = !patterns.is_empty() || !revisions.is_empty() || !generated.is_empty();
    let broad = flag(args, "all") || flag(args, "tracked") || flag(args, "deleted");
    let mut targets = BTreeSet::new();
    for revision in &revisions {
        targets.extend(
            workspace
                .parse_revset(ui, revision)?
                .evaluate_to_commit_ids()?
                .try_collect::<BTreeSet<_>>()
                .await?,
        );
    }
    if !specific && !broad {
        let expression = RevsetExpression::remote_bookmarks(
            RemoteRefSymbolExpression {
                name: StringExpression::all(),
                remote: StringExpression::exact(RemoteName::new(&remote_name)),
            },
            None,
        )
        .range(&RevsetExpression::working_copy(
            workspace.workspace_name().to_owned(),
        ));
        targets = workspace
            .attach_revset_evaluator(expression)
            .evaluate_to_commit_ids()?
            .try_collect()
            .await?;
    }
    let mut names: BTreeSet<String> = generated.keys().cloned().collect();
    for (name, refs) in workspace
        .repo()
        .view()
        .local_remote_bookmarks(RemoteName::new(&remote_name))
    {
        if refs.local_target.is_absent() && !refs.remote_ref.is_tracked() {
            continue;
        }
        let selected = if flag(args, "all") {
            true
        } else if flag(args, "tracked") {
            refs.remote_ref.is_tracked()
        } else if flag(args, "deleted") {
            refs.local_target.is_absent()
        } else {
            matcher.is_match(name.as_str())
                || refs.local_target.added_ids().any(|id| targets.contains(id))
        };
        if selected {
            names.insert(name.as_str().to_owned());
        }
    }
    let mut updates = BTreeMap::new();
    for name in names {
        let reference = format!("refs/heads/{name}");
        if !valid_git_ref(&reference) {
            return Err(user_error("Unsupported Git bookmark ref"));
        }
        let symbol = RefName::new(&name).to_remote_symbol(RemoteName::new(&remote_name));
        let local = generated
            .get(&name)
            .map(|id| RefTarget::normal(id.clone()))
            .unwrap_or_else(|| {
                workspace
                    .repo()
                    .view()
                    .get_local_bookmark(symbol.name)
                    .clone()
            });
        let targets = LocalAndRemoteRef {
            local_target: &local,
            remote_ref: workspace.repo().view().get_remote_bookmark(symbol),
        };
        match classify_ref_push_action(targets) {
            RefPushAction::AlreadyMatches => continue,
            RefPushAction::Update(update) => {
                let allow_new = flag(args, "all")
                    || generated.contains_key(&name)
                    || (matcher.is_match(&name)
                        && !has_tracked_remote_bookmarks(workspace.repo().as_ref(), symbol.name));
                let allow_delete = flag(args, "deleted") || matcher.is_match(&name);
                if (!targets.remote_ref.is_tracked() && !allow_new)
                    || (update.after.is_none() && !allow_delete)
                {
                    return Err(user_error(format!(
                        "Native bookmark {name} requires explicit creation/tracking or deletion selection"
                    )));
                }
                let expected=accepted.expectation(remote_id,endpoint.fingerprint,&reference).ok_or_else(|| user_error("Git expectation unknown or conflicting; fetch this approved destination before pushing"))?;
                let aligned=match (&update.before,&expected) {
                    (None,None)=>true,
                    (Some(native),Some(oid))=>accepted.values.values().any(|v| matches!(v,GitRecordV1::Representation(v) if v.native_commit.as_bytes()==native.as_bytes() && &v.git_commit==oid)),
                    _=>false,
                };
                if !aligned {
                    return Err(user_error(
                        "Native remote bookmark differs from durable Git evidence; fetch before pushing",
                    ));
                }
                updates.insert(reference, (expected, update.after));
            }
            action => {
                return Err(user_error(format!(
                    "Native bookmark cannot be pushed: {action:?}; track or resolve it first"
                )));
            }
        }
    }
    if updates.is_empty() {
        return crate::presentation::text(ui, helper, "Nothing changed.\n");
    }
    let heads: Vec<_> = updates.values().filter_map(|(_, id)| id.clone()).collect();
    let known: Vec<_> = workspace
        .repo()
        .view()
        .remote_bookmarks(RemoteName::new(&remote_name))
        .flat_map(|(_, v)| v.target.added_ids().cloned())
        .collect();
    let immutable = workspace
        .attach_revset_evaluator(workspace.env().immutable_heads_expression().clone())
        .resolve()?;
    let commits = RevsetExpression::commits(known)
        .union(&immutable)
        .range(&RevsetExpression::commits(heads));
    let private = workspace.settings().get_string("git.private-commits")?;
    let is_private = workspace
        .parse_revset(ui, &RevisionArg::from(private))?
        .evaluate()?
        .containing_fn();
    let evaluated = commits.evaluate(workspace.repo().as_ref())?;
    let mut stream = evaluated.stream().commits(workspace.repo().store());
    while let Some(commit) = stream.try_next().await? {
        if (commit.description().is_empty() && !flag(args, "allow_empty_description"))
            || commit.author().name.is_empty()
            || commit.author().email.is_empty()
            || commit.committer().name.is_empty()
            || commit.committer().email.is_empty()
            || commit.has_conflict()
            || (!flag(args, "allow_private") && is_private(commit.id()).await?)
        {
            return Err(user_error(
                "Selected native history has an empty description, missing identity, conflict or private commit",
            ));
        }
    }
    drop(stream);
    drop(evaluated);
    drop(is_private);
    let cache = device.cache().map_err(error)?;
    transport::qualify(&cache.bare).await.map_err(error)?;
    let native = Arc::new(
        jj_store_jj::objects::NativeObjects::new(
            Arc::new(jj_store_rpc::client::RemoteObjects(context.client.clone())),
            context.client.repository(),
            32 * 1024 * 1024,
        )
        .map_err(error)?,
    );
    let mut converter = jj_store_jj::git_bridge::GitConverter::open(
        workspace.settings(),
        &cache.root.path().join("conversion"),
        &cache.bare,
        native,
        jj_store_jj::git_bridge::Limits::default(),
        Some(accepted),
    )
    .map_err(error)?;
    let mut refs = BTreeMap::new();
    for (name, (expected, native)) in updates {
        let native = native
            .as_ref()
            .map(|v| m::CommitId::from_slice(v.as_bytes()))
            .transpose()
            .map_err(error)?;
        let oid = match native {
            Some(id) => Some(converter.export_commit(id).await.map_err(error)?),
            None => None,
        };
        refs.insert(
            name,
            GitPushRefV1 {
                expected,
                desired: GitRefV1 {
                    oid: oid.clone(),
                    commit_oid: oid,
                    native,
                },
            },
        );
    }
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
    let mut records: Vec<_> = converter
        .records
        .into_iter()
        .filter(|(key, _)| !accepted.values.contains_key(key))
        .map(|(_, value)| value)
        .collect();
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
async fn apply(
    ui: &Ui,
    workspace: &mut WorkspaceCommandHelper,
    context: &BridgeContext,
    intent: &GitPushIntentV1,
    remote_name: &str,
    predecessors: BTreeSet<m::RequestId>,
) -> std::result::Result<(), CommandError> {
    let observation = stage_id(intent.id, "observation");
    let id = stage_id(intent.id, "applied");
    let mut tx = workspace.start_transaction();
    for (name, value) in &intent.refs {
        let name = RefName::new(name.strip_prefix("refs/heads/").unwrap());
        let target = value
            .desired
            .native
            .map(|v| RefTarget::normal(jj_lib::backend::CommitId::from_bytes(v.as_bytes())))
            .unwrap_or_else(RefTarget::absent);
        tx.repo_mut().set_remote_bookmark(
            name.to_remote_symbol(RemoteName::new(remote_name)),
            RemoteRef {
                target,
                state: RemoteRefState::Tracked,
            },
        );
    }
    accept(
        tx,
        ui,
        context,
        vec![
            GitRecordV1::Observation(GitObservationV1 {
                id: observation,
                remote: intent.remote,
                binding: intent.binding,
                operation: intent.operation,
                predecessors: intent.observations.clone(),
                complete_heads: false,
                refs: intent
                    .refs
                    .iter()
                    .map(|(name, value)| (name.clone(), value.desired.clone()))
                    .collect(),
            }),
            GitRecordV1::PushEvent(GitPushEventV1 {
                id,
                attempt: intent.id,
                predecessors,
                kind: GitPushEventKindV1::Applied { observation },
            }),
        ],
        id,
        format!("apply native Git push {} to {remote_name}", intent.id),
    )
    .await
}

#[derive(Clone, Debug, clap::Args)]
pub struct GitArgs {
    #[command(subcommand)]
    pub command: GitCommand,
}
#[derive(Clone, Debug, clap::Subcommand)]
pub enum GitCommand {
    /// Read retained attempts without reconciling code or contacting Git
    Status { attempt: Option<m::RequestId> },
    /// Inspect the approved destination and repair bookkeeping without pushing
    Recover { attempt: m::RequestId },
}
pub async fn run_store(
    ui: &mut Ui,
    helper: &CommandHelper,
    registry: &crate::binding::FactoryRegistry,
    args: GitArgs,
) -> std::result::Result<(), CommandError> {
    let format = helper
        .matches()
        .subcommand_matches("store")
        .and_then(|v| v.get_one::<crate::commands::Format>("format"))
        .copied()
        .unwrap_or_default();
    let loaded = helper.load_workspace()?;
    if loaded.repo_loader().store().backend().name() != "jj-store" {
        return Err(user_error(
            "Git attempt commands require a native repository",
        ));
    }
    let context = registry.context(loaded.repo_path()).map_err(error)?;
    let state = context.client.git_bridge_state().await.map_err(error)?;
    let git = state
        .git
        .as_ref()
        .ok_or_else(|| user_error("No Git attempts"))?;
    match args.command {
        GitCommand::Status { attempt } => {
            if attempt.is_some_and(|v| git.push_intent(v).is_none()) {
                return Err(user_error("Git attempt not found"));
            }
            let intents: Vec<_> = git
                .values
                .values()
                .filter_map(|v| match v {
                    GitRecordV1::PushIntent(v) if attempt.is_none_or(|id| id == v.id) => Some(v),
                    _ => None,
                })
                .collect();
            if intents.len() > 1024 {
                return Err(user_error(
                    "Git status list exceeds limit; select an attempt ID",
                ));
            }
            let text = intents
                .iter()
                .map(|intent| {
                    format!(
                        "{} {} {}\n",
                        intent.id,
                        git.push_status(intent.id),
                        intent.refs.keys().cloned().collect::<Vec<_>>().join(", ")
                    )
                })
                .collect::<String>();
            let attempts: Vec<_> = intents.iter().map(|intent| serde_json::json!({"intent":intent,"status":git.push_status(intent.id),"events":git.push_events(intent.id)})).collect();
            crate::presentation::result(
                ui,
                helper,
                format,
                &serde_json::json!({"version":1,"repository":context.client.repository(),"attempts":attempts}),
                &text,
            )
        }
        GitCommand::Recover { attempt } => {
            mutating(helper)?;
            context.client.authorize(true).await.map_err(error)?;
            let intent = git
                .push_intent(attempt)
                .ok_or_else(|| user_error("Git attempt not found"))?;
            if intent.executor != state.principal {
                return Err(user_error("Git attempt belongs to another executor"));
            }
            if git.push_status(attempt) == "applied" {
                return crate::presentation::result(
                    ui,
                    helper,
                    format,
                    &serde_json::json!({"version":1,"attempt":attempt,"native_apply":"applied","status":"already_applied"}),
                    &format!("Attempt {attempt} already applied.\n"),
                );
            }
            let remotes = git.remotes().map_err(error)?;
            let remote_name = remotes
                .iter()
                .find(|(_, id)| **id == intent.remote)
                .map(|(name, _)| name)
                .ok_or_else(|| user_error("Attempt remote removed or conflicted"))?;
            let device = device::Device::open(context.client.repository(), loaded.workspace_root())
                .map_err(error)?;
            let endpoint = device.endpoint(intent.remote).map_err(error)?;
            if endpoint.fingerprint != intent.binding {
                return Err(user_error(
                    "Attempt destination approval changed; recovery refused",
                ));
            }
            let cache = device.cache().map_err(error)?;
            let actual = transport::inspect(&cache.bare, &endpoint.url, intent.refs.keys())
                .await
                .map_err(error)?;
            if intent
                .refs
                .iter()
                .any(|(name, value)| actual.get(name) != value.desired.oid.as_ref())
            {
                return Err(user_error(format!(
                    "Attempt {attempt} remains unknown; current refs differ from desired. Fetch and review before replanning"
                )));
            }
            let current = git.push_events(attempt);
            let mut predecessor: BTreeSet<_> = current.iter().map(|v| v.id).collect();
            if git.push_status(attempt) != "apply_pending" {
                let result = device::id().map_err(error)?;
                record(
                    &context,
                    result,
                    vec![GitRecordV1::PushEvent(GitPushEventV1 {
                        id: result,
                        attempt,
                        predecessors: predecessor,
                        kind: GitPushEventKindV1::ResultRecorded {
                            refs: intent
                                .refs
                                .keys()
                                .map(|name| (name.clone(), GitPushRefOutcomeV1::ObservedSatisfied))
                                .collect(),
                        },
                    })],
                )
                .await?;
                predecessor = BTreeSet::from([result]);
            }
            let mut workspace = helper.workspace_helper(ui).await?;
            apply(
                ui,
                &mut workspace,
                &context,
                intent,
                remote_name,
                predecessor,
            )
            .await?;
            crate::presentation::result(
                ui,
                helper,
                format,
                &serde_json::json!({"version":1,"attempt":attempt,"native_apply":"applied","status":"observed_satisfied","authorship":"unproven_by_observation"}),
                &format!(
                    "Attempt {attempt}: observed_satisfied; native bookkeeping applied. This observation does not prove attempt authorship.\n"
                ),
            )
        }
    }
}
