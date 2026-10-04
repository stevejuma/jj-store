//! Receipt-backed typed Thread/Reaction commands for authenticated transports.
use crate::{auth::Permission, native::NativeSession};
use jj_collab::{CollabCommandResultV1, CollabCommandV1};
use jj_store_model as m;
use jj_store_wal::{
    self as wal,
    publication::{
        CollabCommandError, CollabCommandReceiptV1, CollabCommandRequestV1 as WalRequest,
        PublishOutcome, Snapshot,
    },
};
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("collaboration command is not authorized")]
    Denied,
    #[error("collaboration command precondition is stale")]
    Stale,
    #[error("request identity was already used for different intent")]
    IntentConflict,
    #[error("collaboration capability is unsupported")]
    Unsupported,
    #[error("collaboration command target was not found")]
    NotFound,
    #[error("invalid collaboration command")]
    Invalid,
    #[error("collaboration receipt failed integrity validation")]
    Corrupt,
    #[error("collaboration command resource limit exceeded")]
    Limit,
    #[error("collaboration command authority is unavailable")]
    Unavailable,
}
pub type Result<T> = std::result::Result<T, Error>;

impl From<CollabCommandError> for Error {
    fn from(error: CollabCommandError) -> Self {
        match error {
            CollabCommandError::Publication(error) => error.into(),
            CollabCommandError::Model(jj_collab::Error::Stale) => Self::Stale,
            CollabCommandError::Model(jj_collab::Error::TargetUnavailable) => Self::NotFound,
            CollabCommandError::Model(jj_collab::Error::ReadUnavailable) => Self::Unavailable,
            CollabCommandError::Model(jj_collab::Error::Limit) => Self::Limit,
            CollabCommandError::Model(_) => Self::Invalid,
        }
    }
}

