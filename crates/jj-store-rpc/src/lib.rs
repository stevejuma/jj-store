//! Bounded native gRPC V1; no upstream JJ types or storage control records.
#![forbid(unsafe_code)]
pub mod administration;
pub mod agent;
mod application;
pub mod client;
mod collab;
mod inventory;
pub mod review;
pub mod server;
mod work;
pub mod wire {
    tonic::include_proto!("jj_store.native.v1");
}
use jj_store_model as m;
use jj_store_service::transport::{RecordKind, RecordRef};
use tonic::Status;
pub const CHUNK_BYTES: usize = 64 * 1024;
pub const REQUEST_BYTES: usize = 8 * 1024 * 1024;
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
fn record(id: Option<wire::ControlRecordId>) -> Result<RecordRef, Status> {
    let id = id.ok_or_else(|| Status::invalid_argument("control record ID required"))?;
    let kind = match id.kind {
        1 => RecordKind::Catalog,
        2 => RecordKind::Inventory,
        7 => RecordKind::AgentRoot,
        8 => RecordKind::AgentObject,
        9 => RecordKind::ReviewRoot,
        10 => RecordKind::ReviewObject,
        13 => RecordKind::GitRoot,
        14 => RecordKind::GitObject,
        _ => return Err(Status::invalid_argument("control record kind")),
    };
    let digest = m::RecordDigest::from_slice(&id.digest)
        .map_err(|_| Status::invalid_argument("control record digest"))?;
    Ok(RecordRef {
        kind,
        digest,
        attempt: None,
    })
}
fn wire_record(reference: RecordRef) -> wire::ControlRecordId {
    let kind = match reference.kind {
        RecordKind::Catalog => 1,
        RecordKind::Inventory => 2,
        RecordKind::AgentRoot => 7,
        RecordKind::AgentObject => 8,
        RecordKind::ReviewRoot => 9,
        RecordKind::ReviewObject => 10,
        RecordKind::GitRoot => 13,
        RecordKind::GitObject => 14,
        _ => 0,
    };
    wire::ControlRecordId {
        kind,
        digest: reference.digest.as_bytes().to_vec(),
    }
}
fn service_error(error: jj_store_service::transport::Error) -> Status {
    use jj_store_service::transport::Error;
    match error {
        Error::Unsupported => {
            Status::failed_precondition("repository profile does not support origin replication")
        }
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
