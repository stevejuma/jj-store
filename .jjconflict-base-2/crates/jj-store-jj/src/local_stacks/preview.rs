use super::*;
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "LocalStacksPreview")]
pub struct Preview {
    pub title: String,
    pub summary: String,
    pub paths: Vec<String>,
}
/// Advisory read. Does not acquire/create settings storage, snapshot or publish.
pub async fn preview(access: Arc<dyn Access>, root: PathBuf, request: Request) -> Result<Preview> {
    let (workspace, repo, config, settings) = load(
        &access,
        &root,
        request.version,
        request.binding_generation,
        request.operation,
        request.revision,
    )
    .await?;
    let wc = repo
        .store()
        .get_commit(
            repo.view()
                .get_wc_commit_id(workspace.workspace_name())
                .ok_or(Error::Invalid)?,
        )
        .map_err(jj)?;
    let result = match request.command {
        Command::Commit {
            branch,
            message,
            selections,
        } => {
            mutations::validate(
                &repo,
                workspace.workspace_name(),
                &settings,
                &config,
                &wc,
                &branch,
                &message,
                &selections,
            )
            .await?;
            Preview {
                title: "Create commit".into(),
                summary: format!(
                    "Create “{}” on {branch} from {} selected files. Other unfinished changes remain in the working copy.",
                    message.lines().next().unwrap_or_default(),
                    selections.len()
                ),
                paths: selections.into_iter().map(|s| s.path).collect(),
            }
        }
        ref command @ (Command::CreateStack { .. }
        | Command::CreateBranch { .. }
        | Command::RenameBranch { .. }
        | Command::Reword { .. }
        | Command::Empty { .. }) => {
            basic::validate(
                &repo,
                workspace.workspace_name(),
                &settings,
                &config,
                command,
            )
            .await?
        }
        ref command @ (Command::Transfer { .. } | Command::Split { .. } | Command::Edit { .. }) => {
            content::validate(
                &repo,
                workspace.workspace_name(),
                &settings,
                &config,
                command,
            )
            .await?
        }
        ref command @ (Command::Move { .. }
        | Command::Squash { .. }
        | Command::Uncommit { .. }
        | Command::Rebase { .. }
        | Command::SquashBranch { .. }) => {
            graph::validate(
                &repo,
                workspace.workspace_name(),
                &settings,
                &config,
                command,
            )
            .await?
        }
        ref command @ (Command::DeleteBranch { .. }
        | Command::Abandon { .. }
        | Command::Discard { .. }) => {
            destructive::validate(
                &repo,
                workspace.workspace_name(),
                &settings,
                &config,
                command,
            )
            .await?
        }
        Command::Resolve {
            ref path,
            ref content,
        } => {
            conflicts::validate(
                &repo,
                workspace.workspace_name(),
                &settings,
                &config,
                path,
                content,
            )
            .await?
        }
        Command::Reconcile => {
            parents(repo.as_ref(), config.active().ok_or(Error::Invalid)?)?;
            Preview{title:"Reapply saved workspace branches".into(),summary:"Explicitly rebuild membership from the saved applied bookmarks and preserve WIP. This changes native graph state; TOML is not an Undo journal.".into(),paths:vec![]}
        }
        Command::Undo | Command::Redo => {
            history::target(&repo, matches!(request.command, Command::Redo)).await?;
            Preview{title:"Restore native JJ operation".into(),summary:"Repository-wide JJ Undo/Redo includes CLI operations. Code and bookmarks follow the native operation log; local TOML remains separate and is not reapplied.".into(),paths:vec![]}
        }
        _ => return Err(Error::Invalid),
    };
    access.authorize().await.map_err(|_| Error::Denied)?;
    Ok(result)
}

pub(super) async fn load(
    access: &Arc<dyn Access>,
    root: &PathBuf,
    version: u32,
    generation: u64,
    operation: Option<jj_store_model::OperationId>,
    revision: Option<u64>,
) -> Result<(
    Workspace,
    Arc<ReadonlyRepo>,
    Config,
    jj_lib::settings::UserSettings,
)> {
    if version != 1 || generation != access.generation() {
        return Err(Error::Stale);
    }
    access.authorize().await.map_err(|_| Error::Denied)?;
    let settings = settings::load(root)?;
    let workspace = Workspace::load(
        &settings,
        root,
        &default_backend_factories(),
        &default_working_copy_factories(),
    )
    .map_err(jj)?;
    if workspace.workspace_root() != root.as_path()
        || workspace.repo_path().canonicalize().map_err(jj)?
            != access.metadata_path().canonicalize().map_err(jj)?
    {
        return Err(Error::Denied);
    }
    let repo = workspace.repo_loader().load_at_head().await.map_err(jj)?;
    let config = config::Storage::read_only(root)?;
    if operation.as_ref().map(|v| v.to_string()) != Some(repo.op_id().hex())
        || revision != Some(config.revision)
    {
        return Err(Error::Stale);
    }
    Ok((workspace, repo, config, settings))
}
