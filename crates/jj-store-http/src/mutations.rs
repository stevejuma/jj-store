//! Thin authenticated transport over the shared durable native command service.
use crate::*;
use axum::{Extension, Json, extract::rejection::JsonRejection};
use jj_store_service::mutations as service;
use std::sync::OnceLock;

pub(crate) const MAX_REQUEST_BYTES: usize = 512 * 1024;

/// Request-local identity only, never a principal or an authorization grant.
/// The outer transport timeout uses this if a dispatched write may have committed.
#[derive(Clone, Default)]
pub(crate) struct Pending(Arc<OnceLock<m::RequestId>>);
impl Pending {
    pub(crate) fn response(&self) -> Option<Response> {
        self.0.get().map(|&request_id| {
            // Fixed small outcome envelope is independent of the larger result cap.
            Json(service::Outcome::OutcomeUnknown { request_id }).into_response()
        })
    }
}
pub(super) struct WriteContext(pub(super) Context);
impl FromRequestParts<Arc<State>> for WriteContext {
    type Rejection = Error;
    async fn from_request_parts(parts: &mut Parts, state: &Arc<State>) -> Result<Self> {
        let context = Context::from_request_parts(parts, state).await?;
        context
            .session
            .principal(Permission::Write)
            .await
            .map_err(reads::Error::from)?;
        Ok(Self(context))
    }
}
type Input = std::result::Result<Json<service::Request>, JsonRejection>;
fn input(value: Input) -> Result<service::Request> {
    value.map(|Json(value)| value).map_err(|error| {
        if error.status() == StatusCode::PAYLOAD_TOO_LARGE {
            Error::limit()
        } else {
            Error::invalid()
        }
    })
}
pub(super) async fn preview(
    ExtractState(state): ExtractState<Arc<State>>,
    WriteContext(context): WriteContext,
    args: routes::Input<routes::Empty>,
    body: Input,
) -> Result<Response> {
    routes::input(args)?;
    let request = input(body)?;
    let preview = state.commands.preview(context.session, &request).await?;
    state.json(&preview)
}
pub(super) async fn apply(
    ExtractState(state): ExtractState<Arc<State>>,
    WriteContext(context): WriteContext,
    args: routes::Input<routes::Empty>,
    Extension(pending): Extension<Pending>,
    body: Input,
) -> Result<Response> {
    routes::input(args)?;
    let request = input(body)?;
    let _ = pending.0.set(request.request_id);
    let result = state.commands.apply(context.session, &request).await?;
    Ok(state
        .json(&result)
        .unwrap_or_else(|_| pending.response().unwrap()))
}
pub(super) async fn status(
    ExtractState(state): ExtractState<Arc<State>>,
    WriteContext(context): WriteContext,
    args: routes::Input<routes::Empty>,
    Extension(pending): Extension<Pending>,
) -> Result<Response> {
    routes::input(args)?;
    let request_id = context.id("request_id")?;
    let _ = pending.0.set(request_id);
    let result = state.commands.status(context.session, request_id).await?;
    Ok(state
        .json(&result)
        .unwrap_or_else(|_| pending.response().unwrap()))
}
