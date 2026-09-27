//! Bounded native gRPC V1; no upstream JJ types or storage control records.
#![forbid(unsafe_code)]
pub mod administration;
pub mod client;
pub mod server;
pub mod wire {
    tonic::include_proto!("jj_store.native.v1");
}
use jj_store_model as m;
use tonic::Status;
pub const CHUNK_BYTES: usize = 64 * 1024;
pub const REQUEST_BYTES: usize = 128 * 1024;
pub const RESPONSE_BYTES: usize = 8 * 1024 * 1024;
pub const MAX_IDS: usize = 100_000;

fn object(id: Option<wire::ObjectId>) -> Result<m::ObjectKey, Status> {
    let id = id.ok_or_else(|| Status::invalid_argument("object ID required"))?;
    m::ObjectKey::from_parts(
        u8::try_from(id.kind).map_err(|_| Status::invalid_argument("object kind"))?,
        id.id
            .try_into()
            .map_err(|_| Status::invalid_argument("object ID length"))?,
    )
    .ok_or_else(|| Status::invalid_argument("object kind"))
}
fn wire_object(key: m::ObjectKey) -> wire::ObjectId {
    wire::ObjectId {
        kind: key.kind() as u32,
        id: key.as_bytes().to_vec(),
    }
}
fn service_error(error: jj_store_service::transport::Error) -> Status {
    use jj_store_service::transport::Error;
    match error {
        Error::Denied => Status::permission_denied("repository access denied"),
        Error::Invalid => Status::invalid_argument("invalid native request"),
        Error::NotFound => Status::not_found("native object not found"),
        Error::Limit => Status::resource_exhausted("native resource limit"),
        Error::Unknown => {
            Status::unavailable("publication outcome unknown; retry original transition")
        }
        Error::Conflict => Status::aborted("publication conflict"),
        Error::Unavailable => Status::unavailable("native service unavailable"),
    }
}