impl From<wal::Error> for Error {
    fn from(error: wal::Error) -> Self {
        match error {
            wal::Error::Denied => Self::Denied,
            wal::Error::Stale => Self::Stale,
            wal::Error::IntentConflict => Self::IntentConflict,
            wal::Error::Unsupported(_) => Self::Unsupported,
            wal::Error::Limit(_) => Self::Limit,
            wal::Error::Corrupt(_) | wal::Error::Json(_) => Self::Corrupt,
            _ => Self::Unavailable,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[schemars(rename = "CollabCommandRequest")]
#[serde(deny_unknown_fields)]
pub struct Request {
    pub version: u32,
    pub request_id: m::RequestId,
    pub command: CollabCommandV1,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[schemars(rename = "CollabCommandReceipt")]
pub struct Receipt {
    pub version: u32,
    pub request_id: m::RequestId,
    pub publication_id: m::PublicationId,
    pub request_digest: m::IntentDigest,
    pub accepted: Snapshot,
    pub actor: m::PrincipalId,
    pub accepted_at_millis: i64,
    pub result: CollabCommandResultV1,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[schemars(rename = "CollabCommandOutcome")]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Outcome {
    Committed { receipt: Box<Receipt> },
    OutcomeUnknown { request_id: m::RequestId },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[schemars(rename = "CollabCommandStatus")]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Status {
    Committed { receipt: Box<Receipt> },
    NotKnown { request_id: m::RequestId },
    OutcomeUnknown { request_id: m::RequestId },
}

fn publication_id(
    repository: m::RepositoryId,
    principal: m::PrincipalId,
    request: m::RequestId,
) -> m::PublicationId {
    let mut hash = Sha256::new_with_prefix(b"jj-store/collab-publication/v1\0");
    hash.update(repository.as_bytes());
    hash.update(principal.as_bytes());
    hash.update(request.as_bytes());
    let mut id = [0; 16];
    id.copy_from_slice(&hash.finalize()[..16]);
    m::PublicationId::new(id)
}

fn decode(
    repository: m::RepositoryId,
    principal: m::PrincipalId,
    request_id: m::RequestId,
    expected_digest: Option<m::IntentDigest>,
    receipt: wal::publication::Receipt,
) -> Result<Receipt> {
    if receipt.principal != principal
        || receipt.request_id != request_id
        || receipt.publication_id != publication_id(repository, principal, request_id)
        || expected_digest.is_some_and(|digest| digest != receipt.intent)
    {
        return Err(Error::IntentConflict);
    }
    if receipt.result.len() > 1024 * 1024 {
        return Err(Error::Corrupt);
    }
    let result = CollabCommandReceiptV1::decode(&receipt.result).map_err(|_| Error::Corrupt)?;
    Ok(Receipt {
        version: 1,
        request_id,
        publication_id: receipt.publication_id,
        request_digest: receipt.intent,
        accepted: receipt.accepted,
        actor: principal,
        accepted_at_millis: result.accepted_at_millis,
        result: result.result,
    })
}

async fn reauthorize(
    session: &NativeSession,
    principal: m::PrincipalId,
    epoch: m::EpochId,
) -> Result<()> {
    if session.principal(Permission::Write).await?.id() != principal {
        return Err(Error::Denied);
    }
    if session.snapshot().await?.manifest().epoch != epoch {
        return Err(Error::Stale);
    }
    if session.principal(Permission::Write).await?.id() != principal {
        return Err(Error::Denied);
    }
    Ok(())
}

#[derive(Clone, Debug, Default)]
pub struct CollabCommandService;

impl CollabCommandService {
    pub async fn apply(&self, session: Arc<NativeSession>, request: &Request) -> Result<Outcome> {
        if request.version != 1 {
            return Err(Error::Invalid);
        }
        let timeout = session.limits.timeout;
        let started = tokio::time::Instant::now();
        let unknown = || Outcome::OutcomeUnknown {
            request_id: request.request_id,
        };
        match tokio::time::timeout(timeout, self.apply_inner(session, request)).await {
            Ok(Err(Error::Unavailable)) | Err(_) => Ok(unknown()),
            Ok(Err(error)) => Err(error),
            Ok(Ok(outcome)) => {
                if started.elapsed() >= timeout {
                    Ok(unknown())
                } else {
                    Ok(outcome)
                }
            }
        }
    }
    async fn apply_inner(&self, session: Arc<NativeSession>, request: &Request) -> Result<Outcome> {
        let principal = session.principal(Permission::Write).await?.id();
        let publication_id = publication_id(session.repository, principal, request.request_id);
        let command = WalRequest {
            schema_version: 1,
            principal,
            request_id: request.request_id,
            publication_id,
            command: request.command.clone(),
        };
        let digest = command.intent_digest(session.repository)?;
        match session.coordinator.publish_collab_command(&command).await? {
            PublishOutcome::Committed(receipt) => {
                session.events.published(session.repository);
                reauthorize(&session, principal, receipt.accepted.epoch).await?;
                let receipt = decode(
                    session.repository,
                    principal,
                    request.request_id,
                    Some(digest),
                    receipt,
                )?;
                Ok(Outcome::Committed {
                    receipt: Box::new(receipt),
                })
            }
            PublishOutcome::OutcomeUnknown { .. } => Ok(Outcome::OutcomeUnknown {
                request_id: request.request_id,
            }),
        }
    }

    pub async fn status(
        &self,
        session: Arc<NativeSession>,
        request_id: m::RequestId,
    ) -> Result<Status> {
        match tokio::time::timeout(session.limits.timeout, async {
            let principal = session.principal(Permission::Write).await?.id();
            let current = session.snapshot().await?;
            let receipt = session
                .coordinator
                .receipt_at(&current, principal, request_id)
                .await?;
            reauthorize(&session, principal, current.manifest().epoch).await?;
            Ok(match receipt {
                Some(receipt) => Status::Committed {
                    receipt: Box::new(decode(
                        session.repository,
                        principal,
                        request_id,
                        None,
                        receipt,
                    )?),
                },
                None => Status::NotKnown { request_id },
            })
        })
        .await
        {
            Ok(Err(Error::Unavailable)) | Err(_) => Ok(Status::OutcomeUnknown { request_id }),
            Ok(result) => result,
        }
    }
}
