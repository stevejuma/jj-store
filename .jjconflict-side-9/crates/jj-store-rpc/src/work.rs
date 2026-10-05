//! Shared bounded JSON codec for the native WorkStore RPC surface.
use crate::wire;
use serde::{Serialize, de::DeserializeOwned};
use tonic::Status;

const MAX_JSON_BYTES: usize = 8 * 1024 * 1024;

pub(crate) fn reply(value: &impl Serialize) -> Result<wire::JsonReply, Status> {
    let json = serde_json::to_vec(value).map_err(|_| Status::internal("work JSON encoding"))?;
    if json.len() > MAX_JSON_BYTES {
        return Err(Status::resource_exhausted("work JSON response limit"));
    }
    Ok(wire::JsonReply { json })
}

pub(crate) fn request<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, Status> {
    if bytes.is_empty() || bytes.len() > MAX_JSON_BYTES {
        return Err(Status::invalid_argument("work JSON request limit"));
    }
    serde_json::from_slice(bytes).map_err(|_| Status::invalid_argument("work JSON request"))
}

pub(crate) fn response<T: DeserializeOwned>(reply: wire::JsonReply) -> Result<T, Status> {
    if reply.json.is_empty() || reply.json.len() > MAX_JSON_BYTES {
        return Err(Status::data_loss("work JSON response limit"));
    }
    serde_json::from_slice(&reply.json).map_err(|_| Status::data_loss("work JSON response"))
}

pub(crate) fn error(error: jj_store_service::work::Error) -> Status {
    use jj_store_service::work::Error as E;
    match error {
        E::Denied => Status::permission_denied("work access denied"),
        E::Stale => Status::failed_precondition("stale work command"),
        E::IntentConflict => Status::already_exists("work request intent conflict"),
        E::NotFound => Status::not_found("work target not found"),
        E::Invalid | E::InvalidCursor => Status::invalid_argument("invalid work request"),
        E::CursorReset => Status::failed_precondition("work cursor reset"),
        E::Limit | E::Busy => Status::resource_exhausted("work resource limit"),
        E::Corrupt => Status::data_loss("work integrity failure"),
        E::Unavailable => Status::unavailable("work authority unavailable"),
        E::ReplicationUnsupported => {
            Status::failed_precondition("WorkStore origin replication unsupported")
        }
    }
}
