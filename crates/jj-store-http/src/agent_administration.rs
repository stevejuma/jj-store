//! Authenticated operator routes, never LocalTrusted runtime management.
use crate::*;
use axum::{Json, extract::DefaultBodyLimit, routing::get};
use jj_store_service::{agent_administration as a, server_settings as s};
fn failure(error: s::Error) -> Error {
    let (status, code) = match error {
        s::Error::Denied => (StatusCode::FORBIDDEN, "agent_administration_denied"),
        s::Error::Invalid => (StatusCode::BAD_REQUEST, "invalid_agent_configuration"),
        s::Error::Stale => (StatusCode::CONFLICT, "agent_configuration_changed"),
        s::Error::Busy => (StatusCode::CONFLICT, "agent_configuration_busy"),
        s::Error::Unknown => (
            StatusCode::SERVICE_UNAVAILABLE,
            "agent_configuration_outcome_unknown",
        ),
        s::Error::Unavailable => (
            StatusCode::SERVICE_UNAVAILABLE,
            "agent_administration_unavailable",
        ),
    };
    Error::new(status, code)
}
fn bearer(state: &State, headers: &HeaderMap) -> Result<BearerToken> {
    if !matches!(state.config.access, Access::Authenticated) {
        return Err(Error::denied());
    }
    crate::repositories::bearer(headers)
}
pub(super) fn router() -> Router<Arc<State>> {
    Router::new()
        .route("/operator/access", get(access))
        .route("/operator/agents", get(status).post(save))
        .layer(DefaultBodyLimit::max(128 * 1024))
}
async fn access(
    ExtractState(state): ExtractState<Arc<State>>,
    headers: HeaderMap,
    args: routes::Input<routes::Empty>,
) -> Result<Response> {
    routes::input(args)?;
    let token = bearer(&state, &headers)?;
    // Capability discovery is for an admitted installation user, not anonymous probing.
    state
        .config
        .directory
        .as_ref()
        .ok_or_else(Error::unavailable)?
        .principal(&jj_store_service::directory::Caller::Token(token.clone()))
        .await
        .map_err(|_| Error::denied())?;
    let policy = state
        .config
        .directory_operators
        .as_ref()
        .ok_or_else(Error::denied)?
        .current()
        .await
        .map_err(|_| Error::denied())?;
    let now =
        jj_store_service::auth::Clock::unix_seconds(&SystemClock).map_err(|_| Error::denied())?;
    state.json(&a::Access {
        agents: state.config.agent_administration.is_some()
            && policy
                .authorize(&token, Permission::ManageAgents, now)
                .is_ok(),
        identities: policy
            .authorize(&token, Permission::ManageIdentities, now)
            .is_ok(),
    })
}
async fn status(
    ExtractState(state): ExtractState<Arc<State>>,
    headers: HeaderMap,
    args: routes::Input<routes::Empty>,
) -> Result<Response> {
    routes::input(args)?;
    let token = bearer(&state, &headers)?;
    state.json(
        &state
            .config
            .agent_administration
            .as_ref()
            .ok_or_else(Error::unavailable)?
            .status(token)
            .await
            .map_err(failure)?,
    )
}
async fn save(
    ExtractState(state): ExtractState<Arc<State>>,
    headers: HeaderMap,
    args: routes::Input<routes::Empty>,
    Json(request): Json<a::Save>,
) -> Result<Response> {
    routes::input(args)?;
    let token = bearer(&state, &headers)?;
    state.json(
        &state
            .config
            .agent_administration
            .as_ref()
            .ok_or_else(Error::unavailable)?
            .save(token, request)
            .await
            .map_err(failure)?,
    )
}
pub(super) fn document(doc: &mut contract::Document) {
    use contract::Reply;
    doc.endpoint::<routes::Empty, a::Access>(
        "get",
        "/operator/access",
        "operator_access",
        Reply::Json(200),
    );
    doc.endpoint::<routes::Empty, a::Status>(
        "get",
        "/operator/agents",
        "operator_agents",
        Reply::Json(200),
    );
    doc.endpoint::<routes::Empty, a::Status>(
        "post",
        "/operator/agents",
        "operator_agents_save",
        Reply::Json(200),
    );
    doc.body::<a::Save>("/v1/operator/agents", "post");
}
