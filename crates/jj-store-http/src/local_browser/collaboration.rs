//! Inbox collaboration keeps exact companion pins and the existing CLI device actor.
use super::*;
use jj_store_service::{collab_model as model, collab_pages as pages, local_repository as repo};
#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "LocalInboxQuery")]
struct Query {
    metadata_commit: Option<repo::MetadataCommit>,
    query: pages::Query,
    cursor: Option<String>,
}
#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "LocalInboxDetailQuery")]
struct Detail {
    metadata_commit: Option<repo::MetadataCommit>,
    thread_id: model::ThreadId,
}
#[derive(Serialize, JsonSchema)]
#[schemars(rename = "LocalInboxPageResponse")]
struct Page {
    data: repo::LocalInboxCollabPage,
    next_cursor: Option<String>,
}
macro_rules! endpoints {
    ($register:ident) => { $register! {
        (get, "/local/repos/{repo}/collaboration/snapshot", snapshot, "local_collab_snapshot", routes::Empty, repo::LocalInboxSelection),
        (post, "/local/repos/{repo}/collaboration/query", query, "local_collab_query", Query, Page),
        (post, "/local/repos/{repo}/collaboration/thread", detail, "local_collab_detail", Detail, repo::LocalInboxThreadDetail),
        (post, "/local/repos/{repo}/collaboration/commands", apply, "local_collab_apply", repo::LocalCollabRequest, repo::LocalCollabOutcome),
        (get, "/local/repos/{repo}/collaboration/commands/{request_id}", status, "local_collab_status", routes::Empty, repo::LocalCollabOutcome),
        (get, "/local/repos/{repo}/collaboration/assets/{asset}", asset, "local_collab_asset", routes::Empty, model::ImageAssetV1)
    } };
}
pub(super) fn router() -> Router<Arc<State>> {
    macro_rules! register { ($(($method:ident,$path:literal,$handler:ident,$id:literal,$body:ty,$response:ty)),*) => { Router::new()$(.route($path,$method($handler)))* }; }
    endpoints!(register)
}
async fn snapshot(
    ExtractState(state): ExtractState<Arc<State>>,
    Path(repository): Path<m::RepositoryId>,
    args: routes::Input<routes::Empty>,
) -> Result<Response> {
    routes::input(args)?;
    let generation = binding(&state, repository).await?.generation();
    let manager = manager(&state)?;
    let runtime = tokio::runtime::Handle::current();
    let data = blocking(&state, move || {
        runtime.block_on(async {
            manager
                .open(repository)
                .map_err(map)?
                .inbox_selection()
                .await
                .map_err(Error::from)
        })
    })
    .await?;
    work::check(&state, repository, generation).await?;
    state.json(&data)
}
async fn query(
    ExtractState(state): ExtractState<Arc<State>>,
    Path(repository): Path<m::RepositoryId>,
    args: routes::Input<routes::Empty>,
    body: Body<Query>,
) -> Result<Response> {
    routes::input(args)?;
    let mut request = decode_body(body)?;
    let generation = binding(&state, repository).await?.generation();
    let raw = request.cursor.take();
    let digest = work::digest("local-inbox-query", &request)?;
    let after = work::resume(&state, raw, repository, generation, &digest)?;
    let manager = manager(&state)?;
    let runtime = tokio::runtime::Handle::current();
    let mut data = blocking(&state, move || {
        runtime.block_on(async {
            manager
                .open(repository)
                .map_err(map)?
                .inbox_page(request.metadata_commit, &request.query, after)
                .await
                .map_err(Error::from)
        })
    })
    .await?;
    work::check(&state, repository, generation).await?;
    let next_cursor = work::continuation(
        &state,
        data.next_after.take(),
        repository,
        generation,
        digest,
    )?;
    state.json(&Page { data, next_cursor })
}
async fn detail(
    ExtractState(state): ExtractState<Arc<State>>,
    Path(repository): Path<m::RepositoryId>,
    args: routes::Input<routes::Empty>,
    body: Body<Detail>,
) -> Result<Response> {
    routes::input(args)?;
    let request = decode_body(body)?;
    let generation = binding(&state, repository).await?.generation();
    let manager = manager(&state)?;
    let runtime = tokio::runtime::Handle::current();
    let data = blocking(&state, move || {
        runtime.block_on(async {
            manager
                .open(repository)
                .map_err(map)?
                .inbox_thread(request.metadata_commit, request.thread_id)
                .await
                .map_err(Error::from)
        })
    })
    .await?;
    work::check(&state, repository, generation).await?;
    state.json(&data)
}
async fn apply(
    ExtractState(state): ExtractState<Arc<State>>,
    Path(repository): Path<m::RepositoryId>,
    args: routes::Input<routes::Empty>,
    axum::Extension(pending): axum::Extension<mutations::Pending>,
    body: Body<repo::LocalCollabRequest>,
) -> Result<Response> {
    routes::input(args)?;
    let request = decode_body(body)?;
    if matches!(
        &request.command,
        model::CollabCommandV1::Thread {
            intent: model::ThreadCommandV1::SetCodebase { .. }
                | model::ThreadCommandV1::CreateIntentional { .. }
                | model::ThreadCommandV1::CreateConversation { .. }
        }
    ) {
        return Err(Error::denied());
    }
    let _ = binding(&state, repository).await?;
    let manager = manager(&state)?;
    let runtime = tokio::runtime::Handle::current();
    pending.set(request.request_id);
    let data = blocking(&state, move || {
        runtime.block_on(async {
            manager
                .open(repository)
                .map_err(map)?
                .apply_collab_command(&request)
                .await
                .map_err(Error::from)
        })
    })
    .await?;
    Ok(state
        .json(&data)
        .unwrap_or_else(|_| pending.response().unwrap()))
}
async fn status(
    ExtractState(state): ExtractState<Arc<State>>,
    Path((repository, request)): Path<(m::RepositoryId, m::RequestId)>,
    args: routes::Input<routes::Empty>,
    axum::Extension(pending): axum::Extension<mutations::Pending>,
) -> Result<Response> {
    routes::input(args)?;
    let _ = binding(&state, repository).await?;
    let manager = manager(&state)?;
    pending.set(request);
    let runtime = tokio::runtime::Handle::current();
    let data = blocking(&state, move || {
        runtime.block_on(async {
            manager
                .open(repository)
                .map_err(map)?
                .collab_store_status(request)
                .await
                .map_err(Error::from)
        })
    })
    .await?;
    Ok(state
        .json(&data)
        .unwrap_or_else(|_| pending.response().unwrap()))
}
async fn asset(
    ExtractState(state): ExtractState<Arc<State>>,
    Path((repository, asset)): Path<(m::RepositoryId, model::CollabObjectId)>,
    args: routes::Input<routes::Empty>,
) -> Result<Response> {
    routes::input(args)?;
    let generation = binding(&state, repository).await?.generation();
    let manager = manager(&state)?;
    let runtime = tokio::runtime::Handle::current();
    let data = blocking(&state, move || {
        runtime.block_on(async {
            manager
                .open(repository)
                .map_err(map)?
                .inbox_asset(asset)
                .await
                .map_err(Error::from)
        })
    })
    .await?;
    work::check(&state, repository, generation).await?;
    state.json(&data)
}
pub(super) fn document(document: &mut crate::contract::Document) {
    macro_rules! register { ($(($method:ident,$path:literal,$handler:ident,$id:literal,$body:ty,$response:ty)),*) => { $(document.local_endpoint::<routes::Empty,$response>(stringify!($method),$path,$id); if stringify!($method)=="post" { document.body::<$body>(&format!("/v1{}",$path),"post"); })* }; }
    endpoints!(register);
}
