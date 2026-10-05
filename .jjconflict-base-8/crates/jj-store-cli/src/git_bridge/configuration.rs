//! Native remote lifecycle, separate from device endpoint approvals.
use super::*;
pub(super) async fn run(
    ui: &mut Ui,
    helper: &CommandHelper,
    registry: &crate::binding::FactoryRegistry,
    args: &clap::ArgMatches,
    operation: &str,
) -> std::result::Result<(), CommandError> {
    let (name, args) = args.subcommand().unwrap();
    if !matches!(name, "add" | "list" | "set-url" | "rename" | "remove") {
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
    let mut device = device::Device::open(context.client.repository(), workspace.workspace_root())
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
    if matches!(name, "rename" | "remove") {
        let old = string(args, if name == "rename" { "old" } else { "remote" })
            .ok_or_else(|| user_error("Git remote name required"))?;
        let id = *remotes
            .get(&old)
            .ok_or_else(|| user_error("Git logical remote not found"))?;
        let new = if name == "rename" {
            let new =
                string(args, "new").ok_or_else(|| user_error("New Git remote name required"))?;
            if !valid_remote_name(&new) || remotes.contains_key(&new) {
                return Err(user_error("New Git remote name invalid or already exists"));
            }
            new
        } else {
            old.clone()
        };
        let event = device::id().map_err(error)?;
        let predecessors = state
            .git
            .as_ref()
            .unwrap()
            .remote_versions(id)
            .iter()
            .map(|v| v.event)
            .collect();
        let mut workspace = helper.workspace_helper(ui).await?;
        let mut tx = workspace.start_transaction();
        if name == "rename" {
            tx.repo_mut()
                .rename_remote(RemoteName::new(&old), RemoteName::new(&new));
        } else {
            tx.repo_mut().remove_remote(RemoteName::new(&old));
        }
        accept(
            tx,
            ui,
            &context,
            vec![GitRecordV1::Remote(GitRemoteV1 {
                remote: id,
                event,
                predecessors,
                name: new.clone(),
                active: name == "rename",
            })],
            event,
            format!("{name} native Git remote {old}"),
        )
        .await?;
        // Device approval is keyed by stable logical ID. Retain removed
        // bindings for historical diagnostics; the selected tombstone gates
        // every future contact. Rename must not rotate destination leases.
        if name == "rename" {
            device.rename(id, &new).map_err(error)?;
        }
        return Ok(());
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
