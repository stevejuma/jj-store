//! Companion-pinned reviews use the same reconstruction and command policy as CLI.
use super::*;
use jj_store_service::{local_repository as repo, review_mutations as service};

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "LocalReviewPageQuery")]
struct PageQuery {
    metadata_commit: repo::MetadataCommit,
    limit: usize,
    cursor: Option<String>,
}
#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "LocalReviewDetailQuery")]
struct DetailQuery {
    metadata_commit: repo::MetadataCommit,
    thread_id: jj_store_service::collaboration::ReviewThreadIdentity,
}
#[derive(Serialize, JsonSchema)]
#[schemars(rename = "LocalReviewPageResponse")]
struct Page {
    data: repo::LocalReviewThreadPage,
    next_cursor: Option<String>,
}
macro_rules! endpoints {
    ($register:ident) => { $register! {
        (post, "/local/repos/{repo}/reviews/page", page, "local_review_page", PageQuery, Page),
        (post, "/local/repos/{repo}/reviews/detail", detail, "local_review_detail", DetailQuery, repo::LocalReviewThreadDetail),
        (post, "/local/repos/{repo}/reviews/commands", apply, "local_review_apply", service::Request, repo::LocalReviewOutcome),
        (get, "/local/repos/{repo}/reviews/commands/{request_id}", status, "local_review_status", routes::Empty, repo::LocalReviewOutcome)
    } };
}
pub(super) fn router() -> Router<Arc<State>> {
    macro_rules! register {
        ($(($method:ident,$path:literal,$handler:ident,$id:literal,$body:ty,$response:ty)),*) => {
            Router::new()$(.route($path,$method($handler)))*
        };
    }
    endpoints!(register)
}
async fn page(
    ExtractState(state): ExtractState<Arc<State>>,
    Path(repository): Path<m::RepositoryId>,
    args: routes::Input<routes::Empty>,
    body: Body<PageQuery>,
) -> Result<Response> {
    routes::input(args)?;
    let mut request = decode_body(body)?;
    let generation = binding(&state, repository).await?.generation();
    let raw = request.cursor.take();
    let digest = work::digest("local-review-page", &request)?;
    let after = work::resume(&state, raw, repository, generation, &digest)?
        .map(|raw| serde_json::from_str(&raw).map_err(|_| Error::invalid()))
        .transpose()?;
    let manager = manager(&state)?;
    let runtime = tokio::runtime::Handle::current();
    let data = blocking(&state, move || {
        runtime.block_on(async {
            manager
                .open(repository)
                .map_err(map)?
                .review_threads_at(Some(request.metadata_commit), request.limit, after)
                .await
                .map_err(Error::from)
        })
    })
    .await?;
    work::check(&state, repository, generation).await?;
    let raw = data
        .next_after
        .map(|key| serde_json::to_string(&key).map_err(|_| Error::unavailable()))
        .transpose()?;
    let next_cursor = work::continuation(&state, raw, repository, generation, digest)?;
    state.json(&Page { data, next_cursor })
}
async fn detail(
    ExtractState(state): ExtractState<Arc<State>>,
    Path(repository): Path<m::RepositoryId>,
    args: routes::Input<routes::Empty>,
    body: Body<DetailQuery>,
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
                .review_thread_at(Some(request.metadata_commit), request.thread_id)
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
    body: Body<service::Request>,
) -> Result<Response> {
    routes::input(args)?;
    let request = decode_body(body)?;
    let _ = binding(&state, repository).await?;
    let manager = manager(&state)?;
    pending.set(request.request_id);
    let runtime = tokio::runtime::Handle::current();
    let outcome = blocking(&state, move || {
        runtime.block_on(async {
            manager
                .open(repository)
                .map_err(map)?
                .apply_review_command(&repo::LocalReviewRequest {
                    version: request.version,
                    request_id: request.request_id,
                    command: repo::LocalReviewCommand::Apply {
                        intent: Box::new(request.command),
                    },
                })
                .await
                .map_err(Error::from)
        })
    })
    .await?;
    Ok(state
        .json(&outcome)
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
    let outcome = blocking(&state, move || {
        manager
            .open(repository)
            .map_err(map)?
            .review_command_status(request)
            .map_err(Error::from)
    })
    .await?;
    Ok(state
        .json(&outcome)
        .unwrap_or_else(|_| pending.response().unwrap()))
}
pub(super) fn document(document: &mut crate::contract::Document) {
    macro_rules! register {
        ($(($method:ident,$path:literal,$handler:ident,$id:literal,$body:ty,$response:ty)),*) => {
            $(document.local_endpoint::<routes::Empty,$response>(stringify!($method),$path,$id);
            if stringify!($method)=="post" { document.body::<$body>(&format!("/v1{}",$path),"post"); })*
        };
    }
    endpoints!(register);
}
