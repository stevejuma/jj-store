//! Actor-free HTTP intents dispatched through the shared receipt service.
use super::*;
use crate::mutations::{Pending, WriteContext};
use axum::{Extension, Json, extract::rejection::JsonRejection};
use jj_store_service::collab_mutations as service;
pub(crate) const MAX_REQUEST_BYTES: usize = 10 * 1024 * 1024;
pub(super) type Input<T> = std::result::Result<Json<T>, JsonRejection>;
pub(super) fn input<T>(value: Input<T>) -> Result<T> {
    value.map(|Json(value)| value).map_err(|e| {
        if e.status() == StatusCode::PAYLOAD_TOO_LARGE {
            Error::limit()
        } else {
            Error::invalid()
        }
    })
}
#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct ThreadRequest {
    version: u32,
    request_id: m::RequestId,
    command: ThreadCommandV1,
}
#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct ReactionRequest {
    version: u32,
    request_id: m::RequestId,
    command: ReactionCommandV1,
}
#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct RetireRequest {
    version: u32,
    request_id: m::RequestId,
    emoji_id: CustomEmojiId,
    expected: CollabObjectId,
}
pub(super) async fn dispatch(
    state: &State,
    context: Context,
    pending: Pending,
    request: service::Request,
) -> Result<Response> {
    pending.set(request.request_id);
    let result = service::CollabCommandService
        .apply(context.session, &request)
        .await?;
    Ok(state
        .json(&result)
        .unwrap_or_else(|_| pending.response().unwrap()))
}
pub(crate) async fn thread_apply(
    ExtractState(state): ExtractState<Arc<State>>,
    WriteContext(context): WriteContext,
    args: routes::Input<routes::Empty>,
    Extension(pending): Extension<Pending>,
    body: Input<ThreadRequest>,
) -> Result<Response> {
    routes::input(args)?;
    let body = input(body)?;
    if matches!(
        body.command,
        ThreadCommandV1::SetCodebase { .. }
            | ThreadCommandV1::CreateIntentional { .. }
            | ThreadCommandV1::CreateConversation { .. }
    ) {
        return Err(Error::denied());
    }
    dispatch(
        &state,
        context,
        pending,
        service::Request {
            version: body.version,
            request_id: body.request_id,
            command: CollabCommandV1::Thread {
                intent: body.command,
            },
        },
    )
    .await
}
pub(crate) async fn reaction_apply(
    ExtractState(state): ExtractState<Arc<State>>,
    WriteContext(context): WriteContext,
    args: routes::Input<routes::Empty>,
    Extension(pending): Extension<Pending>,
    body: Input<ReactionRequest>,
) -> Result<Response> {
    routes::input(args)?;
    let body = input(body)?;
    dispatch(
        &state,
        context,
        pending,
        service::Request {
            version: body.version,
            request_id: body.request_id,
            command: CollabCommandV1::Reaction {
                intent: body.command,
            },
        },
    )
    .await
}
pub(crate) async fn retire(
    ExtractState(state): ExtractState<Arc<State>>,
    WriteContext(context): WriteContext,
    args: routes::Input<routes::Empty>,
    Extension(pending): Extension<Pending>,
    body: Input<RetireRequest>,
) -> Result<Response> {
    routes::input(args)?;
    let body = input(body)?;
    dispatch(
        &state,
        context,
        pending,
        service::Request {
            version: body.version,
            request_id: body.request_id,
            command: CollabCommandV1::Emoji {
                intent: CustomEmojiCommandV1::Retire {
                    emoji_id: body.emoji_id,
                    expected: CollabObjectRef {
                        kind: CollabObjectKind::CustomEmojiVersion,
                        id: body.expected,
                    },
                },
            },
        },
    )
    .await
}
pub(crate) async fn status(
    ExtractState(state): ExtractState<Arc<State>>,
    WriteContext(context): WriteContext,
    args: routes::Input<routes::Empty>,
    Extension(pending): Extension<Pending>,
) -> Result<Response> {
    routes::input(args)?;
    let id = context.id("request_id")?;
    pending.set(id);
    let result = service::CollabCommandService
        .status(context.session, id)
        .await?;
    Ok(state
        .json(&result)
        .unwrap_or_else(|_| pending.response().unwrap()))
}
