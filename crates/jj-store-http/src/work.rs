//! Thin LocalTrusted/authenticated transport for durable WorkStore commands.
use crate::*;
use axum::{Extension, Json, extract::rejection::JsonRejection};
use jj_store_service::work as service;

pub(crate) const MAX_REQUEST_BYTES: usize = 2 * 1024 * 1024;

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

pub(super) async fn apply(
    ExtractState(state): ExtractState<Arc<State>>,
    mutations::WriteContext(context): mutations::WriteContext,
    args: routes::Input<routes::Empty>,
    Extension(pending): Extension<mutations::Pending>,
    body: Input,
) -> Result<Response> {
    routes::input(args)?;
    let request = input(body)?;
    pending.set(request.request_id);
    let result = service::WorkCommandService
        .apply(context.session, &request)
        .await?;
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
    let request_id = context.id("request_id")?;
    pending.set(request_id);
    let result = service::WorkCommandService
        .status(context.session, request_id)
        .await?;
    Ok(state
        .json(&result)
        .unwrap_or_else(|_| pending.response().unwrap()))
}

pub(super) async fn page(
    ExtractState(state): ExtractState<Arc<State>>,
    context: Context,
    args: routes::Input<service::WorkQuery>,
) -> Result<Response> {
    let query = routes::input(args)?;
    state.json(&state.work.page(context.session, query).await?)
}

pub(super) async fn browse(
    ExtractState(state): ExtractState<Arc<State>>,
    context: Context,
    args: routes::Input<service::WorkBrowseQuery>,
) -> Result<Response> {
    state.json(
        &state
            .work
            .browse(context.session, routes::input(args)?)
            .await?,
    )
}
