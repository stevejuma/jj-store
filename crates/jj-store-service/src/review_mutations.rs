//! Receipt-backed typed ReviewStore commands for authenticated transports.
use crate::{auth::Permission, native::NativeSession};
use jj_store_model as m;
use jj_store_wal::{
    self as wal,
    publication::{
        PublishOutcome, ReviewCommandError, ReviewCommandRequest as WalRequest,
        ReviewCommandResultV1, ReviewCommandV1, Snapshot, decode_command_result,
        encode_command_result,
    },
};
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("review command is not authorized")]
    Denied,
    #[error("review command precondition is stale")]
    Stale,
    #[error("request identity was already used for different intent")]
    IntentConflict,
    #[error("review thread is resolved")]
    ThreadResolved,
    #[error("review thread is already open")]
    ThreadOpen,
    #[error("review command target was not found")]
    NotFound,
    #[error("invalid review command")]
    Invalid,
    #[error("review receipt failed integrity validation")]
    Corrupt,
    #[error("review command resource limit exceeded")]
    Limit,
    #[error("review command authority is unavailable")]
    Unavailable,
}
pub type Result<T> = std::result::Result<T, Error>;

impl From<ReviewCommandError> for Error {
    fn from(error: ReviewCommandError) -> Self {
        match error {
            ReviewCommandError::Publication(error) => error.into(),
            ReviewCommandError::Stale => Self::Stale,
            ReviewCommandError::ThreadResolved => Self::ThreadResolved,
            ReviewCommandError::ThreadOpen => Self::ThreadOpen,
            ReviewCommandError::NotFound(_) => Self::NotFound,
            ReviewCommandError::Invalid(_) => Self::Invalid,
        }
    }
}

impl From<wal::Error> for Error {
    fn from(error: wal::Error) -> Self {
        match error {
            wal::Error::Denied => Self::Denied,
            wal::Error::Stale => Self::Stale,
            wal::Error::IntentConflict => Self::IntentConflict,
            wal::Error::Limit(_) => Self::Limit,
            wal::Error::Corrupt(_) | wal::Error::Json(_) => Self::Corrupt,
            _ => Self::Unavailable,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, schemars::JsonSchema)]
#[schemars(rename = "ReviewCommandRequest")]
#[serde(deny_unknown_fields)]
pub struct Request {
    pub version: u32,
    pub request_id: m::RequestId,
    pub command: ReviewCommandV1,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "ReviewCommandReceipt")]
pub struct Receipt {
    pub version: u32,
    pub request_id: m::RequestId,
    pub publication_id: m::PublicationId,
    pub request_digest: m::IntentDigest,
    pub accepted: Snapshot,
    pub result: ReviewCommandResultV1,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "ReviewCommandOutcome")]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Outcome {
    Committed { receipt: Box<Receipt> },
    OutcomeUnknown { request_id: m::RequestId },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "ReviewCommandStatus")]
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
    let mut hash = Sha256::new_with_prefix(b"jj-store/review-publication/v1\0");
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
    let result = decode_command_result(&receipt.result).map_err(|_| Error::Corrupt)?;
    result.validate().map_err(|_| Error::Corrupt)?;
    if encode_command_result(&result).map_err(|_| Error::Corrupt)? != receipt.result {
        return Err(Error::Corrupt);
    }
    Ok(Receipt {
        version: 1,
        request_id,
        publication_id: receipt.publication_id,
        request_digest: receipt.intent,
        accepted: receipt.accepted,
        result,
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
pub struct ReviewCommandService;

impl ReviewCommandService {
    pub async fn apply(&self, session: Arc<NativeSession>, request: &Request) -> Result<Outcome> {
        if request.version != 1 {
            return Err(Error::Invalid);
        }
        let principal = session.principal(Permission::Write).await?.id();
        let publication_id = publication_id(session.repository, principal, request.request_id);
        let command = WalRequest::new(
            principal,
            request.request_id,
            publication_id,
            request.command.clone(),
        )?;
        let digest = command.intent_digest(session.repository)?;
        match session.coordinator.publish_review_command(&command).await? {
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
