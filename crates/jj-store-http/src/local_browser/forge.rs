//! Local forge effects share the ordinary JJ provider and admission boundary.
use super::*;
use jj_store_service::forge as f;
use local::Access as _;
pub(super) fn router() -> Router<Arc<State>> {
    Router::new()
        .route("/local/repos/{repo}/forge", post(execute))
        .route("/local/integrations", get(inventory).post(account_command))
}
async fn inventory(
    ExtractState(state): ExtractState<Arc<State>>,
    args: routes::Input<routes::Empty>,
) -> Result<Response> {
    routes::input(args)?;
    admit(&state)?;
    let value = blocking(&state, || {
        f::accounts::native()
            .and_then(|m| m.inventory())
            .map_err(map)
    })
    .await?;
    state.json(&value)
}
async fn account_command(
    ExtractState(state): ExtractState<Arc<State>>,
    args: routes::Input<routes::Empty>,
    body: Body<f::accounts::Command>,
) -> Result<Response> {
    routes::input(args)?;
    admit(&state)?;
    let command = decode_body(body)?;
    let runtime = tokio::runtime::Handle::current();
    let value = blocking(&state, move || {
        runtime
            .block_on(async {
                f::accounts::native()?
                    .execute(command, f::accounts::now()?)
                    .await
            })
            .map_err(map)
    })
    .await?;
    state.json(&value)
}
async fn execute(
    ExtractState(state): ExtractState<Arc<State>>,
    Path(repo): Path<m::RepositoryId>,
    args: routes::Input<routes::Empty>,
    body: Body<f::Request>,
) -> Result<Response> {
    routes::input(args)?;
    let request = decode_body(body)?;
    let access = bound_folder(&state, repo).await?;
    access.authorize().await?;
    let root = std::path::PathBuf::from(&access.folder.path);
    let runtime = tokio::runtime::Handle::current();
    let value = blocking(&state, move || {
        runtime
            .block_on(management::stacks::forge::handle(access, root, request))
            .map_err(map)
    })
    .await?;
    state.json(&value)
}
fn map(error: f::Error) -> Error {
    let (status, code) = match error {
        f::Error::Invalid => (StatusCode::BAD_REQUEST, "invalid_forge_request"),
        f::Error::Denied => (StatusCode::FORBIDDEN, "forge_access_denied"),
        f::Error::Stale => (StatusCode::CONFLICT, "stale_forge_request"),
        f::Error::Unsupported => (
            StatusCode::UNPROCESSABLE_ENTITY,
            "forge_capability_unavailable",
        ),
        f::Error::RateLimited => (StatusCode::TOO_MANY_REQUESTS, "forge_rate_limited"),
        f::Error::Unknown => (StatusCode::CONFLICT, "forge_outcome_unknown"),
        f::Error::State => (StatusCode::SERVICE_UNAVAILABLE, "forge_state_busy"),
        f::Error::Limit => (StatusCode::PAYLOAD_TOO_LARGE, "forge_resource_limit"),
        f::Error::Rejected(_) => (StatusCode::UNPROCESSABLE_ENTITY, "forge_rejected"),
    };
    Error::new(status, code)
}
pub(super) fn document(d: &mut crate::contract::Document) {
    for method in ["get", "post"] {
        d.local_endpoint::<routes::Empty, f::accounts::Inventory>(
            method,
            "/local/integrations",
            if method == "get" {
                "local_integrations"
            } else {
                "local_integrations_command"
            },
        );
    }
    d.body::<f::accounts::Command>("/v1/local/integrations", "post");
    d.local_endpoint::<routes::Empty, f::Response>(
        "post",
        "/local/repos/{repo}/forge",
        "local_forge",
    );
    d.body::<f::Request>("/v1/local/repos/{repo}/forge", "post");
}
