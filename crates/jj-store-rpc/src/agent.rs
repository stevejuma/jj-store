//! Canonical AgentStore request codecs shared by native RPC peers.
use crate::wire;
use jj_store_model as m;
use jj_store_service::transport::{
    AgentDeltaRequest, NormalizerProfileV1, PublishOutcome, Receipt, SanitizedObservationBatchV1,
    Snapshot,
};
use serde::{Serialize, de::DeserializeOwned};
use std::collections::BTreeSet;
use tonic::Status;

const PROFILE_BYTES: usize = 4 * 1024;
const CAPTURE_BYTES: usize = 6 * 1024 * 1024;
const DELTA_BYTES: usize = 2 * 1024 * 1024;
const RESULT_BYTES: usize = 1024 * 1024;

pub(crate) struct PreparedCapture {
    pub repository: Vec<u8>,
    pub request_id: m::RequestId,
    pub publication_id: m::PublicationId,
    pub profile: NormalizerProfileV1,
    pub batch: SanitizedObservationBatchV1,
    pub result: Vec<u8>,
}

fn invalid() -> Status {
    Status::invalid_argument("invalid agent publication request")
}

fn encode<T: Serialize>(value: &T, limit: usize) -> Result<Vec<u8>, Status> {
    let bytes = serde_json::to_vec(value).map_err(|_| invalid())?;
    if bytes.len() > limit {
        return Err(Status::resource_exhausted("agent publication byte limit"));
    }
    Ok(bytes)
}

fn decode<T: Serialize + DeserializeOwned>(bytes: &[u8], limit: usize) -> Result<T, Status> {
    if bytes.is_empty() || bytes.len() > limit {
        return Err(Status::resource_exhausted("agent publication byte limit"));
    }
    let value = serde_json::from_slice(bytes).map_err(|_| invalid())?;
    if encode(&value, limit)? != bytes {
        return Err(invalid());
    }
    Ok(value)
}

pub(crate) fn prepare_request(
    repository: m::RepositoryId,
    request_id: m::RequestId,
    publication_id: m::PublicationId,
    profile: &NormalizerProfileV1,
    batch: &SanitizedObservationBatchV1,
    result: Vec<u8>,
) -> Result<wire::AgentCapturePrepareRequest, Status> {
    profile.validate().map_err(|_| invalid())?;
    batch.validate().map_err(|_| invalid())?;
    if result.len() > RESULT_BYTES {
        return Err(Status::resource_exhausted("agent result byte limit"));
    }
    Ok(wire::AgentCapturePrepareRequest {
        repository: repository.as_bytes().to_vec(),
        request_id: request_id.as_bytes().to_vec(),
        publication_id: publication_id.as_bytes().to_vec(),
        profile_json: encode(profile, PROFILE_BYTES)?,
        batch_json: encode(batch, CAPTURE_BYTES)?,
        result,
    })
}

pub(crate) fn prepared_capture(
    value: wire::AgentCapturePrepareRequest,
) -> Result<PreparedCapture, Status> {
    if value.result.len() > RESULT_BYTES {
        return Err(Status::resource_exhausted("agent result byte limit"));
    }
    let profile: NormalizerProfileV1 = decode(&value.profile_json, PROFILE_BYTES)?;
    let batch: SanitizedObservationBatchV1 = decode(&value.batch_json, CAPTURE_BYTES)?;
    profile.validate().map_err(|_| invalid())?;
    batch.validate().map_err(|_| invalid())?;
    if profile.integration != batch.integration {
        return Err(invalid());
    }
    Ok(PreparedCapture {
        repository: value.repository,
        request_id: m::RequestId::from_slice(&value.request_id).map_err(|_| invalid())?,
        publication_id: m::PublicationId::from_slice(&value.publication_id)
            .map_err(|_| invalid())?,
        profile,
        batch,
        result: value.result,
    })
}

pub(crate) fn wire_prepared(
    request: &AgentDeltaRequest,
) -> Result<wire::AgentCapturePrepared, Status> {
    Ok(wire::AgentCapturePrepared {
        request_json: encode(request, DELTA_BYTES)?,
    })
}

pub(crate) fn prepared(
    value: wire::AgentCapturePrepared,
    request_id: m::RequestId,
    publication_id: m::PublicationId,
    result: &[u8],
) -> Result<AgentDeltaRequest, Status> {
    let request = delta_request(&value.request_json)?;
    if request.thread_binding.is_some()
        || request.request_id != request_id
        || request.publication_id != publication_id
        || request.result != result
    {
        return Err(Status::data_loss("agent preparation identity mismatch"));
    }
    Ok(request)
}

