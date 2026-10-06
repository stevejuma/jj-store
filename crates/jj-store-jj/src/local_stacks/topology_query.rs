//! Read-only discovery at an exact native operation, including before setup.
use super::*;
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "LocalStacksTopologyRequest")]
pub struct TopologyRequest {
    pub version: u32,
    pub binding_generation: u64,
    pub operation: jj_store_model::OperationId,
    pub target: String,
    pub target_commit: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "LocalStacksTopologyResponse")]
pub struct TopologyResponse {
    pub version: u32,
    pub operation: jj_store_model::OperationId,
    pub target: String,
    pub target_commit: String,
    pub topology: Topology,
}
/// Never snapshots, reconciles operation heads, or creates local configuration.
pub async fn read_topology(
    access: Arc<dyn Access>,
    root: PathBuf,
    request: TopologyRequest,
) -> Result<TopologyResponse> {
    if request.version != 1 || access.generation() != request.binding_generation {
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
    let loader = workspace.repo_loader();
    let op = loader
        .load_operation(&jj_lib::op_store::OperationId::from_bytes(
            request.operation.as_bytes(),
        ))
        .await
        .map_err(jj)?;
    let repo = loader.load_at(&op).await.map_err(jj)?;
    if resolve(repo.as_ref(), &request.target)?.id().hex() != request.target_commit {
        return Err(Error::Stale);
    }
    let topology = topology::discover(&repo, &request.target, &[]).await?;
    access.authorize().await.map_err(|_| Error::Denied)?;
    Ok(TopologyResponse {
        version: 1,
        operation: request.operation,
        target: request.target,
        target_commit: request.target_commit,
        topology,
    })
}
