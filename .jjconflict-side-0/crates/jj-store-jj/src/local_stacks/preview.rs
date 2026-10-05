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
    if request.version != 1 || request.binding_generation != access.generation() {
        return Err(Error::Stale);
    }
    access.authorize().await.map_err(|_| Error::Denied)?;
    let settings = settings::load(&root)?;
    let workspace = Workspace::load(
        &settings,
        &root,
        &default_backend_factories(),
        &default_working_copy_factories(),
    )
    .map_err(jj)?;
    if workspace.workspace_root() != root
        || workspace.repo_path().canonicalize().map_err(jj)?
            != access.metadata_path().canonicalize().map_err(jj)?
    {
        return Err(Error::Denied);
    }
    let repo = workspace.repo_loader().load_at_head().await.map_err(jj)?;
    let config = config::Storage::read_only(&root)?;
    if request.operation.as_ref().map(|v| v.to_string()) != Some(repo.op_id().hex())
        || request.revision != Some(config.revision)
    {
        return Err(Error::Stale);
    }
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
        _ => return Err(Error::Invalid),
    };
    access.authorize().await.map_err(|_| Error::Denied)?;
    Ok(result)
}
