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

mod device;
mod transport;
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
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};
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
    if !matches!(name, "remote" | "fetch") {
        return Err(unavailable(operation));
    }
    if name == "remote" {
        let (name, args) = args.subcommand().unwrap();
        if !matches!(name, "add" | "list" | "set-url") {
            return Err(unavailable(operation));
        }
        if string(args, "push_url").is_some() || string(args, "push").is_some() {
            return Err(unavailable("distinct push URL"));
        }
        if name != "list" {
            mutating(helper)?;
        }
        let workspace = helper.load_workspace()?;
        let context = registry.context(workspace.repo_path()).map_err(error)?;
        let state = context.client.git_bridge_state().await.map_err(error)?;
        let remotes = state
            .git
            .as_ref()
            .map(GitStoreSnapshot::remotes)
            .transpose()
            .map_err(error)?
            .unwrap_or_default();
        let mut device =
            device::Device::open(context.client.repository(), workspace.workspace_root())
                .map_err(error)?;
        if name == "list" {
            let mut output = String::new();
            for (name, id) in remotes {
                let binding = device.endpoint(id).ok();
                output.push_str(&format!(
                    "{name} {}\n",
                    binding.map_or("(unbound on this device)", |v| v.url.as_str())
                ));
            }
            return crate::presentation::text(ui, helper, &output);
        }
        context.client.authorize(true).await.map_err(error)?;
        if name == "add" && !state.activation_available {
            return Err(unavailable("code repository origin activation"));
        }
        let remote_name =
            string(args, "remote").ok_or_else(|| user_error("Git remote name required"))?;
        if !valid_remote_name(&remote_name) {
            return Err(user_error("unsupported Git remote name"));
        }
        let url = string(args, "url")
            .or_else(|| string(args, "fetch"))
            .ok_or_else(|| user_error("Git fetch endpoint required"))?;
        let url = device::approved_url(helper.cwd(), &url).map_err(error)?;
        if name == "set-url" {
            let id = remotes
                .get(&remote_name)
                .ok_or_else(|| user_error("Git logical remote not found"))?;
            device.bind(*id, &remote_name, url).map_err(error)?;
            return Ok(());
        }
        if remotes.contains_key(&remote_name) {
            return Err(user_error("Git remote already exists"));
        }
        let id = device::id().map_err(error)?;
        device.bind(id, &remote_name, url).map_err(error)?;
        let mut workspace = helper.workspace_helper(ui).await?;
        let mut tx = workspace.start_transaction();
        tx.repo_mut().ensure_remote(RemoteName::new(&remote_name));
        return accept(
            tx,
            ui,
            &context,
            vec![GitRecordV1::Remote(GitRemoteV1 {
                remote: id,
                event: id,
                predecessors: BTreeSet::new(),
                name: remote_name.clone(),
                active: true,
            })],
            id,
            format!("add native Git remote {remote_name}"),
        )
        .await;
    }
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