pub(crate) fn publish_request(
    repository: m::RepositoryId,
    request: &AgentDeltaRequest,
) -> Result<wire::AgentDeltaPublishRequest, Status> {
    Ok(wire::AgentDeltaPublishRequest {
        repository: repository.as_bytes().to_vec(),
        request_json: encode(request, DELTA_BYTES)?,
    })
}

pub(crate) fn delta_request(bytes: &[u8]) -> Result<AgentDeltaRequest, Status> {
    let value: AgentDeltaRequest = decode(bytes, DELTA_BYTES)?;
    value.validate().map_err(|_| invalid())?;
    Ok(value)
}

pub(crate) fn wire_outcome(value: PublishOutcome) -> wire::AgentPublicationReply {
    use wire::agent_publication_reply::Outcome;
    let outcome = match value {
        PublishOutcome::Committed(receipt) => Outcome::Committed(wire_receipt(receipt)),
        PublishOutcome::OutcomeUnknown {
            request_id,
            publication_id,
        } => Outcome::Unknown(wire::AgentPublicationIdentity {
            request_id: request_id.as_bytes().to_vec(),
            publication_id: publication_id.as_bytes().to_vec(),
        }),
    };
    wire::AgentPublicationReply {
        outcome: Some(outcome),
    }
}

pub(crate) fn outcome(
    value: wire::AgentPublicationReply,
    request: &AgentDeltaRequest,
) -> Result<PublishOutcome, Status> {
    use wire::agent_publication_reply::Outcome;
    let outcome = match value.outcome.ok_or_else(invalid)? {
        Outcome::Committed(value) => PublishOutcome::Committed(receipt(value)?),
        Outcome::Unknown(value) => PublishOutcome::OutcomeUnknown {
            request_id: m::RequestId::from_slice(&value.request_id).map_err(|_| invalid())?,
            publication_id: m::PublicationId::from_slice(&value.publication_id)
                .map_err(|_| invalid())?,
        },
    };
    let (principal, request_id, publication_id, result) = match &outcome {
        PublishOutcome::Committed(value) => (
            Some(value.principal),
            value.request_id,
            value.publication_id,
            Some(value.result.as_slice()),
        ),
        PublishOutcome::OutcomeUnknown {
            request_id,
            publication_id,
        } => (None, *request_id, *publication_id, None),
    };
    if principal.is_some_and(|value| value != request.principal)
        || request_id != request.request_id
        || publication_id != request.publication_id
        || result.is_some_and(|value| value != request.result)
    {
        return Err(Status::data_loss("agent publication identity mismatch"));
    }
    Ok(outcome)
}

fn wire_receipt(value: Receipt) -> wire::AgentReceipt {
    wire::AgentReceipt {
        principal: value.principal.as_bytes().to_vec(),
        request_id: value.request_id.as_bytes().to_vec(),
        publication_id: value.publication_id.as_bytes().to_vec(),
        intent: value.intent.as_bytes().to_vec(),
        accepted: Some(wire_snapshot(value.accepted)),
        result: value.result,
    }
}

fn receipt(value: wire::AgentReceipt) -> Result<Receipt, Status> {
    Ok(Receipt {
        principal: m::PrincipalId::from_slice(&value.principal).map_err(|_| invalid())?,
        request_id: m::RequestId::from_slice(&value.request_id).map_err(|_| invalid())?,
        publication_id: m::PublicationId::from_slice(&value.publication_id)
            .map_err(|_| invalid())?,
        intent: m::IntentDigest::from_slice(&value.intent).map_err(|_| invalid())?,
        accepted: snapshot(value.accepted)?,
        result: value.result,
    })
}

fn wire_snapshot(value: Snapshot) -> wire::Snapshot {
    wire::Snapshot {
        epoch: value.epoch.as_bytes().to_vec(),
        position: value.position.0,
        heads: value
            .heads
            .into_iter()
            .map(|id| id.as_bytes().to_vec())
            .collect(),
        code_state_version: value.code_state_version,
    }
}

fn snapshot(value: Option<wire::Snapshot>) -> Result<Snapshot, Status> {
    let value = value.ok_or_else(invalid)?;
    if value.heads.is_empty() || value.heads.len() > 256 {
        return Err(invalid());
    }
    let heads: BTreeSet<_> = value
        .heads
        .iter()
        .map(|id| m::OperationId::from_slice(id).map_err(|_| invalid()))
        .collect::<Result<_, _>>()?;
    if heads.len() != value.heads.len() {
        return Err(invalid());
    }
    Ok(Snapshot {
        epoch: m::EpochId::from_slice(&value.epoch).map_err(|_| invalid())?,
        position: m::WalPosition(value.position),
        code_state_version: value.code_state_version,
        heads,
    })
}

#[cfg(test)]
#[path = "agent/binding_tests.rs"]
mod binding_tests;
