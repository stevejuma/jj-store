//! Receipt-backed typed WorkStore commands for authenticated transports.
use crate::{auth::Permission, native::NativeSession};
use jj_store_model as m;
use jj_store_wal::{
    self as wal,
    publication::{
        PublishOutcome, Snapshot, WorkCommandRequest as WalRequest, WorkCommandResultV1,
    },
};
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use std::sync::Arc;

use super::{Error, Result};
pub use jj_store_git::work_api::Request;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[schemars(rename = "WorkCommandReceipt")]
pub struct Receipt {
    pub version: u32,
    pub request_id: m::RequestId,
    pub publication_id: m::PublicationId,
    pub request_digest: m::IntentDigest,
    pub accepted: Snapshot,
    pub result: WorkCommandResultV1,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[schemars(rename = "WorkCommandOutcome")]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Outcome {
    Committed { receipt: Box<Receipt> },
    OutcomeUnknown { request_id: m::RequestId },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[schemars(rename = "WorkCommandStatus")]
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
    let mut hash = Sha256::new_with_prefix(b"jj-store/work-publication/v1\0");
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
    let result: WorkCommandResultV1 =
        serde_json::from_slice(&receipt.result).map_err(|_| Error::Corrupt)?;
    if result.project.is_none() && result.task.is_none() && result.comment.is_none() {
        return Err(Error::Corrupt);
    }
    if serde_json::to_vec(&result).map_err(|_| Error::Corrupt)? != receipt.result {
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
pub struct WorkCommandService;

impl WorkCommandService {
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
        let stores = session.collaboration_stores().await?;
        let adapted = jj_store_core::StoreRequest::new(request.request_id, request.command.clone())
            .with_publication_id(publication_id);
        let outcome = stores
            .work()
            .ok_or(jj_store_core::Error::Unsupported("WorkStore disabled"))?
            .execute(&adapted)
            .await?;
        match crate::native::publication_outcome(outcome, publication_id)? {
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
            let stores = session.collaboration_stores().await?;
            let outcome = stores
                .work()
                .ok_or(jj_store_core::Error::Unsupported("store disabled"))?
                .status(request_id)
                .await?;
            let receipt = match outcome {
                jj_store_core::StoreOutcome::Committed(value) => {
                    Some(crate::native::native_receipt(value)?)
                }
                jj_store_core::StoreOutcome::NotKnown { .. } => None,
                jj_store_core::StoreOutcome::Unknown { .. } => {
                    return Ok(Status::OutcomeUnknown { request_id });
                }
            };
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
