//! Shared bounded JSON codec for receipt-backed native application mutations.
use crate::wire;
use jj_store_service::{commands, mutations};
use serde::{Serialize, de::DeserializeOwned};
use tonic::Status;

const MAX_JSON_BYTES: usize = 8 * 1024 * 1024;

pub(crate) fn reply(value: &impl Serialize) -> Result<wire::JsonReply, Status> {
    let json = serde_json::to_vec(value)
        .map_err(|_| Status::internal("application mutation JSON encoding"))?;
    if json.len() > MAX_JSON_BYTES {
        return Err(Status::resource_exhausted(
            "application mutation JSON response limit",
        ));
    }
    Ok(wire::JsonReply { json })
}

pub(crate) fn request<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, Status> {
    if bytes.is_empty() || bytes.len() > MAX_JSON_BYTES {
        return Err(Status::invalid_argument(
            "application mutation JSON request limit",
        ));
    }
    serde_json::from_slice(bytes)
        .map_err(|_| Status::invalid_argument("application mutation JSON request"))
}

pub(crate) fn response<T: DeserializeOwned>(reply: wire::JsonReply) -> Result<T, Status> {
    if reply.json.is_empty() || reply.json.len() > MAX_JSON_BYTES {
        return Err(Status::data_loss(
            "application mutation JSON response limit",
        ));
    }
    serde_json::from_slice(&reply.json)
        .map_err(|_| Status::data_loss("application mutation JSON response"))
}

pub(crate) fn error(error: mutations::Error) -> Status {
    use commands::Error as CommandError;
    use mutations::Error;
    match error {
        Error::Denied | Error::Protected => Status::permission_denied(error.to_string()),
        Error::Stale => Status::failed_precondition(error.to_string()),
        Error::IntentConflict => Status::already_exists(error.to_string()),
        Error::Limit => Status::resource_exhausted(error.to_string()),
        Error::Invalid => Status::invalid_argument(error.to_string()),
        Error::Corrupt => Status::data_loss(error.to_string()),
        Error::Unavailable => Status::unavailable(error.to_string()),
        Error::Preparation(
            CommandError::Stale
            | CommandError::MultipleHeads
            | CommandError::StaleBookmark
            | CommandError::StaleConflict,
        ) => Status::failed_precondition(error.to_string()),
        Error::Preparation(CommandError::Query(_)) => Status::unavailable(error.to_string()),
        Error::Preparation(_) => Status::invalid_argument(error.to_string()),
    }
}
