//! Local stack commands use ordinary JJ. No client paths or native manifest publication.
use super::*;
use local::Access as _;
use management::stacks;
pub(super) fn router() -> Router<Arc<State>> {
    Router::new()
        .route("/local/repos/{repo}/stacks", post(execute))
        .route("/local/repos/{repo}/stacks/preview", post(preview))
        .route("/local/repos/{repo}/stacks/inspect", post(inspect))
        .route("/local/repos/{repo}/stacks/hunks", post(hunks))
        .route("/local/repos/{repo}/stacks/conflict", post(conflict))
}
async fn conflict(
    ExtractState(state): ExtractState<Arc<State>>,
    Path(repo): Path<m::RepositoryId>,
    args: routes::Input<routes::Empty>,
    body: Body<stacks::HunkRequest>,
) -> Result<Response> {
    routes::input(args)?;
    let request = decode_body(body)?;
    let access = bound_folder(&state, repo).await?;
    access.authorize().await?;
    let root = std::path::PathBuf::from(&access.folder.path);
    let runtime = tokio::runtime::Handle::current();
    let value = blocking(&state, move || {
        runtime
            .block_on(stacks::conflict(access, root, request))
            .map_err(map_error)
    })
    .await?;
    state.json(&value)
}
async fn hunks(
    ExtractState(state): ExtractState<Arc<State>>,
    Path(repo): Path<m::RepositoryId>,
    args: routes::Input<routes::Empty>,
    body: Body<stacks::HunkRequest>,
) -> Result<Response> {
    routes::input(args)?;
    let request = decode_body(body)?;
    let access = bound_folder(&state, repo).await?;
    access.authorize().await?;
    let root = std::path::PathBuf::from(&access.folder.path);
    let runtime = tokio::runtime::Handle::current();
    let value = blocking(&state, move || {
        runtime
            .block_on(stacks::hunks(access, root, request))
            .map_err(map_error)
    })
    .await?;
    state.json(&value)
}
async fn inspect(
    ExtractState(state): ExtractState<Arc<State>>,
    Path(repo): Path<m::RepositoryId>,
    args: routes::Input<routes::Empty>,
    body: Body<stacks::InspectionRequest>,
) -> Result<Response> {
    routes::input(args)?;
    let request = decode_body(body)?;
    let access = bound_folder(&state, repo).await?;
    access.authorize().await?;
    let root = std::path::PathBuf::from(&access.folder.path);
    let runtime = tokio::runtime::Handle::current();
    let value = blocking(&state, move || {
        runtime
            .block_on(stacks::inspect(access, root, request))
            .map_err(map_error)
    })
    .await?;
    state.json(&value)
}
async fn preview(
    ExtractState(state): ExtractState<Arc<State>>,
    Path(repo): Path<m::RepositoryId>,
    args: routes::Input<routes::Empty>,
    body: Body<stacks::Request>,
) -> Result<Response> {
    routes::input(args)?;
    let request = decode_body(body)?;
    let access = bound_folder(&state, repo).await?;
    access.authorize().await?;
    let root = std::path::PathBuf::from(&access.folder.path);
    let runtime = tokio::runtime::Handle::current();
    let value = blocking(&state, move || {
        runtime
            .block_on(stacks::preview(access, root, request))
            .map_err(map_error)
    })
    .await?;
    state.json(&value)
}
async fn execute(
    ExtractState(state): ExtractState<Arc<State>>,
    Path(repo): Path<m::RepositoryId>,
    args: routes::Input<routes::Empty>,
    body: Body<stacks::Request>,
) -> Result<Response> {
    routes::input(args)?;
    let request = decode_body(body)?;
    if request.version != 1 {
        return Err(Error::invalid());
    }
    let access = bound_folder(&state, repo).await?;
    if request.binding_generation != access.generation {
        return Err(Error::new(StatusCode::CONFLICT, "stale_local_stacks"));
    }
    access.authorize().await?;
    let root = std::path::PathBuf::from(&access.folder.path);
    let runtime = tokio::runtime::Handle::current();
    let value = blocking(&state, move || {
        runtime
            .block_on(stacks::execute(access, root, request))
            .map_err(map_error)
    })
    .await?;
    state.json(&value)
}
fn map_error(error: stacks::Error) -> Error {
    match error {
        stacks::Error::Stale => Error::new(StatusCode::CONFLICT, "stale_local_stacks"),
        stacks::Error::Invalid => Error::new(StatusCode::BAD_REQUEST, "invalid_local_stacks"),
        stacks::Error::Busy => Error::busy(),
        stacks::Error::Denied => Error::denied(),
        stacks::Error::Jj(_) => Error::new(StatusCode::UNPROCESSABLE_ENTITY, "local_stacks_failed"),
    }
}
pub(super) fn document(document: &mut crate::contract::Document) {
    document.local_endpoint::<routes::Empty, stacks::Conflict>(
        "post",
        "/local/repos/{repo}/stacks/conflict",
        "local_stacks_conflict",
    );
    document.body::<stacks::HunkRequest>("/v1/local/repos/{repo}/stacks/conflict", "post");
    document.local_endpoint::<routes::Empty, stacks::HunkResponse>(
        "post",
        "/local/repos/{repo}/stacks/hunks",
        "local_stacks_hunks",
    );
    document.body::<stacks::HunkRequest>("/v1/local/repos/{repo}/stacks/hunks", "post");
    document.local_endpoint::<routes::Empty, stacks::Inspection>(
        "post",
        "/local/repos/{repo}/stacks/inspect",
        "local_stacks_inspect",
    );
    document.body::<stacks::InspectionRequest>("/v1/local/repos/{repo}/stacks/inspect", "post");
    document.local_endpoint::<routes::Empty, stacks::State>(
        "post",
        "/local/repos/{repo}/stacks",
        "local_stacks_execute",
    );
    document.body::<stacks::Request>("/v1/local/repos/{repo}/stacks", "post");
    document.local_endpoint::<routes::Empty, stacks::Preview>(
        "post",
        "/local/repos/{repo}/stacks/preview",
        "local_stacks_preview",
    );
    document.body::<stacks::Request>("/v1/local/repos/{repo}/stacks/preview", "post");
}
#[cfg(test)]
mod tests;
