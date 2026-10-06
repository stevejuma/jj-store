use super::*;
const PATH: &str = "/local/repos/{repo}/commits/{commit}/blame";
pub(super) fn router() -> Router<Arc<State>> {
    Router::new().route(PATH, get(read))
}
async fn read(
    ExtractState(state): ExtractState<Arc<State>>,
    Path((repo, commit)): Path<(m::RepositoryId, m::GitSha1Id)>,
    args: routes::Input<routes::Blame>,
) -> Result<Response> {
    let args = routes::input(args)?;
    let access = binding(&state, repo).await?;
    let snapshot = local::Snapshot {
        version: 1,
        profile: local::PROFILE,
        repository: repo,
        generation: access.generation(),
        operation: args.at_op,
    };
    match state
        .code_indexes
        .local_blame(access, args.at_op, commit, args.path, args.continuation)
        .await
    {
        Ok(page) => state.json(&page),
        Err(q::Error::IndexRebuildRequired) => Err(Error::local_rebuild(snapshot)),
        Err(error) => Err(error.into()),
    }
}
pub(super) fn document(document: &mut crate::contract::Document) {
    document.local_endpoint::<routes::Blame, q::blame::Page>("get", PATH, "local_file_blame");
}
