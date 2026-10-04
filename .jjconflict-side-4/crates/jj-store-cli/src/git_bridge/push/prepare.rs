//! Validate selected history and convert only the required Git closure.
use super::*;
pub(super) struct Prepared {
    pub refs: BTreeMap<String, GitPushRefV1>,
    pub cache: device::Cache,
    pub records: Vec<GitRecordV1>,
}
pub(super) struct Input<'a> {
    pub args: &'a clap::ArgMatches,
    pub accepted: &'a GitStoreSnapshot,
    pub device: &'a device::Device,
    pub remote_name: &'a str,
}
pub(super) async fn prepare(
    ui: &Ui,
    workspace: &WorkspaceCommandHelper,
    context: &BridgeContext,
    input: Input<'_>,
    updates: selection::Updates,
) -> std::result::Result<Prepared, CommandError> {
    let Input {
        args,
        accepted,
        device,
        remote_name,
    } = input;
    let heads: Vec<_> = updates.values().filter_map(|(_, id)| id.clone()).collect();
    let known: Vec<_> = workspace
        .repo()
        .view()
        .remote_bookmarks(RemoteName::new(remote_name))
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
    let records = converter
        .records
        .into_iter()
        .filter(|(key, _)| !accepted.values.contains_key(key))
        .map(|(_, value)| value)
        .collect();
    Ok(Prepared {
        refs,
        cache,
        records,
    })
}
