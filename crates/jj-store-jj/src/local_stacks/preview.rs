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
        Command::UnapplyBranches { ref branches } => {
            topology::validate_unapply(&config, branches)?;
            Preview {
                title: "Unapply stack".into(),
                summary: format!(
                    "Remove {} from the workspace together. Branches and unfinished work are retained.",
                    branches.join(", ")
                ),
                paths: vec![],
            }
        }
        Command::Reconcile => {
            let saved = config.active().ok_or(Error::Invalid)?;
            parents(repo.as_ref(), saved)?;
            let obsolete = cleanup::candidates(&repo, saved).await?;
            let cleanup = if obsolete.is_empty() {
                String::new()
            } else {
                format!(
                    " Retire {} obsolete internal merge heads: {}. They remain recoverable through JJ operation history.",
                    obsolete.len(),
                    obsolete
                        .iter()
                        .map(|c| c.id().hex())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            };
            Preview {
                title: "Reapply saved workspace branches".into(),
                summary: format!(
                    "Compose the target and saved applied bookmarks, preserving the merge and WIP changes.{cleanup} This changes native graph state; TOML is not an Undo journal."
                ),
                paths: vec![],
            }
        }
        Command::Undo | Command::Redo => {
            history::target(&repo, matches!(request.command, Command::Redo)).await?;
            Preview{title:"Restore native JJ operation".into(),summary:"Repository-wide JJ Undo/Redo includes CLI operations. Code and bookmarks follow the native operation log; local TOML remains separate and is not reapplied.".into(),paths:vec![]}
        }
        Command::Setup {
            ref name,
            ref target,
        } => {
            if !config.workspaces.is_empty() {
                return Err(Error::Invalid);
            }
            config.validate_name(name, None)?;
            resolve(repo.as_ref(), target)?;
            Preview {
                title: "Set up workspace".into(),
                summary: format!(
                    "Create {name} targeting {target}, retaining current unfinished work."
                ),
                paths: vec![],
            }
        }
        Command::Create { ref name } => {
            config.validate_name(name, None)?;
            if config.workspaces.is_empty() || config.workspaces.len() >= 64 {
                return Err(Error::Invalid);
            }
            Preview {
                title: "Create workspace".into(),
                summary: format!(
                    "Create and switch to {name}, inheriting the target and starting empty."
                ),
                paths: vec![],
            }
        }
        Command::Switch { ref workspace } | Command::Delete { ref workspace } => {
            let saved = config
                .workspaces
                .iter()
                .find(|w| &w.id == workspace)
                .ok_or(Error::Invalid)?;
            if matches!(request.command, Command::Switch { .. }) {
                resolve_any(repo.as_ref(), &saved.wip_ref)?;
            }
            Preview {
                title: "Workspace action".into(),
                summary: operation_description::describe(&request.command, &config),
                paths: vec![],
            }
        }
        Command::Apply { ref branch }
        | Command::Unapply { ref branch }
        | Command::SetTarget { ref branch } => {
            let saved = config.active().ok_or(Error::Invalid)?;
            if matches!(request.command, Command::Unapply { .. }) {
                if !saved.applied.contains(branch) {
                    return Err(Error::Invalid);
                }
            } else {
                resolve(repo.as_ref(), branch)?;
            }
            Preview {
                title: "Workspace membership".into(),
                summary: operation_description::describe(&request.command, &config),
                paths: vec![],
            }
        }
        Command::Update {
            ref workspace,
            ref name,
            ref target,
        } => {
            if config.active.as_ref() != Some(workspace) {
                return Err(Error::Invalid);
            }
            config.validate_name(name, Some(workspace))?;
            resolve(repo.as_ref(), target)?;
            Preview {
                title: "Update workspace".into(),
                summary: format!("Save {name} targeting {target}; preserve unfinished work."),
                paths: vec![],
            }
        }
        Command::Refresh => return Err(Error::Invalid),
    };
    access.authorize().await.map_err(|_| Error::Denied)?;
    Ok(result)
}

pub(super) async fn load(
    access: &Arc<dyn Access>,
    root: &std::path::Path,
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
    let (workspace, repo, config, settings) = read::load(access, root).await?;
    if operation.as_ref().map(|v| v.to_string()) != Some(repo.op_id().hex())
        || revision != Some(config.revision)
    {
        return Err(Error::Stale);
    }
    Ok((workspace, repo, config, settings))
}
