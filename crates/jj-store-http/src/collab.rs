//! HTTP reads over the shared captured-root collaboration service.
pub(super) mod assets;
pub(super) mod commands;
use crate::*;
use jj_store_service::{
    collab_model::{
        CollabCommandV1, CollabObjectId, CollabObjectKind, CollabObjectRef, CustomEmojiCommandV1,
        CustomEmojiId, ImageMediaTypeV1, MAX_IMAGE_BYTES, ReactionCommandV1, ReactionTargetV1,
        ThreadCommandV1,
    },
    collab_pages as pages, collab_reads as reads,
};
use serde::Deserialize;

fn limit() -> u32 {
    50
}
#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct PageQuery {
    #[serde(default = "limit")]
    limit: u32,
    cursor: Option<String>,
}
#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct ReactionQuery {
    pub target: ReactionTargetV1,
    #[serde(default = "limit")]
    pub limit: u32,
    pub cursor: Option<String>,
}
async fn pin(session: Arc<NativeSession>) -> Result<reads::PinnedCollabRead> {
    Ok(
        reads::PinnedCollabRead::open(session, Arc::new(reads::RepositoryMemberReactionActors))
            .await?,
    )
}
pub(super) async fn threads(
    ExtractState(state): ExtractState<Arc<State>>,
    context: Context,
    args: routes::Input<PageQuery>,
) -> Result<Response> {
    let args = routes::input(args)?;
    state.json(
        &state
            .collab
            .page(
                context.session,
                pages::Query::Threads { limit: args.limit },
                args.cursor.as_deref(),
            )
            .await?,
    )
}
pub(super) async fn events(
    ExtractState(state): ExtractState<Arc<State>>,
    context: Context,
    args: routes::Input<PageQuery>,
) -> Result<Response> {
    let args = routes::input(args)?;
    let thread_id = context.id("thread_id")?;
    state.json(
        &state
            .collab
            .page(
                context.session,
                pages::Query::Events {
                    thread_id,
                    limit: args.limit,
                },
                args.cursor.as_deref(),
            )
            .await?,
    )
}
pub(super) async fn emojis(
    ExtractState(state): ExtractState<Arc<State>>,
    context: Context,
    args: routes::Input<PageQuery>,
) -> Result<Response> {
    let args = routes::input(args)?;
    state.json(
        &state
            .collab
            .page(
                context.session,
                pages::Query::Emojis { limit: args.limit },
                args.cursor.as_deref(),
            )
            .await?,
    )
}
pub(super) async fn reactions(
    ExtractState(state): ExtractState<Arc<State>>,
    context: Context,
    args: routes::Input<routes::Empty>,
    body: commands::Input<ReactionQuery>,
) -> Result<Response> {
    routes::input(args)?;
    let body = commands::input(body)?;
    state.json(
        &state
            .collab
            .page(
                context.session,
                pages::Query::Reactions {
                    target: body.target,
                    limit: body.limit,
                },
                body.cursor.as_deref(),
            )
            .await?,
    )
}
pub(super) async fn thread(
    ExtractState(state): ExtractState<Arc<State>>,
    context: Context,
    args: routes::Input<routes::Empty>,
) -> Result<Response> {
    routes::input(args)?;
    let id = context.id("thread_id")?;
    let pin = pin(context.session.clone()).await?;
    state.json(&reads::ThreadDetailV1 {
        snapshot: pin.snapshot().clone(),
        thread: pin.thread(context.session, id).await?,
    })
}
pub(super) async fn message(
    ExtractState(state): ExtractState<Arc<State>>,
    context: Context,
    args: routes::Input<routes::Empty>,
) -> Result<Response> {
    routes::input(args)?;
    let target = ReactionTargetV1::HumanMessage {
        thread_id: context.id("thread_id")?,
        message_id: context.id("message_id")?,
        revision: CollabObjectRef {
            kind: CollabObjectKind::HumanMessageRevision,
            id: context.id("message_revision")?,
        },
    };
    let pin = pin(context.session.clone()).await?;
    let message = pin.message(context.session, &target).await?;
    state.json(&reads::HumanMessageDetailV1 {
        snapshot: pin.snapshot().clone(),
        target,
        message,
    })
}
