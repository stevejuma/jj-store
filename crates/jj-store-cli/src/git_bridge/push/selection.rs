//! Resolve all native bookmark selectors once against the frozen view.
use super::*;
pub(super) type Updates = BTreeMap<String, (Option<String>, Option<jj_lib::backend::CommitId>)>;
pub(super) struct Selection {
    pub generated: BTreeMap<String, jj_lib::backend::CommitId>,
    pub updates: Updates,
}
pub(super) fn explicit(args: &clap::ArgMatches, name: &str) -> bool {
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
pub(super) async fn select(
    ui: &Ui,
    workspace: &WorkspaceCommandHelper,
    args: &clap::ArgMatches,
    accepted: &GitStoreSnapshot,
    remote_id: m::RequestId,
    remote_name: &str,
    endpoint: &device::Endpoint,
) -> std::result::Result<Selection, CommandError> {
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
            add_generated(workspace, &mut generated, name, id, true)?;
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
            workspace,
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
                remote: StringExpression::exact(RemoteName::new(remote_name)),
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
        .local_remote_bookmarks(RemoteName::new(remote_name))
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
                || refs
                    .local_target
                    .present_adds()
                    .any(|id| targets.contains(id))
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
        let symbol = RefName::new(&name).to_remote_symbol(RemoteName::new(remote_name));
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
    Ok(Selection { generated, updates })
}
