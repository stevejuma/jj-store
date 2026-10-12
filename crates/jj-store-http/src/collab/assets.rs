//! Retained raster uploads and reads without JSON expansion or external URLs.
use super::*;
use axum::{Extension, body::to_bytes};
use jj_store_service::collab_mutations;
#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct DefineQuery {
    version: u32,
    request_id: m::RequestId,
    expected: Option<CollabObjectId>,
    shortcode: String,
    label: String,
}
pub(crate) async fn define(
    ExtractState(state): ExtractState<Arc<State>>,
    crate::mutations::WriteContext(context): crate::mutations::WriteContext,
    args: routes::Input<DefineQuery>,
    Extension(pending): Extension<crate::mutations::Pending>,
    body: Body,
) -> Result<Response> {
    let query = routes::input(args)?;
    let emoji_id = context.id("emoji_id")?;
    let bytes = to_bytes(body, MAX_IMAGE_BYTES)
        .await
        .map_err(|_| Error::limit())?
        .to_vec();
    commands::dispatch(
        &state,
        context,
        pending,
        collab_mutations::Request {
            version: query.version,
            request_id: query.request_id,
            command: CollabCommandV1::Emoji {
                intent: CustomEmojiCommandV1::Define {
                    emoji_id,
                    expected: query.expected.map(|id| CollabObjectRef {
                        kind: CollabObjectKind::CustomEmojiVersion,
                        id,
                    }),
                    shortcode: query.shortcode,
                    label: query.label,
                    bytes,
                },
            },
        },
    )
    .await
}
pub(crate) async fn image(
    ExtractState(state): ExtractState<Arc<State>>,
    context: Context,
    args: routes::Input<routes::Empty>,
) -> Result<Response> {
    routes::input(args)?;
    if context.headers.contains_key(header::RANGE) {
        return Err(Error::invalid());
    }
    let reference = CollabObjectRef {
        kind: CollabObjectKind::ImageAsset,
        id: context.id("asset")?,
    };
    let pin = state
        .collab
        .pin(context.session.clone(), reads::ReadScope::Assets)
        .await?;
    let asset = pin.image(context.session, reference).await?;
    let media = match asset.media_type {
        ImageMediaTypeV1::Png => "image/png",
        ImageMediaTypeV1::Jpeg => "image/jpeg",
        ImageMediaTypeV1::Gif => "image/gif",
        ImageMediaTypeV1::Webp => "image/webp",
    };
    let mut response = ([(header::CONTENT_TYPE, media)], asset.bytes).into_response();
    for (name, value) in [
        ("x-jj-store-repository", pin.snapshot().repo_id.to_string()),
        ("x-jj-store-epoch", pin.snapshot().epoch.to_string()),
        (
            "x-jj-store-position",
            pin.snapshot().published_seq.0.to_string(),
        ),
        (
            "x-jj-store-revision",
            pin.snapshot().manifest_digest.to_string(),
        ),
        ("x-jj-store-reaction-asset", reference.id.to_string()),
    ] {
        response
            .headers_mut()
            .insert(name, value.parse().map_err(|_| Error::unavailable())?);
    }
    Ok(response)
}
