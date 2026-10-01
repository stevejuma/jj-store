//! Bounded import and atomic native ref/observation acceptance.
use super::*;
pub(super) async fn run(
    ui: &mut Ui,
    helper: &CommandHelper,
    registry: &crate::binding::FactoryRegistry,
    args: &clap::ArgMatches,
) -> std::result::Result<(), CommandError> {
    mutating(helper)?;
    if flag(args, "tracked") || flag(args, "all_remotes") || !strings(args, "tags").is_empty() {
        return Err(unavailable("fetch selection"));
    }
    let loaded = helper.load_workspace()?;
    let context = registry.context(loaded.repo_path()).map_err(error)?;
    context.client.authorize(true).await.map_err(error)?;
    let state = context.client.git_bridge_state().await.map_err(error)?;
    if state.git.is_none() {
        return Err(user_error(
            "NATIVE_GIT_CAPABILITY: add a native Git remote before fetch",
        ));
    }
    let mut workspace = helper.workspace_helper(ui).await?;
    let accepted = state
        .git
        .as_ref()
        .ok_or_else(|| user_error("Add a native Git remote before fetch"))?;
    let (remote_name, id) = remote(
        args,
        &accepted.remotes().map_err(error)?,
        workspace.settings(),
        true,
    )?;
    let device = device::Device::open(context.client.repository(), workspace.workspace_root())
        .map_err(error)?;
    let endpoint = device.endpoint(id).map_err(error)?.clone();
    let remote_settings = workspace.settings().remote_settings()?;
    let configured = remote_settings.get(RemoteName::new(&remote_name));
    if configured.is_some_and(|v| v.fetch_tags.as_ref().is_some_and(|v| v != "*")) {
        return Err(unavailable("tag fetch configuration"));
    }
    let mut patterns = strings(args, "branches");
    if patterns.is_empty() {
        patterns.push(
            configured
                .and_then(|v| v.fetch_bookmarks.clone())
                .unwrap_or_else(|| "*".into()),
        );
    }
    if patterns.len() > 64
        || patterns.iter().any(|v| {
            v.matches('*').count() > 1
                || !valid_git_ref(&format!("refs/heads/{}", v.replace('*', "x")))
        })
    {
        return Err(unavailable("fetch bookmark pattern"));
    }
    let cache = device.cache().map_err(error)?;
    let refs = transport::fetch(&cache.bare, &endpoint.url, &patterns)
        .await
        .map_err(error)?;
    let objects = Arc::new(
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
        objects,
        jj_store_jj::git_bridge::Limits::default(),
        Some(accepted),
    )
    .map_err(error)?;
    let mut observed = BTreeMap::new();
    for (name, oid) in refs {
        converter.retain_object(&oid).await.map_err(error)?;
        let repo = gix::open(&cache.bare).map_err(error)?;
        let commit_oid = repo
            .find_object(gix::ObjectId::from_hex(oid.as_bytes()).map_err(error)?)
            .map_err(error)?
            .peel_to_kind(gix::objs::Kind::Commit)
            .map_err(error)?
            .id
            .to_string();
        let native = converter
            .import_commit(commit_oid.clone())
            .await
            .map_err(error)?;
        observed.insert(
            name,
            GitRefV1 {
                oid: Some(oid),
                commit_oid: Some(commit_oid),
                native: Some(native),
            },
        );
    }
    let complete = patterns == ["*"];
    let source = m::OperationId::from_slice(workspace.repo().op_id().as_bytes()).map_err(error)?;
    let git_settings = jj_lib::git::GitSettings::from_settings(workspace.settings())?;
    let options = jj_cli::git_util::load_git_import_options(ui, &git_settings, &remote_settings)?;
    let matcher = jj_cli::revset_util::parse_union_name_patterns(ui, &patterns)?.to_matcher();
    let old_names: Vec<_> = workspace
        .repo()
        .view()
        .remote_bookmarks(RemoteName::new(&remote_name))
        .map(|(name, _)| name.as_str().to_owned())
        .collect();
    for name in old_names {
        if matcher.is_match(&name) {
            observed
                .entry(format!("refs/heads/{name}"))
                .or_insert(GitRefV1 {
                    oid: None,
                    commit_oid: None,
                    native: None,
                });
        }
    }
    // All supported commit tags are fetched, so their missing names are proven
    // absent independently of the requested bookmark scope.
    for (name, _) in workspace
        .repo()
        .view()
        .remote_tags(RemoteName::new(&remote_name))
    {
        observed
            .entry(format!("refs/tags/{}", name.as_str()))
            .or_insert(GitRefV1 {
                oid: None,
                commit_oid: None,
                native: None,
            });
    }
    let mut tx = workspace.start_transaction();
    for (reference, value) in &observed {
        let tag = reference.starts_with("refs/tags/");
        let name = RefName::new(
            reference
                .strip_prefix(if tag { "refs/tags/" } else { "refs/heads/" })
                .unwrap(),
        );
        let symbol = name.to_remote_symbol(RemoteName::new(&remote_name));
        let old = if tag {
            tx.repo().view().get_remote_tag(symbol)
        } else {
            tx.repo().view().get_remote_bookmark(symbol)
        }
        .clone();
        let target = value
            .native
            .map(|id| RefTarget::normal(jj_lib::backend::CommitId::from_bytes(id.as_bytes())))
            .unwrap_or_else(RefTarget::absent);
        if old.target == target {
            continue;
        }
        // Match upstream import: only changed refs introduce visible heads.
        // Re-adding an unchanged ancestor makes an otherwise empty transaction
        // appear changed before JJ normalizes its redundant heads.
        if old.target != target
            && let Some(id) = value.native
        {
            let commit = tx
                .repo()
                .store()
                .get_commit_async(&jj_lib::backend::CommitId::from_bytes(id.as_bytes()))
                .await?;
            tx.repo_mut().add_head(&commit).await?;
        }
        let tracked = old.is_tracked()
            || tag
            || (old.is_absent()
                && options
                    .remote_auto_track_bookmarks
                    .get(RemoteName::new(&remote_name))
                    .is_some_and(|matcher| matcher.is_match(name.as_str())));
        if tracked {
            if tag {
                tx.repo_mut()
                    .merge_local_tag(name, old.tracked_target(), &target)
                    .await?;
            } else {
                tx.repo_mut()
                    .merge_local_bookmark(name, old.tracked_target(), &target)
                    .await?;
            }
        }
        let new = RemoteRef {
            target,
            state: if tracked {
                RemoteRefState::Tracked
            } else {
                RemoteRefState::New
            },
        };
        if tag {
            tx.repo_mut().set_remote_tag(symbol, new);
        } else {
            tx.repo_mut().set_remote_bookmark(symbol, new);
        }
    }
    let mut records: Vec<_> = converter
        .records
        .into_iter()
        .filter(|(key, _)| !accepted.values.contains_key(key))
        .map(|(_, v)| v)
        .collect();
    let predecessors: BTreeSet<_> = accepted
        .observations(id, endpoint.fingerprint)
        .iter()
        .map(|v| v.id)
        .collect();
    let unchanged = records.is_empty()
        && !tx.repo().has_changes()
        && accepted.observations(id, endpoint.fingerprint).len() == 1
        && accepted
            .observations(id, endpoint.fingerprint)
            .as_slice()
            .iter()
            .any(|v| v.refs == observed && v.complete_heads == complete);
    if unchanged {
        return crate::presentation::text(ui, helper, "Nothing changed.\n");
    }
    let request = device::id().map_err(error)?;
    records.push(GitRecordV1::Observation(GitObservationV1 {
        id: request,
        remote: id,
        binding: endpoint.fingerprint,
        predecessors,
        operation: source,
        complete_heads: complete,
        refs: observed,
    }));
    accept(
        tx,
        ui,
        &context,
        records,
        request,
        format!("fetch native Git remote {remote_name}"),
    )
    .await?;
    crate::presentation::text(ui, helper, &format!("Fetched {remote_name}.\n"))
}
