//! Shared bounded JSON codec for the native ReviewStore RPC surface.
use crate::wire;
use jj_store_service::{collaboration, review_mutations};
use serde::{Serialize, de::DeserializeOwned};
use tonic::Status;

const MAX_JSON_BYTES: usize = 8 * 1024 * 1024;

pub(crate) fn reply(value: &impl Serialize) -> Result<wire::JsonReply, Status> {
    let json = serde_json::to_vec(value).map_err(|_| Status::internal("review JSON encoding"))?;
    if json.len() > MAX_JSON_BYTES {
        return Err(Status::resource_exhausted("review JSON response limit"));
    }
    Ok(wire::JsonReply { json })
}

pub(crate) fn request<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, Status> {
    if bytes.is_empty() || bytes.len() > MAX_JSON_BYTES {
        return Err(Status::invalid_argument("review JSON request limit"));
    }
    serde_json::from_slice(bytes).map_err(|_| Status::invalid_argument("review JSON request"))
}

pub(crate) fn response<T: DeserializeOwned>(reply: wire::JsonReply) -> Result<T, Status> {
    if reply.json.is_empty() || reply.json.len() > MAX_JSON_BYTES {
        return Err(Status::data_loss("review JSON response limit"));
    }
    serde_json::from_slice(&reply.json).map_err(|_| Status::data_loss("review JSON response"))
}

pub(crate) fn read_error(error: collaboration::Error) -> Status {
    use collaboration::Error;
    match error {
        Error::Denied => Status::permission_denied("review read denied"),
        Error::InvalidCursor => Status::invalid_argument("invalid review cursor"),
        Error::NotFound => Status::not_found("review thread not found"),
        Error::CursorReset => Status::failed_precondition("review cursor reset"),
        Error::Limit | Error::Busy => Status::resource_exhausted("review read limit"),
        Error::Corrupt => Status::data_loss("review state corrupt"),
        Error::Unavailable => Status::unavailable("review state unavailable"),
    }
}

pub(crate) fn command_error(error: review_mutations::Error) -> Status {
    use review_mutations::Error;
    match error {
        Error::Denied => Status::permission_denied("review command denied"),
        Error::Stale => Status::failed_precondition("stale review command"),
        Error::IntentConflict => Status::already_exists("review request intent conflict"),
        Error::ThreadResolved => Status::failed_precondition("review thread resolved"),
        Error::ThreadOpen => Status::failed_precondition("review thread already open"),
        Error::NotFound => Status::not_found("review command target not found"),
        Error::Invalid => Status::invalid_argument("invalid review command"),
        Error::Corrupt => Status::data_loss("review receipt corrupt"),
        Error::Limit => Status::resource_exhausted("review command limit"),
        Error::Unavailable => Status::unavailable("review command unavailable"),
    }
}
