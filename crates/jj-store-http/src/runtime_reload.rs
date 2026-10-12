use crate::*;
use axum::{
    Json,
    extract::DefaultBodyLimit,
    routing::{get, post},
};
use jj_store_service::runtime_reload as r;
fn caller(state: &State, headers: &HeaderMap) -> Result<r::Caller> {
    match state.config.access {
        Access::LocalTrusted {
            principal,
            permission: Permission::Admin,
        } => Ok(r::Caller::Local(principal)),
        Access::Authenticated => Ok(r::Caller::Token(repositories::bearer(headers)?)),
        _ => Err(Error::denied()),
    }
}
fn failure(error: r::Error) -> Error {
    let (status, code) = match error {
        r::Error::Denied => (StatusCode::FORBIDDEN, "configuration_reload_denied"),
        r::Error::Invalid => (StatusCode::BAD_REQUEST, "invalid_configuration_reload"),
        r::Error::Stale => (StatusCode::CONFLICT, "configuration_reload_changed"),
        r::Error::Conflict => (StatusCode::CONFLICT, "configuration_reload_intent_conflict"),
        r::Error::Busy => (StatusCode::CONFLICT, "configuration_reload_busy"),
        r::Error::Unavailable => (
            StatusCode::SERVICE_UNAVAILABLE,
            "configuration_reload_unavailable",
        ),
        r::Error::Unknown => (
            StatusCode::SERVICE_UNAVAILABLE,
            "configuration_reload_outcome_unknown",
        ),
    };
    Error::new(status, code)
}
pub(crate) fn router() -> Router<Arc<State>> {
    Router::new()
        .route("/configuration/reload", get(status).post(reload))
        .route("/configuration/reload/outcome", post(outcome))
        .layer(DefaultBodyLimit::max(4096))
}
async fn status(
    ExtractState(state): ExtractState<Arc<State>>,
    headers: HeaderMap,
    args: routes::Input<routes::Empty>,
) -> Result<Response> {
    routes::input(args)?;
    let caller = caller(&state, &headers)?;
    state.json(
        &state
            .config
            .runtime_reload
            .as_ref()
            .ok_or_else(Error::unavailable)?
            .status(caller)
            .await
            .map_err(failure)?,
    )
}
async fn reload(
    ExtractState(state): ExtractState<Arc<State>>,
    headers: HeaderMap,
    args: routes::Input<routes::Empty>,
    Json(command): Json<r::Command>,
) -> Result<Response> {
    routes::input(args)?;
    let caller = caller(&state, &headers)?;
    state.json(
        &state
            .config
            .runtime_reload
            .as_ref()
            .ok_or_else(Error::unavailable)?
            .reload(caller, command)
            .await
            .map_err(failure)?,
    )
}
async fn outcome(
    ExtractState(state): ExtractState<Arc<State>>,
    headers: HeaderMap,
    args: routes::Input<routes::Empty>,
    Json(command): Json<r::Command>,
) -> Result<Response> {
    routes::input(args)?;
    let caller = caller(&state, &headers)?;
    state.json(
        &state
            .config
            .runtime_reload
            .as_ref()
            .ok_or_else(Error::unavailable)?
            .outcome(caller, command)
            .await
            .map_err(failure)?,
    )
}
pub(crate) fn document(document: &mut crate::contract::Document) {
    document.endpoint::<routes::Empty, r::Status>(
        "get",
        "/configuration/reload",
        "runtime_reload_status",
        crate::contract::Reply::Json(200),
    );
    for (path, id) in [
        ("/configuration/reload", "runtime_reload"),
        ("/configuration/reload/outcome", "runtime_reload_outcome"),
    ] {
        document.endpoint::<routes::Empty, r::Outcome>(
            "post",
            path,
            id,
            crate::contract::Reply::Json(200),
        );
        document.body::<r::Command>(&format!("/v1{path}"), "post");
    }
}
