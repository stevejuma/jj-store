//! Bounded service JSON plus an unambiguous binary carrier for retained rasters.
use crate::wire;
use jj_store_service::{
    collab_model::*, collab_mutations as commands, collab_pages as pages, collab_reads::*,
};
use serde::{Serialize, de::DeserializeOwned};
use tonic::Status;
const MAX_JSON: usize = 4 * 1024 * 1024;
#[cfg(test)]
#[path = "collab/codec_tests.rs"]
mod tests;
pub(crate) fn encode(value: &impl Serialize) -> std::result::Result<Vec<u8>, Status> {
    let bytes =
        serde_json::to_vec(value).map_err(|_| Status::internal("collaboration encoding"))?;
    if bytes.len() > MAX_JSON {
        return Err(Status::resource_exhausted("collaboration JSON limit"));
    }
    Ok(bytes)
}
pub(crate) fn decode<T: DeserializeOwned>(bytes: &[u8]) -> std::result::Result<T, Status> {
    if bytes.is_empty() || bytes.len() > MAX_JSON {
        return Err(Status::invalid_argument("collaboration JSON limit"));
    }
    serde_json::from_slice(bytes).map_err(|_| Status::invalid_argument("collaboration JSON"))
}
pub(crate) fn reply(value: &impl Serialize) -> std::result::Result<wire::JsonReply, Status> {
    Ok(wire::JsonReply {
        json: encode(value)?,
    })
}
pub(crate) fn response<T: DeserializeOwned>(
    value: wire::JsonReply,
) -> std::result::Result<T, Status> {
    decode(&value.json).map_err(|_| Status::data_loss("collaboration response"))
}
pub(crate) fn command_wire(
    repository: jj_store_model::RepositoryId,
    request: &commands::Request,
) -> std::result::Result<wire::CollabCommandRequest, Status> {
    let mut request = request.clone();
    let image = match &mut request.command {
        CollabCommandV1::Emoji {
            intent: CustomEmojiCommandV1::Define { bytes, .. },
        } => std::mem::take(bytes),
        _ => Vec::new(),
    };
    if image.len() > MAX_IMAGE_BYTES {
        return Err(Status::resource_exhausted("collaboration raster limit"));
    }
    Ok(wire::CollabCommandRequest {
        repository: repository.as_bytes().to_vec(),
        request_json: encode(&request)?,
        image,
    })
}
pub(crate) fn command(
    value: &wire::CollabCommandRequest,
) -> std::result::Result<commands::Request, Status> {
    if value.image.len() > MAX_IMAGE_BYTES {
        return Err(Status::resource_exhausted("collaboration raster limit"));
    }
    let mut request: commands::Request = decode(&value.request_json)?;
    match &mut request.command {
        CollabCommandV1::Emoji {
            intent: CustomEmojiCommandV1::Define { bytes, .. },
        } => {
            if !bytes.is_empty() {
                return Err(Status::invalid_argument(
                    "inline collaboration raster bytes",
                ));
            }
            *bytes = value.image.clone();
        }
        _ if !value.image.is_empty() => {
            return Err(Status::invalid_argument(
                "unexpected collaboration raster bytes",
            ));
        }
        _ => {}
    }
    Ok(request)
}
pub(crate) fn asset_reply(
    pin: &PinnedCollabRead,
    mut asset: ImageAssetV1,
) -> std::result::Result<wire::CollabAssetReply, Status> {
    if asset.bytes.len() > MAX_IMAGE_BYTES {
        return Err(Status::resource_exhausted("collaboration raster limit"));
    }
    let image = std::mem::take(&mut asset.bytes);
    Ok(wire::CollabAssetReply {
        metadata_json: encode(&asset)?,
        image,
        snapshot_json: encode(pin.snapshot())?,
    })
}
pub(crate) fn asset_response(
    reference: CollabObjectRef,
    value: wire::CollabAssetReply,
) -> std::result::Result<ImageDetailV1, Status> {
    if value.image.is_empty()
        || value.image.len() > MAX_IMAGE_BYTES
        || value.metadata_json.len() > 4096
    {
        return Err(Status::data_loss("collaboration raster response limit"));
    }
    let mut asset: ImageAssetV1 = decode(&value.metadata_json)
        .map_err(|_| Status::data_loss("collaboration raster metadata"))?;
    if !asset.bytes.is_empty() {
        return Err(Status::data_loss("ambiguous collaboration raster response"));
    }
    asset.bytes = value.image;
    asset
        .validate()
        .map_err(|_| Status::data_loss("collaboration raster integrity"))?;
    if CollabObjectV1::ImageAsset(asset.clone())
        .reference()
        .map_err(|_| Status::data_loss("collaboration raster identity"))?
        != reference
    {
        return Err(Status::data_loss("collaboration raster identity"));
    }
    let snapshot = decode(&value.snapshot_json)
        .map_err(|_| Status::data_loss("collaboration raster snapshot"))?;
    Ok(ImageDetailV1 {
        snapshot,
        reference,
        asset,
    })
}
pub(crate) fn read_error(error: pages::Error) -> Status {
    match error {
        pages::Error::Read(error) => command_error(error),
        pages::Error::InvalidCursor => Status::invalid_argument("invalid collaboration cursor"),
        pages::Error::CursorReset => Status::failed_precondition("collaboration cursor reset"),
        pages::Error::Busy | pages::Error::Limit => {
            Status::resource_exhausted("collaboration page limit")
        }
        pages::Error::Unavailable => Status::unavailable("collaboration pagination unavailable"),
    }
}
pub(crate) fn command_error(error: commands::Error) -> Status {
    match error {
        commands::Error::Denied => Status::permission_denied("collaboration access denied"),
        commands::Error::Stale => Status::failed_precondition("stale collaboration precondition"),
        commands::Error::IntentConflict => {
            Status::already_exists("collaboration request intent conflict")
        }
        commands::Error::Unsupported => {
            Status::failed_precondition("collaboration capability unsupported")
        }
        commands::Error::NotFound => Status::not_found("collaboration target not found"),
        commands::Error::Invalid => Status::invalid_argument("invalid collaboration request"),
        commands::Error::Corrupt => Status::data_loss("collaboration state integrity"),
        commands::Error::Limit => Status::resource_exhausted("collaboration resource limit"),
        commands::Error::Unavailable => Status::unavailable("collaboration authority unavailable"),
    }
}
