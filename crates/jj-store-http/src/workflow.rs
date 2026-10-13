//! Thin workflow transport; native service retains all semantic authority.
use crate::*;
use axum::{Extension, Json, extract::rejection::JsonRejection};
use jj_store_service::workflow as w;
fn service(state: &State) -> w::WorkflowService {
    w::WorkflowService {
        hosts: state
            .config
            .workflow_admission
            .clone()
            .unwrap_or_else(|| state.config.workflow_hosts.clone().into()),
    }
}
fn error(error: w::Error) -> Error {
    match error {
        w::Error::Denied => Error::denied(),
        w::Error::Limit => Error::limit(),
        w::Error::Invalid => Error::invalid(),
        w::Error::Stale => Error::new(StatusCode::CONFLICT, "stale_workflow"),
        w::Error::IntentConflict => Error::new(StatusCode::CONFLICT, "intent_conflict"),
        w::Error::NotFound => Error::new(StatusCode::NOT_FOUND, "workflow_not_found"),
        w::Error::Corrupt => Error::new(StatusCode::INTERNAL_SERVER_ERROR, "workflow_integrity"),
        w::Error::Unavailable => Error::unavailable(),
    }
}
pub(super) async fn page(
    ExtractState(state): ExtractState<Arc<State>>,
    context: Context,
    args: routes::Input<w::Query>,
) -> Result<Response> {
    state.json(
        &service(&state)
            .page(context.session, routes::input(args)?)
            .await
            .map_err(error)?,
    )
}
pub(super) async fn events(
    ExtractState(state): ExtractState<Arc<State>>,
    context: Context,
    args: routes::Input<w::EventQuery>,
) -> Result<Response> {
    state.json(
        &service(&state)
            .source_events(context.session, routes::input(args)?)
            .await
            .map_err(error)?,
    )
}
pub(super) async fn apply(
    ExtractState(state): ExtractState<Arc<State>>,
    mutations::WriteContext(context): mutations::WriteContext,
    args: routes::Input<routes::Empty>,
    Extension(pending): Extension<mutations::Pending>,
    body: std::result::Result<Json<w::Request>, JsonRejection>,
) -> Result<Response> {
    routes::input(args)?;
    let Json(request) = body.map_err(|e| {
        if e.status() == StatusCode::PAYLOAD_TOO_LARGE {
            Error::limit()
        } else {
            Error::invalid()
        }
    })?;
    pending.set(request.request_id);
    let result = service(&state)
        .apply(context.session, &request)
        .await
        .map_err(error)?;
    Ok(state
        .json(&result)
        .unwrap_or_else(|_| pending.response().unwrap()))
}
pub(super) async fn status(
    ExtractState(state): ExtractState<Arc<State>>,
    mutations::WriteContext(context): mutations::WriteContext,
    args: routes::Input<routes::Empty>,
    Extension(pending): Extension<mutations::Pending>,
) -> Result<Response> {
    routes::input(args)?;
    let request = context.id("request_id")?;
    pending.set(request);
    let result = service(&state)
        .status(context.session, request)
        .await
        .map_err(error)?;
    Ok(state
        .json(&result)
        .unwrap_or_else(|_| pending.response().unwrap()))
}
