//! Authenticated streaming body adapter; storage and binding policy stay in service.
use crate::mutations::WriteContext;
use crate::*;
use futures::TryStreamExt as _;
use jj_store_service::uploads as service;
#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct Query {
    epoch: m::EpochId,
    position: u64,
    code_state_version: u64,
    operation: m::OperationId,
    commit: m::CommitId,
    path: String,
    fingerprint: m::ConflictFingerprint,
}
pub(super) async fn stage(
    ExtractState(state): ExtractState<Arc<State>>,
    WriteContext(context): WriteContext,
    args: routes::Input<Query>,
    request: axum::extract::Request,
) -> Result<Response> {
    let args = routes::input(args)?;
    let headers = request.headers();
    if headers.get_all(header::CONTENT_TYPE).iter().count() != 1
        || headers
            .get(header::CONTENT_TYPE)
            .is_none_or(|value| value != "application/octet-stream")
        || headers.contains_key(header::CONTENT_ENCODING)
    {
        return Err(Error::new(
            StatusCode::UNSUPPORTED_MEDIA_TYPE,
            "unsupported_media_type",
        ));
    }
    if headers.get_all(header::CONTENT_LENGTH).iter().count() > 1 {
        return Err(Error::invalid());
    }
    let declared = headers
        .get(header::CONTENT_LENGTH)
        .map(|v| {
            let v = v.to_str().map_err(|_| Error::invalid())?;
            if v.is_empty() || !v.bytes().all(|c| c.is_ascii_digit()) {
                return Err(Error::invalid());
            }
            v.parse::<u64>().map_err(|_| Error::invalid())
        })
        .transpose()?;
    if declared.is_some() && headers.contains_key(header::TRANSFER_ENCODING) {
        return Err(Error::invalid());
    }
    let binding = service::Binding {
        expected: service::Snapshot {
            epoch: args.epoch,
            position: m::WalPosition(args.position),
            code_state_version: args.code_state_version,
            heads: std::collections::BTreeSet::from([args.operation]),
        },
        commit: args.commit,
        path: args.path,
        fingerprint: args.fingerprint,
    };
    let stream = request
        .into_body()
        .into_data_stream()
        .map_err(std::io::Error::other);
    let upload = state
        .uploads
        .stage(
            context.session,
            binding,
            tokio_util::io::StreamReader::new(stream),
            declared,
        )
        .await?;
    state.json(&upload)
}
