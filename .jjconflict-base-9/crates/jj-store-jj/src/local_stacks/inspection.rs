//! Bounded point inspection; no enumeration, snapshots or configuration creation.
use super::*;
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "LocalStacksInspectionRequest")]
pub struct InspectionRequest {
    pub version: u32,
    pub binding_generation: u64,
    pub operation: jj_store_model::OperationId,
    pub revision: u64,
    pub commits: Vec<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "LocalStacksCommitEligibility")]
pub struct CommitEligibility {
    pub commit: String,
    pub mutable: bool,
    pub reason: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "LocalStacksInspection")]
pub struct Inspection {
    pub commits: Vec<CommitEligibility>,
}
pub async fn inspect(
    access: Arc<dyn Access>,
    root: PathBuf,
    request: InspectionRequest,
) -> Result<Inspection> {
    if request.commits.len() > 500 {
        return Err(Error::Invalid);
    }
    let (workspace, repo, config, settings) = preview::load(
        &access,
        &root,
        request.version,
        request.binding_generation,
        Some(request.operation),
        Some(request.revision),
    )
    .await?;
    let commits = request.commits.into_iter().map(|id| {
        let selected = policy::commit(&repo, &id)?;
        let mutable = config.active().is_some() && policy::rewritable(&repo, workspace.workspace_name(), &settings, &config, vec![selected.id().clone()]).is_ok();
        Ok(CommitEligibility { commit: id, mutable, reason: (!mutable).then(|| "This commit or its descendants are protected by JJ immutable policy or workspace target ancestry.".into()) })
    }).collect::<Result<Vec<_>>>()?;
    access.authorize().await.map_err(|_| Error::Denied)?;
    Ok(Inspection { commits })
}
