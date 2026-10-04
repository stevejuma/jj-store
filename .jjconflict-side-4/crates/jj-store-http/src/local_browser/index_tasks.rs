//! LocalTrusted preparation endpoints; tasks never publish repository state.
use super::*;
use jj_store_service::local_index_tasks::LocalIndexTask;

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "LocalIndexPrepare")]
struct Prepare {
    operation: m::OperationId,
}

macro_rules! endpoints {
    ($register:ident) => { $register! {
        (post, "/local/repos/{repo}/code-index/tasks", start_code, "local_code_index_prepare", Prepare),
        (get, "/local/repos/{repo}/code-index/tasks/{task}", status_code, "local_code_index_status", routes::Empty),
        (post, "/local/repos/{repo}/code-index/tasks/{task}/cancel", cancel_code, "local_code_index_cancel", routes::Empty),
        (post, "/local/repos/{repo}/index/tasks", start, "local_index_prepare", Prepare),
        (get, "/local/repos/{repo}/index/tasks/{task}", status, "local_index_status", routes::Empty),
        (post, "/local/repos/{repo}/index/tasks/{task}/cancel", cancel, "local_index_cancel", routes::Empty)
    } };
}
pub(super) fn router() -> Router<Arc<State>> {
    macro_rules! register {
        ($(($method:ident,$path:literal,$handler:ident,$id:literal,$body:ty)),*) => {
            Router::new()$(.route($path,$method($handler)))*
        };
    }
    endpoints!(register)
}
async fn start(
    ExtractState(state): ExtractState<Arc<State>>,
    Path(repo): Path<m::RepositoryId>,
    args: routes::Input<routes::Empty>,
    body: Body<Prepare>,
) -> Result<Response> {
    routes::input(args)?;
    let request = decode_body(body)?;
    let access = binding(&state, repo).await?;
    state.json(&state.local_tasks.start(access, request.operation).await?)
}
async fn status(
    ExtractState(state): ExtractState<Arc<State>>,
    Path((repo, task)): Path<(m::RepositoryId, String)>,
    args: routes::Input<routes::Empty>,
) -> Result<Response> {
    routes::input(args)?;
    let access = binding(&state, repo).await?;
    state.json(&state.local_tasks.status(access, &task).await?)
}
async fn cancel(
    ExtractState(state): ExtractState<Arc<State>>,
    Path((repo, task)): Path<(m::RepositoryId, String)>,
    args: routes::Input<routes::Empty>,
) -> Result<Response> {
    routes::input(args)?;
    let access = binding(&state, repo).await?;
    state.json(&state.local_tasks.cancel(access, &task).await?)
}
async fn start_code(
    ExtractState(state): ExtractState<Arc<State>>,
    Path(repo): Path<m::RepositoryId>,
    args: routes::Input<routes::Empty>,
    body: Body<Prepare>,
) -> Result<Response> {
    routes::input(args)?;
    let request = decode_body(body)?;
    let access = binding(&state, repo).await?;
    state.json(
        &state
            .code_indexes
            .start_local(access, request.operation)
            .await?
            .local()?,
    )
}
async fn status_code(
    ExtractState(state): ExtractState<Arc<State>>,
    Path((repo, task)): Path<(m::RepositoryId, String)>,
    args: routes::Input<routes::Empty>,
) -> Result<Response> {
    routes::input(args)?;
    let access = binding(&state, repo).await?;
    state.json(
        &state
            .code_indexes
            .inspect_local(access, &task, false)
            .await?,
    )
}
async fn cancel_code(
    ExtractState(state): ExtractState<Arc<State>>,
    Path((repo, task)): Path<(m::RepositoryId, String)>,
    args: routes::Input<routes::Empty>,
) -> Result<Response> {
    routes::input(args)?;
    let access = binding(&state, repo).await?;
    state.json(
        &state
            .code_indexes
            .inspect_local(access, &task, true)
            .await?,
    )
}
pub(super) fn document(document: &mut crate::contract::Document) {
    macro_rules! register {
        ($(($method:ident,$path:literal,$handler:ident,$id:literal,$body:ty)),*) => {
            $(document.local_endpoint::<routes::Empty,LocalIndexTask>(stringify!($method),$path,$id);
            if stringify!($method)=="post" && !$path.ends_with("/cancel") { document.body::<$body>(&format!("/v1{}",$path),"post"); })*
        };
    }
    endpoints!(register);
}
